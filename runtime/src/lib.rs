use crate::heap::Heap;
use crate::heap::method_area::MethodArea;
use crate::heap::method_area_deprecated::MethodAreaDeprecated;
use crate::interpreter::Interpreter;
use crate::interpreter_deprecated::InterpreterDeprecated;
use crate::native::NativeRegistry;
use crate::stack::FrameStack;
use crate::thread::JavaThreadState;
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
mod interpreter_deprecated;
mod native;
pub mod rt;
mod stack;
pub mod stack_deprecated;
mod thread;
mod throw;

pub type ClassIdDeprecated = Spur;
pub type ThreadId = u16;

pub type MethodIdDeprecated = usize;
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct MethodId(NonZeroU32);

impl MethodId {
    pub fn from_usize(index: usize) -> Self {
        MethodId(NonZeroU32::new(index as u32).unwrap())
    }
    pub fn to_index(&self) -> usize {
        (self.0.get() - 1) as usize
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct FieldDescriptorId(NonZeroU32);

impl FieldDescriptorId {
    pub fn from_usize(index: usize) -> Self {
        FieldDescriptorId(NonZeroU32::new(index as u32).unwrap())
    }
    pub fn to_index(&self) -> usize {
        (self.0.get() - 1) as usize
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ClassId(NonZeroU32);

impl ClassId {
    pub fn from_usize(index: usize) -> Self {
        ClassId(NonZeroU32::new(index as u32).unwrap())
    }
    pub fn to_index(&self) -> usize {
        (self.0.get() - 1) as usize
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct FieldId(NonZeroU32);

impl FieldId {
    pub fn from_usize(index: usize) -> Self {
        FieldId(NonZeroU32::new(index as u32).unwrap())
    }
    pub fn to_index(&self) -> usize {
        (self.0.get() - 1) as usize
    }
}

pub type Symbol = Spur;
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct MethodKey {
    pub name: Symbol,
    pub desc: Symbol,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct FieldKey {
    pub name: Symbol,
    pub desc: Symbol,
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

pub struct VirtualMachineDeprecated {
    config: VmConfig,
    heap: Heap,
    method_area_deprecated: MethodAreaDeprecated,
    native_registry: NativeRegistry,
    string_interner: Arc<ThreadedRodeo>,
}

impl VirtualMachineDeprecated {
    pub fn new(config: VmConfig) -> Result<Self, JvmError> {
        config.validate();
        let string_interner = Arc::new(ThreadedRodeo::default());
        let mut method_area = MethodAreaDeprecated::new(&config, string_interner.clone())?;
        let heap = Heap::new(&mut method_area)?;

        let native_registry = NativeRegistry::new(string_interner.clone());
        Ok(Self {
            method_area_deprecated: method_area,
            config,
            heap,
            native_registry,
            string_interner,
        })
    }
}

pub struct VirtualMachine {
    config: VmConfig,
    method_area: MethodArea,
    native_registry: NativeRegistry,
    string_interner: Arc<ThreadedRodeo>,
    rust_thread: JavaThreadState, // TODO: replace with something else
    rust_thread_id: ThreadId,
}

impl VirtualMachine {
    pub fn new(config: VmConfig) -> Result<Self, JvmError> {
        config.validate();
        let string_interner = Arc::new(ThreadedRodeo::default());
        let method_area = MethodArea::new(&config, string_interner.clone())?;

        let native_registry = NativeRegistry::new(string_interner.clone());
        let rust_thread = JavaThreadState {
            thread_obj: 0,
            group_obj: 0,
            name: "".to_string(),
            stack: FrameStack::new(&config),
        };
        let rust_thread_id = u16::MAX;
        Ok(Self {
            config,
            native_registry,
            string_interner,
            method_area,
            rust_thread,
            rust_thread_id,
        })
    }

    // TODO: implement and no mut
    pub fn get_thread_mut(&mut self, thread_id: ThreadId) -> &mut JavaThreadState {
        assert_eq!(thread_id, self.rust_thread_id);
        &mut self.rust_thread
    }
}

fn start(config: VmConfig) -> Result<(), JvmError> {
    let mut vm = VirtualMachine::new(config)?;

    let main_class_sym = vm.string_interner.get_or_intern(&vm.config.main_class);
    let main_class_id = vm.method_area.get_class_id_or_load(main_class_sym)?;
    let main_method_key = MethodKey {
        name: vm.string_interner.get_or_intern("main"),
        desc: vm.string_interner.get_or_intern("([Ljava/lang/String;)V"),
    };
    let main_method_id = vm
        .method_area
        .get_class(&main_class_id)
        .get_special_method_id(&main_method_key)
        .map_err(|_| JvmError::MainClassNotFound(vm.config.main_class.replace('/', ".")))?;
    let rust_thread_id = vm.rust_thread_id;

    Interpreter::invoke_static_method(&mut vm, rust_thread_id, main_method_id, vec![])?;

    Ok(())
}

fn print_stack_trace_deprecated(exception_ref: HeapAddr, interpreter: &mut InterpreterDeprecated) {
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
        .method_area_deprecated
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

pub fn start_deprecated(config: VmConfig) -> Result<(), JvmError> {
    debug!("Starting VM with config: {:?}", config);
    start(config)

    /*
    let vm = VirtualMachine::new(config)?;

    let mut interpreter = InterpreterDeprecated::new(vm);

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

    match interpreter.start_main() {
        Ok(_) => {
            debug!("VM execution finished successfully");
            Ok(())
        }
        Err(e) => match e {
            JvmError::JavaExceptionThrown(addr) => {
                print_stack_trace_deprecated(addr, &mut interpreter);
                Err(e)
            }
            _ => {
                eprintln!("VM execution failed with error: {:?}", e);
                Err(e)
            }
        },
    }
     */
}
