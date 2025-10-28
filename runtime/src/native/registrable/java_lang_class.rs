use crate::VirtualMachine;
use crate::native::{MethodKey, NativeRet};
use crate::stack::FrameStack;
use common::jtype::Value;
use log::debug;

pub(super) fn java_lang_class_register_natives(
    vm: &mut VirtualMachine,
    _frame_stack: &FrameStack,

    _args: &[Value],
) -> NativeRet {
    vm.native_registry.register(
        MethodKey::new_with_str(
            "java/lang/Class",
            "desiredAssertionStatus0",
            "(Ljava/lang/Class;)Z",
            &vm.string_interner,
        ),
        java_lang_class_desired_assertion_status_0,
    );

    vm.native_registry.register(
        MethodKey::new_with_str(
            "java/lang/Class",
            "getPrimitiveClass",
            "(Ljava/lang/String;)Ljava/lang/Class;",
            &vm.string_interner,
        ),
        java_lang_class_get_primitive_class,
    );

    vm.native_registry.register(
        MethodKey::new_with_str(
            "java/lang/Class",
            "initClassName",
            "()Ljava/lang/String;",
            &vm.string_interner,
        ),
        java_lang_class_init_class_name,
    );

    vm.native_registry.register(
        MethodKey::new_with_str("java/lang/Class", "isPrimitive", "()Z", &vm.string_interner),
        java_lang_class_is_primitive,
    );

    Ok(None)
}

fn java_lang_class_desired_assertion_status_0(
    _vm: &mut VirtualMachine,
    _frame_stack: &FrameStack,
    _args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: java.lang.Class.desiredAssertionStatus0");
    Ok(Some(Value::Integer(1)))
}

fn java_lang_class_is_primitive(
    vm: &mut VirtualMachine,
    _frame_stack: &FrameStack,
    args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: java.lang.Class.isPrimitive");
    if let Value::Ref(h) = &args[0] {
        let is_primitive = vm.heap.addr_is_primitive(h);
        Ok(Some(Value::Integer(if is_primitive { 1 } else { 0 })))
    } else {
        panic!("java.lang.Class.isPrimitive: expected object");
    }
}

fn java_lang_class_get_primitive_class(
    vm: &mut VirtualMachine,
    _frame_stack: &FrameStack,
    args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: java.lang.Class.getPrimitiveClass");
    if let Value::Ref(h) = &args[0] {
        let v = vm.heap.get_primitive_mirror_addr(h).unwrap();
        Ok(Some(Value::Ref(v)))
    } else {
        panic!("java.lang.Class.getPrimitiveClass: expected object");
    }
}

fn java_lang_class_init_class_name(
    vm: &mut VirtualMachine,
    _frame_stack: &FrameStack,
    args: &[Value],
) -> NativeRet {
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
        let class = vm.method_area.get_class_by_id(&class_id).unwrap();
        vm.heap
            .write_instance_field(*h, class.get_field_index("name").unwrap(), val)
            .unwrap();
        Ok(Some(val))
    } else {
        panic!("java.lang.Class.initClassName: expected object as argument");
    }
}
