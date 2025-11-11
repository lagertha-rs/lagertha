use crate::native_deprecated::NativeRetDeprecated;
use crate::stack_deprecated::FrameStackDeprecated;
use crate::{FullyQualifiedMethodKey, VirtualMachineDeprecated, throw_exception};
use common::jtype::Value;
use tracing_log::log::debug;

pub(super) fn java_lang_system_register_natives(
    vm: &mut VirtualMachineDeprecated,
    _frame_stack: &FrameStackDeprecated,
    _args: &[Value],
) -> NativeRetDeprecated {
    vm.native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "java/lang/System",
            "arraycopy",
            "(Ljava/lang/Object;ILjava/lang/Object;II)V",
            &vm.string_interner,
        ),
        java_lang_system_arraycopy,
    );
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
    vm: &mut VirtualMachineDeprecated,
    _frame_stack: &FrameStackDeprecated,
    args: &[Value],
) -> NativeRetDeprecated {
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
}

fn java_lang_system_set_err_0(
    vm: &mut VirtualMachineDeprecated,
    _frame_stack: &FrameStackDeprecated,
    args: &[Value],
) -> NativeRetDeprecated {
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
}

fn java_lang_system_arraycopy(
    vm: &mut VirtualMachineDeprecated,
    _frame_stack: &FrameStackDeprecated,
    args: &[Value],
) -> NativeRetDeprecated {
    let src_addr = args[0].as_obj_ref()?;
    let src_pos = args[1].as_int()?;
    let dest_addr = args[2].as_obj_ref()?;
    let dest_pos = args[3].as_int()?;
    let length = args[4].as_int()?;

    let src_class_id = vm.heap.get_class_id(&src_addr);
    let src_class = vm.method_area_deprecated.get_class_by_id(&src_class_id)?;
    if !vm.heap.addr_is_array(&src_addr)? {
        throw_exception!(
            ArrayStoreException,
            "arraycopy: source type {} is not an array",
            src_class.pretty_name()
        )?;
    }
    let name = src_class
        .primitive()
        .map(|p| p.as_str())
        .unwrap_or(src_class.name());

    let dest_class_id = vm.heap.get_class_id(&dest_addr);
    let dest_class = vm.method_area_deprecated.get_class_by_id(&dest_class_id)?;
    if !vm.heap.addr_is_array(&dest_addr)? {
        throw_exception!(
            ArrayStoreException,
            "arraycopy: destination type {} is not an array",
            dest_class.pretty_name()
        )?;
    }

    if length == 0 {
        return Ok(None);
    }

    vm.heap
        .copy_primitive_slice(src_addr, src_pos, dest_addr, dest_pos, length)?;
    Ok(None)
}

fn java_lang_system_identity_hash_code(
    _vm: &mut VirtualMachineDeprecated,
    _frame_stack: &FrameStackDeprecated,
    args: &[Value],
) -> NativeRetDeprecated {
    debug!("TODO: Stub: java.lang.System.identityHashCode");
    if let Value::Ref(h) = &args[0] {
        Ok(Some(Value::Integer(*h as i32)))
    } else {
        panic!("java.lang.System.identityHashCode: expected object as argument");
    }
}

fn java_lang_system_set_in_0(
    _vm: &mut VirtualMachineDeprecated,
    _frame_stack: &FrameStackDeprecated,
    _args: &[Value],
) -> NativeRetDeprecated {
    debug!("TODO: Stub: java.lang.System.setIn0");
    Ok(None)
}
