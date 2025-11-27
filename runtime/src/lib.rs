use crate::heap::Heap;
use crate::heap::method_area::MethodArea;
use crate::interpreter::Interpreter;
use crate::keys::{MethodId, Symbol, ThreadId};
use crate::native::NativeRegistry;
use crate::thread::JavaThreadState;
use crate::vm::bootstrap_registry::BootstrapRegistry;
use crate::vm::stack::FrameStack;
use common::HeapRef;
use common::Value;
use common::error::JvmError;
use lasso::ThreadedRodeo;
use std::path::PathBuf;
use std::sync::Arc;

mod class_loader;
pub mod heap;
mod interpreter;
pub mod keys;
pub mod log_traces;
mod native;
pub mod rt;
mod thread;
mod vm;

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
        if self.version != "25.0.1" {
            panic!(
                "Unsupported Java version: {}. Only 25.0.1 is supported.",
                self.version
            );
        }
    }
}

pub fn start(config: VmConfig) -> Result<(), JvmError> {
    // TODO: it doesn't actually print errors in correct way if any occur during VM initialization. fix
    let (mut vm, main_thread_id) = VirtualMachine::new(config)?;

    #[cfg(feature = "log-runtime-traces")]
    log_traces::debug::init(&vm);

    let main_class_sym = vm.string_interner.get_or_intern(&vm.config.main_class);
    let main_class_id = vm.method_area.get_class_id_or_load(main_class_sym)?;
    let main_method_id = vm
        .method_area
        .get_instance_class(&main_class_id)?
        .get_special_method_id(&vm.br().main_mk)
        .map_err(|_| JvmError::MainClassNotFound(vm.config.main_class.replace('/', ".")))?;
    debug_log_method!(&main_method_id, "Main method found");

    // TODO: it works more or less correctly, but should be improved
    if let Err(e) =
        Interpreter::invoke_static_method(main_thread_id, main_method_id, &mut vm, vec![])
    {
        
            if let Err(unhandled) = vm.handle_uncaught_exception(main_thread_id, exception_ref) {
                unimplemented!(
                    "TODO: handle unhandled exception during uncaught exception handling: {:?}",
                    unhandled
                );
            }
            Err(e)?
        }
    };
    Ok(())
}

pub struct VirtualMachine {
    config: VmConfig,
    method_area: MethodArea,
    heap: Heap,
    native_registry: NativeRegistry,
    string_interner: Arc<ThreadedRodeo>,
    threads: Vec<JavaThreadState>,
    br: Arc<BootstrapRegistry>,
}

impl VirtualMachine {
    pub fn new(config: VmConfig) -> Result<(Self, ThreadId), JvmError> {
        config.validate();
        let string_interner = Arc::new(ThreadedRodeo::default());
        let (method_area, br) = MethodArea::init(&config, string_interner.clone())?;
        let heap = Self::create_heap(string_interner.clone(), &method_area)?;

        let native_registry = NativeRegistry::new(string_interner.clone());

        let mut vm = Self {
            config,
            native_registry,
            string_interner,
            method_area,
            heap,
            threads: Vec::new(),
            br,
        };

        #[cfg(feature = "log-runtime-traces")]
        log_traces::debug::init(&vm);

        // TODO: is something thrown here, it is not printed correctly
        let main_thread_id = vm.create_main_thread()?;
        vm.initialize_main_thread(main_thread_id).inspect_err(|e| {
            if let JvmError::JavaExceptionThrown(exception_ref) = e {
                eprint!("Exception in thread \"rust init thread\" ");
                Self::print_stack_trace_manually(main_thread_id, *exception_ref, &mut vm);
            }
        })?;

        if let Err(e) = vm.initialize_system_class(main_thread_id) {
            todo!()
        }

        Ok((vm, main_thread_id))
    }

    // TODO: improve error handling
    /// Used only in the case when exception is thrown during main thread initialization
    fn print_stack_trace_manually(
        thread_id: ThreadId,
        exception_ref: HeapRef,
        vm: &mut VirtualMachine,
    ) {
        let exception_class_id = vm
            .heap
            .get_class_id(exception_ref)
            .expect("TODO msg: Exception object should exist");
        let exception_class = vm
            .method_area
            .get_instance_class(&exception_class_id)
            .expect("TODO msg: Exception class should exist");
        let print_stack_trace_method_id = exception_class
            .get_vtable_method_id(&vm.br().print_stack_trace_mk)
            .expect("Exception class should have printStackTrace method");
        let args = vec![Value::Ref(exception_ref)];
        Interpreter::invoke_instance_method(thread_id, print_stack_trace_method_id, vm, args)
            .expect("printStackTrace should run without errors");
    }

    fn create_heap(
        interner: Arc<ThreadedRodeo>,
        method_area: &MethodArea,
    ) -> Result<Heap, JvmError> {
        let bootstrap_registry = method_area.br();
        let string_class_id = bootstrap_registry.get_java_lang_string_id()?;
        let byte_array_class_id = bootstrap_registry.get_byte_array_class_id()?;
        let string_instance_size = method_area
            .get_instance_class(&string_class_id)?
            .get_instance_size()?;
        Heap::new(
            1, // TODO: from config
            interner,
            string_class_id,
            string_instance_size,
            byte_array_class_id,
        )
    }

