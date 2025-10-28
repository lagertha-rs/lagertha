use crate::VirtualMachine;
use crate::native::NativeRet;
use crate::stack::FrameStack;
use common::jtype::Value;
use log::debug;

pub(super) fn jdk_internal_misc_scoped_memory_access_register_natives(
    _vm: &mut VirtualMachine,
    _frame_stack: &FrameStack,

    _args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: jdk.internal.misc.ScopedMemoryAccess.registerNatives");
    Ok(None)
}
