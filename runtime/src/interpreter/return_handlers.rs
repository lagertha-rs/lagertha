use crate::VirtualMachine;
use crate::error::JvmError;
use crate::keys::ThreadId;
use crate::vm::Value;

#[inline]
pub(super) fn handle_dreturn(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
) -> Result<Value, JvmError> {
    vm.get_stack_mut(&thread_id)?.pop_double()
}
#[inline]
pub(super) fn handle_ireturn(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
) -> Result<Value, JvmError> {
    vm.get_stack_mut(&thread_id)?.pop_int()
}

#[inline]
pub(super) fn handle_areturn(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
) -> Result<Value, JvmError> {
    vm.get_stack_mut(&thread_id)?.pop_nullable_ref()
}

#[inline]
pub(super) fn handle_lreturn(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
) -> Result<Value, JvmError> {
    vm.get_stack_mut(&thread_id)?.pop_long()
}

#[inline]
pub(super) fn handle_freturn(
    thread_id: ThreadId,
    vm: &mut VirtualMachine,
) -> Result<Value, JvmError> {
    vm.get_stack_mut(&thread_id)?.pop_float()
}
