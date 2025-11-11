use crate::VirtualMachineDeprecated;
use crate::native_deprecated::NativeRetDeprecated;
use crate::stack_deprecated::FrameStackDeprecated;
use common::jtype::Value;
use tracing_log::log::debug;

pub(super) fn java_lang_thread_register_natives(
    _vm: &mut VirtualMachineDeprecated,
    _frame_stack: &FrameStackDeprecated,
    _args: &[Value],
) -> NativeRetDeprecated {
    debug!("Stub: java.lang.Thread.registerNatives()");
    Ok(None)
}
