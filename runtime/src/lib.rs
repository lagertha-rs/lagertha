use crate::heap::Heap;
use crate::heap::method_area::MethodArea;
use crate::interpreter::Interpreter;
use crate::native::NativeRegistry;
use common::error::JvmError;
use common::jtype::{HeapAddr, Value};
use lasso::{Spur, ThreadedRodeo};
use std::num::NonZeroU32;
use std::path::PathBuf;
use std::sync::Arc;
use tracing_log::log::debug;

mod class_loader;
pub mod heap;
mod interpreter;
mod native;
pub mod rt;
pub mod stack;
mod thread;
mod throw;

pub type ClassIdDeprecated = Spur;
pub type ThreadId = usize;

pub type MethodIdDeprecated = usize;

pub type Sym = Spur;
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct MethodKey {
    pub name: Sym,
    pub desc: Sym,
}

#[derive(Debug)]
pub struct VmConfig {
    pub home: PathBuf,
    pub version: String,
    pub main_class: String,
    pub class_path: Vec<String>,
    pub initial_heap_size: usize,
    pub max_heap_size: usize,
    pub frame_stack_size: usize,
    pub operand_stack_size: usize,
}

//TODO: make it better
impl VmConfig {
    pub fn validate(&self) {
        if self.version != "24.0.2" {
            panic!(
                "Unsupported Java version: {}. Only 24.0.2 is supported.",
                self.version
            );
        }
    }
}

pub struct VirtualMachine {
    config: VmConfig,
    heap: Heap,
    method_area: MethodArea,
    native_registry: NativeRegistry,
    string_interner: Arc<ThreadedRodeo>,
}

impl VirtualMachine {
    pub fn new(config: VmConfig) -> Result<Self, JvmError> {
        config.validate();
        let string_interner = Arc::new(ThreadedRodeo::default());
        let mut method_area = MethodArea::new(&config, string_interner.clone())?;
        let heap = Heap::new(&mut method_area)?;

        let native_registry = NativeRegistry::new(string_interner.clone());
        Ok(Self {
            method_area,
            config,
            heap,
            native_registry,
            string_interner,
        })
    }
}

fn print_stack_trace_todo_refactor(exception_ref: HeapAddr, interpreter: &mut Interpreter) {
    let exception_class_id = {
        let exception = interpreter
            .vm()
            .heap
            .get_instance(&exception_ref)
            .expect("Exception object should exist");
        *exception.class_id()
    };
    let exception_class = interpreter
        .vm()
        .method_area
        .get_class_by_id(&exception_class_id)
        .expect("Exception class should exist");
    let print_stack_trace_method = exception_class
        .get_virtual_method("printStackTrace", "()V")
        .expect("Exception class should have printStackTrace method")
        .clone();
    let param = vec![Value::Ref(exception_ref)];
    interpreter
        .run_instance_method(&print_stack_trace_method, param)
        .expect("printStackTrace should run without errors");
}

pub fn start(config: VmConfig) -> Result<(), JvmError> {
    debug!("Starting VM with config: {:?}", config);
    let vm = VirtualMachine::new(config)?;

    let mut interpreter = Interpreter::new(vm);

    /*
    if let Err(e) = interpreter.initialize_main_thread() {
        match e {
            JvmError::JavaExceptionThrown(addr) => {
                print_stack_trace_todo_refactor(addr, &mut interpreter);
                Err(e)?
            }
            _ => {
                eprintln!("VM execution failed with error: {:?}", e);
                Err(e)?
            }
        }
    }
     */

    match interpreter.start_main() {
        Ok(_) => {
            debug!("VM execution finished successfully");
            Ok(())
        }
        Err(e) => match e {
            JvmError::JavaExceptionThrown(addr) => {
                print_stack_trace_todo_refactor(addr, &mut interpreter);
                Err(e)
            }
            _ => {
                eprintln!("VM execution failed with error: {:?}", e);
                Err(e)
            }
        },
    }
}
