use crate::error::JvmError;
use crate::interpreter::Interpreter;
use crate::keys::{FieldKey, MethodKey};
use crate::rt::constant_pool::RuntimeConstant;
use crate::thread::JavaThreadState;
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
pub(super) fn handle_athrow(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let exception_ref = thread.stack.pop_obj_val()?;
    Err(JvmError::JavaExceptionThrown(exception_ref))
}

#[inline]
pub(super) fn handle_aaload(
    thread: &mut JavaThreadState,
    vm: &VirtualMachine,
) -> Result<(), JvmError> {
    let index = thread.stack.pop_int_val()?;
    let array_addr = thread.stack.pop_obj_val()?;
    let value = vm
        .heap_read()
        .read_array_element(array_addr, index)?
        .as_nullable_obj_ref()?;
    thread
        .stack
        .push_operand(value.map(Value::Ref).unwrap_or(Value::Null))
}

#[inline]
pub(super) fn handle_aastore(
    thread: &mut JavaThreadState,
    vm: &VirtualMachine,
) -> Result<(), JvmError> {
    let value = thread.stack.pop_nullable_ref()?;
    let index = thread.stack.pop_int_val()?;
    let array_addr = thread.stack.pop_obj_val()?;
    vm.heap_write()
        .write_array_element(array_addr, index, value)
}

#[inline]
pub(super) fn handle_bastore(
    thread: &mut JavaThreadState,
    vm: &VirtualMachine,
) -> Result<(), JvmError> {
    let value = thread.stack.pop_int()?;
    let index = thread.stack.pop_int_val()?;
    let array_addr = thread.stack.pop_obj_val()?;
    vm.heap_write()
        .write_array_element(array_addr, index, value)
}

#[inline]
pub(super) fn handle_iaload(
    thread: &mut JavaThreadState,
    vm: &VirtualMachine,
) -> Result<(), JvmError> {
    let index = thread.stack.pop_int_val()?;
    let array_addr = thread.stack.pop_obj_val()?;
    let value = vm.heap_read().read_array_element(array_addr, index)?;
    thread.stack.push_operand(value)
}

#[inline]
pub(super) fn handle_caload(
    thread: &mut JavaThreadState,
    vm: &VirtualMachine,
) -> Result<(), JvmError> {
    let index = thread.stack.pop_int_val()?;
    let array_addr = thread.stack.pop_obj_val()?;
    let value = vm.heap_read().read_array_element(array_addr, index)?;
    thread.stack.push_operand(value)
}

#[inline]
pub(super) fn handle_baload(
    thread: &mut JavaThreadState,
    vm: &VirtualMachine,
) -> Result<(), JvmError> {
    let index = thread.stack.pop_int_val()?;
    let array_addr = thread.stack.pop_obj_val()?;
    let value = vm.heap_read().read_array_element(array_addr, index)?;
    thread.stack.push_operand(value)
}

// TODO: stub
#[inline]
pub(super) fn handle_checkcast(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let object_ref = thread.stack.pop_operand()?;
    thread.stack.push_operand(object_ref)
}

#[inline]
pub(super) fn handle_aconst_null(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    thread.stack.push_operand(Value::Null)
}

#[inline]
pub(super) fn handle_aload0(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let value = *thread.stack.cur_java_frame()?.get_local(0)?;
    thread.stack.push_operand(value)
}

#[inline]
pub(super) fn handle_aload1(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let value = *thread.stack.cur_java_frame()?.get_local(1)?;
    thread.stack.push_operand(value)
}

#[inline]
pub(super) fn handle_aload2(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let value = *thread.stack.cur_java_frame()?.get_local(2)?;
    thread.stack.push_operand(value)
}

#[inline]
pub(super) fn handle_aload3(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let value = *thread.stack.cur_java_frame()?.get_local(3)?;
    thread.stack.push_operand(value)
}

#[inline]
pub(super) fn handle_aload(thread: &mut JavaThreadState, pos: u8) -> Result<(), JvmError> {
    let value = *thread.stack.cur_java_frame()?.get_local(pos)?;
    thread.stack.push_operand(value)
}

#[inline]
pub(super) fn handle_anewarray(
    thread: &mut JavaThreadState,
    vm: &VirtualMachine,
    idx: u16,
) -> Result<(), JvmError> {
    let size = thread.stack.pop_int_val()?;
    if size < 0 {
        throw_exception!(NegativeArraySizeException, size.to_string())?
    }
    let cur_frame_method_id = thread.stack.cur_java_frame()?.method_id();
    let target_array_sym = vm
        .method_area_read()
        .get_cp_by_method_id(&cur_frame_method_id)?
        .get_class_sym(&idx, vm.interner())?;
    let target_array_class_id = vm
        .method_area_write()
        .get_class_id_or_load(target_array_sym, thread.id)?;
    let array_ref = vm
        .heap_write()
        .alloc_object_array(target_array_class_id, size)?;
    thread.stack.push_operand(Value::Ref(array_ref))
}

#[inline]
pub(super) fn handle_arraylength(
    thread: &mut JavaThreadState,
    vm: &VirtualMachine,
) -> Result<(), JvmError> {
    let array_ref = thread.stack.pop_obj_val()?;
    let length = vm.heap_read().get_array_length(array_ref)?;
    thread.stack.push_operand(Value::Integer(length))
}

#[inline]
pub(super) fn handle_astore0(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let value = thread.stack.pop_nullable_ref()?;
    thread.stack.set_local(0, value)
}

#[inline]
pub(super) fn handle_astore1(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let value = thread.stack.pop_nullable_ref()?;
    thread.stack.set_local(1, value)
}

#[inline]
pub(super) fn handle_astore2(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let value = thread.stack.pop_nullable_ref()?;
    thread.stack.set_local(2, value)
}

#[inline]
pub(super) fn handle_astore3(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let value = thread.stack.pop_nullable_ref()?;
    thread.stack.set_local(3, value)
}

#[inline]
pub(super) fn handle_astore(thread: &mut JavaThreadState, pos: u8) -> Result<(), JvmError> {
    let value = thread.stack.pop_nullable_ref()?;
    thread.stack.set_local(pos as usize, value)
}

#[inline]
pub(super) fn handle_bipush(thread: &mut JavaThreadState, val: i8) -> Result<(), JvmError> {
    thread.stack.push_operand(Value::Integer(val as i32))
}

