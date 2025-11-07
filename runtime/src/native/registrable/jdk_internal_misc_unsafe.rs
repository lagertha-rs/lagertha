use crate::native::NativeRet;
use crate::{FullyQualifiedMethodKey, ThreadId, VirtualMachine};
use common::jtype::Value;
use log::debug;

pub(super) fn jdk_internal_misc_unsafe_register_natives(
    vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    _args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: jdk.internal.misc.Unsafe.registerNatives");

    vm.native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "jdk/internal/misc/Unsafe",
            "arrayBaseOffset0",
            "(Ljava/lang/Class;)I",
            &vm.string_interner,
        ),
        jdk_internal_misc_unsafe_array_base_offset_0,
    );
    vm.native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "jdk/internal/misc/Unsafe",
            "arrayIndexScale0",
            "(Ljava/lang/Class;)I",
            &vm.string_interner,
        ),
        jdk_internal_misc_unsafe_array_index_scale_0,
    );
    vm.native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "jdk/internal/misc/Unsafe",
            "objectFieldOffset1",
            "(Ljava/lang/Class;Ljava/lang/String;)J",
            &vm.string_interner,
        ),
        jdk_internal_misc_unsafe_object_field_offset_1,
    );
    vm.native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "jdk/internal/misc/Unsafe",
            "fullFence",
            "()V",
            &vm.string_interner,
        ),
        jdk_internal_misc_unsafe_full_fence,
    );
    vm.native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "jdk/internal/misc/Unsafe",
            "compareAndSetInt",
            "(Ljava/lang/Object;JII)Z",
            &vm.string_interner,
        ),
        jdk_internal_misc_unsafe_compare_and_set_int,
    );
    vm.native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "jdk/internal/misc/Unsafe",
            "getReferenceVolatile",
            "(Ljava/lang/Object;J)Ljava/lang/Object;",
            &vm.string_interner,
        ),
        jdk_internal_misc_unsafe_get_reference_volatile,
    );
    vm.native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "jdk/internal/misc/Unsafe",
            "compareAndSetReference",
            "(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z",
            &vm.string_interner,
        ),
        jdk_internal_misc_unsafe_compare_and_set_reference,
    );
    vm.native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "jdk/internal/misc/Unsafe",
            "compareAndSetLong",
            "(Ljava/lang/Object;JJJ)Z",
            &vm.string_interner,
        ),
        jdk_internal_misc_unsafe_compare_and_set_long,
    );

    Ok(None)
}

fn jdk_internal_misc_unsafe_array_base_offset_0(
    _vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    _args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: jdk.internal.misc.Unsafe.arrayBaseOffset0");
    Ok(Some(Value::Integer(0)))
}

fn jdk_internal_misc_unsafe_compare_and_set_int(
    vm: &mut VirtualMachine,
    _thread_id: ThreadId,

    args: &[Value],
) -> NativeRet {
    /*
    debug!("TODO: Stub: jdk.internal.misc.Unsafe.compareAndSetInt");
    let object = match &args[1] {
        Value::Ref(h) => h,
        _ => panic!("jdk.internal.misc.Unsafe.compareAndSetLong: expected object"),
    };
    let offset = match args[2] {
        Value::Long(l) if l >= 0 => l as usize,
        _ => panic!("jdk.internal.misc.Unsafe.compareAndSetLong: expected non-negative offset"),
    };
    let expected = match args[3] {
        Value::Integer(l) => l,
        _ => panic!("jdk.internal.misc.Unsafe.compareAndSetLong: expected long expected value"),
    };
    let new_value = match args[4] {
        Value::Integer(l) => l,
        _ => panic!("jdk.internal.misc.Unsafe.compareAndSetLong: expected long new value"),
    };
    let instance = vm.heap.get_instance_mut(object);
    let field = instance.get_element_mut(offset as i32).unwrap();
    if let Value::Integer(current_value) = field {
        if *current_value == expected {
            *field = Value::Integer(new_value);
            Ok(Some(Value::Integer(1)))
        } else {
            Ok(Some(Value::Integer(0)))
        }
    } else {
        panic!("jdk.internal.misc.Unsafe.compareAndSetLong: field at offset is not long");
    }
     */
    todo!()
}

fn jdk_internal_misc_unsafe_compare_and_set_long(
    vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    args: &[Value],
) -> NativeRet {
    /*
    debug!("TODO: Stub: jdk.internal.misc.Unsafe.compareAndSetLong");
    let object = match &args[1] {
        Value::Ref(h) => h,
        _ => panic!("jdk.internal.misc.Unsafe.compareAndSetLong: expected object"),
    };
    let offset = match args[2] {
        Value::Long(l) if l >= 0 => l as usize,
        _ => panic!("jdk.internal.misc.Unsafe.compareAndSetLong: expected non-negative offset"),
    };
    let expected = match args[3] {
        Value::Long(l) => l,
        _ => panic!("jdk.internal.misc.Unsafe.compareAndSetLong: expected long expected value"),
    };
    let new_value = match args[4] {
        Value::Long(l) => l,
        _ => panic!("jdk.internal.misc.Unsafe.compareAndSetLong: expected long new value"),
    };
    let instance = vm.heap.get_instance_mut(object);
    let field = instance.get_element_mut(offset as i32).unwrap();
    if let Value::Long(current_value) = field {
        if *current_value == expected {
            *field = Value::Long(new_value);
            Ok(Some(Value::Integer(1)))
        } else {
            Ok(Some(Value::Integer(0)))
        }
    } else {
        panic!("jdk.internal.misc.Unsafe.compareAndSetLong: field at offset is not long");
    }
     */
    todo!()
}

