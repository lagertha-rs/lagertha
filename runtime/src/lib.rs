use crate::error::JvmError;
use crate::heap::Heap;
use crate::heap::method_area::MethodArea;
use crate::interpreter::Interpreter;
use crate::native::NativeRegistry;
use crate::stack::FrameStack;
use common::jtype::Value;
use jimage::JImage;
use lasso::{Spur, ThreadedRodeo};
use std::path::PathBuf;
use std::sync::Arc;
use tracing_log::log::debug;

mod class_loader;
pub mod error;
pub mod heap;
mod interpreter;
mod native;
pub mod rt;
pub mod stack;

pub type ClassId = Spur;
pub type MethodId = usize;

#[derive(Debug)]
pub struct VmConfig {
    pub home: PathBuf,
    pub version: String,
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
    frame_stack: FrameStack,
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
            frame_stack: FrameStack::new(&config),
            method_area,
            config,
            heap,
            native_registry,
            string_interner,
        })
    }
}

pub fn start(name: &str, main_class: Vec<u8>, config: VmConfig) -> Result<(), JvmError> {
    debug!("Starting VM with config: {:?}", config);
    let vm = VirtualMachine::new(config)?;

    let mut interpreter = Interpreter::new(vm);
    match interpreter.start(name, main_class) {
        Ok(_) => {
            debug!("VM execution finished successfully");
            Ok(())
        }
        Err(e) => match e {
            JvmError::JavaExceptionThrown(addr) => {
                let exception_class_id = {
                    let exception = interpreter.vm().heap.get_instance(&addr)?;
                    *exception.class_id()
                };
                let exception_class = interpreter
                    .vm()
                    .method_area
                    .get_class_by_id(&exception_class_id)?;
                let print_stack_trace_method = exception_class
                    .get_virtual_method("printStackTrace", "()V")
                    .expect("Exception class should have printStackTrace method")
                    .clone();
                let param = vec![Value::Ref(addr)];
                interpreter.run_instance_method(&print_stack_trace_method, param)?;
                Err(e)
            }
            _ => {
                eprintln!("VM execution failed with error: {:?}", e);
                Err(e)
            }
        },
    }
}
