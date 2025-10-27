use crate::VirtualMachine;
use crate::native::{MethodKey, NativeRet};
use common::jtype::Value;
use log::debug;

pub(super) fn java_lang_system_register_natives(
    vm: &mut VirtualMachine,
    _args: &[Value],
) -> NativeRet {
    vm.native_registry.register(
        MethodKey::new_with_str(
            "java/lang/System",
            "arraycopy",
            "(Ljava/lang/Object;ILjava/lang/Object;II)V",
            &vm.string_interner,
        ),
        java_lang_system_arraycopy,
    );
    vm.native_registry.register(
        MethodKey::new_with_str(
            "java/lang/System",
            "setIn0",
            "(Ljava/io/InputStream;)V",
            &vm.string_interner,
        ),
        java_lang_system_set_in_0,
    );
    vm.native_registry.register(
        MethodKey::new_with_str(
            "java/lang/System",
            "setOut0",
            "(Ljava/io/PrintStream;)V",
            &vm.string_interner,
        ),
        java_lang_system_set_out_0,
    );
    vm.native_registry.register(
        MethodKey::new_with_str(
            "java/lang/System",
            "setErr0",
            "(Ljava/io/PrintStream;)V",
            &vm.string_interner,
        ),
        java_lang_system_set_err_0,
    );
    vm.native_registry.register(
        MethodKey::new_with_str(
            "java/lang/System",
            "identityHashCode",
            "(Ljava/lang/Object;)I",
            &vm.string_interner,
        ),
        java_lang_system_identity_hash_code,
    );
    Ok(None)
}

fn java_lang_system_set_out_0(vm: &mut VirtualMachine, args: &[Value]) -> NativeRet {
    debug!("TODO: Stub: java.lang.System.setIn0");
    let val = match &args[0] {
        Value::Ref(h) => *h,
        _ => panic!("java.lang.System.setOut0: expected PrintStream object"),
    };
    let system_class = vm
        .method_area
        .get_class_or_load_by_name("java/lang/System")
        .unwrap();
    system_class
        .set_static_field("out", "Ljava/io/PrintStream;", Value::Ref(val))
        .unwrap();
    Ok(None)
}

fn java_lang_system_set_err_0(vm: &mut VirtualMachine, args: &[Value]) -> NativeRet {
    debug!("TODO: Stub: java.lang.System.setIn0");
    let val = match &args[0] {
        Value::Ref(h) => *h,
        _ => panic!("java.lang.System.setOut0: expected PrintStream object"),
    };
    let system_class = vm
        .method_area
        .get_class_or_load_by_name("java/lang/System")
        .unwrap();
    system_class
        .set_static_field("err", "Ljava/io/PrintStream;", Value::Ref(val))
        .unwrap();
    Ok(None)
}

fn java_lang_system_arraycopy(vm: &mut VirtualMachine, args: &[Value]) -> NativeRet {
    let src_addr = args[0].as_obj_ref()?;
    let src_pos = args[1].as_int()?;
    let dest_addr = args[2].as_obj_ref()?;
    let dest_pos = args[3].as_int()?;
    let length = args[4].as_int()?;

    let src_class_id = *vm.heap.get_array(&src_addr)?.class_id();
    let src_class = vm.method_area.get_class_by_id(&src_class_id)?;
    let anme = src_class.name();

    if length == 0 {
        return Ok(None);
    }

    vm.heap
        .copy_primitive_slice(src_addr, src_pos, dest_addr, dest_pos, length)?;
    Ok(None)
}

fn java_lang_system_identity_hash_code(_vm: &mut VirtualMachine, args: &[Value]) -> NativeRet {
    debug!("TODO: Stub: java.lang.System.identityHashCode");
    if let Value::Ref(h) = &args[0] {
        Ok(Some(Value::Integer(*h as i32)))
    } else {
        panic!("java.lang.System.identityHashCode: expected object as argument");
    }
}

fn java_lang_system_set_in_0(_vm: &mut VirtualMachine, _args: &[Value]) -> NativeRet {
    debug!("TODO: Stub: java.lang.System.setIn0");
    Ok(None)
}
