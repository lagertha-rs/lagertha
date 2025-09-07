use crate::method_area::MethodArea;
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

    pub fn start(&self, data: Vec<u8>) -> Result<(), JvmError> {
        let main_class = self.method_area.add_class(data)?;
        let main_method = main_class
            .get_main_method()
            .ok_or(JvmError::NoMainClassFound(
                main_class.get_name()?.to_string(),
            ))?;
        debug!("Found main method in class {}", main_class.get_name()?);
        let instructions = main_method.instructions();
        let frame = Frame::new(
            main_class.get_cp().clone(),
            main_method.max_locals(),
            main_method.max_stack(),
        );

        for instruction in instructions {
            debug!("Executing instruction: {:?}", instruction);
            // Here you would implement the logic to execute each instruction
        }

        // Implementation of starting the execution of the main class
        Ok(())
    }
}