fn jdk_internal_misc_unsafe_get_reference_volatile(
    vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    args: &[Value],
) -> NativeRet {
    /*
    debug!("TODO: Stub: jdk.internal.misc.Unsafe.getReferenceVolatile");
    let base = match &args[1] {
        Value::Ref(h) => *h,
        Value::Null => panic!("Unsafe.getReferenceVolatile base is null"),
        _ => panic!("Unsafe.getReferenceVolatile expects an object base"),
    };

    let off = match args[2] {
        Value::Long(x) => x,
        _ => panic!("Unsafe.getReferenceVolatile expects a long offset"),
    };
    match vm.heap.get(base)? {
        HeapObject::Instance(instance) => {
            let field = instance.get_element(off as i32).unwrap();
            match field {
                Value::Ref(_) => Ok(Some(*field)),
                _ => panic!("Unsafe.getReferenceVolatile field is not an object"),
            }
        }
        HeapObject::Array(array) => {
            let idx = off as usize;
            if idx >= array.length() {
                panic!("Unsafe.getReferenceVolatile array index out of bounds");
            }
            let element = &array.elements()[idx];
            match element {
                Value::Ref(_) | Value::Null => Ok(Some(*element)),
                _ => panic!("Unsafe.getReferenceVolatile array element is not an object"),
            }
        }
    }
     */
    todo!()
}

fn jdk_internal_misc_unsafe_object_field_offset_1(
    vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    args: &[Value],
) -> NativeRet {
    /*
    debug!("TODO: Stub: jdk.internal.misc.Unsafe.objectFieldOffset");
    let class_addr = match &args[1] {
        Value::Ref(h) => h,
        _ => panic!("jdk.internal.misc.Unsafe.objectFieldOffset: expected class object"),
    };
    let field_name = match &args[2] {
        Value::Ref(h) => {
            let s = vm.heap.get_string(*h).unwrap();
            s.to_string()
        }
        _ => panic!("jdk.internal.misc.Unsafe.objectFieldOffset: expected field name string"),
    };
    let class = vm.heap.get_class_by_mirror(class_addr).unwrap();
    let offset = class.get_field_index(&field_name).unwrap();
    Ok(Some(Value::Long(offset as i64)))
     */
    todo!()
}

fn jdk_internal_misc_unsafe_array_index_scale_0(
    _vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    _args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: jdk.internal.misc.Unsafe.arrayIndexScale0");
    Ok(Some(Value::Integer(1)))
}

fn jdk_internal_misc_unsafe_full_fence(
    _vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    _args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: jdk.internal.misc.Unsafe.fullFence");
    Ok(None)
}

// TODO: pure mess
fn jdk_internal_misc_unsafe_compare_and_set_reference(
    vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    args: &[Value],
) -> NativeRet {
    /*
    debug!("TODO: Stub: jdk.internal.misc.Unsafe.compareAndSetReference");
    let object = match &args[1] {
        Value::Ref(h) => h,
        _ => panic!("jdk.internal.misc.Unsafe.compareAndSetReference: expected object"),
    };
    let offset = match args[2] {
        Value::Long(l) if l >= 0 => l as usize,
        _ => {
            panic!("jdk.internal.misc.Unsafe.compareAndSetReference: expected non-negative offset")
        }
    };
    let expected = match args[3] {
        Value::Ref(_) | Value::Null => args[3],
        _ => {
            panic!("jdk.internal.misc.Unsafe.compareAndSetReference: expected object")
        }
    };
    let new_value = match args[4] {
        Value::Ref(h) => h,
        _ => panic!("jdk.internal.misc.Unsafe.compareAndSetReference: expected long new value"),
    };
    match vm.heap.get_mut(*object) {
        HeapObject::Array(array) => {
            if offset >= array.length() {
                panic!(
                    "jdk.internal.misc.Unsafe.compareAndSetReference: array index out of bounds"
                );
            }
            let field = &mut array.elements_mut()[offset];
            if *field == expected {
                *field = Value::Ref(new_value);
                Ok(Some(Value::Integer(1)))
            } else {
                Ok(Some(Value::Integer(0)))
            }
        }
        HeapObject::Instance(instance) => {
            let field = instance.get_element_mut(offset as i32).unwrap();
            if *field == expected {
                *field = Value::Ref(new_value);
                Ok(Some(Value::Integer(1)))
            } else {
                Ok(Some(Value::Integer(0)))
            }
        }
    }
     */
    todo!()
}
