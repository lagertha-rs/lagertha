use crate::error::{JavaExceptionFromJvm, JvmError};
use crate::heap::method_area::MethodArea;
use crate::heap::{Heap, HeapRef};
use crate::interpreter::Interpreter;
use crate::keys::{MethodId, MethodKey, Symbol, ThreadId};
use crate::native::NativeRegistry;
use crate::thread::JavaThreadState;
use crate::vm::Value;
use crate::vm::bootstrap_registry::BootstrapRegistry;
use crate::vm::stack::FrameStack;
use lasso::ThreadedRodeo;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::time::Duration;

mod class_loader;
mod error;
pub mod heap;
mod interpreter;
mod jdwp;
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
    pub jdwp_port: Option<u16>,
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
    pub fn new(
        config: VmConfig,
        string_interner: Arc<ThreadedRodeo>,
    ) -> Result<(Self, ThreadId), ()> {
        config.validate();
        let (method_area, br) =
            MethodArea::init(&config, string_interner.clone()).map_err(|e| {
                eprintln!("Error: Could not initialize JVM.");
                eprintln!("Caused by: {}", e.into_pretty_string(&string_interner));
            })?;
        let heap = Self::create_heap(string_interner.clone(), &method_area).map_err(|e| {
            eprintln!("Error: Could not initialize JVM.");
            eprintln!("Caused by: {}", e.into_pretty_string(&string_interner));
        })?;

        let native_registry = NativeRegistry::new(string_interner.clone());

        let mut vm = Self {
            config,
            native_registry,
            string_interner: string_interner.clone(),
            method_area,
            heap,
            threads: Vec::new(),
            br,
        };

        #[cfg(feature = "log-runtime-traces")]
        log_traces::debug::init(&vm);

        let debug_state = Arc::new(jdwp::DebugState::new());
        if let Some(jdwp_port) = vm.config.jdwp_port {
            jdwp::start_jdwp_agent(debug_state.clone(), jdwp_port);
            debug_state.suspend_all(); //TODO: I assume always suspended at start (suspend=y)

            while !debug_state.connected.load(Ordering::SeqCst) {
                std::thread::sleep(Duration::from_millis(100));
            }
        }

        //TODO: I guess hotspot puts it just before main method invocation
        // that's why I don't stop in debugger in initPhase1 etc..
        debug_state.wait_if_suspended();

        let main_thread_id = vm.create_main_thread().map_err(|e| {
            eprintln!("Error: Could not initialize JVM.");
            eprintln!("Caused by: {}", e.into_pretty_string(&string_interner));
        })?;

        vm.initialize_main_thread(main_thread_id).map_err(|e| {
            eprintln!("Error: Could not initialize JVM.");
            eprintln!("Caused by: {}", e.into_pretty_string(&string_interner));
        })?;

        // TODO: need actually refactor error struct, because this is ugly
        vm.initialize_system_class(main_thread_id).map_err(|e| {
            // actually somewhere in java this exception is already caught at this point
            /*
            if let JvmError::JavaException(java_ex) = e {
                let mapped = vm
                    .map_rust_error_to_java_exception(main_thread_id, java_ex)
                    .unwrap();
                vm.unhandled_exception(main_thread_id, JvmError::JavaExceptionThrown(mapped));
            } else {
                vm.unhandled_exception(main_thread_id, e);
            }
             */
        })?;

        Ok((vm, main_thread_id))
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

    fn initialize_system_class(&mut self, thread_id: ThreadId) -> Result<(), JvmError> {
        let system_class_id = self.br().get_java_lang_system_id()?;

        let init_phase1_method_key = self.br().system_init_phase1_mk;
        let init_phase2_method_key = self.br().system_init_phase2_mk;
        let init_phase3_method_key = self.br().system_init_phase3_mk;

        // Run initPhase1

        let init_phase1_method_id = self
            .method_area
            .get_instance_class(&system_class_id)?
            .get_special_method_id(&init_phase1_method_key)?;

        Interpreter::invoke_static_method(thread_id, init_phase1_method_id, self, vec![])?;

        // Run initPhase2
        /*
               let init_phase2_method_id = self
                   .method_area
                   .get_instance_class(&system_class_id)?
                   .get_special_method_id(&init_phase2_method_key)?;

               Interpreter::invoke_static_method(
                   thread_id,
                   init_phase2_method_id,
                   self,
                   vec![Value::Integer(1), Value::Integer(1)],
               )?;

        */

        Ok(())
    }

    // TODO: refactor and improve error handling. ideally can't fail
    //TODO: exception arg should be actually JvmError, like any error
    fn map_rust_error_to_java_exception(
        &mut self,
        thread_id: ThreadId,
        exception: JavaExceptionFromJvm,
    ) -> Result<HeapRef, JvmError> {
        let exception_ref = exception.as_reference();
        let class_id = self
            .method_area
            // TODO: fix interner usage, replace with direct symbol
            .get_class_id_or_load(self.interner().get_or_intern(exception_ref.class))?;
        let (method_id, instance_size) = {
            let class = self.method_area.get_instance_class(&class_id)?;
            (
                class.get_special_method_id(
                    // TODO: fix interner usage, replace with direct symbol
                    &MethodKey {
                        name: self.interner().get_or_intern(exception_ref.name),
                        desc: self.interner().get_or_intern(exception_ref.descriptor),
                    },
                )?,
                class.get_instance_size()?,
            )
        };
        let instance = self.heap.alloc_instance(instance_size, class_id)?;
        let params = if let Some(msg) = exception.message {
            let resolved_msg = msg.into_resolved(self.interner());
            vec![
                Value::Ref(instance),
                Value::Ref(self.heap.alloc_string(&resolved_msg)?),
            ]
        } else {
            vec![Value::Ref(instance)]
        };
        Interpreter::invoke_instance_method(thread_id, method_id, self, params)?;
        Ok(instance)
    }

    //TODO: exception should be allocated on java heap at this point, and be a reference
    //TODO: get rid of unwrap, need to understand how to handle errors here properly
    fn unhandled_exception(&mut self, thread_id: ThreadId, exception: JvmError) {
        if let JvmError::JavaExceptionThrown(exception_ref) = exception {
            let get_thread_group_method_id = self
                .method_area
                .get_class(&self.br().get_java_lang_thread_id().unwrap())
                .get_vtable_method_id(&self.br().thread_get_thread_group_mk)
                .unwrap();
            let thread_group_ref = Interpreter::invoke_instance_method(
                thread_id,
                get_thread_group_method_id,
                self,
                vec![Value::Ref(self.get_thread(&thread_id).thread_obj)],
            )
            .unwrap()
            .unwrap()
            .as_obj_ref()
            .unwrap();
            let uncaught_exception_method_id = self
                .method_area
                .get_class(&self.br().get_java_lang_thread_group_id().unwrap())
                .get_vtable_method_id(&self.br().thread_group_uncaught_exception_mk)
                .unwrap();
            Interpreter::invoke_instance_method(
                thread_id,
                uncaught_exception_method_id,
                self,
                vec![
                    Value::Ref(thread_group_ref),
                    Value::Ref(self.get_thread(&thread_id).thread_obj),
                    Value::Ref(exception_ref),
                ],
            )
            .unwrap();
        } else {
            eprintln!("Unhandled exception: {}", exception);
        }
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

    //TODO: avoid allocations
    pub fn symbol_to_pretty_string(&self, sym: Symbol) -> String {
        self.string_interner.resolve(&sym).replace('/', ".")
    }

    pub fn pretty_method_not_found_message(&self, method_id: &MethodId) -> String {
        let method = self.method_area.get_method(method_id);
        let method_desc = self
            .method_area
            .get_method_descriptor(&method.descriptor_id());
        let class_sym = self.method_area.get_class(&method.class_id()).get_name();
        let class_name = self.interner().resolve(&class_sym);
        let method_name = self.interner().resolve(&method.name);
        method_desc.to_java_signature(class_name, method_name)
    }
}

pub fn start(config: VmConfig) -> Result<(), ()> {
    let string_interner = Arc::new(ThreadedRodeo::default());
    let (mut vm, main_thread_id) =
        VirtualMachine::new(config, string_interner.clone()).map_err(|_| {})?;

    #[cfg(feature = "log-runtime-traces")]
    log_traces::debug::init(&vm);

    let main_class_sym = vm.string_interner.get_or_intern(&vm.config.main_class);
    let main_class_id = vm
        .method_area
        .get_class_id_or_load(main_class_sym)
        .map_err(|e| {
            eprintln!(
                "Error: Could not find or load main class {}",
                vm.config.main_class.replace('/', ".")
            );
            eprintln!("Caused by: {}", e.into_pretty_string(&string_interner));
        })?;
    let main_method_id = vm
        .method_area
        .get_instance_class(&main_class_id)
        .unwrap()
        .get_special_method_id(&vm.br().main_mk)
        .map_err(|_| JvmError::MainClassNotFound(vm.config.main_class.replace('/', ".")))
        .unwrap();
    debug_log_method!(&main_method_id, "Main method found");

    // TODO: it works more or less correctly, but should be improved
    if let Err(e) =
        Interpreter::invoke_static_method(main_thread_id, main_method_id, &mut vm, vec![])
    {
        vm.unhandled_exception(main_thread_id, e);
        Err(())
    } else {
        Ok(())
    }
}
