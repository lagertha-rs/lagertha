use crate::native::NativeRet;
use crate::{FullyQualifiedMethodKey, ThreadId, VirtualMachine};
use common::jtype::Value;
use tracing_log::log::debug;

pub(super) fn java_lang_thread_register_natives(
    vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    _args: &[Value],
) -> NativeRet {
    vm.native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "java/lang/Thread",
            "currentThread",
            "()Ljava/lang/Thread;",
            &vm.string_interner,
        ),
        java_lang_thread_current_thread,
    );
    Ok(None)
}

fn java_lang_thread_current_thread(
    vm: &mut VirtualMachine,
    thread_id: ThreadId,
    _args: &[Value],
) -> NativeRet {
    Ok(Some(Value::Ref(vm.get_thread(&thread_id).thread_obj)))
}
