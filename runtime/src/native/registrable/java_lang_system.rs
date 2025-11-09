use crate::native::NativeRet;
use crate::stack_deprecated::FrameStackDeprecated;
use crate::{FullyQualifiedMethodKey, ThreadId, VirtualMachine, throw_exception};
use common::jtype::Value;
use log::debug;

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
    /*
    debug!("TODO: Stub: java.lang.System.setIn0");
    let val = match &args[0] {
        Value::Ref(h) => *h,
        _ => panic!("java.lang.System.setOut0: expected PrintStream object"),
    };
    let system_class = vm
        .method_area_deprecated
        .get_class_or_load_by_name("java/lang/System")
        .unwrap();
    system_class
        .set_static_field("out", "Ljava/io/PrintStream;", Value::Ref(val))
        .unwrap();
    Ok(None)
     */
    todo!()
}

fn java_lang_system_set_err_0(
    vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    args: &[Value],
) -> NativeRet {
    /*
    debug!("TODO: Stub: java.lang.System.setIn0");
    let val = match &args[0] {
        Value::Ref(h) => *h,
        _ => panic!("java.lang.System.setOut0: expected PrintStream object"),
    };
    let system_class = vm
        .method_area_deprecated
        .get_class_or_load_by_name("java/lang/System")
        .unwrap();
    system_class
        .set_static_field("err", "Ljava/io/PrintStream;", Value::Ref(val))
        .unwrap();
    Ok(None)
     */
    todo!()
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
