use crate::rt::constant_pool::RuntimeConstant;
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
            Instruction::AconstNull => {
                vm.get_stack(&thread_id)?.push_operand(Value::Null)?;
            }
            Instruction::Aload0 => {
                let value = *vm.get_stack(&thread_id)?.cur_frame()?.get_local(0)?;
                vm.get_stack(&thread_id)?.push_operand(value)?;
            }
            Instruction::Aload1 => {
                let value = *vm.get_stack(&thread_id)?.cur_frame()?.get_local(1)?;
                vm.get_stack(&thread_id)?.push_operand(value)?;
            }
            Instruction::Aload2 => {
                let value = *vm.get_stack(&thread_id)?.cur_frame()?.get_local(2)?;
                vm.get_stack(&thread_id)?.push_operand(value)?;
            }
            Instruction::Aload3 => {
                let value = *vm.get_stack(&thread_id)?.cur_frame()?.get_local(3)?;
                vm.get_stack(&thread_id)?.push_operand(value)?;
            }
            Instruction::Aload(pos) => {
                let value = *vm.get_stack(&thread_id)?.cur_frame()?.get_local(pos)?;
                vm.get_stack(&thread_id)?.push_operand(value)?;
            }
            Instruction::Bipush(value) => {
                vm.get_stack(&thread_id)?
                    .push_operand(Value::Integer(value as i32))?;
            }
            Instruction::Dup => {
                vm.get_stack(&thread_id)?.dup_top()?;
            }
            Instruction::Getstatic(idx) => {
                let cur_frame_method_id = vm.get_stack(&thread_id)?.cur_frame()?.method_id();
                let target_field_view = vm
                    .method_area
                    .get_cp_by_method_id(&cur_frame_method_id)?
                    .get_field_view(&idx, vm.interner())?;
                let target_class_id = vm
                    .method_area
                    .get_class_id_or_load(target_field_view.class_sym)?;
                Self::ensure_initialized(thread_id, Some(target_class_id), vm)?;
                let value = vm
                    .method_area
                    .get_instance_class(&target_class_id)?
                    .get_static_field_value(&target_field_view.name_and_type.into())?;
                vm.get_stack(&thread_id)?.push_operand(value)?;
            }
            Instruction::Iconst0 => {
                vm.get_stack(&thread_id)?.push_operand(Value::Integer(0))?;
            }
            Instruction::Iconst1 => {
                vm.get_stack(&thread_id)?.push_operand(Value::Integer(1))?;
            }
            Instruction::Iconst2 => {
                vm.get_stack(&thread_id)?.push_operand(Value::Integer(2))?;
            }
            Instruction::Iconst3 => {
                vm.get_stack(&thread_id)?.push_operand(Value::Integer(3))?;
            }
            Instruction::Iconst4 => {
                vm.get_stack(&thread_id)?.push_operand(Value::Integer(4))?;
            }
            Instruction::Iconst5 => {
                vm.get_stack(&thread_id)?.push_operand(Value::Integer(5))?;
            }
            Instruction::IconstM1 => {
                vm.get_stack(&thread_id)?.push_operand(Value::Integer(-1))?;
            }
            Instruction::Iload0 => {
                let value = *vm.get_stack(&thread_id)?.cur_frame()?.get_local(0)?;
                vm.get_stack(&thread_id)?.push_operand(value)?;
            }
            Instruction::Iload1 => {
                let value = *vm.get_stack(&thread_id)?.cur_frame()?.get_local(1)?;
                vm.get_stack(&thread_id)?.push_operand(value)?;
            }
            Instruction::Iload2 => {
                let value = *vm.get_stack(&thread_id)?.cur_frame()?.get_local(2)?;
                vm.get_stack(&thread_id)?.push_operand(value)?;
            }
            Instruction::Iload3 => {
                let value = *vm.get_stack(&thread_id)?.cur_frame()?.get_local(3)?;
                vm.get_stack(&thread_id)?.push_operand(value)?;
            }
            Instruction::Iload(pos) => {
                let value = *vm.get_stack(&thread_id)?.cur_frame()?.get_local(pos)?;
                vm.get_stack(&thread_id)?.push_operand(value)?;
            }
            Instruction::Ldc(idx) | Instruction::LdcW(idx) | Instruction::Ldc2W(idx) => {
                let cur_method_id = vm.get_stack(&thread_id)?.cur_frame()?.method_id();
                let ldc_operand = {
                    let cp = vm.method_area.get_cp_by_method_id(&cur_method_id)?;
                    match cp.get_constant(&idx, vm.interner())? {
                        RuntimeConstant::Integer(val) => Value::Integer(*val),
                        RuntimeConstant::Float(val) => Value::Float(*val),
                        RuntimeConstant::Long(val) => Value::Long(*val),
                        RuntimeConstant::Double(val) => Value::Double(*val),
                        RuntimeConstant::Class(class_entry) => {
                            unimplemented!("Mirrors aren't supported yet")
                        }
                        RuntimeConstant::String(str_entry) => {
                            todo!()
                        }
                        _ => unimplemented!(),
                    }
                };
            }
            Instruction::New(idx) => {
                let cur_frame_method_id = vm.get_stack(&thread_id)?.cur_frame()?.method_id();
                let target_class_name = vm
                    .method_area
                    .get_cp_by_method_id(&cur_frame_method_id)?
                    .get_class_sym(&idx, vm.interner())?;
                let target_class_id = vm.method_area.get_class_id_or_load(target_class_name)?;
                Self::ensure_initialized(thread_id, Some(target_class_id), vm)?;
                let instance_ref = vm
                    .heap
                    .alloc_instance(&mut vm.method_area, target_class_id)?;
                vm.get_stack(&thread_id)?
                    .push_operand(Value::Ref(instance_ref))?;
            }
            Instruction::Pop => {
                vm.get_stack(&thread_id)?.pop_operand()?;
            }
            Instruction::Putfield(idx) => {
                let value = vm.get_stack(&thread_id)?.pop_operand()?;
                let target_obj_ref = vm.get_stack(&thread_id)?.pop_obj_val()?;
                let cur_frame_method_id = vm.get_stack(&thread_id)?.cur_frame()?.method_id();
                let field_view = vm
                    .method_area
                    .get_cp_by_method_id(&cur_frame_method_id)?
                    .get_field_view(&idx, vm.interner())?;
                let target_class_id = vm.method_area.get_class_id_or_load(field_view.class_sym)?;
                let target_offset = vm
                    .method_area
                    .get_instance_class(&target_class_id)?
                    .get_instance_field_offset(&field_view.name_and_type.into())?
                    as usize;
                vm.heap
                    .write_instance_field(target_obj_ref, target_offset, value)?;
            }
            Instruction::Putstatic(idx) => {
                let value = vm.get_stack(&thread_id)?.pop_operand()?;
                let cur_frame_method_id = vm.get_stack(&thread_id)?.cur_frame()?.method_id();
                let target_field_view = vm
                    .method_area
                    .get_cp_by_method_id(&cur_frame_method_id)?
                    .get_field_view(&idx, vm.interner())?;
                let target_class_id = vm
                    .method_area
                    .get_class_id_or_load(target_field_view.class_sym)?;
                Self::ensure_initialized(thread_id, Some(target_class_id), vm)?;
                vm.method_area
                    .get_instance_class(&target_class_id)?
                    .set_static_field_value(&target_field_view.name_and_type.into(), value)?;
            }
            Instruction::InvokeSpecial(idx) => {
                let cur_frame_method_id = vm.get_stack(&thread_id)?.cur_frame()?.method_id();
                let target_method_view = vm
                    .method_area
                    .get_cp_by_method_id(&cur_frame_method_id)?
                    .get_method_view(&idx, vm.interner())?;
                let target_class_id = vm
                    .method_area
                    .get_class_id_or_load(target_method_view.class_sym)?;
                let target_method_id = vm
                    .method_area
                    .get_instance_class(&target_class_id)?
                    .get_special_method_id(&target_method_view.name_and_type.into())?;
                let args = Self::prepare_method_args(thread_id, target_method_id, vm)?;
                Self::run_method(thread_id, target_method_id, vm, args)?;
            }
            Instruction::InvokeStatic(idx) => {
                let cur_frame_method_id = vm.get_stack(&thread_id)?.cur_frame()?.method_id();
                let target_method_view = vm
                    .method_area
                    .get_cp_by_method_id(&cur_frame_method_id)?
                    .get_method_view(&idx, vm.interner())?;
                let target_class_id = vm
                    .method_area
                    .get_class_id_or_load(target_method_view.class_sym)?;
                Self::ensure_initialized(thread_id, Some(target_class_id), vm)?;
                let target_method_id = vm
                    .method_area
                    .get_instance_class(&target_class_id)?
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
        let mut args_count = vm
            .method_area
            .get_method_descriptor_by_method_id(&method_id)
            .params
            .len();
        if !vm.method_area.get_method(&method_id).is_static() {
            args_count += 1;
        }
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
            && vm.method_area.is_instance_class(&class_id) && !vm
                .method_area
                .get_instance_class(&class_id)?
                .is_initialized_or_initializing()
        {
            vm.method_area.get_instance_class(&class_id)?.set_initializing();
            Self::ensure_initialized(
                thread_id,
                vm.method_area.get_instance_class(&class_id)?.get_super_id(),
                vm,
            )?;
            if let Ok(clinit_method_id) = vm.method_area.get_instance_class(&class_id)?.get_special_method_id(
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
                        .get_instance_class(&class_id)?
                        .get_special_method_id(&MethodKey {
                            name: vm.interner().get_or_intern("initPhase1"),
                            desc: vm.interner().get_or_intern("()V"),
                        })
                        .and_then(|init_sys_method_id| {
                            Self::run_method(thread_id, init_sys_method_id, vm, vec![])
                        })?;
                }
            }
            vm.method_area.get_instance_class(&class_id)?.set_initialized();
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
