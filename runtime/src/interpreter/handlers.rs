use crate::error::JvmError;
use crate::interpreter::Interpreter;
use crate::keys::{FieldKey, MethodKey, ThreadId};
use crate::rt::constant_pool::RuntimeConstant;
use crate::vm::Value;
use crate::{VirtualMachine, throw_exception};
use common::instruction::{ArrayType, LookupSwitchData, TableSwitchData};
use std::cmp::Ordering;
use tracing_log::log::warn;

fn branch16(bci: usize, off: i16) -> usize {
    ((bci as isize) + (off as isize)) as usize
}
fn branch32(bci: usize, off: i32) -> usize {
    ((bci as isize) + (off as isize)) as usize
}

#[inline]
pub(super) fn handle_athrow(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let exception_ref = vm.get_stack_mut(&thread_id)?.pop_obj_val()?;
    Err(JvmError::JavaExceptionThrown(exception_ref))
}

#[inline]
pub(super) fn handle_aaload(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let index = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    let array_addr = vm.get_stack_mut(&thread_id)?.pop_obj_val()?;
    let value = vm
        .heap
        .read_array_element(array_addr, index)?
        .as_nullable_obj_ref()?;
    vm.get_stack_mut(&thread_id)?
        .push_operand(value.map(Value::Ref).unwrap_or(Value::Null))
}

#[inline]
pub(super) fn handle_aastore(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let value = vm.get_stack_mut(&thread_id)?.pop_nullable_ref()?;
    let index = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    let array_addr = vm.get_stack_mut(&thread_id)?.pop_obj_val()?;
    vm.heap.write_array_element(array_addr, index, value)
}

#[inline]
pub(super) fn handle_bastore(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let value = vm.get_stack_mut(&thread_id)?.pop_int()?;
    let index = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    let array_addr = vm.get_stack_mut(&thread_id)?.pop_obj_val()?;
    vm.heap.write_array_element(array_addr, index, value)
}

#[inline]
pub(super) fn handle_iaload(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let index = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    let array_addr = vm.get_stack_mut(&thread_id)?.pop_obj_val()?;
    let value = vm.heap.read_array_element(array_addr, index)?;
    vm.get_stack_mut(&thread_id)?.push_operand(value)
}

#[inline]
pub(super) fn handle_caload(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let index = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    let array_addr = vm.get_stack_mut(&thread_id)?.pop_obj_val()?;
    let value = vm.heap.read_array_element(array_addr, index)?;
    vm.get_stack_mut(&thread_id)?.push_operand(value)
}

#[inline]
pub(super) fn handle_baload(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let index = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    let array_addr = vm.get_stack_mut(&thread_id)?.pop_obj_val()?;
    let value = vm.heap.read_array_element(array_addr, index)?;
    vm.get_stack_mut(&thread_id)?.push_operand(value)
}

// TODO: stub
#[inline]
pub(super) fn handle_checkcast(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
) -> Result<(), JvmError> {
    let object_ref = vm.get_stack_mut(&thread_id)?.pop_operand()?;
    vm.get_stack_mut(&thread_id)?.push_operand(object_ref)
}

#[inline]
pub(super) fn handle_aconst_null(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
) -> Result<(), JvmError> {
    vm.get_stack_mut(&thread_id)?.push_operand(Value::Null)
}

#[inline]
pub(super) fn handle_aload0(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let value = *vm
        .get_stack_mut(&thread_id)?
        .cur_java_frame()?
        .get_local(0)?;
    vm.get_stack_mut(&thread_id)?.push_operand(value)
}

#[inline]
pub(super) fn handle_aload1(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let value = *vm
        .get_stack_mut(&thread_id)?
        .cur_java_frame()?
        .get_local(1)?;
    vm.get_stack_mut(&thread_id)?.push_operand(value)
}

#[inline]
pub(super) fn handle_aload2(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let value = *vm
        .get_stack_mut(&thread_id)?
        .cur_java_frame()?
        .get_local(2)?;
    vm.get_stack_mut(&thread_id)?.push_operand(value)
}

#[inline]
pub(super) fn handle_aload3(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let value = *vm
        .get_stack_mut(&thread_id)?
        .cur_java_frame()?
        .get_local(3)?;
    vm.get_stack_mut(&thread_id)?.push_operand(value)
}

#[inline]
pub(super) fn handle_aload(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
    pos: u8,
) -> Result<(), JvmError> {
    let value = *vm
        .get_stack_mut(&thread_id)?
        .cur_java_frame()?
        .get_local(pos)?;
    vm.get_stack_mut(&thread_id)?.push_operand(value)
}

#[inline]
pub(super) fn handle_anewarray(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
    idx: u16,
) -> Result<(), JvmError> {
    let size = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    if size < 0 {
        throw_exception!(NegativeArraySizeException, size.to_string())?
    }
    let cur_frame_method_id = vm.get_stack_mut(&thread_id)?.cur_java_frame()?.method_id();
    let target_array_sym = vm
        .method_area
        .get_cp_by_method_id(&cur_frame_method_id)?
        .get_class_sym(&idx, vm.interner())?;
    let target_array_class_id = vm.method_area.get_class_id_or_load(target_array_sym)?;
    let array_ref = vm.heap.alloc_object_array(target_array_class_id, size)?;
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Ref(array_ref))
}

#[inline]
pub(super) fn handle_arraylength(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
) -> Result<(), JvmError> {
    let array_ref = vm.get_stack_mut(&thread_id)?.pop_obj_val()?;
    let length = vm.heap.get_array_length(array_ref)?;
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Integer(length))
}

#[inline]
pub(super) fn handle_astore0(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let value = vm.get_stack_mut(&thread_id)?.pop_nullable_ref()?;
    vm.get_stack_mut(&thread_id)?.set_local(0, value)
}

#[inline]
pub(super) fn handle_astore1(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let value = vm.get_stack_mut(&thread_id)?.pop_nullable_ref()?;
    vm.get_stack_mut(&thread_id)?.set_local(1, value)
}

#[inline]
pub(super) fn handle_astore2(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let value = vm.get_stack_mut(&thread_id)?.pop_nullable_ref()?;
    vm.get_stack_mut(&thread_id)?.set_local(2, value)
}

