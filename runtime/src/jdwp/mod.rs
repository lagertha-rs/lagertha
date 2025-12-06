use crate::keys::{ClassId, MethodId};
use crossbeam::channel::{Receiver, Sender, unbounded};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::thread::JoinHandle;

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
}

pub fn start_jdwp_agent(debug: Arc<DebugState>, port: u16) -> JoinHandle<()> {
    std::thread::spawn(move || {
        jdwp_agent_routine(debug, port);
    })
}

fn jdwp_agent_routine(debug: Arc<DebugState>, port: u16) {
    let listener = std::net::TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    println!("JDWP agent listening on port {}", port);

    loop {
        let (mut stream, addr) = listener.accept().unwrap();
        println!("Debugger connected from {}", addr);

        if let Err(e) = perform_handshake(&mut stream) {
            eprintln!("Handshake failed: {}", e);
            continue;
        }

        debug.connected.store(true, Ordering::SeqCst);

        connection_handler(debug.clone(), stream);

        debug.connected.store(false, Ordering::SeqCst);
        debug.resume_all();

        println!("Debugger disconnected from {}", addr);
    }
}

fn connection_handler(debug: Arc<DebugState>, stream: TcpStream) {
    let mut reader = BufReader::new(stream);

    loop {
        let mut buf = String::new();
        let bytes = reader.read_line(&mut buf).unwrap();

        if bytes == 0 {
            break;
        }

        let cmd = buf.trim();

        if cmd == "run" {
            debug.resume_all();
        } else {
            println!("Received unknown command: {}", cmd);
        }
    }
}

fn perform_handshake(stream: &mut TcpStream) -> Result<(), String> {
    let mut buf = [0u8; 14];
    stream.read_exact(&mut buf).map_err(|e| e.to_string())?;

    if &buf != b"JDWP-Handshake" {
        return Err("Invalid JDWP handshake".to_string());
    }

    stream
        .write_all(b"JDWP-Handshake")
        .map_err(|e| e.to_string())?;
    Ok(())
}
