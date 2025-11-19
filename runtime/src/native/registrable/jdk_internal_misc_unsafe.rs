use crate::heap::gc_new_heap::Heap;
use crate::native::NativeRet;
use crate::{FullyQualifiedMethodKey, ThreadId, VirtualMachine};
use common::Value;
use common::jtype::AllocationType;
use tracing_log::log::debug;

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
    let offset = Heap::OBJECT_HEADER_SIZE + Heap::ARRAY_ELEMENTS_OFFSET;
    Ok(Some(Value::Integer(offset as i32)))
}

fn jdk_internal_misc_unsafe_compare_and_set_int(
    vm: &mut VirtualMachine,
    _thread_id: ThreadId,

    args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: jdk.internal.misc.Unsafe.compareAndSetInt");
    let object = match args[1] {
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
    let object_field_value = vm.heap.read_field(object, offset, AllocationType::Int)?;
    if object_field_value == Value::Integer(expected) {
        vm.heap.write_field(
            object,
            offset,
            Value::Integer(new_value),
            AllocationType::Int,
        )?;
        Ok(Some(Value::Integer(1)))
    } else {
        Ok(Some(Value::Integer(0)))
    }
}

fn jdk_internal_misc_unsafe_compare_and_set_long(
    vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: jdk.internal.misc.Unsafe.compareAndSetLong");
    let object = match args[1] {
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
    let object_field_value = vm.heap.read_field(object, offset, AllocationType::Long)?;
    if let Value::Long(current_value) = object_field_value {
        if current_value == expected {
            vm.heap
                .write_field(object, offset, Value::Long(new_value), AllocationType::Long)?;
            Ok(Some(Value::Integer(1)))
        } else {
            Ok(Some(Value::Integer(0)))
        }
    } else {
        panic!("jdk.internal.misc.Unsafe.compareAndSetLong: field at offset is not long");
    }
}

fn jdk_internal_misc_unsafe_get_reference_volatile(
    vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    args: &[Value],
) -> NativeRet {
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
    Ok(Some(vm.heap.read_field(
        base,
        off as usize,
        AllocationType::Reference,
    )?))
}

fn jdk_internal_misc_unsafe_object_field_offset_1(
    vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: jdk.internal.misc.Unsafe.objectFieldOffset1");
    let class_addr = match &args[1] {
        Value::Ref(h) => h,
        _ => panic!("jdk.internal.misc.Unsafe.objectFieldOffset: expected class object"),
    };
    let field_name = match args[2] {
        Value::Ref(h) => vm.heap.get_rust_string_from_java_string(h)?,
        _ => panic!("jdk.internal.misc.Unsafe.objectFieldOffset: expected field name string"),
    };
    let interned_field_name = vm.interner().get_or_intern(&field_name);
    let class_id = vm.method_area.get_class_id_by_mirror(class_addr)?;
    let offset = vm
        .method_area
        .get_instance_class(&class_id)?
        .get_instance_field_by_name(&interned_field_name)?
        .offset;
    Ok(Some(Value::Long(offset as i64)))
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
    debug!("TODO: Stub: jdk.internal.misc.Unsafe.compareAndSetReference");
    let object = match args[1] {
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
    let object_field_value = vm
        .heap
        .read_field(object, offset, AllocationType::Reference)?;
    if object_field_value == expected {
        vm.heap.write_field(
            object,
            offset,
            Value::Ref(new_value),
            AllocationType::Reference,
        )?;
        Ok(Some(Value::Integer(1)))
    } else {
        Ok(Some(Value::Integer(0)))
    }
}
