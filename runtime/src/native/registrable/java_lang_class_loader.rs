use crate::native::NativeRet;
use crate::vm::Value;
use crate::{ThreadId, VirtualMachine};

pub(super) fn java_lang_class_loader_register_natives(
    vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    _args: &[Value],
) -> NativeRet {
    Ok(None)
}

/*
fn java_lang_class_loader_(
    vm: &mut VirtualMachine,
    thread_id: ThreadId,
    _args: &[Value],
) -> NativeRet {
    Ok(Some(Value::Ref(vm.get_thread(&thread_id).thread_obj)))
}
 */
