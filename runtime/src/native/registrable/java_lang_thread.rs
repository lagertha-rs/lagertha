use crate::native::NativeRet;
use crate::{ThreadId, VirtualMachine};
use common::jtype::Value;
use tracing_log::log::debug;

pub(super) fn java_lang_thread_register_natives(
    _vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    _args: &[Value],
) -> NativeRet {
    debug!("Stub: java.lang.Thread.registerNatives()");
    Ok(None)
}
