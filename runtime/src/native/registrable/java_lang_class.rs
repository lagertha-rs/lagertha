use crate::keys::FullyQualifiedMethodKey;
use crate::native::NativeRet;
use crate::{ThreadId, VirtualMachine};
use common::Value;
use common::error::JvmError;
use common::jtype::AllocationType;
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
            "isInterface",
            "()Z",
            &vm.string_interner,
        ),
        java_lang_class_is_interface,
    );

    vm.native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "java/lang/Class",
            "isArray",
            "()Z",
            &vm.string_interner,
        ),
        java_lang_class_is_array,
    );

    vm.native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "java/lang/Class",
            "getModifiers",
            "()I",
            &vm.string_interner,
        ),
        java_lang_class_get_modifiers,
    );

    vm.native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "java/lang/Class",
            "getSuperclass",
            "()Ljava/lang/Class;",
            &vm.string_interner,
        ),
        java_lang_class_get_superclass,
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

fn java_lang_class_is_interface(
    vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    args: &[Value],
) -> NativeRet {
    let mirror_ref = args
        .first()
        .ok_or(JvmError::Todo(
            "java.lang.Class.isInterface: missing 0 argument".to_string(),
        ))?
        .as_obj_ref()?;
    let target_class_id = vm.method_area.get_class_id_by_mirror(&mirror_ref)?;
    let is_interface = vm.method_area.get_class(&target_class_id).is_interface();
    Ok(Some(Value::Integer(if is_interface { 1 } else { 0 })))
}

fn java_lang_class_is_array(
    vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    args: &[Value],
) -> NativeRet {
    let mirror_ref = args
        .first()
        .ok_or(JvmError::Todo(
            "java.lang.Class.isArray: missing 0 argument".to_string(),
        ))?
        .as_obj_ref()?;
    let target_class_id = vm.method_area.get_class_id_by_mirror(&mirror_ref)?;
    let is_array = vm.method_area.get_class(&target_class_id).is_array();
    Ok(Some(Value::Integer(if is_array { 1 } else { 0 })))
}

fn java_lang_class_get_modifiers(
    vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    args: &[Value],
) -> NativeRet {
    let mirror_ref = args
        .first()
        .ok_or(JvmError::Todo(
            "java.lang.Class.getModifiers: missing 0 argument".to_string(),
        ))?
        .as_obj_ref()?;
    let target_class_id = vm.method_area.get_class_id_by_mirror(&mirror_ref)?;
    let modifiers = vm.method_area.get_class(&target_class_id).get_raw_flags();
    Ok(Some(Value::Integer(modifiers)))
}

fn java_lang_class_get_primitive_class(
    vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    args: &[Value],
) -> NativeRet {
    let primitive_name_ref = args
        .first()
        .ok_or(JvmError::Todo(
            "java.lang.Class.getPrimitiveClass: missing 0 argument".to_string(),
        ))?
        .as_obj_ref()?;
    let primitive_name = vm
        .heap
        .get_rust_string_from_java_string(primitive_name_ref)?;
    let class_id = vm
        .method_area
        .get_class_id_or_load(vm.interner().get_or_intern(&primitive_name))?;
    let v = vm
        .method_area
        .get_mirror_ref_or_create(class_id, &mut vm.heap)?;
    Ok(Some(Value::Ref(v)))
}

fn java_lang_class_init_class_name(
    vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    args: &[Value],
) -> NativeRet {
    let mirror_ref = args
        .first()
        .ok_or(JvmError::Todo(
            "java.lang.Class.initClassName: missing 0 argument".to_string(),
        ))?
        .as_obj_ref()?;
    let class_class_id = vm.br.get_java_lang_class_id()?;
    let class_name_fk = vm.br.class_name_fk;
    let target_class_id = vm.method_area.get_class_id_by_mirror(&mirror_ref)?;
    let name_sym = vm.method_area.get_class(&target_class_id).get_name();
    let name_ref = vm.heap.alloc_string_from_interned_with_char_mapping(
        name_sym,
        Some(&|c| {
            if c == '/' { '.' } else { c }
        }),
    )?;
    let name_field = vm
        .method_area
        .get_instance_class(&class_class_id)?
        .get_instance_field(&class_name_fk)?;
    vm.heap.write_field(
        mirror_ref,
        name_field.offset,
        Value::Ref(name_ref),
        AllocationType::Reference,
    )?;
    Ok(Some(Value::Ref(name_ref)))
}

fn java_lang_class_get_superclass(
    vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    args: &[Value],
) -> NativeRet {
    let mirror_ref = args
        .first()
        .ok_or(JvmError::Todo(
            "java.lang.Class.getSuperclass: missing 0 argument".to_string(),
        ))?
        .as_obj_ref()?;
    let target_class_id = vm.method_area.get_class_id_by_mirror(&mirror_ref)?;
    let target_class = vm.method_area.get_class(&target_class_id);
    let super_class_id = target_class.get_super_id();
    if target_class.is_interface() || target_class.is_primitive() {
        Ok(Some(Value::Null))
    } else if let Some(super_id) = super_class_id {
        let super_mirror_ref = vm
            .method_area
            .get_mirror_ref_or_create(super_id, &mut vm.heap)?;
        Ok(Some(Value::Ref(super_mirror_ref)))
    } else {
        Ok(Some(Value::Null))
    }
}
