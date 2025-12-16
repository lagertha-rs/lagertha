use crate::jdwp::agent::command::{EventModifier, EventRequest};
use crate::jdwp::class_matcher::ClassPatternMatcher;
use crate::keys::{ClassId, MethodId, ThreadId};
use crossbeam::channel::{Receiver, Sender, unbounded};
use dashmap::DashMap;
use num_enum::TryFromPrimitive;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicU32, Ordering};
use std::sync::{Condvar, Mutex, RwLock};

pub mod agent;
mod class_matcher;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EventRequestId(pub i32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TypeTag {
    Class = 1,
    Interface = 2,
    Array = 3,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClassStatus {
    Verified = 1,
    Prepared = 2,
    Initialized = 4,
    Error = 8,
}

pub enum DebugEvent {
    VMStart,
    VMDeath,
    ClassPrepare {
        request_id: EventRequestId, // The requestID from the original EventRequest.Set
        thread_id: ThreadId,        // The thread loading the class (or null if VM thread)
        ref_type_tag: TypeTag,      // JDWP.TypeTag: 1=CLASS, 2=INTERFACE, 3=ARRAY
        type_id: ClassId,           // Your internal ID for this class
        signature: String,          // "Ljava/lang/String;" format
        status: ClassStatus, // JDWP.ClassStatus bitmask (VERIFIED=1, PREPARED=2, INITIALIZED=4, ERROR=8)
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum SuspendPolicy {
    None = 0,
    EventThread = 1,
    All = 2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, TryFromPrimitive)]
#[repr(u8)]
pub enum EventKind {
    SingleStep = 1,
    Breakpoint = 2,
    FramePop = 3,
    Exception = 4,
    UserDefined = 5,
    ThreadStart = 6,
    ThreadDeath = 7,
    ClassPrepare = 8,
    ClassUnload = 9,
    ClassLoad = 10,
    FieldAccess = 20,
    FieldModification = 21,
    ExceptionCatch = 30,
    MethodEntry = 40,
    MethodExit = 41,
    MethodExitWithReturnValue = 42,
    MonitorContendedEnter = 43,
    MonitorContendedEntered = 44,
    MonitorWait = 45,
    MonitorWaited = 46,
    VmStart = 90,
    VmDeath = 99,
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct BreakpointLocation {
    pub class_id: ClassId,
    pub method_id: MethodId,
    pub pc: u16,
}

pub struct DebugState {
    pub suspended: AtomicBool,
    pub suspend_lock: Mutex<()>,
    pub resume_signal: Condvar,

    pub breakpoints: DashMap<BreakpointLocation, u32>,
    pub suspend_policies: DashMap<EventRequestId, SuspendPolicy>,
    pub class_prepare_events: RwLock<ClassPatternMatcher>,

    pub event_tx: Sender<DebugEvent>,
    pub event_rx: Receiver<DebugEvent>,

    pub connected: AtomicBool,
    pub connected_lock: Mutex<()>,
    pub connected_signal: Condvar,

    pub next_event_id: AtomicI32,
}

impl DebugState {
    pub fn new() -> Self {
        let (event_tx, event_rx) = unbounded();
        Self {
            suspended: AtomicBool::new(false),
            suspend_lock: Mutex::new(()),
            resume_signal: Condvar::new(),
            breakpoints: DashMap::new(),
            suspend_policies: DashMap::new(),
            class_prepare_events: RwLock::new(ClassPatternMatcher::new()),
            event_tx,
            event_rx,
            connected: AtomicBool::new(false),
            connected_lock: Mutex::new(()),
            connected_signal: Condvar::new(),
            next_event_id: AtomicI32::new(1),
        }
    }

    #[inline]
    pub fn should_check(&self) -> bool {
        self.connected.load(Ordering::Relaxed)
    }

    pub fn resume_all(&self) {
        self.suspended.store(false, Ordering::SeqCst);
        self.resume_signal.notify_all();
    }

    pub fn suspend_all(&self) {
        self.suspended.store(true, Ordering::SeqCst);
    }

    pub fn wait_if_suspended(&self) {
        if !self.suspended.load(Ordering::Relaxed) {
            return;
        }

        println!("Thread suspending");

        let guard = self.suspend_lock.lock().unwrap();
        let mut guard = guard;
        while self.suspended.load(Ordering::SeqCst) {
            guard = self.resume_signal.wait(guard).unwrap();
        }
    }

    pub fn wait_until_connected(&self) {
        if self.connected.load(Ordering::Relaxed) {
            return;
        }

        println!("Waiting for debugger to connect");

        let guard = self.connected_lock.lock().unwrap();
        let mut guard = guard;
        while !self.connected.load(Ordering::SeqCst) {
            guard = self.connected_signal.wait(guard).unwrap();
        }
    }

    pub fn set_connected(&self, connected: bool) {
        self.connected.store(connected, Ordering::SeqCst);
        self.connected_signal.notify_all();
    }

    pub fn get_next_event_id(&self) -> EventRequestId {
        EventRequestId(self.next_event_id.fetch_add(1, Ordering::SeqCst))
    }

    pub fn matches_class_prepare(&self, class_name: &str) -> Option<Vec<EventRequestId>> {
        let matcher = self.class_prepare_events.read().unwrap();
        matcher.matches(class_name)
    }

    pub fn add_event_request(&self, event_request: EventRequest) {
        let event_kind = event_request.event_kind;
        let event_id = event_request.id;

        match event_request.event_kind {
            EventKind::ClassPrepare => {
                for modifier in &event_request.modifiers {
                    match modifier {
                        EventModifier::ClassMatch { class_pattern } => {
                            let mut matcher = self.class_prepare_events.write().unwrap();
                            matcher.add(class_pattern.clone(), event_id);
                        }
                        _ => unimplemented!(),
                    }
                }
            }
            EventKind::ClassUnload => {
                // TODO: I never unload classes, so this is unimplemented for now
            }
            EventKind::ThreadStart => {
                // TODO: I have only a single thread right now, so this is unimplemented for now
                // TODO: Should be done for the main thread? Check docs or hotspot behavior
            }
            EventKind::ThreadDeath => {
                // TODO: I have only a single thread right now, so this is unimplemented for now
            }
            unhandled => unimplemented!("Event kind {:?} not implemented", unhandled),
        }

        self.suspend_policies
            .insert(event_id, event_request.suspend_policy);
    }

    pub fn send_event(&self, event: DebugEvent) {
        let _ = self.event_tx.send(event);
    }
}