#[inline]
pub(super) fn handle_astore3(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let value = vm.get_stack_mut(&thread_id)?.pop_nullable_ref()?;
    vm.get_stack_mut(&thread_id)?.set_local(3, value)
}

#[inline]
pub(super) fn handle_astore(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
    pos: u8,
) -> Result<(), JvmError> {
    let value = vm.get_stack_mut(&thread_id)?.pop_nullable_ref()?;
    vm.get_stack_mut(&thread_id)?.set_local(pos as usize, value)
}

#[inline]
pub(super) fn handle_bipush(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
    val: i8,
) -> Result<(), JvmError> {
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Integer(val as i32))
}

#[inline]
pub(super) fn handle_castore(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let value = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    let index = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    let array_ref = vm.get_stack_mut(&thread_id)?.pop_obj_val()?;
    vm.heap
        .write_array_element(array_ref, index, Value::Integer(value))
}

#[inline]
pub(super) fn handle_dadd(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let v2 = vm.get_stack_mut(&thread_id)?.pop_double_val()?;
    let v1 = vm.get_stack_mut(&thread_id)?.pop_double_val()?;
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Double(v1 + v2))
}

#[inline]
pub(super) fn handle_dcmpl(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let v2 = vm.get_stack_mut(&thread_id)?.pop_double_val()?;
    let v1 = vm.get_stack_mut(&thread_id)?.pop_double_val()?;
    let res = match v1.total_cmp(&v2) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    };
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Integer(res))
}

#[inline]
pub(super) fn handle_dcmpg(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let v2 = vm.get_stack_mut(&thread_id)?.pop_double_val()?;
    let v1 = vm.get_stack_mut(&thread_id)?.pop_double_val()?;
    let res = match v1.total_cmp(&v2) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    };
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Integer(res))
}

#[inline]
pub(super) fn handle_ddiv(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let v2 = vm.get_stack_mut(&thread_id)?.pop_double_val()?;
    let v1 = vm.get_stack_mut(&thread_id)?.pop_double_val()?;
    // TODO: zero division handling
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Double(v1 / v2))
}

#[inline]
pub(super) fn handle_dconst0(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Double(0.0))
}

#[inline]
pub(super) fn handle_dconst1(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Double(1.0))
}

#[inline]
pub(super) fn handle_dload0(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let value = *vm.get_stack_mut(&thread_id)?.get_local_double(0)?;
    vm.get_stack_mut(&thread_id)?.push_operand(value)
}

#[inline]
pub(super) fn handle_dload1(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let value = *vm.get_stack_mut(&thread_id)?.get_local_double(1)?;
    vm.get_stack_mut(&thread_id)?.push_operand(value)
}

#[inline]
pub(super) fn handle_dload2(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let value = *vm.get_stack_mut(&thread_id)?.get_local_double(2)?;
    vm.get_stack_mut(&thread_id)?.push_operand(value)
}

#[inline]
pub(super) fn handle_dload3(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let value = *vm.get_stack_mut(&thread_id)?.get_local_double(3)?;
    vm.get_stack_mut(&thread_id)?.push_operand(value)
}

#[inline]
pub(super) fn handle_dmul(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let v2 = vm.get_stack_mut(&thread_id)?.pop_double_val()?;
    let v1 = vm.get_stack_mut(&thread_id)?.pop_double_val()?;
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Double(v1 * v2))
}

#[inline]
pub(super) fn handle_dload(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
    n: u8,
) -> Result<(), JvmError> {
    let value = *vm.get_stack_mut(&thread_id)?.get_local_double(n)?;
    vm.get_stack_mut(&thread_id)?.push_operand(value)
}

#[inline]
pub(super) fn handle_dstore(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
    n: u8,
) -> Result<(), JvmError> {
    let value = vm.get_stack_mut(&thread_id)?.pop_double()?;
    vm.get_stack_mut(&thread_id)?.set_local(n as usize, value)
}

#[inline]
pub(super) fn handle_dup(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    vm.get_stack_mut(&thread_id)?.dup_top()
}

#[inline]
pub(super) fn handle_dup2(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    match vm.get_stack(&thread_id)?.peek_operand()? {
        Value::Long(_) | Value::Double(_) => {
            let value = *vm.get_stack(&thread_id)?.peek_operand()?;
            vm.get_stack_mut(&thread_id)?.push_operand(value)
        }
        _ => {
            let value1 = *vm.get_stack(&thread_id)?.peek_operand()?;
            let value2 = *vm.get_stack(&thread_id)?.peek_operand_at(1)?;
            vm.get_stack_mut(&thread_id)?.push_operand(value2)?;
            vm.get_stack_mut(&thread_id)?.push_operand(value1)
        }
    }
}

#[inline]
pub(super) fn handle_dup_x1(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let value1 = vm.get_stack_mut(&thread_id)?.pop_operand()?;
    let value2 = vm.get_stack_mut(&thread_id)?.pop_operand()?;
    vm.get_stack_mut(&thread_id)?.push_operand(value1)?;
    vm.get_stack_mut(&thread_id)?.push_operand(value2)?;
    vm.get_stack_mut(&thread_id)?.push_operand(value1)
}

#[inline]
pub(super) fn handle_fcmpl(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let v2 = vm.get_stack_mut(&thread_id)?.pop_float_val()?;
    let v1 = vm.get_stack_mut(&thread_id)?.pop_float_val()?;
    let res = match v1.total_cmp(&v2) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    };
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Integer(res))
}

#[inline]
pub(super) fn handle_fcmpg(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let v2 = vm.get_stack_mut(&thread_id)?.pop_float_val()?;
    let v1 = vm.get_stack_mut(&thread_id)?.pop_float_val()?;
    let res = match v1.total_cmp(&v2) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    };
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Integer(res))
}

#[inline]
pub(super) fn handle_fconst0(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Float(0.0))
}

#[inline]
pub(super) fn handle_fconst1(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Float(1.0))
}

#[inline]
pub(super) fn handle_fload0(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let value = *vm.get_stack_mut(&thread_id)?.get_local_float(0)?;
    vm.get_stack_mut(&thread_id)?.push_operand(value)
}

