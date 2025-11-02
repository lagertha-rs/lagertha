use crate::stack::JavaFrame;
use crate::{ClassId, MethodId, MethodKey, ThreadId, VirtualMachine};
use common::error::JvmError;
use common::instruction::Instruction;
use common::jtype::Value;
use lasso::Interner;
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
            Instruction::AconstNull => {
                vm.get_stack(&thread_id)?.push_operand(Value::Null)?;
            }
            Instruction::Bipush(value) => {
                vm.get_stack(&thread_id)?
                    .push_operand(Value::Integer(value as i32))?;
            }
            Instruction::Getstatic(idx) => {
                let cur_frame_method_id = vm.get_stack(&thread_id)?.cur_frame()?.method_id();
                let target_field_view = vm
                    .method_area
                    .get_cp_by_method_id(&cur_frame_method_id)
                    .get_field(&idx, vm.interner())?;
                let target_class_id = vm
                    .method_area
                    .get_class_id_or_load(target_field_view.class_sym)?;
                Self::ensure_initialized(thread_id, Some(target_class_id), vm)?;
                let value = vm
                    .method_area
                    .get_class(&target_class_id)
                    .get_static_field_value(&target_field_view.name_and_type.into())?;
                vm.get_stack(&thread_id)?.push_operand(value)?;
            }
            Instruction::Putstatic(idx) => {
                let value = vm.get_stack(&thread_id)?.pop_operand()?;
                let cur_frame_method_id = vm.get_stack(&thread_id)?.cur_frame()?.method_id();
                let target_field_view = vm
                    .method_area
                    .get_cp_by_method_id(&cur_frame_method_id)
                    .get_field(&idx, vm.interner())?;
                let target_class_id = vm
                    .method_area
                    .get_class_id_or_load(target_field_view.class_sym)?;
                Self::ensure_initialized(thread_id, Some(target_class_id), vm)?;
                vm.method_area
                    .get_class(&target_class_id)
                    .set_static_field_value(&target_field_view.name_and_type.into(), value)?;
            }
            Instruction::InvokeStatic(idx) => {
                let cur_frame_method_id = vm.get_stack(&thread_id)?.cur_frame()?.method_id();
                let target_method_view = vm
                    .method_area
                    .get_cp_by_method_id(&cur_frame_method_id)
                    .get_method(&idx, vm.interner())?;
                let target_class_id = vm
                    .method_area
                    .get_class_id_or_load(target_method_view.class_sym)?;
                Self::ensure_initialized(thread_id, Some(target_class_id), vm)?;
                let target_method_id = vm
                    .method_area
                    .get_class(&target_class_id)
                    .get_special_method_id(&target_method_view.name_and_type.into())?;
                let args = Self::prepare_method_args(thread_id, target_method_id, vm)?;
                Self::invoke_static_method(thread_id, target_method_id, vm, args)?;
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

    fn prepare_method_args(
        thread_id: ThreadId,
        method_id: MethodId,
        vm: &mut VirtualMachine,
    ) -> Result<Vec<Value>, JvmError> {
        let args_count = vm
            .method_area
            .get_method_descriptor_by_method_id(&method_id)
            .params
            .len();
        // TODO: I saw somewhere a data structure with fixed capacity, that can avoid heap allocation
        let mut args = Vec::with_capacity(args_count);
        for _ in 0..args_count {
            args.push(vm.get_stack(&thread_id)?.pop_operand()?);
        }
        args.reverse();
        Ok(args)
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
        let method = vm.method_area.get_method(&method_id);
        if method.is_native() {
            let method_key = vm.method_area.build_fully_qualified_method_key(&method_id);
            let native = vm.native_registry.get(&method_key).unwrap();
            match native(vm, thread_id, args.as_slice()) {
                Ok(Some(ret_val)) => {
                    todo!()
                }
                Ok(None) => Ok(()),
                Err(e) => Err(e),
            }
        } else {
            let (max_stack, max_locals) = vm
                .method_area
                .get_method(&method_id)
                .get_frame_attributes()?;
            let frame = JavaFrame::new(method_id, max_stack, max_locals, args);
            vm.get_stack(&thread_id)?.push_frame(frame)?;
            Self::interpret_method(thread_id, method_id, vm)
        }
    }
    fn ensure_initialized(
        thread_id: ThreadId,
        class_id: Option<ClassId>,
        vm: &mut VirtualMachine,
    ) -> Result<(), JvmError> {
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
                    name: vm.interner().get_or_intern("<clinit>"),
                    desc: vm.interner().get_or_intern("()V"),
                },
            ) {
                Self::run_method(thread_id, clinit_method_id, vm, vec![])?;
                if vm
                    .method_area
                    // TODO: it calculates spur each time,
                    .get_class_id_by_name(&vm.interner().get_or_intern("java/lang/System"))
                    == class_id
                {
                    // TODO: make method key registry?
                    vm.method_area
                        .get_class(&class_id)
                        .get_special_method_id(&MethodKey {
                            name: vm.interner().get_or_intern("initPhase1"),
                            desc: vm.interner().get_or_intern("()V"),
                        })
                        .and_then(|init_sys_method_id| {
                            Self::run_method(thread_id, init_sys_method_id, vm, vec![])
                        })?;
                }
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
        Self::run_method(thread_id, method_id, vm, args)?;
        Ok(())
    }
}
