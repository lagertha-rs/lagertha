use crate::VirtualMachine;
use crate::native::NativeRet;
use crate::stack::FrameStack;
use common::jtype::Value;
use log::debug;

pub(super) fn java_lang_thread_register_natives(
    _vm: &mut VirtualMachine,
    _frame_stack: &FrameStack,
    _args: &[Value],
) -> NativeRet {
    debug!("Stub: java.lang.Thread.registerNatives()");
    Ok(None)
}