    fn initialize_main_thread(&mut self, main_thread_id: ThreadId) -> Result<(), JvmError> {
        let system_thread_group_ref = self.create_system_thread_group(main_thread_id)?;
        let main_thread_group_ref =
            self.create_main_thread_group(main_thread_id, system_thread_group_ref)?;

        let thread_class_id = self.br().get_java_lang_thread_id()?;
        let thread_constructor_id = self
            .method_area
            .get_instance_class(&thread_class_id)?
            .get_special_method_id(&self.br().thread_thread_group_and_name_constructor_mk)?;
        Interpreter::invoke_instance_method(
            main_thread_id,
            thread_constructor_id,
            self,
            vec![
                Value::Ref(self.get_thread(&main_thread_id).thread_obj),
                Value::Ref(main_thread_group_ref),
                Value::Ref(self.get_thread(&main_thread_id).name),
            ],
        )?;
        Ok(())
    }

    fn create_main_thread(&mut self) -> Result<ThreadId, JvmError> {
        let thread_class_id = self.br().get_java_lang_thread_id()?;
        let thread_instance_size = self
            .method_area
            .get_instance_class(&thread_class_id)?
            .get_instance_size()?;
        let main_thread_ref = self
            .heap
            .alloc_instance(thread_instance_size, thread_class_id)?;
        let main_string_ref = self.heap.get_str_from_pool_or_new(self.br().main_sym)?;
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
        let system_thread_group_class_id = self.br().get_java_lang_thread_group_id()?;
        let (thread_group_no_arg_constructor_id, thread_group_instance_size) = {
            let thread_group_class = self
                .method_area
                .get_instance_class(&system_thread_group_class_id)?;
            (
                thread_group_class.get_special_method_id(&self.br().no_arg_constructor_mk)?,
                thread_group_class.get_instance_size()?,
            )
        };
        let system_thread_group_ref = self
            .heap
            .alloc_instance(thread_group_instance_size, system_thread_group_class_id)?;
        Interpreter::invoke_instance_method(
            main_thread_id,
            thread_group_no_arg_constructor_id,
            self,
            vec![Value::Ref(system_thread_group_ref)],
        )?;

        Ok(system_thread_group_ref)
    }

    fn create_main_thread_group(
        &mut self,
        main_thread_id: ThreadId,
        system_thread_group_ref: HeapRef,
    ) -> Result<HeapRef, JvmError> {
        let system_thread_group_class_id = self.br().get_java_lang_thread_group_id()?;
        let (thread_group_constructor_id, thread_group_instance_size) = {
            let thread_group_class = self
                .method_area
                .get_instance_class(&system_thread_group_class_id)?;
            (
                thread_group_class.get_special_method_id(
                    &self.br().thread_group_parent_and_name_constructor_mk,
                )?,
                thread_group_class.get_instance_size()?,
            )
        };
        let main_thread_group_ref = self
            .heap
            .alloc_instance(thread_group_instance_size, system_thread_group_class_id)?;
        let main_string_ref = self.heap.get_str_from_pool_or_new(self.br().main_sym)?;
        Interpreter::invoke_instance_method(
            main_thread_id,
            thread_group_constructor_id,
            self,
            vec![
                Value::Ref(main_thread_group_ref),
                Value::Ref(system_thread_group_ref),
                Value::Ref(main_string_ref),
            ],
        )?;
        Ok(main_thread_group_ref)
    }

    pub fn handle_uncaught_exception(
        &mut self,
        thread_id: ThreadId,
        exception_ref: HeapRef,
    ) -> Result<(), JvmError> {
        let get_thread_group_method_id = self
            .method_area
            .get_class(&self.br().get_java_lang_thread_id()?)
            .get_vtable_method_id(&self.br().thread_get_thread_group_mk)?;
        let thread_group_ref = Interpreter::invoke_instance_method(
            thread_id,
            get_thread_group_method_id,
            self,
            vec![Value::Ref(self.get_thread(&thread_id).thread_obj)],
        )?
        .unwrap()
        .as_obj_ref()?;
        let uncaught_exception_method_id = self
            .method_area
            .get_class(&self.br().get_java_lang_thread_group_id()?)
            .get_vtable_method_id(&self.br().thread_group_uncaught_exception_mk)?;
        Interpreter::invoke_instance_method(
            thread_id,
            uncaught_exception_method_id,
            self,
            vec![
                Value::Ref(thread_group_ref),
                Value::Ref(self.get_thread(&thread_id).thread_obj),
                Value::Ref(exception_ref),
            ],
        )?;
        Ok(())
    }

    fn initialize_system_class(&mut self, thread_id: ThreadId) -> Result<(), JvmError> {
        let system_class_id = self.br().get_java_lang_system_id()?;
        let init_phase1_method_key = self.br().system_init_phase1_mk;

        // Run initPhase1
        let init_phase1_method_id = self
            .method_area
            .get_instance_class(&system_class_id)?
            .get_special_method_id(&init_phase1_method_key)?;
        Interpreter::invoke_static_method(thread_id, init_phase1_method_id, self, vec![])?;
        Ok(())
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

    pub fn br(&self) -> &BootstrapRegistry {
        &self.br
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
