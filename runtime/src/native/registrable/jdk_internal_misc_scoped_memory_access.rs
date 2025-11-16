use crate::native::NativeRet;
use crate::{ThreadId, VirtualMachine};
use common::Value;
use tracing_log::log::debug;

pub(super) fn jdk_internal_misc_scoped_memory_access_register_natives(
    _vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    _args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: jdk.internal.misc.ScopedMemoryAccess.registerNatives");
    Ok(None)
}
