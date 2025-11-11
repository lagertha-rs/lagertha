use crate::heap::Heap;
use crate::heap::method_area::MethodArea;
use crate::heap_deprecated::HeapDeprecated;
use crate::heap_deprecated::method_area_deprecated::MethodAreaDeprecated;
use crate::interpreter::Interpreter;
use crate::native::NativeRegistry;
use crate::native_deprecated::NativeRegistryDeprecated;
use crate::stack::FrameStack;
use crate::thread::JavaThreadState;
use common::error::JvmError;
use common::error::JvmError::JavaExceptionThrown;
use common::jtype::{HeapRef, Value};
use lasso::{Spur, ThreadedRodeo};
use std::num::NonZeroU32;
use std::path::PathBuf;
use std::sync::Arc;

mod class_loader;
pub mod debug_log;
pub mod heap;
pub mod heap_deprecated;
mod interpreter;
mod interpreter_deprecated;
mod native;
mod native_deprecated;
pub mod rt;
mod stack;
pub mod stack_deprecated;
mod thread;
mod throw;

pub type ClassIdDeprecated = Spur;
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ThreadId(NonZeroU32);

impl ThreadId {
    pub fn new(val: NonZeroU32) -> Self {
        ThreadId(val)
    }
    pub fn from_usize(index: usize) -> Self {
        ThreadId(NonZeroU32::new(index as u32).unwrap())
    }
    pub fn as_usize(&self) -> usize {
        self.0.get() as usize
    }
    pub fn to_index(&self) -> usize {
        (self.0.get() - 1) as usize
    }
}
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

    //TODO: bad
    pub fn to_i32(&self) -> i32 {
        self.0.get() as i32
    }

    //TODO: also need but needs for previous bad :D
    pub fn from_i32(index: i32) -> Self {
        MethodId(NonZeroU32::new(index as u32).unwrap())
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct TypeDescriptorId(NonZeroU32);

impl TypeDescriptorId {
    pub fn from_usize(index: usize) -> Self {
        TypeDescriptorId(NonZeroU32::new(index as u32).unwrap())
    }
    pub fn to_index(&self) -> usize {
        (self.0.get() - 1) as usize
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct MethodDescriptorId(NonZeroU32);

impl MethodDescriptorId {
    pub fn from_usize(index: usize) -> Self {
        MethodDescriptorId(NonZeroU32::new(index as u32).unwrap())
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

    //TODO: bad
    pub fn to_i32(&self) -> i32 {
        self.0.get() as i32
    }

    //TODO: also need but needs for previous bad :D
    pub fn from_i32(index: i32) -> Self {
        ClassId(NonZeroU32::new(index as u32).unwrap())
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

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct FullyQualifiedMethodKey {
    pub class: Option<Symbol>,
    pub name: Symbol,
    pub desc: Symbol,
}

impl FullyQualifiedMethodKey {
    pub fn new(class: Symbol, name: Symbol, desc: Symbol) -> Self {
        Self {
            class: Some(class),
            name,
            desc,
        }
    }

    pub fn new_internal(name: Symbol, desc: Symbol) -> Self {
        Self {
            class: None,
            name,
            desc,
        }
    }

    pub fn new_internal_with_str(name: &str, desc: &str, interner: &ThreadedRodeo) -> Self {
        Self {
            class: None,
            name: interner.get_or_intern(name),
            desc: interner.get_or_intern(desc),
        }
    }

    pub fn new_with_str(class: &str, name: &str, desc: &str, interner: &ThreadedRodeo) -> Self {
        Self {
            class: Some(interner.get_or_intern(class)),
            name: interner.get_or_intern(name),
            desc: interner.get_or_intern(desc),
        }
    }
}

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

impl FieldKey {
    pub fn new(name: Symbol, desc: Symbol) -> Self {
        Self { name, desc }
    }
}

#[derive(Debug, Clone)]
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
    heap: HeapDeprecated,
    method_area_deprecated: MethodAreaDeprecated,
    native_registry: NativeRegistryDeprecated,
    string_interner: Arc<ThreadedRodeo>,
}

impl VirtualMachineDeprecated {
    pub fn new(config: VmConfig) -> Result<Self, JvmError> {
        config.validate();
        let string_interner = Arc::new(ThreadedRodeo::default());
        let mut method_area = MethodAreaDeprecated::new(&config, string_interner.clone())?;
        let heap = HeapDeprecated::new(&mut method_area)?;

        let native_registry = NativeRegistryDeprecated::new(string_interner.clone());
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
    heap: Heap,
    native_registry: NativeRegistry,
    string_interner: Arc<ThreadedRodeo>,
    thread_registry: Vec<JavaThreadState>,
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
            name: "init thread".to_string(),
            stack: FrameStack::new(&config),
        };
        let rust_thread_id = ThreadId::new(NonZeroU32::MAX);
        Ok(Self {
            config,
            native_registry,
            string_interner,
            method_area,
            heap: Heap::new()?,
            thread_registry: Vec::new(),
            rust_thread,
            rust_thread_id,
        })
    }

    // TODO: implement and no mut
    pub fn get_thread_mut(&mut self, thread_id: ThreadId) -> &mut JavaThreadState {
        assert_eq!(thread_id, self.rust_thread_id);
        &mut self.rust_thread
    }

    pub fn get_stack_mut(&mut self, thread_id: &ThreadId) -> Result<&mut FrameStack, JvmError> {
        if *thread_id == self.rust_thread_id {
            Ok(&mut self.rust_thread.stack)
        } else {
            self.thread_registry
                .get_mut(thread_id.to_index())
                .ok_or(JvmError::Todo("No such thread".to_string()))
                .map(|t| &mut t.stack)
        }
    }
    pub fn get_stack(&self, thread_id: &ThreadId) -> Result<&FrameStack, JvmError> {
        if *thread_id == self.rust_thread_id {
            Ok(&self.rust_thread.stack)
        } else {
            self.thread_registry
                .get(thread_id.to_index())
                .ok_or(JvmError::Todo("No such thread".to_string()))
                .map(|t| &t.stack)
        }
    }

    pub fn interner(&self) -> &ThreadedRodeo {
        &self.string_interner
    }

    pub fn symbol_to_pretty_string(&self, sym: Symbol) -> String {
        self.string_interner.resolve(&sym).replace('/', ".")
    }
}

fn print_stack_trace(exception_ref: HeapRef, vm: &mut VirtualMachine) {
    let exception_class_id = vm
        .heap
        .get_class_id(&exception_ref)
        .expect("TODO msg: Exception object should exist");
    let exception_class = vm
        .method_area
        .get_instance_class(&exception_class_id)
        .expect("TODO msg: Exception class should exist");
    let print_stack_trace_method_id = exception_class
        .get_vtable_method_id(&vm.method_area.br().print_stack_trace_mk)
        .expect("Exception class should have printStackTrace method");
    let args = vec![Value::Ref(exception_ref)];
    Interpreter::run_method(vm.rust_thread_id, print_stack_trace_method_id, vm, args)
        .expect("printStackTrace should run without errors");
}

pub fn start(config: VmConfig) -> Result<(), JvmError> {
    let mut vm = VirtualMachine::new(config)?;

    #[cfg(feature = "debug-log")]
    debug_log::debug::init(&vm);

    let main_class_sym = vm.string_interner.get_or_intern(&vm.config.main_class);
    let main_class_id = vm.method_area.get_class_id_or_load(main_class_sym)?;
    let main_method_id = vm
        .method_area
        .get_instance_class(&main_class_id)?
        .get_special_method_id(&vm.method_area.br().main_mk)
        .map_err(|_| JvmError::MainClassNotFound(vm.config.main_class.replace('/', ".")))?;
    debug_log_method!(&main_method_id, "Main method found");
    let rust_thread_id = vm.rust_thread_id;

    Interpreter::invoke_static_method(rust_thread_id, main_method_id, &mut vm, vec![]).inspect_err(
        |e| {
            if let JavaExceptionThrown(exception_ref) = e {
                print_stack_trace(*exception_ref, &mut vm);
            }
        },
    )
}