#[inline]
pub(super) fn handle_castore(
    thread: &mut JavaThreadState,
    vm: &VirtualMachine,
) -> Result<(), JvmError> {
    let value = thread.stack.pop_int_val()?;
    let index = thread.stack.pop_int_val()?;
    let array_ref = thread.stack.pop_obj_val()?;
    vm.heap_write()
        .write_array_element(array_ref, index, Value::Integer(value))
}

#[inline]
pub(super) fn handle_dadd(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let v2 = thread.stack.pop_double_val()?;
    let v1 = thread.stack.pop_double_val()?;
    thread.stack.push_operand(Value::Double(v1 + v2))
}

#[inline]
pub(super) fn handle_dcmpl(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let v2 = thread.stack.pop_double_val()?;
    let v1 = thread.stack.pop_double_val()?;
    let res = match v1.total_cmp(&v2) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    };
    thread.stack.push_operand(Value::Integer(res))
}

#[inline]
pub(super) fn handle_dcmpg(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let v2 = thread.stack.pop_double_val()?;
    let v1 = thread.stack.pop_double_val()?;
    let res = match v1.total_cmp(&v2) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    };
    thread.stack.push_operand(Value::Integer(res))
}

#[inline]
pub(super) fn handle_ddiv(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let v2 = thread.stack.pop_double_val()?;
    let v1 = thread.stack.pop_double_val()?;
    // TODO: zero division handling
    thread.stack.push_operand(Value::Double(v1 / v2))
}

#[inline]
pub(super) fn handle_dconst0(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    thread.stack.push_operand(Value::Double(0.0))
}

#[inline]
pub(super) fn handle_dconst1(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    thread.stack.push_operand(Value::Double(1.0))
}

#[inline]
pub(super) fn handle_dload0(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let value = *thread.stack.get_local_double(0)?;
    thread.stack.push_operand(value)
}

#[inline]
pub(super) fn handle_dload1(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let value = *thread.stack.get_local_double(1)?;
    thread.stack.push_operand(value)
}

#[inline]
pub(super) fn handle_dload2(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let value = *thread.stack.get_local_double(2)?;
    thread.stack.push_operand(value)
}

#[inline]
pub(super) fn handle_dload3(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let value = *thread.stack.get_local_double(3)?;
    thread.stack.push_operand(value)
}

#[inline]
pub(super) fn handle_dmul(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let v2 = thread.stack.pop_double_val()?;
    let v1 = thread.stack.pop_double_val()?;
    thread.stack.push_operand(Value::Double(v1 * v2))
}

#[inline]
pub(super) fn handle_dload(thread: &mut JavaThreadState, n: u8) -> Result<(), JvmError> {
    let value = *thread.stack.get_local_double(n)?;
    thread.stack.push_operand(value)
}

#[inline]
pub(super) fn handle_dstore(thread: &mut JavaThreadState, n: u8) -> Result<(), JvmError> {
    let value = thread.stack.pop_double()?;
    thread.stack.set_local(n as usize, value)
}

#[inline]
pub(super) fn handle_dup(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    thread.stack.dup_top()
}

#[inline]
pub(super) fn handle_dup2(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    match thread.stack.peek_operand()? {
        Value::Long(_) | Value::Double(_) => {
            let value = *thread.stack.peek_operand()?;
            thread.stack.push_operand(value)
        }
        _ => {
            let value1 = *thread.stack.peek_operand()?;
            let value2 = *thread.stack.peek_operand_at(1)?;
            thread.stack.push_operand(value2)?;
            thread.stack.push_operand(value1)
        }
    }
}

#[inline]
pub(super) fn handle_dup_x1(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let value1 = thread.stack.pop_operand()?;
    let value2 = thread.stack.pop_operand()?;
    thread.stack.push_operand(value1)?;
    thread.stack.push_operand(value2)?;
    thread.stack.push_operand(value1)
}

#[inline]
pub(super) fn handle_fcmpl(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let v2 = thread.stack.pop_float_val()?;
    let v1 = thread.stack.pop_float_val()?;
    let res = match v1.total_cmp(&v2) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    };
    thread.stack.push_operand(Value::Integer(res))
}

#[inline]
pub(super) fn handle_fcmpg(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let v2 = thread.stack.pop_float_val()?;
    let v1 = thread.stack.pop_float_val()?;
    let res = match v1.total_cmp(&v2) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    };
    thread.stack.push_operand(Value::Integer(res))
}

#[inline]
pub(super) fn handle_fconst0(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    thread.stack.push_operand(Value::Float(0.0))
}

#[inline]
pub(super) fn handle_fconst1(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    thread.stack.push_operand(Value::Float(1.0))
}

#[inline]
pub(super) fn handle_fload0(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let value = *thread.stack.get_local_float(0)?;
    thread.stack.push_operand(value)
}

#[inline]
pub(super) fn handle_fload1(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let value = *thread.stack.get_local_float(1)?;
    thread.stack.push_operand(value)
}

#[inline]
pub(super) fn handle_fload2(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let value = *thread.stack.get_local_float(2)?;
    thread.stack.push_operand(value)
}

#[inline]
pub(super) fn handle_fload3(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let value = *thread.stack.get_local_float(3)?;
    thread.stack.push_operand(value)
}

#[inline]
pub(super) fn handle_fload(thread: &mut JavaThreadState, n: u8) -> Result<(), JvmError> {
    let value = *thread.stack.get_local_float(n)?;
    thread.stack.push_operand(value)
}

#[inline]
pub(super) fn handle_fstore0(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let value = thread.stack.pop_float()?;
    thread.stack.set_local(0, value)
}

#[inline]
pub(super) fn handle_fstore1(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let value = thread.stack.pop_float()?;
    thread.stack.set_local(1, value)
}

#[inline]
pub(super) fn handle_fstore2(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let value = thread.stack.pop_float()?;
    thread.stack.set_local(2, value)
}

#[inline]
pub(super) fn handle_fstore3(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let value = thread.stack.pop_float()?;
    thread.stack.set_local(3, value)
}

#[inline]
pub(super) fn handle_fstore(thread: &mut JavaThreadState, n: u8) -> Result<(), JvmError> {
    let value = thread.stack.pop_float()?;
    thread.stack.set_local(n as usize, value)
}

