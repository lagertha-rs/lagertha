use crate::jdwp::agent::command::EventRequest;
use crate::keys::{ClassId, MethodId};
use crossbeam::channel::{Receiver, Sender, unbounded};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Condvar, Mutex, RwLock};

pub mod agent;

pub enum DebugEvent {}

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

    pub breakpoints: RwLock<HashMap<BreakpointLocation, u32>>,

    pub event_tx: Sender<DebugEvent>,
    pub event_rx: Receiver<DebugEvent>,

    pub connected: AtomicBool,
    pub connected_lock: Mutex<()>,
    pub connected_signal: Condvar,

    pub next_event_id: AtomicU32,
    pub events: RwLock<HashMap<u32, EventRequest>>,
}

impl DebugState {
    pub fn new() -> Self {
        let (event_tx, event_rx) = unbounded();
        Self {
            suspended: AtomicBool::new(false),
            suspend_lock: Mutex::new(()),
            resume_signal: Condvar::new(),
            breakpoints: RwLock::new(HashMap::new()),
            event_tx,
            event_rx,
            connected: AtomicBool::new(false),
            connected_lock: Mutex::new(()),
            connected_signal: Condvar::new(),
            next_event_id: AtomicU32::new(1),
            events: RwLock::new(HashMap::new()),
        }
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

    pub fn get_next_event_id(&self) -> u32 {
        self.next_event_id.fetch_add(1, Ordering::SeqCst)
    }

    pub fn add_event_request(&self, event_request: EventRequest) {
        let mut events = self.events.write().unwrap();
        events.insert(event_request.id, event_request);
    }
}
