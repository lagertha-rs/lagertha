use crate::VirtualMachine;
use crate::heap::HeapObject;
use crate::native::{MethodKey, NativeRegistry, NativeRet};
use common::jtype::Value;
use log::debug;

pub(super) fn jdk_internal_misc_scoped_memory_access_register_natives(
    _vm: &mut VirtualMachine,
    _args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: jdk.internal.misc.ScopedMemoryAccess.registerNatives");
    Ok(None)
}