#[inline]
pub(super) fn handle_getfield(
    thread: &mut JavaThreadState,
    vm: &VirtualMachine,
    idx: u16,
) -> Result<(), JvmError> {
    let target_obj_ref = thread.stack.pop_obj_val()?;
    let cur_frame_method_id = thread.stack.cur_java_frame()?.method_id();
    let field_view = vm
        .method_area_read()
        .get_cp_by_method_id(&cur_frame_method_id)?
        .get_field_view(&idx, vm.interner())?;
    let target_class_id = vm
        .method_area_write()
        .get_class_id_or_load(field_view.class_sym, thread.id)?;
    let (target_field_offset, target_field_descriptor_id) = {
        let ma = vm.method_area_read();
        let target_field =
            ma.get_instance_field(&target_class_id, &field_view.name_and_type.into())?;
        (target_field.offset, target_field.descriptor_id)
    };
    let value = vm.heap_read().read_field(
        target_obj_ref,
        target_field_offset,
        vm.method_area_read()
            .get_field_descriptor(&target_field_descriptor_id)
            .as_allocation_type(),
    )?;
    thread.stack.push_operand(value)
}

#[inline]
pub(super) fn handle_getstatic(
    thread: &mut JavaThreadState,
    vm: &VirtualMachine,
    idx: u16,
) -> Result<(), JvmError> {
    let cur_frame_method_id = thread.stack.cur_java_frame()?.method_id();
    let target_field_view = vm
        .method_area_read()
        .get_cp_by_method_id(&cur_frame_method_id)?
        .get_field_view(&idx, vm.interner())?;
    let target_class_id = vm
        .method_area_write()
        .get_class_id_or_load(target_field_view.class_sym, thread.id)?;
    Interpreter::ensure_initialized(thread, Some(target_class_id), vm)?;
    let field_key: FieldKey = target_field_view.name_and_type.into();
    let actual_static_field_class_id = vm
        .method_area_read()
        .resolve_static_field_actual_class_id(target_class_id, &field_key)?;
    let value = vm
        .method_area_read()
        .get_static_field_value(&actual_static_field_class_id, &field_key)?;
    thread.stack.push_operand(value)
}

#[inline]
pub(super) fn handle_goto(thread: &mut JavaThreadState, offset: i16) -> Result<(), JvmError> {
    let pc = thread.stack.pc()?;
    let new_pc = branch16(pc, offset);
    *thread.stack.pc_mut()? = new_pc;
    Ok(())
}

#[inline]
pub(super) fn handle_iadd(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let value2 = thread.stack.pop_int_val()?;
    let value1 = thread.stack.pop_int_val()?;
    let result = value1.wrapping_add(value2);
    thread.stack.push_operand(Value::Integer(result))
}
#[inline]
pub(super) fn handle_iconst0(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    thread.stack.push_operand(Value::Integer(0))
}

#[inline]
pub(super) fn handle_iconst1(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    thread.stack.push_operand(Value::Integer(1))
}

#[inline]
pub(super) fn handle_iconst2(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    thread.stack.push_operand(Value::Integer(2))
}

#[inline]
pub(super) fn handle_iconst3(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    thread.stack.push_operand(Value::Integer(3))
}

#[inline]
pub(super) fn handle_iconst4(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    thread.stack.push_operand(Value::Integer(4))
}

#[inline]
pub(super) fn handle_iconst5(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    thread.stack.push_operand(Value::Integer(5))
}

#[inline]
pub(super) fn handle_iconst_m1(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    thread.stack.push_operand(Value::Integer(-1))
}

#[inline]
pub(super) fn handle_idiv(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let value2 = thread.stack.pop_int_val()?;
    let value1 = thread.stack.pop_int_val()?;
    if value2 == 0 {
        throw_exception!(ArithmeticException, "/ by zero")?
    }
    let result = value1.wrapping_div(value2);
    thread.stack.push_operand(Value::Integer(result))
}

#[inline]
pub(super) fn handle_ifeq(
    thread: &mut JavaThreadState,
    offset: i16,
    size: u16,
) -> Result<(), JvmError> {
    let pc = thread.stack.pc()?;
    let value = thread.stack.pop_int_val()?;
    let new_pc = if value == 0 {
        branch16(pc, offset)
    } else {
        pc + size as usize
    };
    *thread.stack.pc_mut()? = new_pc;
    Ok(())
}

#[inline]
pub(super) fn handle_ifge(
    thread: &mut JavaThreadState,
    offset: i16,
    size: u16,
) -> Result<(), JvmError> {
    let pc = thread.stack.pc()?;
    let value = thread.stack.pop_int_val()?;
    let new_pc = if value >= 0 {
        branch16(pc, offset)
    } else {
        pc + size as usize
    };
    *thread.stack.pc_mut()? = new_pc;
    Ok(())
}

#[inline]
pub(super) fn handle_ifgt(
    thread: &mut JavaThreadState,
    offset: i16,
    size: u16,
) -> Result<(), JvmError> {
    let pc = thread.stack.pc()?;
    let value = thread.stack.pop_int_val()?;
    let new_pc = if value > 0 {
        branch16(pc, offset)
    } else {
        pc + size as usize
    };
    *thread.stack.pc_mut()? = new_pc;
    Ok(())
}

#[inline]
pub(super) fn handle_ifnull(
    thread: &mut JavaThreadState,
    offset: i16,
    size: u16,
) -> Result<(), JvmError> {
    let pc = thread.stack.pc()?;
    let value = thread.stack.pop_nullable_ref_val()?;
    let new_pc = if value.is_none() {
        branch16(pc, offset)
    } else {
        pc + size as usize
    };
    *thread.stack.pc_mut()? = new_pc;
    Ok(())
}

#[inline]
pub(super) fn handle_ificmplt(
    thread: &mut JavaThreadState,
    offset: i16,
    size: u16,
) -> Result<(), JvmError> {
    let pc = thread.stack.pc()?;
    let v2 = thread.stack.pop_int_val()?;
    let v1 = thread.stack.pop_int_val()?;

    let new_pc = if v1 < v2 {
        branch16(pc, offset)
    } else {
        pc + size as usize
    };
    *thread.stack.pc_mut()? = new_pc;
    Ok(())
}