#[inline]
pub(super) fn handle_fload1(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let value = *vm.get_stack_mut(&thread_id)?.get_local_float(1)?;
    vm.get_stack_mut(&thread_id)?.push_operand(value)
}

#[inline]
pub(super) fn handle_fload2(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let value = *vm.get_stack_mut(&thread_id)?.get_local_float(2)?;
    vm.get_stack_mut(&thread_id)?.push_operand(value)
}

#[inline]
pub(super) fn handle_fload3(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let value = *vm.get_stack_mut(&thread_id)?.get_local_float(3)?;
    vm.get_stack_mut(&thread_id)?.push_operand(value)
}

#[inline]
pub(super) fn handle_fload(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
    n: u8,
) -> Result<(), JvmError> {
    let value = *vm.get_stack_mut(&thread_id)?.get_local_float(n)?;
    vm.get_stack_mut(&thread_id)?.push_operand(value)
}

#[inline]
pub(super) fn handle_fstore0(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let value = vm.get_stack_mut(&thread_id)?.pop_float()?;
    vm.get_stack_mut(&thread_id)?.set_local(0, value)
}

#[inline]
pub(super) fn handle_fstore1(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let value = vm.get_stack_mut(&thread_id)?.pop_float()?;
    vm.get_stack_mut(&thread_id)?.set_local(1, value)
}

#[inline]
pub(super) fn handle_fstore2(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let value = vm.get_stack_mut(&thread_id)?.pop_float()?;
    vm.get_stack_mut(&thread_id)?.set_local(2, value)
}

#[inline]
pub(super) fn handle_fstore3(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let value = vm.get_stack_mut(&thread_id)?.pop_float()?;
    vm.get_stack_mut(&thread_id)?.set_local(3, value)
}

#[inline]
pub(super) fn handle_fstore(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
    n: u8,
) -> Result<(), JvmError> {
    let value = vm.get_stack_mut(&thread_id)?.pop_float()?;
    vm.get_stack_mut(&thread_id)?.set_local(n as usize, value)
}

#[inline]
pub(super) fn handle_getfield(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
    idx: u16,
) -> Result<(), JvmError> {
    let target_obj_ref = vm.get_stack_mut(&thread_id)?.pop_obj_val()?;
    let cur_frame_method_id = vm.get_stack_mut(&thread_id)?.cur_java_frame()?.method_id();
    let field_view = vm
        .method_area
        .get_cp_by_method_id(&cur_frame_method_id)?
        .get_field_view(&idx, vm.interner())?;
    let target_class_id = vm.method_area.get_class_id_or_load(field_view.class_sym)?;
    let target_field = vm
        .method_area
        .get_instance_class(&target_class_id)?
        .get_instance_field(&field_view.name_and_type.into())?;
    let value = vm.heap.read_field(
        target_obj_ref,
        target_field.offset,
        vm.method_area
            .get_field_descriptor(&target_field.descriptor_id)
            .as_allocation_type(),
    )?;
    vm.get_stack_mut(&thread_id)?.push_operand(value)
}

#[inline]
pub(super) fn handle_getstatic(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
    idx: u16,
) -> Result<(), JvmError> {
    let cur_frame_method_id = vm.get_stack_mut(&thread_id)?.cur_java_frame()?.method_id();
    let target_field_view = vm
        .method_area
        .get_cp_by_method_id(&cur_frame_method_id)?
        .get_field_view(&idx, vm.interner())?;
    let target_class_id = vm
        .method_area
        .get_class_id_or_load(target_field_view.class_sym)?;
    Interpreter::ensure_initialized(thread_id, Some(target_class_id), vm)?;
    let field_key: FieldKey = target_field_view.name_and_type.into();
    let actual_static_field_class_id = vm
        .method_area
        .resolve_static_field_actual_class_id(target_class_id, &field_key)?;
    let value = vm
        .method_area
        .get_class(&actual_static_field_class_id)
        .get_static_field_value(&field_key)?;
    vm.get_stack_mut(&thread_id)?.push_operand(value)
}

#[inline]
pub(super) fn handle_goto(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
    offset: i16,
) -> Result<(), JvmError> {
    let pc = vm.get_stack_mut(&thread_id)?.pc()?;
    let new_pc = branch16(pc, offset);
    *vm.get_stack_mut(&thread_id)?.pc_mut()? = new_pc;
    Ok(())
}

#[inline]
pub(super) fn handle_iadd(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let value2 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    let value1 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    let result = value1.wrapping_add(value2);
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Integer(result))
}
#[inline]
pub(super) fn handle_iconst0(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Integer(0))
}

#[inline]
pub(super) fn handle_iconst1(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Integer(1))
}

#[inline]
pub(super) fn handle_iconst2(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Integer(2))
}

#[inline]
pub(super) fn handle_iconst3(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Integer(3))
}

#[inline]
pub(super) fn handle_iconst4(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Integer(4))
}

#[inline]
pub(super) fn handle_iconst5(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Integer(5))
}

#[inline]
pub(super) fn handle_iconst_m1(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
) -> Result<(), JvmError> {
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Integer(-1))
}

#[inline]
pub(super) fn handle_idiv(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let value2 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    let value1 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    if value2 == 0 {
        throw_exception!(ArithmeticException, "/ by zero")?
    }
    let result = value1.wrapping_div(value2);
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Integer(result))
}

#[inline]
pub(super) fn handle_ifeq(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
    offset: i16,
    size: u16,
) -> Result<(), JvmError> {
    let pc = vm.get_stack_mut(&thread_id)?.pc()?;
    let value = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    let new_pc = if value == 0 {
        branch16(pc, offset)
    } else {
        pc + size as usize
    };
    *vm.get_stack_mut(&thread_id)?.pc_mut()? = new_pc;
    Ok(())
}

#[inline]
pub(super) fn handle_ifge(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
    offset: i16,
    size: u16,
) -> Result<(), JvmError> {
    let pc = vm.get_stack_mut(&thread_id)?.pc()?;
    let value = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    let new_pc = if value >= 0 {
        branch16(pc, offset)
    } else {
        pc + size as usize
    };
    *vm.get_stack_mut(&thread_id)?.pc_mut()? = new_pc;
    Ok(())
}

