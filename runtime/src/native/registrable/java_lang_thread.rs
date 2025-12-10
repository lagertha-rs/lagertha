use crate::keys::FullyQualifiedMethodKey;
use crate::native::NativeRet;
use crate::vm::Value;
use crate::{ThreadId, VirtualMachine};

pub(super) fn java_lang_thread_register_natives(
    vm: &mut VirtualMachine,
    _thread: &mut JavaThreadState,
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
    thread: &mut JavaThreadState,
    _args: &[Value],
) -> NativeRet {
    Ok(Some(Value::Ref(vm.get_thread(&thread_id).thread_obj)))
}
