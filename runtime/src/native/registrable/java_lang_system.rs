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
    None
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
    None
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
    None
}

fn java_lang_system_arraycopy(vm: &mut VirtualMachine, args: &[Value]) -> NativeRet {
    debug!("TODO: Stub: java.lang.System.arraycopy");
    let src = match &args[0] {
        Value::Ref(h) => *h,
        _ => panic!("java.lang.System.arraycopy: expected source array object"),
    };
    let src_pos = match args[1] {
        Value::Integer(i) if i >= 0 => i as usize,
        _ => panic!("java.lang.System.arraycopy: expected non-negative source position"),
    };
    let dest = match &args[2] {
        Value::Ref(h) => *h,
        _ => panic!("java.lang.System.arraycopy: expected destination array object"),
    };
    let dest_pos = match args[3] {
        Value::Integer(i) if i >= 0 => i as usize,
        _ => panic!("java.lang.System.arraycopy: expected non-negative destination position"),
    };
    let length = match args[4] {
        Value::Integer(i) if i >= 0 => i as usize,
        _ => panic!("java.lang.System.arraycopy: expected non-negative length"),
    };
    let tmp: Vec<Value> = {
        let arr = vm.heap.get_array(&src);
        let src_array_len = arr.length();
        let slice: &[Value] = arr.elements();
        if src_pos
            .checked_add(length)
            .map_or(true, |end| end > src_array_len)
        {
            panic!("java.lang.System.arraycopy: source index out of bounds");
        }
        slice[src_pos..src_pos + length].to_vec()
    };

    {
        let dest_array_len = vm.heap.get_array(&dest).length();
        if dest_pos
            .checked_add(length)
            .map_or(true, |end| end > dest_array_len)
        {
            panic!("java.lang.System.arraycopy: destination index out of bounds");
        }

        for i in 0..length {
            vm.heap
                .write_array_element(dest, (dest_pos + i) as i32, tmp[i].clone())
                .unwrap();
        }
    }
    None
}

fn java_lang_system_identity_hash_code(_vm: &mut VirtualMachine, args: &[Value]) -> NativeRet {
    debug!("TODO: Stub: java.lang.System.identityHashCode");
    if let Value::Ref(h) = &args[0] {
        Some(Value::Integer(*h as i32))
    } else {
        panic!("java.lang.System.identityHashCode: expected object as argument");
    }
}

fn java_lang_system_set_in_0(_vm: &mut VirtualMachine, _args: &[Value]) -> NativeRet {
    debug!("TODO: Stub: java.lang.System.setIn0");
    None
}