#[inline]
pub(super) fn handle_ifgt(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
    offset: i16,
    size: u16,
) -> Result<(), JvmError> {
    let pc = vm.get_stack_mut(&thread_id)?.pc()?;
    let value = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    let new_pc = if value > 0 {
        branch16(pc, offset)
    } else {
        pc + size as usize
    };
    *vm.get_stack_mut(&thread_id)?.pc_mut()? = new_pc;
    Ok(())
}

#[inline]
pub(super) fn handle_ifnull(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
    offset: i16,
    size: u16,
) -> Result<(), JvmError> {
    let pc = vm.get_stack_mut(&thread_id)?.pc()?;
    let value = vm.get_stack_mut(&thread_id)?.pop_nullable_ref_val()?;
    let new_pc = if value.is_none() {
        branch16(pc, offset)
    } else {
        pc + size as usize
    };
    *vm.get_stack_mut(&thread_id)?.pc_mut()? = new_pc;
    Ok(())
}

#[inline]
pub(super) fn handle_ificmplt(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
    offset: i16,
    size: u16,
) -> Result<(), JvmError> {
    let pc = vm.get_stack_mut(&thread_id)?.pc()?;
    let v2 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    let v1 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;

    let new_pc = if v1 < v2 {
        branch16(pc, offset)
    } else {
        pc + size as usize
    };
    *vm.get_stack_mut(&thread_id)?.pc_mut()? = new_pc;
    Ok(())
}

#[inline]
pub(super) fn handle_ifle(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
    offset: i16,
    size: u16,
) -> Result<(), JvmError> {
    let pc = vm.get_stack_mut(&thread_id)?.pc()?;
    let value = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    let new_pc = if value <= 0 {
        branch16(pc, offset)
    } else {
        pc + size as usize
    };
    *vm.get_stack_mut(&thread_id)?.pc_mut()? = new_pc;
    Ok(())
}

#[inline]
pub(super) fn handle_iflt(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
    offset: i16,
    size: u16,
) -> Result<(), JvmError> {
    let pc = vm.get_stack_mut(&thread_id)?.pc()?;
    let value = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    let new_pc = if value < 0 {
        branch16(pc, offset)
    } else {
        pc + size as usize
    };
    *vm.get_stack_mut(&thread_id)?.pc_mut()? = new_pc;
    Ok(())
}

#[inline]
pub(super) fn handle_ifacmpeq(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
    offset: i16,
    size: u16,
) -> Result<(), JvmError> {
    let pc = vm.get_stack_mut(&thread_id)?.pc()?;
    let v2 = vm.get_stack_mut(&thread_id)?.pop_nullable_ref_val()?;
    let v1 = vm.get_stack_mut(&thread_id)?.pop_nullable_ref_val()?;
    let new_pc = if v1 == v2 {
        branch16(pc, offset)
    } else {
        pc + size as usize
    };
    *vm.get_stack_mut(&thread_id)?.pc_mut()? = new_pc;
    Ok(())
}

#[inline]
pub(super) fn handle_ifacmpne(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
    offset: i16,
    size: u16,
) -> Result<(), JvmError> {
    let pc = vm.get_stack_mut(&thread_id)?.pc()?;
    let v2 = vm.get_stack_mut(&thread_id)?.pop_nullable_ref_val()?;
    let v1 = vm.get_stack_mut(&thread_id)?.pop_nullable_ref_val()?;
    let new_pc = if v1 != v2 {
        branch16(pc, offset)
    } else {
        pc + size as usize
    };
    *vm.get_stack_mut(&thread_id)?.pc_mut()? = new_pc;
    Ok(())
}

#[inline]
pub(super) fn handle_ificmpne(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
    offset: i16,
    size: u16,
) -> Result<(), JvmError> {
    let pc = vm.get_stack_mut(&thread_id)?.pc()?;
    let v2 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    let v1 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    let new_pc = if v1 != v2 {
        branch16(pc, offset)
    } else {
        pc + size as usize
    };
    *vm.get_stack_mut(&thread_id)?.pc_mut()? = new_pc;
    Ok(())
}

#[inline]
pub(super) fn handle_ificmpge(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
    offset: i16,
    size: u16,
) -> Result<(), JvmError> {
    let pc = vm.get_stack_mut(&thread_id)?.pc()?;
    let v2 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    let v1 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    let new_pc = if v1 >= v2 {
        branch16(pc, offset)
    } else {
        pc + size as usize
    };
    *vm.get_stack_mut(&thread_id)?.pc_mut()? = new_pc;
    Ok(())
}

#[inline]
pub(super) fn handle_ificmpgt(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
    offset: i16,
    size: u16,
) -> Result<(), JvmError> {
    let pc = vm.get_stack_mut(&thread_id)?.pc()?;
    let v2 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    let v1 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    let new_pc = if v1 > v2 {
        branch16(pc, offset)
    } else {
        pc + size as usize
    };
    *vm.get_stack_mut(&thread_id)?.pc_mut()? = new_pc;
    Ok(())
}

#[inline]
pub(super) fn handle_ificmpeq(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
    offset: i16,
    size: u16,
) -> Result<(), JvmError> {
    let pc = vm.get_stack_mut(&thread_id)?.pc()?;
    let v2 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    let v1 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    let new_pc = if v1 == v2 {
        branch16(pc, offset)
    } else {
        pc + size as usize
    };
    *vm.get_stack_mut(&thread_id)?.pc_mut()? = new_pc;
    Ok(())
}

#[inline]
pub(super) fn handle_ificmple(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
    offset: i16,
    size: u16,
) -> Result<(), JvmError> {
    let pc = vm.get_stack_mut(&thread_id)?.pc()?;
    let v2 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    let v1 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    let new_pc = if v1 <= v2 {
        branch16(pc, offset)
    } else {
        pc + size as usize
    };
    *vm.get_stack_mut(&thread_id)?.pc_mut()? = new_pc;
    Ok(())
}

