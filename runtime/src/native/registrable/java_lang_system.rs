use crate::native::NativeRet;
use crate::{FullyQualifiedMethodKey, ThreadId, VirtualMachine};
use common::jtype::Value;
use tracing_log::log::debug;

pub(super) fn java_lang_system_register_natives(
    vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    _args: &[Value],
) -> NativeRet {
    vm.native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "java/lang/System",
            "setIn0",
            "(Ljava/io/InputStream;)V",
            &vm.string_interner,
        ),
        java_lang_system_set_in_0,
    );
    vm.native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "java/lang/System",
            "setOut0",
            "(Ljava/io/PrintStream;)V",
            &vm.string_interner,
        ),
        java_lang_system_set_out_0,
    );
    vm.native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "java/lang/System",
            "setErr0",
            "(Ljava/io/PrintStream;)V",
            &vm.string_interner,
        ),
        java_lang_system_set_err_0,
    );
    vm.native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "java/lang/System",
            "identityHashCode",
            "(Ljava/lang/Object;)I",
            &vm.string_interner,
        ),
        java_lang_system_identity_hash_code,
    );
    Ok(None)
}

fn java_lang_system_set_out_0(
    vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: java.lang.System.setIn0");
    let val = match &args[0] {
        Value::Ref(h) => *h,
        _ => panic!("java.lang.System.setOut0: expected PrintStream object"),
    };
    let system_class_id = vm
        .method_area
        .get_class_id_or_load(vm.method_area.br().java_lang_system_sym)?;
    vm.method_area
        .get_class_like(&system_class_id)?
        .set_static_field_value(&vm.method_area.br().system_out_fk, Value::Ref(val))?;
    Ok(None)
}

fn java_lang_system_set_err_0(
    vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: java.lang.System.setIn0");
    let val = match &args[0] {
        Value::Ref(h) => *h,
        _ => panic!("java.lang.System.setOut0: expected PrintStream object"),
    };
    let system_class_id = vm
        .method_area
        .get_class_id_or_load(vm.method_area.br().java_lang_system_sym)?;
    vm.method_area
        .get_class_like(&system_class_id)?
        .set_static_field_value(&vm.method_area.br().system_err_fk, Value::Ref(val))?;
    Ok(None)
}

fn java_lang_system_identity_hash_code(
    _vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: java.lang.System.identityHashCode");
    if let Value::Ref(h) = &args[0] {
        Ok(Some(Value::Integer(*h as i32)))
    } else {
        panic!("java.lang.System.identityHashCode: expected object as argument");
    }
}

fn java_lang_system_set_in_0(
    _vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    _args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: java.lang.System.setIn0");
    Ok(None)
}
