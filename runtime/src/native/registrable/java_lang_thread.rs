use crate::keys::FullyQualifiedMethodKey;
use crate::native::NativeRet;
use crate::thread::JavaThreadState;
use crate::vm::Value;
use crate::{ThreadId, VirtualMachine};

pub(super) fn java_lang_thread_register_natives(
    _thread: &mut JavaThreadState,
    _args: &[Value],
) -> NativeRet {
    let vm = VirtualMachine::global();
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

fn java_lang_thread_current_thread(thread: &mut JavaThreadState, _args: &[Value]) -> NativeRet {
    Ok(Some(Value::Ref(thread.thread_obj)))
}