#[inline]
pub(super) fn handle_ifnonnull(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
    offset: i16,
    size: u16,
) -> Result<(), JvmError> {
    let pc = vm.get_stack_mut(&thread_id)?.pc()?;
    let obj = vm.get_stack_mut(&thread_id)?.pop_nullable_ref_val()?;
    let new_pc = if obj.is_some() {
        branch16(pc, offset)
    } else {
        pc + size as usize
    };
    *vm.get_stack_mut(&thread_id)?.pc_mut()? = new_pc;
    Ok(())
}

#[inline]
pub(super) fn handle_ifne(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
    offset: i16,
    size: u16,
) -> Result<(), JvmError> {
    let pc = vm.get_stack_mut(&thread_id)?.pc()?;
    let i = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    let new_pc = if i != 0 {
        branch16(pc, offset)
    } else {
        pc + size as usize
    };
    *vm.get_stack_mut(&thread_id)?.pc_mut()? = new_pc;
    Ok(())
}

#[inline]
pub(super) fn handle_lcmp(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let v2 = vm.get_stack_mut(&thread_id)?.pop_long_val()?;
    let v1 = vm.get_stack_mut(&thread_id)?.pop_long_val()?;
    let res = match v1.cmp(&v2) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    };
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Integer(res))
}

#[inline]
pub(super) fn handle_lconst0(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    vm.get_stack_mut(&thread_id)?.push_operand(Value::Long(0))
}

#[inline]
pub(super) fn handle_lconst1(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    vm.get_stack_mut(&thread_id)?.push_operand(Value::Long(1))
}

#[inline]
pub(super) fn handle_lookupswitch(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
    switch: LookupSwitchData,
) -> Result<(), JvmError> {
    let key = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    let pc = vm.get_stack(&thread_id)?.pc()?;
    let target_offset = match switch.pairs.binary_search_by_key(&key, |p| p.0) {
        Ok(i) => switch.pairs[i].1,
        Err(_) => switch.default_offset,
    };
    let new_pc = branch32(pc, target_offset);
    *vm.get_stack_mut(&thread_id)?.pc_mut()? = new_pc;
    Ok(())
}
#[inline]
pub(super) fn handle_iload0(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let value = *vm
        .get_stack_mut(&thread_id)?
        .cur_java_frame()?
        .get_local(0)?;
    vm.get_stack_mut(&thread_id)?.push_operand(value)
}
#[inline]
pub(super) fn handle_iload1(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let value = *vm
        .get_stack_mut(&thread_id)?
        .cur_java_frame()?
        .get_local(1)?;
    vm.get_stack_mut(&thread_id)?.push_operand(value)
}

#[inline]
pub(super) fn handle_iload2(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let value = *vm
        .get_stack_mut(&thread_id)?
        .cur_java_frame()?
        .get_local(2)?;
    vm.get_stack_mut(&thread_id)?.push_operand(value)
}

#[inline]
pub(super) fn handle_iload3(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let value = *vm
        .get_stack_mut(&thread_id)?
        .cur_java_frame()?
        .get_local(3)?;
    vm.get_stack_mut(&thread_id)?.push_operand(value)
}

#[inline]
pub(super) fn handle_iload(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
    pos: u8,
) -> Result<(), JvmError> {
    let value = *vm
        .get_stack_mut(&thread_id)?
        .cur_java_frame()?
        .get_local(pos)?;
    vm.get_stack_mut(&thread_id)?.push_operand(value)
}

#[inline]
pub(super) fn handle_invokevirtual(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
    idx: u16,
) -> Result<(), JvmError> {
    let cur_frame_method_id = vm.get_stack_mut(&thread_id)?.cur_java_frame()?.method_id();
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
        .peek_operand_at(arg_count - 1)?
        .as_obj_ref()?;
    let actual_class_id = vm.heap.get_class_id(object_ref)?;

    let target_method_id = vm
        .method_area
        .get_class(&actual_class_id)
        .get_vtable_method_id(&method_key)?;
    let args = Interpreter::prepare_method_args(thread_id, target_method_id, vm)?;
    Interpreter::invoke_method_internal(thread_id, target_method_id, args, vm)
}

#[inline]
pub(super) fn handle_instanceof(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
    idx: u16,
) -> Result<(), JvmError> {
    let cur_frame_method_id = vm.get_stack_mut(&thread_id)?.cur_java_frame()?.method_id();
    let class_name_sym = vm
        .method_area
        .get_cp_by_method_id(&cur_frame_method_id)?
        .get_class_sym(&idx, vm.interner())?;

    let obj_ref = vm.get_stack_mut(&thread_id)?.pop_nullable_ref_val()?;
    if let Some(obj_ref) = obj_ref {
        let target_class = vm.heap.get_class_id(obj_ref)?;
        let res = vm.method_area.instance_of(target_class, class_name_sym);
        vm.get_stack_mut(&thread_id)?
            .push_operand(Value::Integer(if res { 1 } else { 0 }))
    } else {
        vm.get_stack_mut(&thread_id)?
            .push_operand(Value::Integer(0))
    }
}
#[inline]
pub(super) fn handle_fmul(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let v2 = vm.get_stack_mut(&thread_id)?.pop_float_val()?;
    let v1 = vm.get_stack_mut(&thread_id)?.pop_float_val()?;
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Float(v1 * v2))
}

#[inline]
pub(super) fn handle_fdiv(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let v2 = vm.get_stack_mut(&thread_id)?.pop_float_val()?;
    let v1 = vm.get_stack_mut(&thread_id)?.pop_float_val()?;
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Float(v1 / v2))
}

#[inline]
pub(super) fn handle_irem(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let v2 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    let v1 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    if v2 == 0 {
        throw_exception!(ArithmeticException, "/ by zero")?
    }
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Integer(v1 % v2))
}

#[inline]
pub(super) fn handle_ladd(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let v2 = vm.get_stack_mut(&thread_id)?.pop_long_val()?;
    let v1 = vm.get_stack_mut(&thread_id)?.pop_long_val()?;
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Long(v1.wrapping_add(v2)))
}

#[inline]
pub(super) fn handle_ldiv(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let v2 = vm.get_stack_mut(&thread_id)?.pop_long_val()?;
    let v1 = vm.get_stack_mut(&thread_id)?.pop_long_val()?;
    if v2 == 0 {
        throw_exception!(ArithmeticException, "/ by zero")?
    }
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Long(v1.wrapping_div(v2)))
}

