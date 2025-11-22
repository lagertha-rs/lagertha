use crate::keys::FullyQualifiedMethodKey;
use crate::native::NativeRet;
use crate::rt::JvmClass;
use crate::{ThreadId, VirtualMachine};
use common::Value;
use tracing_log::log::debug;

pub(super) fn java_lang_class_register_natives(
    vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    _args: &[Value],
) -> NativeRet {
    vm.native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "java/lang/Class",
            "desiredAssertionStatus0",
            "(Ljava/lang/Class;)Z",
            &vm.string_interner,
        ),
        java_lang_class_desired_assertion_status_0,
    );

    vm.native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "java/lang/Class",
            "getPrimitiveClass",
            "(Ljava/lang/String;)Ljava/lang/Class;",
            &vm.string_interner,
        ),
        java_lang_class_get_primitive_class,
    );

    vm.native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "java/lang/Class",
            "initClassName",
            "()Ljava/lang/String;",
            &vm.string_interner,
        ),
        java_lang_class_init_class_name,
    );

    vm.native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "java/lang/Class",
            "isPrimitive",
            "()Z",
            &vm.string_interner,
        ),
        java_lang_class_is_primitive,
    );

    Ok(None)
}

fn java_lang_class_desired_assertion_status_0(
    _vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    _args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: java.lang.Class.desiredAssertionStatus0");
    Ok(Some(Value::Integer(1)))
}

fn java_lang_class_is_primitive(
    vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: java.lang.Class.isPrimitive");
    if let Value::Ref(h) = &args[0] {
        let target_class_id = vm.method_area.get_class_id_by_mirror(h)?;
        let is_primitive = matches!(
            vm.method_area.get_class(&target_class_id),
            JvmClass::Primitive(_)
        );
        Ok(Some(Value::Integer(if is_primitive { 1 } else { 0 })))
    } else {
        panic!("java.lang.Class.isPrimitive: expected object");
    }
}

fn java_lang_class_get_primitive_class(
    vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: java.lang.Class.getPrimitiveClass");
    if let Value::Ref(h) = args[0] {
        let primitive_name = vm.heap.get_rust_string_from_java_string(h)?;

        let class_id = vm
            .method_area
            .get_class_id_or_load(vm.interner().get_or_intern(&primitive_name))?;
        let v = vm
            .method_area
            .get_mirror_ref_or_create(class_id, &mut vm.heap)?;
        Ok(Some(Value::Ref(v)))
    } else {
        panic!("java.lang.Class.getPrimitiveClass: expected object");
    }
}

fn java_lang_class_init_class_name(
    vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    args: &[Value],
) -> NativeRet {
    /*
    debug!("TODO: Stub: java.lang.Class.initClassName");
    if let Value::Ref(h) = &args[0] {
        let class_name = vm
            .heap
            .get_class_by_mirror(h)
            .unwrap()
            .name()
            .replace('/', ".");
        let val = Value::Ref(vm.heap.get_or_new_string(&class_name));
        let class_id = vm.heap.get_class_id(h);
        let class = vm
            .method_area_deprecated
            .get_class_by_id(&class_id)
            .unwrap();
        vm.heap
            .write_instance_field(*h, class.get_field_index("name").unwrap(), val)
            .unwrap();
        Ok(Some(val))
    } else {
        panic!("java.lang.Class.initClassName: expected object as argument");
    }
     */
    todo!()
}
