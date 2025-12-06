use crate::VirtualMachine;
use crate::keys::{FullyQualifiedMethodKey, ThreadId};
use crate::native::{NativeRegistry, NativeRet};
use crate::vm::Value;
use tracing_log::log::debug;

pub(super) fn do_register_java_lang_module_preregistered_natives(
    native_registry: &mut NativeRegistry,
) {
    native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "java/lang/Module",
            "defineModule0",
            "(Ljava/lang/Module;ZLjava/lang/String;Ljava/lang/String;[Ljava/lang/Object;)V",
            &native_registry.string_interner,
        ),
        java_lang_module_define_module_0,
    )
}

fn java_lang_module_define_module_0(
    _vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    _args: &[Value],
) -> NativeRet {
    debug!("Stub: java/lang/Module.defineModule0()");
    Ok(None)
}