#[inline]
pub(super) fn handle_lmul(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let v2 = vm.get_stack_mut(&thread_id)?.pop_long_val()?;
    let v1 = vm.get_stack_mut(&thread_id)?.pop_long_val()?;
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Long(v1.wrapping_mul(v2)))
}

#[inline]
pub(super) fn handle_lrem(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let v2 = vm.get_stack_mut(&thread_id)?.pop_long_val()?;
    let v1 = vm.get_stack_mut(&thread_id)?.pop_long_val()?;
    if v2 == 0 {
        throw_exception!(ArithmeticException, "/ by zero")?
    }
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Long(v1 % v2))
}

#[inline]
pub(super) fn handle_land(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let v2 = vm.get_stack_mut(&thread_id)?.pop_long_val()?;
    let v1 = vm.get_stack_mut(&thread_id)?.pop_long_val()?;
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Long(v1 & v2))
}

#[inline]
pub(super) fn handle_lor(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let v2 = vm.get_stack_mut(&thread_id)?.pop_long_val()?;
    let v1 = vm.get_stack_mut(&thread_id)?.pop_long_val()?;
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Long(v1 | v2))
}

#[inline]
pub(super) fn handle_lxor(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let v2 = vm.get_stack_mut(&thread_id)?.pop_long_val()?;
    let v1 = vm.get_stack_mut(&thread_id)?.pop_long_val()?;
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Long(v1 ^ v2))
}

#[inline]
pub(super) fn handle_iand(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let v2 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    let v1 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Integer(v1 & v2))
}

#[inline]
pub(super) fn handle_ior(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let v2 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    let v1 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Integer(v1 | v2))
}

#[inline]
pub(super) fn handle_ixor(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let v2 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    let v1 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Integer(v1 ^ v2))
}

#[inline]
pub(super) fn handle_l2i(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let v = vm.get_stack_mut(&thread_id)?.pop_long_val()?;
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Integer(v as i32))
}

#[inline]
pub(super) fn handle_l2f(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let v = vm.get_stack_mut(&thread_id)?.pop_long_val()?;
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Float(v as f32))
}

#[inline]
pub(super) fn handle_d2i(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let v = vm.get_stack_mut(&thread_id)?.pop_double_val()?;
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Integer(v as i32))
}

#[inline]
pub(super) fn handle_d2l(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let v = vm.get_stack_mut(&thread_id)?.pop_double_val()?;
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Long(v as i64))
}

#[inline]
pub(super) fn handle_f2i(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let v = vm.get_stack_mut(&thread_id)?.pop_float_val()?;
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Integer(v as i32))
}

#[inline]
pub(super) fn handle_f2d(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let v = vm.get_stack_mut(&thread_id)?.pop_float_val()?;
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Double(v as f64))
}

#[inline]
pub(super) fn handle_ineg(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let v = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Integer(-v))
}

#[inline]
pub(super) fn handle_i2s(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let v = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Integer((v as i16) as i32))
}

#[inline]
pub(super) fn handle_i2c(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let v = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Integer((v as u16) as i32))
}

#[inline]
pub(super) fn handle_i2l(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let v = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Long(v as i64))
}

#[inline]
pub(super) fn handle_i2f(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let v = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Float(v as f32))
}

#[inline]
pub(super) fn handle_i2d(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let v = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Double(v as f64))
}

#[inline]
pub(super) fn handle_i2b(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let v = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Integer((v as i8) as i32))
}

#[inline]
pub(super) fn handle_istore0(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let value = vm.get_stack_mut(&thread_id)?.pop_int()?;
    vm.get_stack_mut(&thread_id)?.set_local(0, value)
}

#[inline]
pub(super) fn handle_istore1(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let value = vm.get_stack_mut(&thread_id)?.pop_int()?;
    vm.get_stack_mut(&thread_id)?.set_local(1, value)
}

#[inline]
pub(super) fn handle_istore2(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let value = vm.get_stack_mut(&thread_id)?.pop_int()?;
    vm.get_stack_mut(&thread_id)?.set_local(2, value)
}

#[inline]
pub(super) fn handle_istore3(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let value = vm.get_stack_mut(&thread_id)?.pop_int()?;
    vm.get_stack_mut(&thread_id)?.set_local(3, value)
}

#[inline]
pub(super) fn handle_istore(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
    idx: u8,
) -> Result<(), JvmError> {
    let value = vm.get_stack_mut(&thread_id)?.pop_int()?;
    vm.get_stack_mut(&thread_id)?.set_local(idx as usize, value)
}

#[inline]
pub(super) fn handle_isub(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let v2 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    let v1 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Integer(v1 - v2))
}

#[inline]
pub(super) fn handle_imul(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let v2 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    let v1 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Integer(v1.wrapping_mul(v2)))
}

#[inline]
pub(super) fn handle_iinc(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
    idx: u8,
    const_val: i8,
) -> Result<(), JvmError> {
    let value = vm.get_stack_mut(&thread_id)?.get_local_int_val(idx)?;
    vm.get_stack_mut(&thread_id)?
        .set_local(idx as usize, Value::Integer(value + (const_val as i32)))
}

#[inline]
pub(super) fn handle_ldc_ldcw_ldc2w(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
    idx: u16,
) -> Result<(), JvmError> {
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
                let string_ref = vm.heap.get_str_from_pool_or_new(string_sym)?;
                Value::Ref(string_ref)
            }
            _ => unimplemented!(),
        }
    };
    vm.get_stack_mut(&thread_id)?.push_operand(ldc_operand)
}

#[inline]
pub(super) fn handle_new(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
    idx: u16,
) -> Result<(), JvmError> {
    let cur_frame_method_id = vm.get_stack_mut(&thread_id)?.cur_java_frame()?.method_id();
    let target_class_name = vm
        .method_area
        .get_cp_by_method_id(&cur_frame_method_id)?
        .get_class_sym(&idx, vm.interner())?;
    let target_class_id = vm.method_area.get_class_id_or_load(target_class_name)?;
    Interpreter::ensure_initialized(thread_id, Some(target_class_id), vm)?;
    let instance_ref = vm.heap.alloc_instance(
        vm.method_area
            .get_instance_class(&target_class_id)?
            .get_instance_size()?,
        target_class_id,
    )?;
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Ref(instance_ref))
}