#[inline]
pub(super) fn handle_ifle(
    thread: &mut JavaThreadState,
    offset: i16,
    size: u16,
) -> Result<(), JvmError> {
    let pc = thread.stack.pc()?;
    let value = thread.stack.pop_int_val()?;
    let new_pc = if value <= 0 {
        branch16(pc, offset)
    } else {
        pc + size as usize
    };
    *thread.stack.pc_mut()? = new_pc;
    Ok(())
}

#[inline]
pub(super) fn handle_iflt(
    thread: &mut JavaThreadState,
    offset: i16,
    size: u16,
) -> Result<(), JvmError> {
    let pc = thread.stack.pc()?;
    let value = thread.stack.pop_int_val()?;
    let new_pc = if value < 0 {
        branch16(pc, offset)
    } else {
        pc + size as usize
    };
    *thread.stack.pc_mut()? = new_pc;
    Ok(())
}

#[inline]
pub(super) fn handle_ifacmpeq(
    thread: &mut JavaThreadState,
    offset: i16,
    size: u16,
) -> Result<(), JvmError> {
    let pc = thread.stack.pc()?;
    let v2 = thread.stack.pop_nullable_ref_val()?;
    let v1 = thread.stack.pop_nullable_ref_val()?;
    let new_pc = if v1 == v2 {
        branch16(pc, offset)
    } else {
        pc + size as usize
    };
    *thread.stack.pc_mut()? = new_pc;
    Ok(())
}

#[inline]
pub(super) fn handle_ifacmpne(
    thread: &mut JavaThreadState,
    offset: i16,
    size: u16,
) -> Result<(), JvmError> {
    let pc = thread.stack.pc()?;
    let v2 = thread.stack.pop_nullable_ref_val()?;
    let v1 = thread.stack.pop_nullable_ref_val()?;
    let new_pc = if v1 != v2 {
        branch16(pc, offset)
    } else {
        pc + size as usize
    };
    *thread.stack.pc_mut()? = new_pc;
    Ok(())
}

#[inline]
pub(super) fn handle_ificmpne(
    thread: &mut JavaThreadState,
    offset: i16,
    size: u16,
) -> Result<(), JvmError> {
    let pc = thread.stack.pc()?;
    let v2 = thread.stack.pop_int_val()?;
    let v1 = thread.stack.pop_int_val()?;
    let new_pc = if v1 != v2 {
        branch16(pc, offset)
    } else {
        pc + size as usize
    };
    *thread.stack.pc_mut()? = new_pc;
    Ok(())
}

#[inline]
pub(super) fn handle_ificmpge(
    thread: &mut JavaThreadState,
    offset: i16,
    size: u16,
) -> Result<(), JvmError> {
    let pc = thread.stack.pc()?;
    let v2 = thread.stack.pop_int_val()?;
    let v1 = thread.stack.pop_int_val()?;
    let new_pc = if v1 >= v2 {
        branch16(pc, offset)
    } else {
        pc + size as usize
    };
    *thread.stack.pc_mut()? = new_pc;
    Ok(())
}

#[inline]
pub(super) fn handle_ificmpgt(
    thread: &mut JavaThreadState,
    offset: i16,
    size: u16,
) -> Result<(), JvmError> {
    let pc = thread.stack.pc()?;
    let v2 = thread.stack.pop_int_val()?;
    let v1 = thread.stack.pop_int_val()?;
    let new_pc = if v1 > v2 {
        branch16(pc, offset)
    } else {
        pc + size as usize
    };
    *thread.stack.pc_mut()? = new_pc;
    Ok(())
}

#[inline]
pub(super) fn handle_ificmpeq(
    thread: &mut JavaThreadState,
    offset: i16,
    size: u16,
) -> Result<(), JvmError> {
    let pc = thread.stack.pc()?;
    let v2 = thread.stack.pop_int_val()?;
    let v1 = thread.stack.pop_int_val()?;
    let new_pc = if v1 == v2 {
        branch16(pc, offset)
    } else {
        pc + size as usize
    };
    *thread.stack.pc_mut()? = new_pc;
    Ok(())
}

#[inline]
pub(super) fn handle_ificmple(
    thread: &mut JavaThreadState,
    offset: i16,
    size: u16,
) -> Result<(), JvmError> {
    let pc = thread.stack.pc()?;
    let v2 = thread.stack.pop_int_val()?;
    let v1 = thread.stack.pop_int_val()?;
    let new_pc = if v1 <= v2 {
        branch16(pc, offset)
    } else {
        pc + size as usize
    };
    *thread.stack.pc_mut()? = new_pc;
    Ok(())
}

#[inline]
pub(super) fn handle_ifnonnull(
    thread: &mut JavaThreadState,
    offset: i16,
    size: u16,
) -> Result<(), JvmError> {
    let pc = thread.stack.pc()?;
    let obj = thread.stack.pop_nullable_ref_val()?;
    let new_pc = if obj.is_some() {
        branch16(pc, offset)
    } else {
        pc + size as usize
    };
    *thread.stack.pc_mut()? = new_pc;
    Ok(())
}

#[inline]
pub(super) fn handle_ifne(
    thread: &mut JavaThreadState,
    offset: i16,
    size: u16,
) -> Result<(), JvmError> {
    let pc = thread.stack.pc()?;
    let i = thread.stack.pop_int_val()?;
    let new_pc = if i != 0 {
        branch16(pc, offset)
    } else {
        pc + size as usize
    };
    *thread.stack.pc_mut()? = new_pc;
    Ok(())
}

#[inline]
pub(super) fn handle_lcmp(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let v2 = thread.stack.pop_long_val()?;
    let v1 = thread.stack.pop_long_val()?;
    let res = match v1.cmp(&v2) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    };
    thread.stack.push_operand(Value::Integer(res))
}

#[inline]
pub(super) fn handle_lconst0(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    thread.stack.push_operand(Value::Long(0))
}

#[inline]
pub(super) fn handle_lconst1(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    thread.stack.push_operand(Value::Long(1))
}

#[inline]
pub(super) fn handle_lookupswitch(
    thread: &mut JavaThreadState,
    switch: LookupSwitchData,
) -> Result<(), JvmError> {
    let key = thread.stack.pop_int_val()?;
    let pc = thread.stack.pc()?;
    let target_offset = match switch.pairs.binary_search_by_key(&key, |p| p.0) {
        Ok(i) => switch.pairs[i].1,
        Err(_) => switch.default_offset,
    };
    let new_pc = branch32(pc, target_offset);
    *thread.stack.pc_mut()? = new_pc;
    Ok(())
}
#[inline]
pub(super) fn handle_iload0(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let value = *thread.stack.cur_java_frame()?.get_local(0)?;
    thread.stack.push_operand(value)
}
#[inline]
pub(super) fn handle_iload1(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let value = *thread.stack.cur_java_frame()?.get_local(1)?;
    thread.stack.push_operand(value)
}

