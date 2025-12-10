use crate::error::JvmError;
use crate::thread::JavaThreadState;
use crate::vm::Value;

#[inline]
pub(super) fn handle_dreturn(thread: &mut JavaThreadState) -> Result<Value, JvmError> {
    thread.stack.pop_double()
}
#[inline]
pub(super) fn handle_ireturn(thread: &mut JavaThreadState) -> Result<Value, JvmError> {
    thread.stack.pop_int()
}

#[inline]
pub(super) fn handle_areturn(thread: &mut JavaThreadState) -> Result<Value, JvmError> {
    thread.stack.pop_nullable_ref()
}

#[inline]
pub(super) fn handle_lreturn(thread: &mut JavaThreadState) -> Result<Value, JvmError> {
    thread.stack.pop_long()
}

#[inline]
pub(super) fn handle_freturn(thread: &mut JavaThreadState) -> Result<Value, JvmError> {
    thread.stack.pop_float()
}