#[inline]
pub(super) fn handle_newarray(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
    array_type: ArrayType,
) -> Result<(), JvmError> {
    let size = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    if size < 0 {
        throw_exception!(NegativeArraySizeException, size.to_string())?
    }
    let class_id = vm
        .method_area
        .load_array_class(vm.interner().get_or_intern(array_type.descriptor()))?;
    let array_ref = vm.heap.alloc_primitive_array(class_id, array_type, size)?;
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Ref(array_ref))
}

#[inline]
pub(super) fn handle_pop(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    vm.get_stack_mut(&thread_id)?.pop_operand()?;
    Ok(())
}

#[inline]
pub(super) fn handle_putfield(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
    idx: u16,
) -> Result<(), JvmError> {
    let value = vm.get_stack_mut(&thread_id)?.pop_operand()?;
    let target_obj_ref = vm.get_stack_mut(&thread_id)?.pop_obj_val()?;
    let cur_frame_method_id = vm.get_stack_mut(&thread_id)?.cur_java_frame()?.method_id();
    let field_view = vm
        .method_area
        .get_cp_by_method_id(&cur_frame_method_id)?
        .get_field_view(&idx, vm.interner())?;
    let target_class_id = vm.method_area.get_class_id_or_load(field_view.class_sym)?;
    let target_field = vm
        .method_area
        .get_instance_class(&target_class_id)?
        .get_instance_field(&field_view.name_and_type.into())?;
    vm.heap.write_field(
        target_obj_ref,
        target_field.offset,
        value,
        vm.method_area
            .get_field_descriptor(&target_field.descriptor_id)
            .as_allocation_type(),
    )
}

#[inline]
pub(super) fn handle_putstatic(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
    idx: u16,
) -> Result<(), JvmError> {
    let value = vm.get_stack_mut(&thread_id)?.pop_operand()?;
    let cur_frame_method_id = vm.get_stack_mut(&thread_id)?.cur_java_frame()?.method_id();
    let target_field_view = vm
        .method_area
        .get_cp_by_method_id(&cur_frame_method_id)?
        .get_field_view(&idx, vm.interner())?;
    let target_class_id = vm
        .method_area
        .get_class_id_or_load(target_field_view.class_sym)?;
    Interpreter::ensure_initialized(thread_id, Some(target_class_id), vm)?;
    let field_key: FieldKey = target_field_view.name_and_type.into();
    let actual_static_field_class_id = vm
        .method_area
        .resolve_static_field_actual_class_id(target_class_id, &field_key)?;
    vm.method_area
        .get_class_like(&actual_static_field_class_id)?
        .set_static_field_value(&field_key, value)
}

#[inline]
pub(super) fn handle_invokeinterface(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
    idx: u16,
    count: u8,
) -> Result<(), JvmError> {
    let cur_frame_method_id = vm.get_stack_mut(&thread_id)?.cur_java_frame()?.method_id();
    let target_method_view = vm
        .method_area
        .get_cp_by_method_id(&cur_frame_method_id)?
        .get_interface_method_view(&idx, vm.interner())?;
    let object_ref = vm
        .get_stack(&thread_id)?
        .peek_operand_at(count as usize - 1)?
        .as_obj_ref()?;
    if target_method_view.class_sym
        == vm
            .interner()
            .get_or_intern("jdk/internal/access/JavaLangRefAccess")
        && target_method_view.name_and_type.name_sym == vm.interner().get_or_intern("startThreads")
    {
        warn!("TODO: Stub: Ignoring call to jdk/internal/access/JavaLangRefAccess.startThreads");
        for _ in 0..count {
            let _ = vm.get_stack_mut(&thread_id)?.pop_operand()?;
        }
    } else {
        let target_class_id = vm.heap.get_class_id(object_ref)?;
        let target_method_id = vm
            .method_area
            .get_instance_class(&target_class_id)?
            .get_interface_method_id(&target_method_view.name_and_type.into())?;
        let args = Interpreter::prepare_method_args(thread_id, target_method_id, vm)?;
        Interpreter::invoke_method_internal(thread_id, target_method_id, args, vm)?;
    };
    Ok(())
}

#[inline]
pub(super) fn handle_invokespecial(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
    idx: u16,
) -> Result<(), JvmError> {
    let cur_frame_method_id = vm.get_stack_mut(&thread_id)?.cur_java_frame()?.method_id();
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
    let args = Interpreter::prepare_method_args(thread_id, target_method_id, vm)?;
    Interpreter::invoke_method_internal(thread_id, target_method_id, args, vm)
}

#[inline]
pub(super) fn handle_invokestatic(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
    idx: u16,
) -> Result<(), JvmError> {
    let cur_frame_method_id = vm.get_stack_mut(&thread_id)?.cur_java_frame()?.method_id();
    let target_method_view = vm
        .method_area
        .get_cp_by_method_id(&cur_frame_method_id)?
        .get_method_or_interface_method_view(&idx, vm.interner())?;
    let target_class_id = vm
        .method_area
        .get_class_id_or_load(target_method_view.class_sym)?;
    Interpreter::ensure_initialized(thread_id, Some(target_class_id), vm)?;
    let target_method_id = vm
        .method_area
        .get_class(&target_class_id)
        .get_static_method_id(&target_method_view.name_and_type.into())?;
    let args = Interpreter::prepare_method_args(thread_id, target_method_id, vm)?;
    Interpreter::invoke_static_method(thread_id, target_method_id, vm, args)
}

#[inline]
pub(super) fn handle_invokedynamic(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
    idx: u16,
) -> Result<(), JvmError> {
    let cur_frame_method_id = vm.get_stack_mut(&thread_id)?.cur_java_frame()?.method_id();
    let bootstrap_method = vm
        .method_area
        .get_cp_by_method_id(&cur_frame_method_id)?
        .get_invoke_dynamic_view(&idx, vm.interner())?;
    println!();
    todo!()
}

