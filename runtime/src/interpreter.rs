use crate::stack::JavaFrame;
use crate::{ClassId, MethodId, MethodKey, ThreadId, VirtualMachine};
use common::error::JvmError;
use common::instruction::Instruction;
use common::jtype::Value;
use std::ops::ControlFlow;

pub struct Interpreter;

impl Interpreter {
    fn interpret_instruction(
        thread_id: ThreadId,
        instruction: Instruction,
        vm: &mut VirtualMachine,
    ) -> Result<ControlFlow<()>, JvmError> {
        let is_branch = instruction.is_branch();
        let instruction_byte_size = instruction.byte_size();

        match instruction {
            Instruction::Bipush(value) => {
                vm.get_stack(&thread_id)?
                    .push_operand(Value::Integer(value as i32))?;
            }
            Instruction::Putstatic(idx) => {
                let value = vm.get_stack(&thread_id)?.pop_operand()?;
                let method_id = vm.get_stack(&thread_id)?.cur_frame()?.method_id();
                let class_id = vm.method_area.get_method(&method_id).class_id();
                Self::ensure_initialized(thread_id, Some(class_id), vm)?;
                let field = vm
                    .method_area
                    .get_cp(&class_id)
                    .get_field(&idx, &vm.method_area.string_interner)?;
                vm.method_area
                    .get_class(&class_id)
                    .set_static_field_value(&field.name_and_type.into(), value)?;
            }
            Instruction::Return => {
                vm.get_stack(&thread_id)?.pop_frame()?;
                return Ok(ControlFlow::Break(()));
            }
            instruction => unimplemented!("instruction {:?}", instruction),
        }

        if !is_branch {
            vm.get_stack(&thread_id)?
                .cur_frame_mut()?
                .increment_pc(instruction_byte_size);
        }
        Ok(ControlFlow::Continue(()))
    }
    fn interpret_method(
        thread_id: ThreadId,
        method_id: MethodId,
        vm: &mut VirtualMachine,
    ) -> Result<(), JvmError> {
        let code_ptr = vm.method_area.get_method(&method_id).get_code()? as *const [u8];
        loop {
            // SAFETY: code_ptr is valid as long as method exists in method area (always)
            // need to use pointer to avoid borrow checker issues
            let code = unsafe { &*code_ptr };
            let pc = vm.get_stack(&thread_id)?.pc()?;
            let instruction = Instruction::new_at(code, pc)?;

            if let ControlFlow::Break(_) = Self::interpret_instruction(thread_id, instruction, vm)?
            {
                break;
            }
        }
        Ok(())
    }

    fn run_method(
        thread_id: ThreadId,
        method_id: MethodId,
        vm: &mut VirtualMachine,
        args: Vec<Value>,
    ) -> Result<(), JvmError> {
        let (max_stack, max_locals) = vm
            .method_area
            .get_method(&method_id)
            .get_frame_attributes()?;
        let frame = JavaFrame::new(method_id, max_stack, max_locals, args);
        vm.get_stack(&thread_id)?.push_frame(frame)?;
        Self::interpret_method(thread_id, method_id, vm)
    }
    fn ensure_initialized(
        thread_id: ThreadId,
        class_id: Option<ClassId>,
        vm: &mut VirtualMachine,
    ) -> Result<(), JvmError> {
        // 💡 Combined the `if let` and the inner `if`
        if let Some(class_id) = class_id
            && !vm
                .method_area
                .get_class(&class_id)
                .is_initialized_or_initializing()
        {
            vm.method_area.get_class(&class_id).set_initializing();
            Self::ensure_initialized(
                thread_id,
                vm.method_area.get_class(&class_id).get_super_id(),
                vm,
            )?;
            if let Ok(clinit_method_id) = vm.method_area.get_class(&class_id).get_special_method_id(
                // TODO: make method key registry?
                &MethodKey {
                    name: vm.method_area.string_interner.get_or_intern("<clinit>"),
                    desc: vm.method_area.string_interner.get_or_intern("()V"),
                },
            ) {
                Self::run_method(thread_id, clinit_method_id, vm, vec![])?;
            }
            vm.method_area.get_class(&class_id).set_initialized();
        }
        Ok(())
    }

    pub fn invoke_static_method(
        thread_id: ThreadId,
        method_id: MethodId,
        vm: &mut VirtualMachine,
        args: Vec<Value>,
    ) -> Result<(), JvmError> {
        let class_id = vm.method_area.get_method(&method_id).class_id();
        Self::ensure_initialized(thread_id, Some(class_id), vm)?;
        Ok(())
    }
}