#[inline]
pub(super) fn handle_iload2(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let value = *thread.stack.cur_java_frame()?.get_local(2)?;
    thread.stack.push_operand(value)
}

#[inline]
pub(super) fn handle_iload3(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let value = *thread.stack.cur_java_frame()?.get_local(3)?;
    thread.stack.push_operand(value)
}

#[inline]
pub(super) fn handle_iload(thread: &mut JavaThreadState, pos: u8) -> Result<(), JvmError> {
    let value = *thread.stack.cur_java_frame()?.get_local(pos)?;
    thread.stack.push_operand(value)
}

#[inline]
pub(super) fn handle_invokevirtual(
    thread: &mut JavaThreadState,
    vm: &VirtualMachine,
    idx: u16,
) -> Result<(), JvmError> {
    let cur_frame_method_id = thread.stack.cur_java_frame()?.method_id();
    let target_method_view = vm
        .method_area_read()
        .get_cp_by_method_id(&cur_frame_method_id)?
        .get_method_view(&idx, vm.interner())?;
    let method_key: MethodKey = target_method_view.name_and_type.into();

    let target_method_desc_id = vm
        .method_area_write()
        .get_or_new_method_descriptor_id(&method_key.desc)
        .unwrap();
    let arg_count = vm
        .method_area_read()
        .get_method_descriptor(&target_method_desc_id)
        .params
        .len()
        + 1;

    let object_ref = thread.stack.peek_operand_at(arg_count - 1)?.as_obj_ref()?;
    let actual_class_id = vm.heap_read().get_class_id(object_ref)?;

    let target_method_id = vm
        .method_area_read()
        .get_class(&actual_class_id)
        .get_vtable_method_id(&method_key)?;
    let args = Interpreter::prepare_method_args(thread, target_method_id, vm)?;
    Interpreter::invoke_method_internal(thread, target_method_id, args, vm)
}

#[inline]
pub(super) fn handle_instanceof(
    thread: &mut JavaThreadState,
    vm: &VirtualMachine,
    idx: u16,
) -> Result<(), JvmError> {
    let cur_frame_method_id = thread.stack.cur_java_frame()?.method_id();
    let class_name_sym = vm
        .method_area_read()
        .get_cp_by_method_id(&cur_frame_method_id)?
        .get_class_sym(&idx, vm.interner())?;

    let obj_ref = thread.stack.pop_nullable_ref_val()?;
    if let Some(obj_ref) = obj_ref {
        let target_class = vm.heap_read().get_class_id(obj_ref)?;
        let res = vm
            .method_area_read()
            .instance_of(target_class, class_name_sym);
        thread
            .stack
            .push_operand(Value::Integer(if res { 1 } else { 0 }))
    } else {
        thread.stack.push_operand(Value::Integer(0))
    }
}
#[inline]
pub(super) fn handle_fmul(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let v2 = thread.stack.pop_float_val()?;
    let v1 = thread.stack.pop_float_val()?;
    thread.stack.push_operand(Value::Float(v1 * v2))
}

#[inline]
pub(super) fn handle_fdiv(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let v2 = thread.stack.pop_float_val()?;
    let v1 = thread.stack.pop_float_val()?;
    thread.stack.push_operand(Value::Float(v1 / v2))
}

#[inline]
pub(super) fn handle_irem(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let v2 = thread.stack.pop_int_val()?;
    let v1 = thread.stack.pop_int_val()?;
    if v2 == 0 {
        throw_exception!(ArithmeticException, "/ by zero")?
    }
    thread.stack.push_operand(Value::Integer(v1 % v2))
}

#[inline]
pub(super) fn handle_ladd(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let v2 = thread.stack.pop_long_val()?;
    let v1 = thread.stack.pop_long_val()?;
    thread.stack.push_operand(Value::Long(v1.wrapping_add(v2)))
}

#[inline]
pub(super) fn handle_ldiv(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let v2 = thread.stack.pop_long_val()?;
    let v1 = thread.stack.pop_long_val()?;
    if v2 == 0 {
        throw_exception!(ArithmeticException, "/ by zero")?
    }
    thread.stack.push_operand(Value::Long(v1.wrapping_div(v2)))
}

#[inline]
pub(super) fn handle_lmul(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let v2 = thread.stack.pop_long_val()?;
    let v1 = thread.stack.pop_long_val()?;
    thread.stack.push_operand(Value::Long(v1.wrapping_mul(v2)))
}

#[inline]
pub(super) fn handle_lrem(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let v2 = thread.stack.pop_long_val()?;
    let v1 = thread.stack.pop_long_val()?;
    if v2 == 0 {
        throw_exception!(ArithmeticException, "/ by zero")?
    }
    thread.stack.push_operand(Value::Long(v1 % v2))
}

#[inline]
pub(super) fn handle_land(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let v2 = thread.stack.pop_long_val()?;
    let v1 = thread.stack.pop_long_val()?;
    thread.stack.push_operand(Value::Long(v1 & v2))
}

#[inline]
pub(super) fn handle_lor(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let v2 = thread.stack.pop_long_val()?;
    let v1 = thread.stack.pop_long_val()?;
    thread.stack.push_operand(Value::Long(v1 | v2))
}

#[inline]
pub(super) fn handle_lxor(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let v2 = thread.stack.pop_long_val()?;
    let v1 = thread.stack.pop_long_val()?;
    thread.stack.push_operand(Value::Long(v1 ^ v2))
}

#[inline]
pub(super) fn handle_iand(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let v2 = thread.stack.pop_int_val()?;
    let v1 = thread.stack.pop_int_val()?;
    thread.stack.push_operand(Value::Integer(v1 & v2))
}

#[inline]
pub(super) fn handle_ior(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let v2 = thread.stack.pop_int_val()?;
    let v1 = thread.stack.pop_int_val()?;
    thread.stack.push_operand(Value::Integer(v1 | v2))
}

