use crate::method_area::MethodArea;
use crate::rt::class::class::Class;
use crate::stack::{Frame, FrameStack};
use crate::{JvmError, VmConfig};
use common::instruction::Instruction;
use common::jtype::TypeValue;
use std::sync::Arc;
use tracing_log::log::debug;

pub struct Executor {
    method_area: Arc<MethodArea>,
    frame_stack: FrameStack,
    native_stack: (),
    pc: (),
}

impl Executor {
    pub fn new(vm_config: &VmConfig, method_area: Arc<MethodArea>) -> Self {
        let thread_stack = FrameStack::new(vm_config);
        Self {
            method_area,
            frame_stack: thread_stack,
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

                let frame = Frame::new(
                    class.cp().clone(),
                    initializer.max_locals(),
                    initializer.max_stack(),
                );

                self.frame_stack.push_frame(frame)?;

                let instructions = initializer.instructions();
                for instruction in instructions {
                    self.execute_instruction(instruction)?;
                }

                // TODO: delete, since I don't have return in clinit and tests for it
                // just to be sure that no operands are left in the stack before popping the frame
                assert!(self.frame_stack.cur_frame_pop_operand().is_err());
                self.frame_stack.pop_frame()?;
                class.set_initialized();
                debug!("Class {} initialized", class.name()?);
            }
        }
        Ok(())
    }

    fn execute_instruction(&self, instruction: &Instruction) -> Result<(), JvmError> {
        debug!("Executing instruction: {:?}", instruction);
        match instruction {
            Instruction::Iconst0 => {
                self.frame_stack.cur_frame_push_operand(TypeValue::Int(0))?;
            }
            Instruction::Putstatic(idx) => {
                let cp = self.frame_stack.cur_frame_cp()?;
                let field_ref = cp.get_fieldref(idx)?;
                let class = self.method_area.get_class(field_ref.class()?.name()?)?;
                // self.insure_initialized(Some(&class))?; TODO: need to add smth like statuses for initialization. I try initialze the same class recursively
                let field_nat = field_ref.name_and_type()?;
                class.set_static_field(field_nat, self.frame_stack.cur_frame_pop_operand()?)?;
            }
            _ => {}
        }
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
        self.frame_stack.push_frame(frame)?;

        debug!("Executing main method...");

        for instruction in instructions {
            self.execute_instruction(instruction)?;
        }

        self.frame_stack.pop_frame()?;

        //TODO: delete, since I don't have return in main and tests for it
        // just to be sure that stack is empty
        assert!(self.frame_stack.pop_frame().is_err());

        Ok(())
    }
}
