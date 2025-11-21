use crate::heap::Heap;
use crate::heap::method_area::MethodArea;
use crate::native::NativeRegistry;
use crate::thread::JavaThreadState;
use crate::vm::interpreter::Interpreter;
use crate::vm::stack::FrameStack;
use common::HeapRef;
use common::Value;
use common::error::JvmError;
use lasso::{Spur, ThreadedRodeo};
use std::num::NonZeroU32;
use std::path::PathBuf;
use std::sync::Arc;

mod class_loader;
pub mod debug_log;
pub mod heap;
mod native;
pub mod rt;
mod thread;
mod vm;

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

    pub fn from_index(index: usize) -> Self {
        ThreadId(NonZeroU32::new((index as u32) + 1).unwrap())
    }

    pub fn as_usize(&self) -> usize {
        self.0.get() as usize
    }
    pub fn to_index(&self) -> usize {
        (self.0.get() - 1) as usize
    }
}
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

pub struct VirtualMachine {
    config: VmConfig,
    method_area: MethodArea,
    heap: Heap,
    native_registry: NativeRegistry,
    string_interner: Arc<ThreadedRodeo>,
    threads: Vec<JavaThreadState>,
}

impl VirtualMachine {
    pub fn new(config: VmConfig) -> Result<(Self, ThreadId), JvmError> {
        config.validate();
        let string_interner = Arc::new(ThreadedRodeo::default());
        let method_area = MethodArea::new(&config, string_interner.clone())?;
        let heap = Self::create_heap(string_interner.clone(), &method_area)?;

        let native_registry = NativeRegistry::new(string_interner.clone());

        let mut vm = Self {
            config,
            native_registry,
            string_interner,
            method_area,
            heap,
            threads: Vec::new(),
        };

        #[cfg(feature = "debug-log")]
        debug_log::debug::init(&vm);

        let main_thread_id = vm.create_main_thread()?;
        vm.initialize_main_thread(main_thread_id).inspect_err(|e| {
            if let JvmError::JavaExceptionThrown(exception_ref) = e {
                eprint!("Exception in thread \"rust init thread\" ");
                print_stack_trace(main_thread_id, *exception_ref, &mut vm);
            }
        })?;

        Ok((vm, main_thread_id))
    }

    fn create_heap(
        interner: Arc<ThreadedRodeo>,
        method_area: &MethodArea,
    ) -> Result<Heap, JvmError> {
        let bootstrap_registry = method_area.br();
        let string_class_id = bootstrap_registry.get_java_lang_string_id()?;
        let char_array_class_id = bootstrap_registry.get_char_array_class_id()?;
        let string_instance_size = method_area
            .get_instance_class(&string_class_id)?
            .get_instance_size()?;
        Heap::new(
            1, // TODO: from config
            interner,
            string_class_id,
            string_instance_size,
            char_array_class_id,
        )
    }

    fn initialize_main_thread(&mut self, main_thread_id: ThreadId) -> Result<(), JvmError> {
        let system_thread_group_ref = self.create_system_thread_group(main_thread_id)?;
        let main_thread_group_ref =
            self.create_main_thread_group(main_thread_id, system_thread_group_ref)?;

        let thread_class_id = self.method_area.br().get_java_lang_thread_id()?;
        let thread_constructor_id = self
            .method_area
            .get_instance_class(&thread_class_id)?
            .get_special_method_id(
                &self
                    .method_area
                    .br()
                    .thread_thread_group_and_name_constructor_mk,
            )?;
        Interpreter::run_method(
            main_thread_id,
            thread_constructor_id,
            vec![
                Value::Ref(self.get_thread(&main_thread_id).thread_obj),
                Value::Ref(main_thread_group_ref),
                Value::Ref(self.get_thread(&main_thread_id).name),
            ],
            self,
        )?;
        Ok(())
    }

    fn create_main_thread(&mut self) -> Result<ThreadId, JvmError> {
        let thread_class_id = self.method_area.br().get_java_lang_thread_id()?;
        let thread_instance_size = self
            .method_area
            .get_instance_class(&thread_class_id)?
            .get_instance_size()?;
        let main_thread_ref = self
            .heap
            .alloc_instance(thread_instance_size, thread_class_id)?;
        let main_string_ref = self
            .heap
            .get_str_from_pool_or_new(self.method_area.br().main_sym)?;
        self.threads.push(JavaThreadState {
            thread_obj: main_thread_ref,
            group_obj: 0,
            name: main_string_ref,
            stack: FrameStack::new(&self.config),
        });
        Ok(ThreadId::from_index(0))
    }

    fn create_system_thread_group(
        &mut self,
        main_thread_id: ThreadId,
    ) -> Result<HeapRef, JvmError> {
        let system_thread_group_class_id = self.method_area.br().get_java_lang_thread_group_id()?;
        let (thread_group_no_arg_constructor_id, thread_group_instance_size) = {
            let thread_group_class = self
                .method_area
                .get_instance_class(&system_thread_group_class_id)?;
            (
                thread_group_class
                    .get_special_method_id(&self.method_area.br().no_arg_constructor_mk)?,
                thread_group_class.get_instance_size()?,
            )
        };
        let system_thread_group_ref = self
            .heap
            .alloc_instance(thread_group_instance_size, system_thread_group_class_id)?;
        Interpreter::run_method(
            main_thread_id,
            thread_group_no_arg_constructor_id,
            vec![Value::Ref(system_thread_group_ref)],
            self,
        )?;

        Ok(system_thread_group_ref)
    }