#[inline]
pub(super) fn handle_ixor(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let v2 = thread.stack.pop_int_val()?;
    let v1 = thread.stack.pop_int_val()?;
    thread.stack.push_operand(Value::Integer(v1 ^ v2))
}

#[inline]
pub(super) fn handle_l2i(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let v = thread.stack.pop_long_val()?;
    thread.stack.push_operand(Value::Integer(v as i32))
}

#[inline]
pub(super) fn handle_l2f(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let v = thread.stack.pop_long_val()?;
    thread.stack.push_operand(Value::Float(v as f32))
}

#[inline]
pub(super) fn handle_d2i(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let v = thread.stack.pop_double_val()?;
    thread.stack.push_operand(Value::Integer(v as i32))
}

#[inline]
pub(super) fn handle_d2l(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let v = thread.stack.pop_double_val()?;
    thread.stack.push_operand(Value::Long(v as i64))
}

#[inline]
pub(super) fn handle_f2i(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let v = thread.stack.pop_float_val()?;
    thread.stack.push_operand(Value::Integer(v as i32))
}

#[inline]
pub(super) fn handle_f2d(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let v = thread.stack.pop_float_val()?;
    thread.stack.push_operand(Value::Double(v as f64))
}

#[inline]
pub(super) fn handle_ineg(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let v = thread.stack.pop_int_val()?;
    thread.stack.push_operand(Value::Integer(-v))
}

#[inline]
pub(super) fn handle_i2s(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let v = thread.stack.pop_int_val()?;
    thread.stack.push_operand(Value::Integer((v as i16) as i32))
}

#[inline]
pub(super) fn handle_i2c(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let v = thread.stack.pop_int_val()?;
    thread.stack.push_operand(Value::Integer((v as u16) as i32))
}

#[inline]
pub(super) fn handle_i2l(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let v = thread.stack.pop_int_val()?;
    thread.stack.push_operand(Value::Long(v as i64))
}

#[inline]
pub(super) fn handle_i2f(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let v = thread.stack.pop_int_val()?;
    thread.stack.push_operand(Value::Float(v as f32))
}

#[inline]
pub(super) fn handle_i2d(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let v = thread.stack.pop_int_val()?;
    thread.stack.push_operand(Value::Double(v as f64))
}

#[inline]
pub(super) fn handle_i2b(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let v = thread.stack.pop_int_val()?;
    thread.stack.push_operand(Value::Integer((v as i8) as i32))
}

#[inline]
pub(super) fn handle_istore0(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let value = thread.stack.pop_int()?;
    thread.stack.set_local(0, value)
}

#[inline]
pub(super) fn handle_istore1(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let value = thread.stack.pop_int()?;
    thread.stack.set_local(1, value)
}

#[inline]
pub(super) fn handle_istore2(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let value = thread.stack.pop_int()?;
    thread.stack.set_local(2, value)
}

#[inline]
pub(super) fn handle_istore3(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let value = thread.stack.pop_int()?;
    thread.stack.set_local(3, value)
}

#[inline]
pub(super) fn handle_istore(thread: &mut JavaThreadState, idx: u8) -> Result<(), JvmError> {
    let value = thread.stack.pop_int()?;
    thread.stack.set_local(idx as usize, value)
}

#[inline]
pub(super) fn handle_isub(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let v2 = thread.stack.pop_int_val()?;
    let v1 = thread.stack.pop_int_val()?;
    thread.stack.push_operand(Value::Integer(v1 - v2))
}

#[inline]
pub(super) fn handle_imul(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let v2 = thread.stack.pop_int_val()?;
    let v1 = thread.stack.pop_int_val()?;
    thread
        .stack
        .push_operand(Value::Integer(v1.wrapping_mul(v2)))
}

#[inline]
pub(super) fn handle_iinc(
    thread: &mut JavaThreadState,
    idx: u8,
    const_val: i8,
) -> Result<(), JvmError> {
    let value = thread.stack.get_local_int_val(idx)?;
    thread
        .stack
        .set_local(idx as usize, Value::Integer(value + (const_val as i32)))
}

#[inline]
pub(super) fn handle_ldc_ldcw_ldc2w(
    thread: &mut JavaThreadState,
    vm: &VirtualMachine,
    idx: u16,
) -> Result<(), JvmError> {
    let cur_method_id = thread.stack.cur_java_frame()?.method_id();
    let ldc_operand = {
        let ma = vm.method_area_read();
        let cp = ma.get_cp_by_method_id(&cur_method_id)?;
        match cp.get_constant(&idx, vm.interner())? {
            RuntimeConstant::Integer(val) => Value::Integer(*val),
            RuntimeConstant::Float(val) => Value::Float(*val),
            RuntimeConstant::Long(val) => Value::Long(*val),
            RuntimeConstant::Double(val) => Value::Double(*val),
            RuntimeConstant::Class(class_entry) => {
                let class_name_sym = class_entry.get_name_sym()?;
                drop(ma);
                let class_id = vm
                    .method_area_write()
                    .get_class_id_or_load(class_name_sym, thread.id)?;
                Value::Ref(
                    vm.method_area_write()
                        .get_mirror_ref_or_create(class_id, &vm.heap)?,
                )
            }
            RuntimeConstant::String(str_entry) => {
                let string_sym = str_entry.get_string_sym()?;
                let string_ref = vm.heap_write().get_str_from_pool_or_new(string_sym)?;
                Value::Ref(string_ref)
            }
            _ => unimplemented!(),
        }
    };
    thread.stack.push_operand(ldc_operand)
}

#[inline]
pub(super) fn handle_new(
    thread: &mut JavaThreadState,
    vm: &VirtualMachine,
    idx: u16,
) -> Result<(), JvmError> {
    let cur_frame_method_id = thread.stack.cur_java_frame()?.method_id();
    let target_class_name = vm
        .method_area_read()
        .get_cp_by_method_id(&cur_frame_method_id)?
        .get_class_sym(&idx, vm.interner())?;
    let target_class_id = vm
        .method_area_write()
        .get_class_id_or_load(target_class_name, thread.id)?;
    Interpreter::ensure_initialized(thread, Some(target_class_id), vm)?;
    let instance_ref = vm.heap_write().alloc_instance(
        vm.method_area_read()
            .get_instance_class(&target_class_id)?
            .get_instance_size()?,
        target_class_id,
    )?;
    thread.stack.push_operand(Value::Ref(instance_ref))
}

