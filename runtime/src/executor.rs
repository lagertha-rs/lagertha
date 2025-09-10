use crate::method_area::MethodArea;
use crate::rt::class::class::Class;
use crate::stack::{Frame, ThreadStack};
use crate::{JvmError, VmConfig};
use std::sync::Arc;
use tracing_log::log::debug;

pub struct Executor {
    method_area: Arc<MethodArea>,
    thread_stack: ThreadStack,
    native_stack: (),
    pc: (),
}

impl Executor {
    pub fn new(vm_config: &Arc<VmConfig>, method_area: Arc<MethodArea>) -> Self {
        let thread_stack = ThreadStack::new(vm_config.stack_size_per_thread);
        Self {
            method_area,
            thread_stack,
            native_stack: (),
            pc: (),
        }
    }

    fn insure_initialized(&self, class: Option<&Arc<Class>>) -> Result<(), JvmError> {
        if let Some(class) = class {
            if let Some(super_class) = &class.super_class() {
                self.insure_initialized(Some(super_class))?;
            }
            if !class.initialized()
                && let Some(initializer) = class.initializer()
            {
                debug!("Initializing class {}", class.name()?);
                let instructions = initializer.instructions();
                for instruction in instructions {
                    debug!("Executing instruction: {:?}", instruction);
                    // Here you would implement the logic to execute each instruction
                }
                class.set_initialized();
                debug!("Class {} initialized", class.name()?);
            }
        }
        // Implementation of class initialization logic
        Ok(())
    }

    pub fn start(&self, data: Vec<u8>) -> Result<(), JvmError> {
        let main_class = self.method_area.add_class(data)?;
        let main_method = main_class
            .find_main_method()
            .ok_or(JvmError::NoMainClassFound(main_class.name()?.to_string()))?;
        debug!("Found main method in class {}", main_class.name()?);
        self.insure_initialized(Some(&main_class))?;
        let instructions = main_method.instructions();
        let frame = Frame::new(
            main_class.cp().clone(),
            main_method.max_locals(),
            main_method.max_stack(),
        );

        debug!("Executing main method...");

        for instruction in instructions {
            debug!("Executing instruction: {:?}", instruction);
            // Here you would implement the logic to execute each instruction
        }

        // Implementation of starting the execution of the main class
        Ok(())
    }
}
