use crate::{MethodId, ThreadId, VirtualMachine};
use common::error::JvmError;
use common::jtype::Value;

pub struct Interpreter;

impl Interpreter {
    pub fn invoke_static_method(
        vm: &mut VirtualMachine,
        thread_id: ThreadId,
        method_id: MethodId,
        args: Vec<Value>,
    ) -> Result<(), JvmError> {
        Ok(())
    }
}