#[inline]
pub(super) fn handle_newarray(
    thread: &mut JavaThreadState,
    vm: &VirtualMachine,
    array_type: ArrayType,
) -> Result<(), JvmError> {
    let size = thread.stack.pop_int_val()?;
    if size < 0 {
        throw_exception!(NegativeArraySizeException, size.to_string())?
    }
    let class_id = vm.method_area_write().load_array_class(
        vm.interner().get_or_intern(array_type.descriptor()),
        thread.id,
    )?;
    let array_ref = vm
        .heap_write()
        .alloc_primitive_array(class_id, array_type, size)?;
    thread.stack.push_operand(Value::Ref(array_ref))
}

#[inline]
pub(super) fn handle_pop(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    thread.stack.pop_operand()?;
    Ok(())
}

#[inline]
pub(super) fn handle_putfield(
    thread: &mut JavaThreadState,
    vm: &VirtualMachine,
    idx: u16,
) -> Result<(), JvmError> {
    let value = thread.stack.pop_operand()?;
    let target_obj_ref = thread.stack.pop_obj_val()?;
    let cur_frame_method_id = thread.stack.cur_java_frame()?.method_id();
    let field_view = vm
        .method_area_read()
        .get_cp_by_method_id(&cur_frame_method_id)?
        .get_field_view(&idx, vm.interner())?;
    let target_class_id = vm
        .method_area_write()
        .get_class_id_or_load(field_view.class_sym, thread.id)?;
    let (target_field_offset, target_field_descriptor_id) = {
        let ma = vm.method_area_read();
        let target_field =
            ma.get_instance_field(&target_class_id, &field_view.name_and_type.into())?;
        (target_field.offset, target_field.descriptor_id)
    };
    vm.heap_write().write_field(
        target_obj_ref,
        target_field_offset,
        value,
        vm.method_area_read()
            .get_field_descriptor(&target_field_descriptor_id)
            .as_allocation_type(),
    )
}

#[inline]
pub(super) fn handle_putstatic(
    thread: &mut JavaThreadState,
    vm: &VirtualMachine,
    idx: u16,
) -> Result<(), JvmError> {
    let value = thread.stack.pop_operand()?;
    let cur_frame_method_id = thread.stack.cur_java_frame()?.method_id();
    let target_field_view = vm
        .method_area_read()
        .get_cp_by_method_id(&cur_frame_method_id)?
        .get_field_view(&idx, vm.interner())?;
    let target_class_id = vm
        .method_area_write()
        .get_class_id_or_load(target_field_view.class_sym, thread.id)?;
    Interpreter::ensure_initialized(thread, Some(target_class_id), vm)?;
    let field_key: FieldKey = target_field_view.name_and_type.into();
    let actual_static_field_class_id = vm
        .method_area_read()
        .resolve_static_field_actual_class_id(target_class_id, &field_key)?;
    vm.method_area_read()
        .get_class_like(&actual_static_field_class_id)?
        .set_static_field_value(&field_key, value)
}

#[inline]
pub(super) fn handle_invokeinterface(
    thread: &mut JavaThreadState,
    vm: &VirtualMachine,
    idx: u16,
    count: u8,
) -> Result<(), JvmError> {
    let cur_frame_method_id = thread.stack.cur_java_frame()?.method_id();
    let target_method_view = vm
        .method_area_read()
        .get_cp_by_method_id(&cur_frame_method_id)?
        .get_interface_method_view(&idx, vm.interner())?;
    let object_ref = thread
        .stack
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
            let _ = thread.stack.pop_operand()?;
        }
    } else {
        let target_class_id = vm.heap_read().get_class_id(object_ref)?;
        let target_method_id = vm
            .method_area_read()
            .get_instance_class(&target_class_id)?
            .get_interface_method_id(&target_method_view.name_and_type.into())?;
        let args = Interpreter::prepare_method_args(thread, target_method_id, vm)?;
        Interpreter::invoke_method_internal(thread, target_method_id, args, vm)?;
    };
    Ok(())
}

#[inline]
pub(super) fn handle_invokespecial(
    thread: &mut JavaThreadState,
    vm: &VirtualMachine,
    idx: u16,
) -> Result<(), JvmError> {
    let cur_frame_method_id = thread.stack.cur_java_frame()?.method_id();
    let target_method_view = vm
        .method_area_read()
        .get_cp_by_method_id(&cur_frame_method_id)?
        .get_method_view(&idx, vm.interner())?;
    let target_class_id = vm
        .method_area_write()
        .get_class_id_or_load(target_method_view.class_sym, thread.id)?;
    let target_method_id = vm
        .method_area_read()
        .get_instance_class(&target_class_id)?
        .get_special_method_id(&target_method_view.name_and_type.into())?;
    let args = Interpreter::prepare_method_args(thread, target_method_id, vm)?;
    Interpreter::invoke_method_internal(thread, target_method_id, args, vm)
}

#[inline]
pub(super) fn handle_invokestatic(
    thread: &mut JavaThreadState,
    vm: &VirtualMachine,
    idx: u16,
) -> Result<(), JvmError> {
    let cur_frame_method_id = thread.stack.cur_java_frame()?.method_id();
    let target_method_view = vm
        .method_area_read()
        .get_cp_by_method_id(&cur_frame_method_id)?
        .get_method_or_interface_method_view(&idx, vm.interner())?;
    let target_class_id = vm
        .method_area_write()
        .get_class_id_or_load(target_method_view.class_sym, thread.id)?;
    Interpreter::ensure_initialized(thread, Some(target_class_id), vm)?;
    let target_method_id = vm
        .method_area_read()
        .get_static_method_id(&target_class_id, target_method_view.name_and_type.into())?;
    let args = Interpreter::prepare_method_args(thread, target_method_id, vm)?;
    Interpreter::invoke_static_method(thread, target_method_id, vm, args)
}

#[inline]
pub(super) fn handle_invokedynamic(
    thread: &mut JavaThreadState,
    vm: &VirtualMachine,
    idx: u16,
) -> Result<(), JvmError> {
    let cur_frame_method_id = thread.stack.cur_java_frame()?.method_id();
    let bootstrap_method = vm
        .method_area_read()
        .get_cp_by_method_id(&cur_frame_method_id)?
        .get_invoke_dynamic_view(&idx, vm.interner())?;
    todo!()
}