#[inline]
pub(super) fn handle_lload0(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let value = *vm
        .get_stack_mut(&thread_id)?
        .cur_java_frame()?
        .get_local(0)?;
    vm.get_stack_mut(&thread_id)?.push_operand(value)
}

#[inline]
pub(super) fn handle_lload1(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let value = *vm
        .get_stack_mut(&thread_id)?
        .cur_java_frame()?
        .get_local(1)?;
    vm.get_stack_mut(&thread_id)?.push_operand(value)
}

#[inline]
pub(super) fn handle_lload2(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let value = *vm
        .get_stack_mut(&thread_id)?
        .cur_java_frame()?
        .get_local(2)?;
    vm.get_stack_mut(&thread_id)?.push_operand(value)
}

#[inline]
pub(super) fn handle_lload3(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let value = *vm
        .get_stack_mut(&thread_id)?
        .cur_java_frame()?
        .get_local(3)?;
    vm.get_stack_mut(&thread_id)?.push_operand(value)
}

#[inline]
pub(super) fn handle_lload(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
    pos: u8,
) -> Result<(), JvmError> {
    let value = *vm
        .get_stack_mut(&thread_id)?
        .cur_java_frame()?
        .get_local(pos)?;
    vm.get_stack_mut(&thread_id)?.push_operand(value)
}

#[inline]
pub(super) fn handle_iushr(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let v2 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    let v1 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    let shift = (v2 & 0x1F) as u32;
    let result = ((v1 as u32) >> shift) as i32;
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Integer(result))
}

#[inline]
pub(super) fn handle_lshl(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let v2 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    let v1 = vm.get_stack_mut(&thread_id)?.pop_long_val()?;
    let shift = (v2 & 0x3F) as u32;
    let result = v1.wrapping_shl(shift);
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Long(result))
}

#[inline]
pub(super) fn handle_lushr(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let v2 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    let v1 = vm.get_stack_mut(&thread_id)?.pop_long_val()?;
    let shift = (v2 & 0x3F) as u32;
    let result = ((v1 as u64) >> shift) as i64;
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Long(result))
}

#[inline]
pub(super) fn handle_lshr(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let v2 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    let v1 = vm.get_stack_mut(&thread_id)?.pop_long_val()?;
    let shift = (v2 & 0x3F) as u32;
    let result = v1.wrapping_shr(shift);
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Long(result))
}

#[inline]
pub(super) fn handle_lstore0(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let value = vm.get_stack_mut(&thread_id)?.pop_long()?;
    vm.get_stack_mut(&thread_id)?.set_local(0, value)
}

#[inline]
pub(super) fn handle_lstore1(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let value = vm.get_stack_mut(&thread_id)?.pop_long()?;
    vm.get_stack_mut(&thread_id)?.set_local(1, value)
}

#[inline]
pub(super) fn handle_lstore2(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let value = vm.get_stack_mut(&thread_id)?.pop_long()?;
    vm.get_stack_mut(&thread_id)?.set_local(2, value)
}

#[inline]
pub(super) fn handle_lstore3(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let value = vm.get_stack_mut(&thread_id)?.pop_long()?;
    vm.get_stack_mut(&thread_id)?.set_local(3, value)
}

#[inline]
pub(super) fn handle_lstore(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
    idx: u8,
) -> Result<(), JvmError> {
    let value = vm.get_stack_mut(&thread_id)?.pop_long()?;
    vm.get_stack_mut(&thread_id)?.set_local(idx as usize, value)
}

#[inline]
pub(super) fn handle_lsub(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let v2 = vm.get_stack_mut(&thread_id)?.pop_long_val()?;
    let v1 = vm.get_stack_mut(&thread_id)?.pop_long_val()?;
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Long(v1.wrapping_sub(v2)))
}

#[inline]
pub(super) fn handle_iastore(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let value = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    let index = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    let array_ref = vm.get_stack_mut(&thread_id)?.pop_obj_val()?;
    vm.heap
        .write_array_element(array_ref, index, Value::Integer(value))
}

#[inline]
pub(super) fn handle_ishl(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let v2 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    let v1 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    let shift = (v2 & 0x1F) as u32;
    let result = v1.wrapping_shl(shift);
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Integer(result))
}

#[inline]
pub(super) fn handle_ishr(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let v2 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    let v1 = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    let shift = (v2 & 0x1F) as u32;
    let result = v1.wrapping_shr(shift);
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Integer(result))
}

#[inline]
pub(super) fn handle_saload(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let index = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    let array_ref = vm.get_stack_mut(&thread_id)?.pop_obj_val()?;
    let value = vm.heap.read_array_element(array_ref, index)?;
    vm.get_stack_mut(&thread_id)?.push_operand(value)
}

#[inline]
pub(super) fn handle_sastore(thread_id: ThreadId, vm: &mut VirtualMachine) -> Result<(), JvmError> {
    let value = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    let index = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    let array_ref = vm.get_stack_mut(&thread_id)?.pop_obj_val()?;
    vm.heap
        .write_array_element(array_ref, index, Value::Integer(value))
}

#[inline]
pub(super) fn handle_sipush(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
    value: i16,
) -> Result<(), JvmError> {
    vm.get_stack_mut(&thread_id)?
        .push_operand(Value::Integer(value as i32))
}

#[inline]
pub(super) fn handle_tableswitch(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
    switch: TableSwitchData,
) -> Result<(), JvmError> {
    let index = vm.get_stack_mut(&thread_id)?.pop_int_val()?;
    let pc = vm.get_stack(&thread_id)?.pc()?;
    let target_offset = if index < switch.low || index > switch.high {
        switch.default_offset
    } else {
        let idx = (index - switch.low) as usize;
        switch.offsets[idx]
    };
    let new_pc = branch32(pc, target_offset);
    *vm.get_stack_mut(&thread_id)?.pc_mut()? = new_pc;
    Ok(())
}

#[inline]
pub(super) fn handle_monitorenter(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
) -> Result<(), JvmError> {
    let _obj = vm.get_stack_mut(&thread_id)?.pop_obj_val()?;
    Ok(())
}

#[inline]
pub(super) fn handle_monitorexit(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
) -> Result<(), JvmError> {
    let _obj = vm.get_stack_mut(&thread_id)?.pop_obj_val()?;
    Ok(())
}
