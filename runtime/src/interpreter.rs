use crate::rt::constant_pool::RuntimeConstant;
use crate::stack::JavaFrame;
use crate::{
    ClassId, MethodId, MethodKey, ThreadId, VirtualMachine, debug_log_instruction,
    debug_log_method, throw_exception,
};
use common::error::JvmError;
use common::instruction::Instruction;
use common::jtype::Value;
use std::cmp::Ordering;
use std::ops::ControlFlow;

pub struct Interpreter;

impl Interpreter {
    fn branch16(bci: usize, off: i16) -> usize {
        ((bci as isize) + (off as isize)) as usize
    }
    fn branch32(bci: usize, off: i32) -> usize {
        ((bci as isize) + (off as isize)) as usize
    }

    fn interpret_instruction(
        thread_id: ThreadId,
        instruction: Instruction,
        vm: &mut VirtualMachine,
    ) -> Result<ControlFlow<()>, JvmError> {
        let is_branch = instruction.is_branch();
        let instruction_byte_size = instruction.byte_size();

        debug_log_instruction!(&instruction, &thread_id);

        match instruction {
            Instruction::Aaload => {
                let index = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                let array_addr = vm.get_stack_mut(&thread_id)?.pop_obj_val()?;
                let value = *vm.heap.get_array(&array_addr)?.get_element(index)?;
                if matches!(value, Value::Ref(_) | Value::Null) {
                    vm.get_stack_mut(&thread_id)?.push_operand(value)?;
                } else {
                    panic!("Expected object reference in aaload");
                }
            }
            Instruction::Aastore => {
                let value = vm.get_stack_mut(&thread_id)?.pop_nullable_ref()?;
                let index = vm.get_stack_mut(&thread_id)?.pop_int_val()? as usize;
                let array_addr = vm.get_stack_mut(&thread_id)?.pop_obj_val()?;
                vm.heap.write_array_element(array_addr, index, value)?;
            }
            Instruction::Caload | Instruction::Baload | Instruction::Iaload => {
                let index = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                let array_addr = vm.get_stack_mut(&thread_id)?.pop_obj_val()?;
                let value = *vm.heap.get_array(&array_addr)?.get_element(index)?;
                if let Value::Integer(i) = value {
                    vm.get_stack_mut(&thread_id)?
                        .push_operand(Value::Integer(i & 0xFF))?;
                } else {
                    panic!("Expected integer value in caload");
                }
            }
            Instruction::Checkcast(_idx) => {
                //TODO: stub
                let object_ref = vm.get_stack_mut(&thread_id)?.pop_operand()?;
                vm.get_stack_mut(&thread_id)?.push_operand(object_ref)?;
            }
            Instruction::AconstNull => {
                vm.get_stack_mut(&thread_id)?.push_operand(Value::Null)?;
            }
            Instruction::Aload0 => {
                let value = *vm.get_stack_mut(&thread_id)?.cur_frame()?.get_local(0)?;
                vm.get_stack_mut(&thread_id)?.push_operand(value)?;
            }
            Instruction::Aload1 => {
                let value = *vm.get_stack_mut(&thread_id)?.cur_frame()?.get_local(1)?;
                vm.get_stack_mut(&thread_id)?.push_operand(value)?;
            }
            Instruction::Aload2 => {
                let value = *vm.get_stack_mut(&thread_id)?.cur_frame()?.get_local(2)?;
                vm.get_stack_mut(&thread_id)?.push_operand(value)?;
            }
            Instruction::Aload3 => {
                let value = *vm.get_stack_mut(&thread_id)?.cur_frame()?.get_local(3)?;
                vm.get_stack_mut(&thread_id)?.push_operand(value)?;
            }
            Instruction::Aload(pos) => {
                let value = *vm.get_stack_mut(&thread_id)?.cur_frame()?.get_local(pos)?;
                vm.get_stack_mut(&thread_id)?.push_operand(value)?;
            }
            Instruction::Anewarray(idx) => {
                let size = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                if size < 0 {
                    throw_exception!(NegativeArraySizeException, size.to_string())?
                }
                let cur_frame_method_id = vm.get_stack_mut(&thread_id)?.cur_frame()?.method_id();
                let target_array_sym = vm
                    .method_area
                    .get_cp_by_method_id(&cur_frame_method_id)?
                    .get_class_sym(&idx, vm.interner())?;
                let target_array_class_id =
                    vm.method_area.get_class_id_or_load(target_array_sym)?;
                let array_ref = vm.heap.alloc_array_with_default_value(
                    target_array_class_id,
                    Value::Null,
                    size as usize,
                )?;
                vm.get_stack_mut(&thread_id)?
                    .push_operand(Value::Ref(array_ref))?;
            }
            Instruction::ArrayLength => {
                let array_ref = vm.get_stack_mut(&thread_id)?.pop_obj_val()?;
                let length = vm.heap.get_array_len(&array_ref)?;
                vm.get_stack_mut(&thread_id)?
                    .push_operand(Value::Integer(length as i32))?;
            }
            Instruction::Astore0 => {
                let value = vm.get_stack_mut(&thread_id)?.pop_nullable_ref()?;
                vm.get_stack_mut(&thread_id)?.set_local(0, value)?;
            }
            Instruction::Astore1 => {
                let value = vm.get_stack_mut(&thread_id)?.pop_nullable_ref()?;
                vm.get_stack_mut(&thread_id)?.set_local(1, value)?;
            }
            Instruction::Astore2 => {
                let value = vm.get_stack_mut(&thread_id)?.pop_nullable_ref()?;
                vm.get_stack_mut(&thread_id)?.set_local(2, value)?;
            }
            Instruction::Astore3 => {
                let value = vm.get_stack_mut(&thread_id)?.pop_nullable_ref()?;
                vm.get_stack_mut(&thread_id)?.set_local(3, value)?;
            }
            Instruction::Astore(pos) => {
                let value = vm.get_stack_mut(&thread_id)?.pop_nullable_ref()?;
                vm.get_stack_mut(&thread_id)?
                    .set_local(pos as usize, value)?;
            }
            Instruction::Bipush(value) => {
                vm.get_stack_mut(&thread_id)?
                    .push_operand(Value::Integer(value as i32))?;
            }
            Instruction::Castore => {
                let value = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                let index = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                let array_ref = vm.get_stack_mut(&thread_id)?.pop_obj_val()?;
                vm.heap
                    .write_array_element(array_ref, index as usize, Value::Integer(value))?;
            }
            Instruction::Dup => {
                vm.get_stack_mut(&thread_id)?.dup_top()?;
            }
            Instruction::DupX1 => {
                let value1 = vm.get_stack_mut(&thread_id)?.pop_operand()?;
                let value2 = vm.get_stack_mut(&thread_id)?.pop_operand()?;
                vm.get_stack_mut(&thread_id)?.push_operand(value1)?;
                vm.get_stack_mut(&thread_id)?.push_operand(value2)?;
                vm.get_stack_mut(&thread_id)?.push_operand(value1)?;
            }
            Instruction::Fcmpl => {
                let v2 = vm.get_stack_mut(&thread_id)?.pop_float_val()?;
                let v1 = vm.get_stack_mut(&thread_id)?.pop_float_val()?;
                let res = match v1.total_cmp(&v2) {
                    Ordering::Less => -1,
                    Ordering::Equal => 0,
                    Ordering::Greater => 1,
                };
                vm.get_stack_mut(&thread_id)?
                    .push_operand(Value::Integer(res))?;
            }
            Instruction::Fcmpg => {
                let v2 = vm.get_stack_mut(&thread_id)?.pop_float_val()?;
                let v1 = vm.get_stack_mut(&thread_id)?.pop_float_val()?;
                let res = match v1.total_cmp(&v2) {
                    Ordering::Less => -1,
                    Ordering::Equal => 0,
                    Ordering::Greater => 1,
                };
                vm.get_stack_mut(&thread_id)?
                    .push_operand(Value::Integer(res))?;
            }
            Instruction::Fconst0 => {
                vm.get_stack_mut(&thread_id)?
                    .push_operand(Value::Float(0.0))?;
            }
            Instruction::Fconst1 => {
                vm.get_stack_mut(&thread_id)?
                    .push_operand(Value::Float(1.0))?;
            }
            Instruction::Fload0 => {
                let value = *vm.get_stack_mut(&thread_id)?.get_local_float(0)?;
                vm.get_stack_mut(&thread_id)?.push_operand(value)?;
            }
            Instruction::Fload1 => {
                let value = *vm.get_stack_mut(&thread_id)?.get_local_float(1)?;
                vm.get_stack_mut(&thread_id)?.push_operand(value)?;
            }
            Instruction::Fload2 => {
                let value = *vm.get_stack_mut(&thread_id)?.get_local_float(2)?;
                vm.get_stack_mut(&thread_id)?.push_operand(value)?;
            }
            Instruction::Fload3 => {
                let value = *vm.get_stack_mut(&thread_id)?.get_local_float(3)?;
                vm.get_stack_mut(&thread_id)?.push_operand(value)?;
            }
            Instruction::Fload(n) => {
                let value = *vm.get_stack_mut(&thread_id)?.get_local_float(n)?;
                vm.get_stack_mut(&thread_id)?.push_operand(value)?;
            }
            Instruction::Fstore(n) => {
                let value = vm.get_stack_mut(&thread_id)?.pop_float()?;
                vm.get_stack_mut(&thread_id)?.set_local(n as usize, value)?;
            }
            Instruction::Fstore0 => {
                let value = vm.get_stack_mut(&thread_id)?.pop_float()?;
                vm.get_stack_mut(&thread_id)?.set_local(0, value)?;
            }
            Instruction::Fstore1 => {
                let value = vm.get_stack_mut(&thread_id)?.pop_float()?;
                vm.get_stack_mut(&thread_id)?.set_local(1, value)?;
            }
            Instruction::Fstore2 => {
                let value = vm.get_stack_mut(&thread_id)?.pop_float()?;
                vm.get_stack_mut(&thread_id)?.set_local(2, value)?;
            }
            Instruction::Fstore3 => {
                let value = vm.get_stack_mut(&thread_id)?.pop_float()?;
                vm.get_stack_mut(&thread_id)?.set_local(3, value)?;
            }
            Instruction::Getfield(idx) => {
                let target_obj_ref = vm.get_stack_mut(&thread_id)?.pop_obj_val()?;
                let cur_frame_method_id = vm.get_stack_mut(&thread_id)?.cur_frame()?.method_id();
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
                let value = vm
                    .heap
                    .read_instance_field(&target_obj_ref, target_offset)?;
                vm.get_stack_mut(&thread_id)?.push_operand(value)?;
            }
            Instruction::Getstatic(idx) => {
                let cur_frame_method_id = vm.get_stack_mut(&thread_id)?.cur_frame()?.method_id();
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
                vm.get_stack_mut(&thread_id)?.push_operand(value)?;
            }
            Instruction::Goto(offset) => {
                let pc = vm.get_stack_mut(&thread_id)?.pc()?;
                let new_pc = Self::branch16(pc, offset);
                *vm.get_stack_mut(&thread_id)?.pc_mut()? = new_pc;
            }
            Instruction::Iadd => {
                let value2 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                let value1 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                let result = value1.wrapping_add(value2);
                vm.get_stack_mut(&thread_id)?
                    .push_operand(Value::Integer(result))?;
            }
            Instruction::Iconst0 => {
                vm.get_stack_mut(&thread_id)?
                    .push_operand(Value::Integer(0))?;
            }
            Instruction::Iconst1 => {
                vm.get_stack_mut(&thread_id)?
                    .push_operand(Value::Integer(1))?;
            }
            Instruction::Iconst2 => {
                vm.get_stack_mut(&thread_id)?
                    .push_operand(Value::Integer(2))?;
            }
            Instruction::Iconst3 => {
                vm.get_stack_mut(&thread_id)?
                    .push_operand(Value::Integer(3))?;
            }
            Instruction::Iconst4 => {
                vm.get_stack_mut(&thread_id)?
                    .push_operand(Value::Integer(4))?;
            }
            Instruction::Iconst5 => {
                vm.get_stack_mut(&thread_id)?
                    .push_operand(Value::Integer(5))?;
            }
            Instruction::IconstM1 => {
                vm.get_stack_mut(&thread_id)?
                    .push_operand(Value::Integer(-1))?;
            }
            Instruction::Idiv => {
                let value2 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                let value1 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                if value2 == 0 {
                    throw_exception!(ArithmeticException, "division by zero")?
                }
                let result = value1.wrapping_div(value2);
                vm.get_stack_mut(&thread_id)?
                    .push_operand(Value::Integer(result))?;
            }
            Instruction::IfEq(offset) => {
                let pc = vm.get_stack_mut(&thread_id)?.pc()?;
                let value = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                let new_pc = if value == 0 {
                    Self::branch16(pc, offset)
                } else {
                    pc + instruction.byte_size() as usize
                };
                *vm.get_stack_mut(&thread_id)?.pc_mut()? = new_pc;
            }
            Instruction::IfGe(offset) => {
                let pc = vm.get_stack_mut(&thread_id)?.pc()?;
                let value = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                let new_pc = if value >= 0 {
                    Self::branch16(pc, offset)
                } else {
                    pc + instruction.byte_size() as usize
                };
                *vm.get_stack_mut(&thread_id)?.pc_mut()? = new_pc;
            }
            Instruction::IfGt(offset) => {
                let pc = vm.get_stack_mut(&thread_id)?.pc()?;
                let value = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                let new_pc = if value > 0 {
                    Self::branch16(pc, offset)
                } else {
                    pc + instruction.byte_size() as usize
                };
                *vm.get_stack_mut(&thread_id)?.pc_mut()? = new_pc;
            }
            Instruction::Lcmp => {
                let v2 = vm.get_stack_mut(&thread_id)?.pop_long_val()?;
                let v1 = vm.get_stack_mut(&thread_id)?.pop_long_val()?;
                let res = match v1.cmp(&v2) {
                    Ordering::Less => -1,
                    Ordering::Equal => 0,
                    Ordering::Greater => 1,
                };
                vm.get_stack_mut(&thread_id)?
                    .push_operand(Value::Integer(res))?;
            }
            Instruction::Ifnull(offset) => {
                let pc = vm.get_stack_mut(&thread_id)?.pc()?;
                let value = vm.get_stack_mut(&thread_id)?.pop_nullable_ref_val()?;
                let new_pc = if value.is_none() {
                    Self::branch16(pc, offset)
                } else {
                    pc + instruction.byte_size() as usize
                };
                *vm.get_stack_mut(&thread_id)?.pc_mut()? = new_pc;
            }
            Instruction::IfIcmplt(offset) => {
                let pc = vm.get_stack_mut(&thread_id)?.pc()?;
                let v2 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                let v1 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;

                let new_pc = if v1 < v2 {
                    Self::branch16(pc, offset)
                } else {
                    pc + instruction.byte_size() as usize
                };
                *vm.get_stack_mut(&thread_id)?.pc_mut()? = new_pc;
            }
            Instruction::IfLe(offset) => {
                let pc = vm.get_stack_mut(&thread_id)?.pc()?;
                let value = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                let new_pc = if value <= 0 {
                    Self::branch16(pc, offset)
                } else {
                    pc + instruction.byte_size() as usize
                };
                *vm.get_stack_mut(&thread_id)?.pc_mut()? = new_pc;
            }
            Instruction::IfLt(offset) => {
                let pc = vm.get_stack_mut(&thread_id)?.pc()?;
                let value = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                let new_pc = if value < 0 {
                    Self::branch16(pc, offset)
                } else {
                    pc + instruction.byte_size() as usize
                };
                *vm.get_stack_mut(&thread_id)?.pc_mut()? = new_pc;
            }
            Instruction::IfAcmpEq(offset) => {
                let pc = vm.get_stack_mut(&thread_id)?.pc()?;
                let v2 = vm.get_stack_mut(&thread_id)?.pop_nullable_ref_val()?;
                let v1 = vm.get_stack_mut(&thread_id)?.pop_nullable_ref_val()?;
                let new_pc = if v1 == v2 {
                    Self::branch16(pc, offset)
                } else {
                    pc + instruction.byte_size() as usize
                };
                *vm.get_stack_mut(&thread_id)?.pc_mut()? = new_pc;
            }
            Instruction::IfAcmpNe(offset) => {
                let pc = vm.get_stack_mut(&thread_id)?.pc()?;
                let v2 = vm.get_stack_mut(&thread_id)?.pop_nullable_ref_val()?;
                let v1 = vm.get_stack_mut(&thread_id)?.pop_nullable_ref_val()?;
                let new_pc = if v1 != v2 {
                    Self::branch16(pc, offset)
                } else {
                    pc + instruction.byte_size() as usize
                };
                *vm.get_stack_mut(&thread_id)?.pc_mut()? = new_pc;
            }
            Instruction::IfIcmpne(offset) => {
                let pc = vm.get_stack_mut(&thread_id)?.pc()?;
                let v2 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                let v1 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                let new_pc = if v1 != v2 {
                    Self::branch16(pc, offset)
                } else {
                    pc + instruction.byte_size() as usize
                };
                *vm.get_stack_mut(&thread_id)?.pc_mut()? = new_pc;
            }
            Instruction::IfIcmpge(offset) => {
                let pc = vm.get_stack_mut(&thread_id)?.pc()?;
                let v2 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                let v1 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                let new_pc = if v1 >= v2 {
                    Self::branch16(pc, offset)
                } else {
                    pc + instruction.byte_size() as usize
                };
                *vm.get_stack_mut(&thread_id)?.pc_mut()? = new_pc;
            }
            Instruction::IfIcmpgt(offset) => {
                let pc = vm.get_stack_mut(&thread_id)?.pc()?;
                let v2 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                let v1 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                let new_pc = if v1 > v2 {
                    Self::branch16(pc, offset)
                } else {
                    pc + instruction.byte_size() as usize
                };
                *vm.get_stack_mut(&thread_id)?.pc_mut()? = new_pc;
            }
            Instruction::IfIcmpeq(offset) => {
                let pc = vm.get_stack_mut(&thread_id)?.pc()?;
                let v2 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                let v1 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                let new_pc = if v1 == v2 {
                    Self::branch16(pc, offset)
                } else {
                    pc + instruction.byte_size() as usize
                };
                *vm.get_stack_mut(&thread_id)?.pc_mut()? = new_pc;
            }
            Instruction::IfIcmple(offset) => {
                let pc = vm.get_stack_mut(&thread_id)?.pc()?;
                let v2 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                let v1 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                let new_pc = if v1 <= v2 {
                    Self::branch16(pc, offset)
                } else {
                    pc + instruction.byte_size() as usize
                };
                *vm.get_stack_mut(&thread_id)?.pc_mut()? = new_pc;
            }
            Instruction::Ifnonnull(offset) => {
                let pc = vm.get_stack_mut(&thread_id)?.pc()?;
                let obj = vm.get_stack_mut(&thread_id)?.pop_nullable_ref_val()?;
                let new_pc = if obj.is_some() {
                    Self::branch16(pc, offset)
                } else {
                    pc + instruction.byte_size() as usize
                };
                *vm.get_stack_mut(&thread_id)?.pc_mut()? = new_pc;
            }
            Instruction::IfNe(offset) => {
                let pc = vm.get_stack_mut(&thread_id)?.pc()?;
                let i = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                let new_pc = if i != 0 {
                    Self::branch16(pc, offset)
                } else {
                    pc + instruction.byte_size() as usize
                };
                *vm.get_stack_mut(&thread_id)?.pc_mut()? = new_pc;
            }
            Instruction::Iload0 => {
                let value = *vm.get_stack_mut(&thread_id)?.cur_frame()?.get_local(0)?;
                vm.get_stack_mut(&thread_id)?.push_operand(value)?;
            }
            Instruction::Iload1 => {
                let value = *vm.get_stack_mut(&thread_id)?.cur_frame()?.get_local(1)?;
                vm.get_stack_mut(&thread_id)?.push_operand(value)?;
            }
            Instruction::Iload2 => {
                let value = *vm.get_stack_mut(&thread_id)?.cur_frame()?.get_local(2)?;
                vm.get_stack_mut(&thread_id)?.push_operand(value)?;
            }
            Instruction::Iload3 => {
                let value = *vm.get_stack_mut(&thread_id)?.cur_frame()?.get_local(3)?;
                vm.get_stack_mut(&thread_id)?.push_operand(value)?;
            }
            Instruction::Iload(pos) => {
                let value = *vm.get_stack_mut(&thread_id)?.cur_frame()?.get_local(pos)?;
                vm.get_stack_mut(&thread_id)?.push_operand(value)?;
            }
            Instruction::InvokeVirtual(idx) => {
                let cur_frame_method_id = vm.get_stack_mut(&thread_id)?.cur_frame()?.method_id();
                let target_method_view = vm
                    .method_area
                    .get_cp_by_method_id(&cur_frame_method_id)?
                    .get_method_view(&idx, vm.interner())?;
                let target_class_id = vm
                    .method_area
                    .get_class_id_or_load(target_method_view.class_sym)?;
                /*
                println!("Invoking virtual method: {} of class {}",
                    vm.interner().resolve(&target_method_view.name_and_type.name_sym),
                         vm.interner().resolve(&target_method_view.class_sym));
                 */
                let target_method_id = vm
                    .method_area
                    .get_instance_class(&target_class_id)?
                    .get_vtable_method_id(&target_method_view.name_and_type.into())?;
                let args = Self::prepare_method_args(thread_id, target_method_id, vm)?;
                Self::run_method(thread_id, target_method_id, vm, args)?;
            }
            Instruction::Instanceof(idx) => {
                let cur_frame_method_id = vm.get_stack_mut(&thread_id)?.cur_frame()?.method_id();
                let class_name_sym = vm
                    .method_area
                    .get_cp_by_method_id(&cur_frame_method_id)?
                    .get_class_sym(&idx, vm.interner())?;

                let obj_ref = vm.get_stack_mut(&thread_id)?.pop_nullable_ref_val()?;
                if let Some(obj_ref) = obj_ref {
                    let target_class = vm.heap.get_class_id(&obj_ref)?;
                    let res = vm.method_area.instance_of(target_class, class_name_sym);
                    vm.get_stack_mut(&thread_id)?
                        .push_operand(Value::Integer(if res { 1 } else { 0 }))?;
                } else {
                    vm.get_stack_mut(&thread_id)?
                        .push_operand(Value::Integer(0))?;
                }
            }
            Instruction::Fmul => {
                let v2 = vm.get_stack_mut(&thread_id)?.pop_float_val()?;
                let v1 = vm.get_stack_mut(&thread_id)?.pop_float_val()?;
                vm.get_stack_mut(&thread_id)?
                    .push_operand(Value::Float(v1 * v2))?;
            }
            Instruction::Fdiv => {
                let v2 = vm.get_stack_mut(&thread_id)?.pop_float_val()?;
                let v1 = vm.get_stack_mut(&thread_id)?.pop_float_val()?;
                vm.get_stack_mut(&thread_id)?
                    .push_operand(Value::Float(v1 / v2))?;
            }
            Instruction::Irem => {
                let v2 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                let v1 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                if v2 == 0 {
                    throw_exception!(ArithmeticException, "Division by zero")?
                }
                vm.get_stack_mut(&thread_id)?
                    .push_operand(Value::Integer(v1 % v2))?;
            }
            Instruction::Ladd => {
                let v2 = vm.get_stack_mut(&thread_id)?.pop_long_val()?;
                let v1 = vm.get_stack_mut(&thread_id)?.pop_long_val()?;
                vm.get_stack_mut(&thread_id)?
                    .push_operand(Value::Long(v1.wrapping_add(v2)))?;
            }
            Instruction::Iand => {
                let v2 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                let v1 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                vm.get_stack_mut(&thread_id)?
                    .push_operand(Value::Integer(v1 & v2))?;
            }
            Instruction::Ior => {
                let v2 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                let v1 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                vm.get_stack_mut(&thread_id)?
                    .push_operand(Value::Integer(v1 | v2))?;
            }
            Instruction::Ixor => {
                let v2 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                let v1 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                vm.get_stack_mut(&thread_id)?
                    .push_operand(Value::Integer(v1 ^ v2))?;
            }
            Instruction::L2i => {
                let v = vm.get_stack_mut(&thread_id)?.pop_long_val()?;
                vm.get_stack_mut(&thread_id)?
                    .push_operand(Value::Integer(v as i32))?;
            }
            Instruction::L2f => {
                let v = vm.get_stack_mut(&thread_id)?.pop_long_val()?;
                vm.get_stack_mut(&thread_id)?
                    .push_operand(Value::Float(v as f32))?;
            }
            Instruction::D2l => {
                let v = vm.get_stack_mut(&thread_id)?.pop_double_val()?;
                vm.get_stack_mut(&thread_id)?
                    .push_operand(Value::Long(v as i64))?;
            }
            Instruction::F2i => {
                let v = vm.get_stack_mut(&thread_id)?.pop_float_val()?;
                vm.get_stack_mut(&thread_id)?
                    .push_operand(Value::Integer(v as i32))?;
            }
            Instruction::F2d => {
                let v = vm.get_stack_mut(&thread_id)?.pop_float_val()?;
                vm.get_stack_mut(&thread_id)?
                    .push_operand(Value::Double(v as f64))?;
            }
            Instruction::Ineg => {
                let v = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                vm.get_stack_mut(&thread_id)?
                    .push_operand(Value::Integer(-v))?;
            }
            Instruction::I2s => {
                let v = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                vm.get_stack_mut(&thread_id)?
                    .push_operand(Value::Integer((v as i16) as i32))?;
            }
            Instruction::I2c => {
                let v = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                vm.get_stack_mut(&thread_id)?
                    .push_operand(Value::Integer((v as u16) as i32))?;
            }
            Instruction::I2l => {
                let v = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                vm.get_stack_mut(&thread_id)?
                    .push_operand(Value::Long(v as i64))?;
            }
            Instruction::I2f => {
                let v = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                vm.get_stack_mut(&thread_id)?
                    .push_operand(Value::Float(v as f32))?;
            }
            Instruction::I2b => {
                let v = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                vm.get_stack_mut(&thread_id)?
                    .push_operand(Value::Integer((v as i8) as i32))?;
            }
            Instruction::Istore0 => {
                let value = vm.get_stack_mut(&thread_id)?.pop_int()?;
                vm.get_stack_mut(&thread_id)?.set_local(0, value)?;
            }
            Instruction::Istore1 => {
                let value = vm.get_stack_mut(&thread_id)?.pop_int()?;
                vm.get_stack_mut(&thread_id)?.set_local(1, value)?;
            }
            Instruction::Istore2 => {
                let value = vm.get_stack_mut(&thread_id)?.pop_int()?;
                vm.get_stack_mut(&thread_id)?.set_local(2, value)?;
            }
            Instruction::Istore3 => {
                let value = vm.get_stack_mut(&thread_id)?.pop_int()?;
                vm.get_stack_mut(&thread_id)?.set_local(3, value)?;
            }
            Instruction::Istore(idx) => {
                let value = vm.get_stack_mut(&thread_id)?.pop_int()?;
                vm.get_stack_mut(&thread_id)?
                    .set_local(idx as usize, value)?;
            }
            Instruction::Isub => {
                let v2 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                let v1 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                vm.get_stack_mut(&thread_id)?
                    .push_operand(Value::Integer(v1 - v2))?;
            }
            Instruction::Imul => {
                let v2 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                let v1 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                vm.get_stack_mut(&thread_id)?
                    .push_operand(Value::Integer(v1.wrapping_mul(v2)))?;
            }
            Instruction::Iinc(index, const_val) => {
                let value = vm.get_stack_mut(&thread_id)?.get_local_int_val(index)?;
                vm.get_stack_mut(&thread_id)?
                    .set_local(index as usize, Value::Integer(value + (const_val as i32)))?;
            }
            Instruction::Ldc(idx) | Instruction::LdcW(idx) | Instruction::Ldc2W(idx) => {
                let cur_method_id = vm.get_stack_mut(&thread_id)?.cur_frame()?.method_id();
                let ldc_operand = {
                    let cp = vm.method_area.get_cp_by_method_id(&cur_method_id)?;
                    match cp.get_constant(&idx, vm.interner())? {
                        RuntimeConstant::Integer(val) => Value::Integer(*val),
                        RuntimeConstant::Float(val) => Value::Float(*val),
                        RuntimeConstant::Long(val) => Value::Long(*val),
                        RuntimeConstant::Double(val) => Value::Double(*val),
                        RuntimeConstant::Class(class_entry) => {
                            let class_name_sym = class_entry.get_name_sym()?;
                            let class_id = vm.method_area.get_class_id_or_load(class_name_sym)?;
                            Value::Ref(
                                vm.method_area
                                    .get_mirror_ref_or_create(class_id, &mut vm.heap)?,
                            )
                        }
                        RuntimeConstant::String(str_entry) => {
                            let string_sym = str_entry.get_string_sym()?;
                            let string_ref =
                                vm.heap.get_or_new_string(string_sym, &mut vm.method_area)?;
                            Value::Ref(string_ref)
                        }
                        _ => unimplemented!(),
                    }
                };
                vm.get_stack_mut(&thread_id)?.push_operand(ldc_operand)?;
            }
            Instruction::New(idx) => {
                let cur_frame_method_id = vm.get_stack_mut(&thread_id)?.cur_frame()?.method_id();
                let target_class_name = vm
                    .method_area
                    .get_cp_by_method_id(&cur_frame_method_id)?
                    .get_class_sym(&idx, vm.interner())?;
                let target_class_id = vm.method_area.get_class_id_or_load(target_class_name)?;
                Self::ensure_initialized(thread_id, Some(target_class_id), vm)?;
                let instance_ref = vm
                    .heap
                    .alloc_instance(&mut vm.method_area, target_class_id)?;
                vm.get_stack_mut(&thread_id)?
                    .push_operand(Value::Ref(instance_ref))?;
            }
            Instruction::Newarray(array_type) => {
                let size = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                if size < 0 {
                    throw_exception!(NegativeArraySizeException, size.to_string())?
                }
                let class_id = vm
                    .method_area
                    .load_array_class(vm.interner().get_or_intern(array_type.descriptor()))?;
                let array_ref = vm.heap.alloc_array_with_default_value(
                    class_id,
                    array_type.default_value(),
                    size as usize,
                )?;
                vm.get_stack_mut(&thread_id)?
                    .push_operand(Value::Ref(array_ref))?;
            }
            Instruction::Pop => {
                vm.get_stack_mut(&thread_id)?.pop_operand()?;
            }
            Instruction::Putfield(idx) => {
                let value = vm.get_stack_mut(&thread_id)?.pop_operand()?;
                let target_obj_ref = vm.get_stack_mut(&thread_id)?.pop_obj_val()?;
                let cur_frame_method_id = vm.get_stack_mut(&thread_id)?.cur_frame()?.method_id();
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
                let value = vm.get_stack_mut(&thread_id)?.pop_operand()?;
                let cur_frame_method_id = vm.get_stack_mut(&thread_id)?.cur_frame()?.method_id();
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
                let cur_frame_method_id = vm.get_stack_mut(&thread_id)?.cur_frame()?.method_id();
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
                let cur_frame_method_id = vm.get_stack_mut(&thread_id)?.cur_frame()?.method_id();
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
            Instruction::Iushr => {
                let v2 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                let v1 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                let shift = (v2 & 0x1F) as u32;
                let result = ((v1 as u32) >> shift) as i32;
                vm.get_stack_mut(&thread_id)?
                    .push_operand(Value::Integer(result))?;
            }
            Instruction::Lshl => {
                let v2 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                let v1 = vm.get_stack_mut(&thread_id)?.pop_long_val()?;
                let shift = (v2 & 0x3F) as u32;
                let result = v1.wrapping_shl(shift);
                vm.get_stack_mut(&thread_id)?
                    .push_operand(Value::Long(result))?;
            }
            Instruction::Ishl => {
                let v2 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                let v1 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                let shift = (v2 & 0x1F) as u32;
                let result = v1.wrapping_shl(shift);
                vm.get_stack_mut(&thread_id)?
                    .push_operand(Value::Integer(result))?;
            }
            Instruction::Ishr => {
                let v2 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                let v1 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                let shift = (v2 & 0x1F) as u32;
                let result = v1.wrapping_shr(shift);
                vm.get_stack_mut(&thread_id)?
                    .push_operand(Value::Integer(result))?;
            }
            Instruction::Sipush(value) => {
                vm.get_stack_mut(&thread_id)?
                    .push_operand(Value::Integer(value as i32))?;
            }
            Instruction::Return => {
                vm.get_stack_mut(&thread_id)?.pop_frame()?;
                return Ok(ControlFlow::Break(()));
            }
            Instruction::Ireturn => {
                let ret_value = vm.get_stack_mut(&thread_id)?.pop_int()?;
                vm.get_stack_mut(&thread_id)?.pop_frame()?;
                vm.get_stack_mut(&thread_id)?.push_operand(ret_value)?;
                return Ok(ControlFlow::Break(()));
            }
            Instruction::Areturn => {
                let value = vm.get_stack_mut(&thread_id)?.pop_nullable_ref()?;
                vm.get_stack_mut(&thread_id)?.pop_frame()?;
                vm.get_stack_mut(&thread_id)?.push_operand(value)?;
                return Ok(ControlFlow::Break(()));
            }
            Instruction::Lreturn => {
                let value = vm.get_stack_mut(&thread_id)?.pop_long()?;
                vm.get_stack_mut(&thread_id)?.pop_frame()?;
                vm.get_stack_mut(&thread_id)?.push_operand(value)?;
                return Ok(ControlFlow::Break(()));
            }
            Instruction::Freturn => {
                let value = vm.get_stack_mut(&thread_id)?.pop_float()?;
                vm.get_stack_mut(&thread_id)?.pop_frame()?;
                vm.get_stack_mut(&thread_id)?.push_operand(value)?;
                return Ok(ControlFlow::Break(()));
            }
            instruction => unimplemented!("instruction {:?}", instruction),
        }

        if !is_branch {
            vm.get_stack_mut(&thread_id)?
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
            args.push(vm.get_stack_mut(&thread_id)?.pop_operand()?);
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
            let pc = vm.get_stack_mut(&thread_id)?.pc()?;
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
            if let Some(ret) = native(vm, thread_id, args.as_slice())? {
                vm.get_stack_mut(&thread_id)?.push_operand(ret)?;
            }
        } else {
            let (max_stack, max_locals) = vm
                .method_area
                .get_method(&method_id)
                .get_frame_attributes()?;
            let frame = JavaFrame::new(method_id, max_stack, max_locals, args);
            vm.get_stack_mut(&thread_id)?.push_frame(frame)?;
            Self::interpret_method(thread_id, method_id, vm)?;
        }
        Ok(())
    }
    fn ensure_initialized(
        thread_id: ThreadId,
        class_id: Option<ClassId>,
        vm: &mut VirtualMachine,
    ) -> Result<(), JvmError> {
        if let Some(class_id) = class_id
            && vm.method_area.is_instance_class(&class_id)
            && !vm
                .method_area
                .get_instance_class(&class_id)?
                .is_initialized_or_initializing()
        {
            vm.method_area
                .get_instance_class(&class_id)?
                .set_initializing();
            Self::ensure_initialized(
                thread_id,
                vm.method_area.get_instance_class(&class_id)?.get_super_id(),
                vm,
            )?;
            if let Ok(clinit_method_id) = vm
                .method_area
                .get_instance_class(&class_id)?
                .get_special_method_id(
                    // TODO: make method key registry?
                    &MethodKey {
                        name: vm.interner().get_or_intern("<clinit>"),
                        desc: vm.interner().get_or_intern("()V"),
                    },
                )
            {
                Self::run_method(thread_id, clinit_method_id, vm, vec![])?;
                //TODO: bad
                if vm
                    .interner()
                    .resolve(vm.method_area.get_class(&class_id).get_name())
                    == "java/lang/System"
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
            vm.method_area
                .get_instance_class(&class_id)?
                .set_initialized();
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