#[inline]
pub(super) fn handle_lload0(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let value = *thread.stack.cur_java_frame()?.get_local(0)?;
    thread.stack.push_operand(value)
}

#[inline]
pub(super) fn handle_lload1(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let value = *thread.stack.cur_java_frame()?.get_local(1)?;
    thread.stack.push_operand(value)
}

#[inline]
pub(super) fn handle_lload2(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let value = *thread.stack.cur_java_frame()?.get_local(2)?;
    thread.stack.push_operand(value)
}

#[inline]
pub(super) fn handle_lload3(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let value = *thread.stack.cur_java_frame()?.get_local(3)?;
    thread.stack.push_operand(value)
}

#[inline]
pub(super) fn handle_lload(thread: &mut JavaThreadState, pos: u8) -> Result<(), JvmError> {
    let value = *thread.stack.cur_java_frame()?.get_local(pos)?;
    thread.stack.push_operand(value)
}

#[inline]
pub(super) fn handle_iushr(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let v2 = thread.stack.pop_int_val()?;
    let v1 = thread.stack.pop_int_val()?;
    let shift = (v2 & 0x1F) as u32;
    let result = ((v1 as u32) >> shift) as i32;
    thread.stack.push_operand(Value::Integer(result))
}

#[inline]
pub(super) fn handle_lshl(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let v2 = thread.stack.pop_int_val()?;
    let v1 = thread.stack.pop_long_val()?;
    let shift = (v2 & 0x3F) as u32;
    let result = v1.wrapping_shl(shift);
    thread.stack.push_operand(Value::Long(result))
}

#[inline]
pub(super) fn handle_lushr(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let v2 = thread.stack.pop_int_val()?;
    let v1 = thread.stack.pop_long_val()?;
    let shift = (v2 & 0x3F) as u32;
    let result = ((v1 as u64) >> shift) as i64;
    thread.stack.push_operand(Value::Long(result))
}

#[inline]
pub(super) fn handle_lshr(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let v2 = thread.stack.pop_int_val()?;
    let v1 = thread.stack.pop_long_val()?;
    let shift = (v2 & 0x3F) as u32;
    let result = v1.wrapping_shr(shift);
    thread.stack.push_operand(Value::Long(result))
}

#[inline]
pub(super) fn handle_lstore0(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let value = thread.stack.pop_long()?;
    thread.stack.set_local(0, value)
}

#[inline]
pub(super) fn handle_lstore1(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let value = thread.stack.pop_long()?;
    thread.stack.set_local(1, value)
}

#[inline]
pub(super) fn handle_lstore2(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let value = thread.stack.pop_long()?;
    thread.stack.set_local(2, value)
}

#[inline]
pub(super) fn handle_lstore3(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let value = thread.stack.pop_long()?;
    thread.stack.set_local(3, value)
}

#[inline]
pub(super) fn handle_lstore(thread: &mut JavaThreadState, idx: u8) -> Result<(), JvmError> {
    let value = thread.stack.pop_long()?;
    thread.stack.set_local(idx as usize, value)
}

#[inline]
pub(super) fn handle_lsub(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let v2 = thread.stack.pop_long_val()?;
    let v1 = thread.stack.pop_long_val()?;
    thread.stack.push_operand(Value::Long(v1.wrapping_sub(v2)))
}

#[inline]
pub(super) fn handle_iastore(
    thread: &mut JavaThreadState,
    vm: &VirtualMachine,
) -> Result<(), JvmError> {
    let value = thread.stack.pop_int_val()?;
    let index = thread.stack.pop_int_val()?;
    let array_ref = thread.stack.pop_obj_val()?;
    vm.heap_write()
        .write_array_element(array_ref, index, Value::Integer(value))
}

#[inline]
pub(super) fn handle_ishl(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let v2 = thread.stack.pop_int_val()?;
    let v1 = thread.stack.pop_int_val()?;
    let shift = (v2 & 0x1F) as u32;
    let result = v1.wrapping_shl(shift);
    thread.stack.push_operand(Value::Integer(result))
}

#[inline]
pub(super) fn handle_ishr(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let v2 = thread.stack.pop_int_val()?;
    let v1 = thread.stack.pop_int_val()?;
    let shift = (v2 & 0x1F) as u32;
    let result = v1.wrapping_shr(shift);
    thread.stack.push_operand(Value::Integer(result))
}

#[inline]
pub(super) fn handle_saload(
    thread: &mut JavaThreadState,
    vm: &VirtualMachine,
) -> Result<(), JvmError> {
    let index = thread.stack.pop_int_val()?;
    let array_ref = thread.stack.pop_obj_val()?;
    let value = vm.heap_read().read_array_element(array_ref, index)?;
    thread.stack.push_operand(value)
}

#[inline]
pub(super) fn handle_sastore(
    thread: &mut JavaThreadState,
    vm: &VirtualMachine,
) -> Result<(), JvmError> {
    let value = thread.stack.pop_int_val()?;
    let index = thread.stack.pop_int_val()?;
    let array_ref = thread.stack.pop_obj_val()?;
    vm.heap_write()
        .write_array_element(array_ref, index, Value::Integer(value))
}

#[inline]
pub(super) fn handle_sipush(thread: &mut JavaThreadState, value: i16) -> Result<(), JvmError> {
    thread.stack.push_operand(Value::Integer(value as i32))
}

#[inline]
pub(super) fn handle_tableswitch(
    thread: &mut JavaThreadState,
    switch: TableSwitchData,
) -> Result<(), JvmError> {
    let index = thread.stack.pop_int_val()?;
    let pc = thread.stack.pc()?;
    let target_offset = if index < switch.low || index > switch.high {
        switch.default_offset
    } else {
        let idx = (index - switch.low) as usize;
        switch.offsets[idx]
    };
    let new_pc = branch32(pc, target_offset);
    *thread.stack.pc_mut()? = new_pc;
    Ok(())
}

#[inline]
pub(super) fn handle_monitorenter(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let _obj = thread.stack.pop_obj_val()?;
    Ok(())
}

#[inline]
pub(super) fn handle_monitorexit(thread: &mut JavaThreadState) -> Result<(), JvmError> {
    let _obj = thread.stack.pop_obj_val()?;
    Ok(())
}
