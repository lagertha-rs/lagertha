use crate::VirtualMachine;
use crate::keys::{FullyQualifiedMethodKey, ThreadId};
use crate::native::{NativeRegistry, NativeRet};
use crate::thread::JavaThreadState;
use crate::vm::Value;
use tracing_log::log::debug;

pub(super) fn do_register_jdk_internal_reflect_preregistered_natives(
    native_registry: &mut NativeRegistry,
) {
    native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "jdk/internal/reflect/Reflection",
            "getCallerClass",
            "()Ljava/lang/Class;",
            &native_registry.string_interner,
        ),
        jdk_internal_reflect_reflection_get_caller_class,
    );
    native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "jdk/internal/reflect/Reflection",
            "getClassAccessFlags",
            "(Ljava/lang/Class;)I",
            &native_registry.string_interner,
        ),
        jdk_internal_reflect_reflection_get_class_access_flags,
    );
}

fn jdk_internal_reflect_reflection_get_caller_class(
    vm: &mut VirtualMachine,
    thread: &mut JavaThreadState,
    _args: &[Value],
) -> NativeRet {
    debug!("Stub: jdk/internal/reflect/Reflection.getCallerClass()");
    // TODO: hardcoded. should use @CallerSensitive
    let frame_minus_two = thread.stack.peek_frame_at(2)?;
    let method_id = frame_minus_two.method_id();
    let class_id = vm.method_area_read().get_method(&method_id).class_id();
    let res = vm
        .method_area_write()
        .get_mirror_ref_or_create(class_id, &vm.heap)?;
    Ok(Some(Value::Ref(res)))
}

fn jdk_internal_reflect_reflection_get_class_access_flags(
    vm: &mut VirtualMachine,
    _thread: &mut JavaThreadState,
    args: &[Value],
) -> NativeRet {
    // TODO: implement properly (there are comments in source)
    let mirror_ref = args[0].as_obj_ref()?;
    let class_id = vm.method_area_read().get_class_id_by_mirror(&mirror_ref)?;
    let flags = vm.method_area_read().get_class(&class_id).get_raw_flags();
    Ok(Some(Value::Integer(flags)))
}