    fn create_main_thread_group(
        &mut self,
        main_thread_id: ThreadId,
        system_thread_group_ref: HeapRef,
    ) -> Result<HeapRef, JvmError> {
        let system_thread_group_class_id = self.method_area.br().get_java_lang_thread_group_id()?;
        let (thread_group_constructor_id, thread_group_instance_size) = {
            let thread_group_class = self
                .method_area
                .get_instance_class(&system_thread_group_class_id)?;
            (
                thread_group_class.get_special_method_id(
                    &self
                        .method_area
                        .br()
                        .thread_group_parent_and_name_constructor_mk,
                )?,
                thread_group_class.get_instance_size()?,
            )
        };
        let main_thread_group_ref = self
            .heap
            .alloc_instance(thread_group_instance_size, system_thread_group_class_id)?;
        let main_string_ref = self
            .heap
            .get_str_from_pool_or_new(self.method_area.br().main_sym)?;
        Interpreter::run_method(
            main_thread_id,
            thread_group_constructor_id,
            vec![
                Value::Ref(main_thread_group_ref),
                Value::Ref(system_thread_group_ref),
                Value::Ref(main_string_ref),
            ],
            self,
        )?;
        Ok(main_thread_group_ref)
    }

    // TODO: implement and no mut
    pub fn get_thread_mut(&mut self, thread_id: &ThreadId) -> &mut JavaThreadState {
        self.threads.get_mut(thread_id.to_index()).unwrap()
    }
    pub fn get_thread(&self, thread_id: &ThreadId) -> &JavaThreadState {
        self.threads.get(thread_id.to_index()).unwrap()
    }

    pub fn get_stack_mut(&mut self, thread_id: &ThreadId) -> Result<&mut FrameStack, JvmError> {
        Ok(&mut self.get_thread_mut(thread_id).stack)
    }

    pub fn get_stack(&self, thread_id: &ThreadId) -> Result<&FrameStack, JvmError> {
        Ok(&self.get_thread(thread_id).stack)
    }

    pub fn interner(&self) -> &ThreadedRodeo {
        &self.string_interner
    }

    pub fn symbol_to_pretty_string(&self, sym: Symbol) -> String {
        self.string_interner.resolve(&sym).replace('/', ".")
    }

    pub fn pretty_method_not_found_message(&self, method_id: &MethodId) -> String {
        let method = self.method_area.get_method(method_id);
        let method_desc = self
            .method_area
            .get_method_descriptor(&method.descriptor_id());
        let class_sym = self.method_area.get_class(&method.class_id()).get_name();
        method_desc.to_java_signature(&format!(
            "{}.{}",
            self.symbol_to_pretty_string(class_sym),
            self.symbol_to_pretty_string(method.name)
        ))
    }
}

//TODO: delete it. should be replaced like in start function
fn print_stack_trace(thread_id: ThreadId, exception_ref: HeapRef, vm: &mut VirtualMachine) {
    let exception_class_id = vm
        .heap
        .get_class_id(exception_ref)
        .expect("TODO msg: Exception object should exist");
    let exception_class = vm
        .method_area
        .get_instance_class(&exception_class_id)
        .expect("TODO msg: Exception class should exist");
    let print_stack_trace_method_id = exception_class
        .get_vtable_method_id(&vm.method_area.br().print_stack_trace_mk)
        .expect("Exception class should have printStackTrace method");
    let args = vec![Value::Ref(exception_ref)];
    Interpreter::run_method(thread_id, print_stack_trace_method_id, args, vm)
        .expect("printStackTrace should run without errors");
}

pub fn start(config: VmConfig) -> Result<(), JvmError> {
    // TODO: it doesn't actually print errors in correct way if any occur during VM initialization. fix
    let (mut vm, main_thread_id) = VirtualMachine::new(config)?;

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

    // TODO: it works more or less correctly, but should be improved
    if let Err(e) =
        Interpreter::invoke_static_method(main_thread_id, main_method_id, &mut vm, vec![])
    {
        if let JvmError::JavaExceptionThrown(exception_ref) = e {
            let get_thread_group_method_id = vm
                .method_area
                .get_class(&vm.method_area.br().get_java_lang_thread_id()?)
                .get_vtable_method_id(&vm.method_area.br().thread_get_thread_group_mk)?;
            let thread_group_ref = Interpreter::run_method(
                main_thread_id,
                get_thread_group_method_id,
                vec![Value::Ref(vm.get_thread(&main_thread_id).thread_obj)],
                &mut vm,
            )?
            .unwrap()
            .as_obj_ref()?;
            let uncaught_exception_method_id = vm
                .method_area
                .get_class(&vm.method_area.br().get_java_lang_thread_group_id()?)
                .get_vtable_method_id(&vm.method_area.br().thread_group_uncaught_exception_mk)?;
            Interpreter::run_method(
                main_thread_id,
                uncaught_exception_method_id,
                vec![
                    Value::Ref(thread_group_ref),
                    Value::Ref(vm.get_thread(&main_thread_id).thread_obj),
                    Value::Ref(exception_ref),
                ],
                &mut vm,
            )?;
            Err(e)?
        } else {
            Err(e)?
        }
    };
    Ok(())
}
