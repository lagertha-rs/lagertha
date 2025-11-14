use crate::rt::constant_pool::RuntimeConstant;
use crate::rt::{ClassLike, JvmClass};
use crate::vm::stack::{FrameType, JavaFrame, NativeFrame};
use crate::{
    ClassId, FieldKey, MethodId, MethodKey, ThreadId, VirtualMachine, build_exception,
    debug_log_instruction, throw_exception,
};
use common::error::{JavaExceptionFromJvm, JvmError};
use common::instruction::Instruction;
use common::jtype::Value;
use std::cmp::Ordering;
use std::ops::ControlFlow;
use tracing_log::log::warn;

pub struct Interpreter;

#[cfg_attr(feature = "hotpath", hotpath::measure_all)]
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
    ) -> Result<ControlFlow<Option<Value>>, JvmError> {
        let is_branch = instruction.is_branch();
        let instruction_byte_size = instruction.byte_size();

        debug_log_instruction!(&instruction, &thread_id);

        match instruction {
            Instruction::Athrow => {
                let exception_ref = vm.get_stack_mut(&thread_id)?.pop_obj_val()?;
                Err(JvmError::JavaExceptionThrown(exception_ref))?
            }
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
                let index = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                let array_addr = vm.get_stack_mut(&thread_id)?.pop_obj_val()?;
                vm.heap.write_array_element(array_addr, index, value)?;
            }
            Instruction::Bastore => {
                let value = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                let index = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                let array_addr = vm.get_stack_mut(&thread_id)?.pop_obj_val()?;
                vm.heap
                    .write_array_element(array_addr, index, Value::Integer(value & 0xFF))?;
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
                let value = *vm
                    .get_stack_mut(&thread_id)?
                    .cur_java_frame()?
                    .get_local(0)?;
                vm.get_stack_mut(&thread_id)?.push_operand(value)?;
            }
            Instruction::Aload1 => {
                let value = *vm
                    .get_stack_mut(&thread_id)?
                    .cur_java_frame()?
                    .get_local(1)?;
                vm.get_stack_mut(&thread_id)?.push_operand(value)?;
            }
            Instruction::Aload2 => {
                let value = *vm
                    .get_stack_mut(&thread_id)?
                    .cur_java_frame()?
                    .get_local(2)?;
                vm.get_stack_mut(&thread_id)?.push_operand(value)?;
            }
            Instruction::Aload3 => {
                let value = *vm
                    .get_stack_mut(&thread_id)?
                    .cur_java_frame()?
                    .get_local(3)?;
                vm.get_stack_mut(&thread_id)?.push_operand(value)?;
            }
            Instruction::Aload(pos) => {
                let value = *vm
                    .get_stack_mut(&thread_id)?
                    .cur_java_frame()?
                    .get_local(pos)?;
                vm.get_stack_mut(&thread_id)?.push_operand(value)?;
            }
            Instruction::Anewarray(idx) => {
                let size = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                if size < 0 {
                    throw_exception!(NegativeArraySizeException, size.to_string())?
                }
                let cur_frame_method_id =
                    vm.get_stack_mut(&thread_id)?.cur_java_frame()?.method_id();
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
                    .write_array_element(array_ref, index, Value::Integer(value))?;
            }
            Instruction::Dadd => {
                let v2 = vm.get_stack_mut(&thread_id)?.pop_double_val()?;
                let v1 = vm.get_stack_mut(&thread_id)?.pop_double_val()?;
                vm.get_stack_mut(&thread_id)?
                    .push_operand(Value::Double(v1 + v2))?;
            }
            Instruction::Dconst0 => {
                vm.get_stack_mut(&thread_id)?
                    .push_operand(Value::Double(0.0))?;
            }
            Instruction::Dconst1 => {
                vm.get_stack_mut(&thread_id)?
                    .push_operand(Value::Double(1.0))?;
            }
            Instruction::Dup => {
                vm.get_stack_mut(&thread_id)?.dup_top()?;
            }
            Instruction::Dup2 => match vm.get_stack(&thread_id)?.peek()? {
                Value::Long(_) | Value::Double(_) => {
                    let value = *vm.get_stack(&thread_id)?.peek()?;
                    vm.get_stack_mut(&thread_id)?.push_operand(value)?;
                }
                _ => {
                    let value1 = *vm.get_stack(&thread_id)?.peek()?;
                    let value2 = *vm.get_stack(&thread_id)?.peek_at(1)?;
                    vm.get_stack_mut(&thread_id)?.push_operand(value2)?;
                    vm.get_stack_mut(&thread_id)?.push_operand(value1)?;
                }
            },
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
                let cur_frame_method_id =
                    vm.get_stack_mut(&thread_id)?.cur_java_frame()?.method_id();
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
                let cur_frame_method_id =
                    vm.get_stack_mut(&thread_id)?.cur_java_frame()?.method_id();
                let target_field_view = vm
                    .method_area
                    .get_cp_by_method_id(&cur_frame_method_id)?
                    .get_field_view(&idx, vm.interner())?;
                let target_class_id = vm
                    .method_area
                    .get_class_id_or_load(target_field_view.class_sym)?;
                Self::ensure_initialized(thread_id, Some(target_class_id), vm)?;
                let field_key: FieldKey = target_field_view.name_and_type.into();
                let actual_static_field_class_id = vm
                    .method_area
                    .resolve_static_field_actual_class_id(target_class_id, &field_key)?;
                let value = vm
                    .method_area
                    .get_class(&actual_static_field_class_id)
                    .get_static_field_value(&field_key)?;
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
                    throw_exception!(ArithmeticException, "/ by zero")?
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
            Instruction::Lconst0 => {
                vm.get_stack_mut(&thread_id)?.push_operand(Value::Long(0))?;
            }
            Instruction::Lconst1 => {
                vm.get_stack_mut(&thread_id)?.push_operand(Value::Long(1))?;
            }
            Instruction::Lookupswitch(switch) => {
                let key = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                let pc = vm.get_stack(&thread_id)?.pc()?;
                let target_offset = match switch.pairs.binary_search_by_key(&key, |p| p.0) {
                    Ok(i) => switch.pairs[i].1,
                    Err(_) => switch.default_offset,
                };
                let new_pc = Self::branch32(pc, target_offset);
                *vm.get_stack_mut(&thread_id)?.pc_mut()? = new_pc;
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
                let value = *vm
                    .get_stack_mut(&thread_id)?
                    .cur_java_frame()?
                    .get_local(0)?;
                vm.get_stack_mut(&thread_id)?.push_operand(value)?;
            }
            Instruction::Iload1 => {
                let value = *vm
                    .get_stack_mut(&thread_id)?
                    .cur_java_frame()?
                    .get_local(1)?;
                vm.get_stack_mut(&thread_id)?.push_operand(value)?;
            }
            Instruction::Iload2 => {
                let value = *vm
                    .get_stack_mut(&thread_id)?
                    .cur_java_frame()?
                    .get_local(2)?;
                vm.get_stack_mut(&thread_id)?.push_operand(value)?;
            }
            Instruction::Iload3 => {
                let value = *vm
                    .get_stack_mut(&thread_id)?
                    .cur_java_frame()?
                    .get_local(3)?;
                vm.get_stack_mut(&thread_id)?.push_operand(value)?;
            }
            Instruction::Iload(pos) => {
                let value = *vm
                    .get_stack_mut(&thread_id)?
                    .cur_java_frame()?
                    .get_local(pos)?;
                vm.get_stack_mut(&thread_id)?.push_operand(value)?;
            }
            Instruction::InvokeVirtual(idx) => {
                let cur_frame_method_id =
                    vm.get_stack_mut(&thread_id)?.cur_java_frame()?.method_id();
                let target_method_view = vm
                    .method_area
                    .get_cp_by_method_id(&cur_frame_method_id)?
                    .get_method_view(&idx, vm.interner())?;
                let method_key: MethodKey = target_method_view.name_and_type.into();

                let target_method_desc_id = vm
                    .method_area
                    .get_or_new_method_descriptor_id(&method_key.desc)
                    .unwrap();
                let arg_count = vm
                    .method_area
                    .get_method_descriptor(&target_method_desc_id)
                    .params
                    .len()
                    + 1;

                let object_ref = vm
                    .get_stack(&thread_id)?
                    .peek_at(arg_count - 1)?
                    .as_obj_ref()?;
                let actual_class_id = vm.heap.get_class_id(&object_ref)?;

                let target_method_id = vm
                    .method_area
                    .get_class(&actual_class_id)
                    .get_vtable_method_id(&method_key)?;
                let args = Self::prepare_method_args(thread_id, target_method_id, vm)?;
                Self::invoke_method_internal(thread_id, target_method_id, args, vm)?;
            }
            Instruction::Instanceof(idx) => {
                let cur_frame_method_id =
                    vm.get_stack_mut(&thread_id)?.cur_java_frame()?.method_id();
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
                    throw_exception!(ArithmeticException, "/ by zero")?
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
                let cur_method_id = vm.get_stack_mut(&thread_id)?.cur_java_frame()?.method_id();
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
                            let string_ref = vm
                                .heap
                                .get_str_from_pool_or_new(string_sym, &mut vm.method_area)?;
                            Value::Ref(string_ref)
                        }
                        _ => unimplemented!(),
                    }
                };
                vm.get_stack_mut(&thread_id)?.push_operand(ldc_operand)?;
            }
            Instruction::New(idx) => {
                let cur_frame_method_id =
                    vm.get_stack_mut(&thread_id)?.cur_java_frame()?.method_id();
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
                let cur_frame_method_id =
                    vm.get_stack_mut(&thread_id)?.cur_java_frame()?.method_id();
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
                let cur_frame_method_id =
                    vm.get_stack_mut(&thread_id)?.cur_java_frame()?.method_id();
                let target_field_view = vm
                    .method_area
                    .get_cp_by_method_id(&cur_frame_method_id)?
                    .get_field_view(&idx, vm.interner())?;
                let target_class_id = vm
                    .method_area
                    .get_class_id_or_load(target_field_view.class_sym)?;
                Self::ensure_initialized(thread_id, Some(target_class_id), vm)?;
                let field_key: FieldKey = target_field_view.name_and_type.into();
                let actual_static_field_class_id = vm
                    .method_area
                    .resolve_static_field_actual_class_id(target_class_id, &field_key)?;
                vm.method_area
                    .get_class_like(&actual_static_field_class_id)?
                    .set_static_field_value(&field_key, value)?;
            }
            Instruction::InvokeInterface(idx, count) => {
                let cur_frame_method_id =
                    vm.get_stack_mut(&thread_id)?.cur_java_frame()?.method_id();
                let target_method_view = vm
                    .method_area
                    .get_cp_by_method_id(&cur_frame_method_id)?
                    .get_interface_method_view(&idx, vm.interner())?;
                let object_ref = vm
                    .get_stack(&thread_id)?
                    .peek_at(count as usize - 1)?
                    .as_obj_ref()?;
                if target_method_view.class_sym
                    == vm
                        .interner()
                        .get_or_intern("jdk/internal/access/JavaLangRefAccess")
                    && target_method_view.name_and_type.name_sym
                        == vm.interner().get_or_intern("startThreads")
                {
                    warn!(
                        "TODO: Stub: Ignoring call to jdk/internal/access/JavaLangRefAccess.startThreads"
                    );
                    for _ in 0..count {
                        let _ = vm.get_stack_mut(&thread_id)?.pop_operand()?;
                    }
                } else {
                    let target_class_id = vm.heap.get_class_id(&object_ref)?;
                    let target_method_id = vm
                        .method_area
                        .get_instance_class(&target_class_id)?
                        .get_interface_method_id(&target_method_view.name_and_type.into())?;
                    let args = Self::prepare_method_args(thread_id, target_method_id, vm)?;
                    Self::invoke_method_internal(thread_id, target_method_id, args, vm)?;
                }
            }
            Instruction::InvokeSpecial(idx) => {
                let cur_frame_method_id =
                    vm.get_stack_mut(&thread_id)?.cur_java_frame()?.method_id();
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
                Self::invoke_method_internal(thread_id, target_method_id, args, vm)?;
            }
            Instruction::InvokeStatic(idx) => {
                let cur_frame_method_id =
                    vm.get_stack_mut(&thread_id)?.cur_java_frame()?.method_id();
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
            Instruction::Lload0 => {
                let value = *vm
                    .get_stack_mut(&thread_id)?
                    .cur_java_frame()?
                    .get_local(0)?;
                vm.get_stack_mut(&thread_id)?.push_operand(value)?;
            }
            Instruction::Lload1 => {
                let value = *vm
                    .get_stack_mut(&thread_id)?
                    .cur_java_frame()?
                    .get_local(1)?;
                vm.get_stack_mut(&thread_id)?.push_operand(value)?;
            }
            Instruction::Lload2 => {
                let value = *vm
                    .get_stack_mut(&thread_id)?
                    .cur_java_frame()?
                    .get_local(2)?;
                vm.get_stack_mut(&thread_id)?.push_operand(value)?;
            }
            Instruction::Lload3 => {
                let value = *vm
                    .get_stack_mut(&thread_id)?
                    .cur_java_frame()?
                    .get_local(3)?;
                vm.get_stack_mut(&thread_id)?.push_operand(value)?;
            }
            Instruction::Lload(pos) => {
                let value = *vm
                    .get_stack_mut(&thread_id)?
                    .cur_java_frame()?
                    .get_local(pos)?;
                vm.get_stack_mut(&thread_id)?.push_operand(value)?;
            }
            Instruction::Lshl => {
                let v2 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                let v1 = vm.get_stack_mut(&thread_id)?.pop_long_val()?;
                let shift = (v2 & 0x3F) as u32;
                let result = v1.wrapping_shl(shift);
                vm.get_stack_mut(&thread_id)?
                    .push_operand(Value::Long(result))?;
            }
            Instruction::Lstore0 => {
                let value = vm.get_stack_mut(&thread_id)?.pop_long()?;
                vm.get_stack_mut(&thread_id)?.set_local(0, value)?;
            }
            Instruction::Lstore1 => {
                let value = vm.get_stack_mut(&thread_id)?.pop_long()?;
                vm.get_stack_mut(&thread_id)?.set_local(1, value)?;
            }
            Instruction::Lstore2 => {
                let value = vm.get_stack_mut(&thread_id)?.pop_long()?;
                vm.get_stack_mut(&thread_id)?.set_local(2, value)?;
            }
            Instruction::Lstore3 => {
                let value = vm.get_stack_mut(&thread_id)?.pop_long()?;
                vm.get_stack_mut(&thread_id)?.set_local(3, value)?;
            }
            Instruction::Lstore(idx) => {
                let value = vm.get_stack_mut(&thread_id)?.pop_long()?;
                vm.get_stack_mut(&thread_id)?
                    .set_local(idx as usize, value)?;
            }
            Instruction::Iastore => {
                let value = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                let index = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                let array_ref = vm.get_stack_mut(&thread_id)?.pop_obj_val()?;
                vm.heap
                    .write_array_element(array_ref, index, Value::Integer(value))?;
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
            Instruction::Saload => {
                let index = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                let array_ref = vm.get_stack_mut(&thread_id)?.pop_obj_val()?;
                let value = vm
                    .heap
                    .get_array(&array_ref)?
                    .get_element(index)?
                    .as_int()?;
                vm.get_stack_mut(&thread_id)?
                    .push_operand(Value::Integer(value))?;
            }
            Instruction::Sastore => {
                let value = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                let index = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                let array_ref = vm.get_stack_mut(&thread_id)?.pop_obj_val()?;
                vm.heap
                    .write_array_element(array_ref, index, Value::Integer(value))?;
            }
            Instruction::Sipush(value) => {
                vm.get_stack_mut(&thread_id)?
                    .push_operand(Value::Integer(value as i32))?;
            }
            Instruction::TableSwitch(switch) => {
                let index = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
                let pc = vm.get_stack(&thread_id)?.pc()?;
                let target_offset = if index < switch.low || index > switch.high {
                    switch.default_offset
                } else {
                    let idx = (index - switch.low) as usize;
                    switch.offsets[idx]
                };
                let new_pc = Self::branch32(pc, target_offset);
                *vm.get_stack_mut(&thread_id)?.pc_mut()? = new_pc;
            }
            Instruction::Return => {
                return Ok(ControlFlow::Break(None));
            }
            Instruction::Ireturn => {
                let ret_value = vm.get_stack_mut(&thread_id)?.pop_int()?;
                return Ok(ControlFlow::Break(Some(ret_value)));
            }
            Instruction::Areturn => {
                let value = vm.get_stack_mut(&thread_id)?.pop_nullable_ref()?;
                return Ok(ControlFlow::Break(Some(value)));
            }
            Instruction::Lreturn => {
                let value = vm.get_stack_mut(&thread_id)?.pop_long()?;
                return Ok(ControlFlow::Break(Some(value)));
            }
            Instruction::Freturn => {
                let value = vm.get_stack_mut(&thread_id)?.pop_float()?;
                return Ok(ControlFlow::Break(Some(value)));
            }
            Instruction::Monitorenter => {
                let _obj = vm.get_stack_mut(&thread_id)?.pop_obj_val()?;
            }
            Instruction::Monitorexit => {
                let _obj = vm.get_stack_mut(&thread_id)?.pop_obj_val()?;
            }
            instruction => unimplemented!("instruction {:?}", instruction),
        }

        if !is_branch {
            vm.get_stack_mut(&thread_id)?
                .cur_java_frame_mut()?
                .increment_pc(instruction_byte_size);
        }
        Ok(ControlFlow::Continue(()))
    }

    //TODO: need to move it, refactor and it will still probably will not work for catch
    fn allocate_and_throw(
        thread_id: ThreadId,
        exception: JavaExceptionFromJvm,
        vm: &mut VirtualMachine,
    ) -> Result<(), JvmError> {
        let exception_ref = exception.as_reference();
        let class_id = vm
            .method_area
            // TODO: fix interner usage, replace with direct symbol
            .get_class_id_or_load(vm.interner().get_or_intern(exception_ref.class))?;
        let instance = vm.heap.alloc_instance(&mut vm.method_area, class_id)?;
        let method_id = vm
            .method_area
            .get_instance_class(&class_id)?
            .get_special_method_id(
                // TODO: fix interner usage, replace with direct symbol
                &MethodKey {
                    name: vm.interner().get_or_intern(exception_ref.name),
                    desc: vm.interner().get_or_intern(exception_ref.descriptor),
                },
            )?;
        let params = if let Some(msg) = exception.get_message() {
            vec![
                Value::Ref(instance),
                Value::Ref(vm.heap.alloc_string(msg, &mut vm.method_area)?),
            ]
        } else {
            vec![Value::Ref(instance)]
        };
        Self::invoke_method_internal(thread_id, method_id, params, vm)?;
        Err(JvmError::JavaExceptionThrown(instance))
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
    ) -> Result<Option<Value>, JvmError> {
        let code_ptr = vm.method_area.get_method(&method_id).get_code()? as *const [u8];
        loop {
            // SAFETY: code_ptr is valid as long as method exists in method area (always)
            // need to use pointer to avoid borrow checker issues
            let code = unsafe { &*code_ptr };
            let pc = vm.get_stack_mut(&thread_id)?.pc()?;
            let instruction = Instruction::new_at(code, pc)?;

            match Self::interpret_instruction(thread_id, instruction, vm) {
                Ok(flow) => {
                    if let ControlFlow::Break(res) = flow {
                        return Ok(res);
                    }
                }
                Err(e) => match e {
                    JvmError::JavaException(exception) => {
                        Self::allocate_and_throw(thread_id, exception, vm)?;
                    }
                    e => return Err(e),
                },
            }
        }
    }

    fn invoke_method_core(
        thread_id: ThreadId,
        method_id: MethodId,
        args: Vec<Value>,
        vm: &mut VirtualMachine,
    ) -> Result<Option<Value>, JvmError> {
        let method = vm.method_area.get_method(&method_id);
        if method.is_native() {
            let mut method_key = vm
                .method_area
                .build_fully_qualified_native_method_key(&method_id);
            // native instance method of array special handling (for now, only Object.clone)
            if !method.is_static() && vm.heap.is_array(&args[0].as_obj_ref()?)? {
                method_key.class = None;
            }
            let frame = NativeFrame::new(method_id);
            vm.get_stack_mut(&thread_id)?
                .push_frame(FrameType::NativeFrame(frame))?;
            let native = vm.native_registry.get(&method_key).ok_or(build_exception!(
                NoSuchMethodError,
                vm.pretty_method_not_found_message(&method_id)
            ))?;
            let native_res = native(vm, thread_id, args.as_slice())?;
            vm.get_stack_mut(&thread_id)?.pop_native_frame()?;
            Ok(native_res)
        } else {
            let (max_stack, max_locals) = vm
                .method_area
                .get_method(&method_id)
                .get_frame_attributes()?;
            let frame = JavaFrame::new(method_id, max_stack, max_locals, args);
            vm.get_stack_mut(&thread_id)?
                .push_frame(FrameType::JavaFrame(frame))?;
            let method_ret = Self::interpret_method(thread_id, method_id, vm)?;
            vm.get_stack_mut(&thread_id)?.pop_java_frame()?;
            Ok(method_ret)
        }
    }

    fn invoke_method_internal(
        thread_id: ThreadId,
        method_id: MethodId,
        args: Vec<Value>,
        vm: &mut VirtualMachine,
    ) -> Result<(), JvmError> {
        let method_ret = Self::invoke_method_core(thread_id, method_id, args, vm)?;
        if let Some(ret) = method_ret {
            vm.get_stack_mut(&thread_id)?.push_operand(ret)?;
        }
        Ok(())
    }

    pub fn run_method(
        thread_id: ThreadId,
        method_id: MethodId,
        args: Vec<Value>,
        vm: &mut VirtualMachine,
    ) -> Result<Option<Value>, JvmError> {
        Self::invoke_method_core(thread_id, method_id, args, vm)
    }

    fn interface_needs_initialization(
        interface_id: ClassId,
        vm: &VirtualMachine,
    ) -> Result<bool, JvmError> {
        let interface = vm.method_area.get_interface_class(&interface_id)?;

        Ok(interface.has_clinit()) //TODO: || interface.has_non_constant_static_fields()?
    }

    fn run_clinit_if_exists(
        thread_id: ThreadId,
        class_id: ClassId,
        vm: &mut VirtualMachine,
    ) -> Result<(), JvmError> {
        if let Some(&clinit_method_id) = vm
            .method_area
            .get_class_like(&class_id)?
            .get_clinit_method_id()
        {
            Self::invoke_method_internal(thread_id, clinit_method_id, vec![], vm)?;
        }

        Ok(())
    }

    fn run_init_phase1(
        thread_id: ThreadId,
        class_id: ClassId,
        vm: &mut VirtualMachine,
    ) -> Result<(), JvmError> {
        let init_phase1_method_id = vm
            .method_area
            .get_instance_class(&class_id)?
            .get_special_method_id(&vm.method_area.br().system_init_phase1_mk)?;
        Self::invoke_method_internal(thread_id, init_phase1_method_id, vec![], vm)?;
        Ok(())
    }

    fn ensure_initialized(
        thread_id: ThreadId,
        class_id: Option<ClassId>,
        vm: &mut VirtualMachine,
    ) -> Result<(), JvmError> {
        let Some(class_id) = class_id else {
            return Ok(());
        };

        {
            let class = vm.method_area.get_class_like(&class_id)?;

            if class.is_initialized_or_initializing() {
                return Ok(());
            }

            class.set_initializing();
        }

        match vm.method_area.get_class(&class_id) {
            JvmClass::Instance(inst) => {
                if let Some(super_id) = inst.get_super() {
                    Self::ensure_initialized(thread_id, Some(super_id), vm)?;
                }
                for interface_id in vm
                    .method_area
                    .get_instance_class(&class_id)?
                    .get_interfaces()?
                    .clone()
                {
                    if Self::interface_needs_initialization(interface_id, vm)? {
                        Self::ensure_initialized(thread_id, Some(interface_id), vm)?;
                    }
                }

                Self::run_clinit_if_exists(thread_id, class_id, vm)?;

                let cur_class_name = vm.method_area.get_instance_class(&class_id)?.name();

                // probably in the future we can skip it for something like arraycopy native method
                // I guess it doesn't need the whole class initialization
                if cur_class_name == vm.method_area.br().java_lang_system_sym {
                    Self::run_init_phase1(thread_id, class_id, vm)?;
                }
                //TODO: stub
                if vm.interner().resolve(&cur_class_name) == "jdk/internal/access/SharedSecrets" {
                    warn!(
                        "TODO: Stub: Setting jdk/internal/access/SharedSecrets javaLangRefAccess to non-null value, to avoid NPEs"
                    );
                    let ref_access_fk = FieldKey {
                        name: vm.interner().get_or_intern("javaLangRefAccess"),
                        desc: vm
                            .interner()
                            .get_or_intern("Ljdk/internal/access/JavaLangRefAccess;"),
                    };
                    vm.method_area
                        .get_instance_class(&class_id)?
                        .set_static_field_value(&ref_access_fk, Value::Ref(0))?;
                }
            }
            JvmClass::Interface(interface) => {
                for super_interface_id in interface.get_interfaces()?.clone() {
                    if Self::interface_needs_initialization(super_interface_id, vm)? {
                        Self::ensure_initialized(thread_id, Some(super_interface_id), vm)?;
                    }
                }

                Self::run_clinit_if_exists(thread_id, class_id, vm)?;
            }
            _ => {}
        }

        vm.method_area.get_class_like(&class_id)?.set_initialized();
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
        Self::invoke_method_internal(thread_id, method_id, args, vm)?;
        Ok(())
    }
}
