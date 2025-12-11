use crate::error::JvmError;
use crate::heap::Heap;
use crate::interpreter::Interpreter;
use crate::keys::FullyQualifiedMethodKey;
use crate::native::NativeRet;
use crate::thread::JavaThreadState;
use crate::vm::Value;
use crate::{ThreadId, VirtualMachine};
use common::jtype::AllocationType;
use tracing_log::log::debug;

pub(super) fn jdk_internal_misc_unsafe_register_natives(
    vm: &mut VirtualMachine,
    _thread: &mut JavaThreadState,
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
    vm.native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "jdk/internal/misc/Unsafe",
            "putByte",
            "(Ljava/lang/Object;JB)V",
            &vm.string_interner,
        ),
        jdk_internal_misc_unsafe_put_byte,
    );
    vm.native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "jdk/internal/misc/Unsafe",
            "ensureClassInitialized0",
            "(Ljava/lang/Class;)V",
            &vm.string_interner,
        ),
        jdk_internal_misc_unsafe_ensure_class_initialized_0,
    );
    vm.native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "jdk/internal/misc/Unsafe",
            "getIntVolatile",
            "(Ljava/lang/Object;J)I",
            &vm.string_interner,
        ),
        jdk_internal_misc_unsafe_get_int_volatile,
    );
    vm.native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "jdk/internal/loader/BootLoader",
            "setBootLoaderUnnamedModule0",
            "(Ljava/lang/Module;)V",
            &vm.string_interner,
        ),
        jdk_internal_loader_boot_loader_set_boot_loader_unnamed_module_0,
    );
    vm.native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "jdk/internal/misc/Unsafe",
            "getLong",
            "(Ljava/lang/Object;J)J",
            &vm.string_interner,
        ),
        jdk_internal_misc_unsafe_get_long,
    );

    vm.native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "jdk/internal/misc/Unsafe",
            "getInt",
            "(Ljava/lang/Object;J)I",
            &vm.string_interner,
        ),
        jdk_internal_misc_unsafe_get_int,
    );

    Ok(None)
}

fn jdk_internal_loader_boot_loader_set_boot_loader_unnamed_module_0(
    _vm: &mut VirtualMachine,
    _thread: &mut JavaThreadState,
    _args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: jdk.internal.loader.BootLoader.setBootLoaderUnnamedModule0");
    // TODO: this is unnamed module handling
    // should take arg 0 (Module instance) and set internally it in somewhere in jvm
    // and then this instance should be returned when .getModule() for bootstrap classes is called
    Ok(None)
}

fn jdk_internal_misc_unsafe_ensure_class_initialized_0(
    vm: &mut VirtualMachine,
    thread: &mut JavaThreadState,
    args: &[Value],
) -> NativeRet {
    let mirror_ref = args[1].as_obj_ref()?;
    let class_id = vm.method_area_read().get_class_id_by_mirror(&mirror_ref)?;
    Interpreter::ensure_initialized(thread, Some(class_id), vm)?;
    Ok(None)
}

fn jdk_internal_misc_unsafe_get_int_volatile(
    vm: &mut VirtualMachine,
    _thread: &mut JavaThreadState,
    args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: jdk.internal.misc.Unsafe.getIntVolatile");
    let base = args[1].as_obj_ref()?;
    let off = args[2].as_long()?;
    let value = vm
        .heap_read()
        .read_field(base, off as usize, AllocationType::Int)?
        .as_int()?;
    Ok(Some(Value::Integer(value)))
}

fn jdk_internal_misc_unsafe_get_long(
    vm: &mut VirtualMachine,
    _thread: &mut JavaThreadState,
    args: &[Value],
) -> NativeRet {
    let base = args[1].as_obj_ref()?;
    let off = args[2].as_long()?;
    let value = vm
        .heap_read()
        .read_field(base, off as usize, AllocationType::Long)?
        .as_long()?;
    Ok(Some(Value::Long(value)))
}

fn jdk_internal_misc_unsafe_get_int(
    vm: &mut VirtualMachine,
    _thread: &mut JavaThreadState,
    args: &[Value],
) -> NativeRet {
    let base = args[1].as_obj_ref()?;
    let off = args[2].as_long()?;
    let value = vm
        .heap_read()
        .read_field(base, off as usize, AllocationType::Int)?
        .as_int()?;
    Ok(Some(Value::Integer(value)))
}

fn jdk_internal_misc_unsafe_array_base_offset_0(
    _vm: &mut VirtualMachine,
    _thread: &mut JavaThreadState,
    _args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: jdk.internal.misc.Unsafe.arrayBaseOffset0");
    Ok(Some(Value::Integer(Heap::ARRAY_ELEMENTS_OFFSET as i32)))
}

fn jdk_internal_misc_unsafe_compare_and_set_int(
    vm: &mut VirtualMachine,
    _thread: &mut JavaThreadState,
    args: &[Value],
) -> NativeRet {
    let object = match args[1] {
        Value::Ref(h) => h,
        _ => panic!("compareAndSetInt: expected object"),
    };
    let offset = match args[2] {
        Value::Long(l) if l >= 0 => l as usize,
        _ => panic!("compareAndSetInt: expected non-negative offset"),
    };
    let expected = match args[3] {
        Value::Integer(i) => i,
        _ => panic!("compareAndSetInt: expected int expected"),
    };
    let new_value = match args[4] {
        Value::Integer(i) => i,
        _ => panic!("compareAndSetInt: expected int new_value"),
    };

    let current = vm
        .heap_read()
        .read_field(object, offset, AllocationType::Int)?;
    if current == Value::Integer(expected) {
        vm.heap_write().write_field(
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
    _thread: &mut JavaThreadState,
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
    let object_field_value = vm
        .heap_read()
        .read_field(object, offset, AllocationType::Long)?;
    if let Value::Long(current_value) = object_field_value {
        if current_value == expected {
            vm.heap_write().write_field(
                object,
                offset,
                Value::Long(new_value),
                AllocationType::Long,
            )?;
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
    _thread: &mut JavaThreadState,
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
    Ok(Some(vm.heap_read().read_field(
        base,
        off as usize,
        AllocationType::Reference,
    )?))
}

fn jdk_internal_misc_unsafe_object_field_offset_1(
    vm: &mut VirtualMachine,
    _thread: &mut JavaThreadState,
    args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: jdk.internal.misc.Unsafe.objectFieldOffset1");
    let class_addr = match &args[1] {
        Value::Ref(h) => h,
        _ => panic!("jdk.internal.misc.Unsafe.objectFieldOffset: expected class object"),
    };
    let field_name = match args[2] {
        Value::Ref(h) => vm.heap_read().get_rust_string_from_java_string(h)?,
        _ => panic!("jdk.internal.misc.Unsafe.objectFieldOffset: expected field name string"),
    };
    let interned_field_name = vm.interner().get_or_intern(&field_name);
    let class_id = vm.method_area_read().get_class_id_by_mirror(class_addr)?;
    let offset = {
        let ma = vm.method_area_read();
        ma.get_instance_class(&class_id)?
            .get_instance_field_by_name(&interned_field_name)?
            .offset
    };
    Ok(Some(Value::Long(offset as i64)))
}

fn jdk_internal_misc_unsafe_array_index_scale_0(
    vm: &mut VirtualMachine,
    _thread: &mut JavaThreadState,
    args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: jdk.internal.misc.Unsafe.arrayIndexScale0");
    let class_addr = match &args[1] {
        Value::Ref(h) => h,
        _ => panic!("arrayIndexScale0: expected class"),
    };
    let class_id = vm.method_area_read().get_class_id_by_mirror(class_addr)?;
    let class_name_sym = vm.method_area_read().get_class(&class_id).get_name();
    let class_name = vm.interner().resolve(&class_name_sym);

    // Parse the class name to get element type
    let scale = match class_name {
        "[I" => 4,
        "[J" => 8,
        "[L" => 8,
        "[Z" => 1,
        "[B" => 1,
        "[C" => 2,
        "[S" => 2,
        "[F" => 4,
        "[D" => 8,
        s if s.starts_with("[L") => 8,
        _ => panic!("Unknown array type: {}", class_name),
    };
    Ok(Some(Value::Integer(scale)))
}

fn jdk_internal_misc_unsafe_full_fence(
    _vm: &mut VirtualMachine,
    _thread: &mut JavaThreadState,
    _args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: jdk.internal.misc.Unsafe.fullFence");
    Ok(None)
}

fn jdk_internal_misc_unsafe_compare_and_set_reference(
    vm: &mut VirtualMachine,
    _thread: &mut JavaThreadState,
    args: &[Value],
) -> NativeRet {
    let object = match args[1] {
        Value::Ref(h) => h,
        _ => panic!("compareAndSetReference: expected object"),
    };
    let offset = match args[2] {
        Value::Long(l) if l >= 0 => l as usize,
        _ => panic!("compareAndSetReference: expected non-negative offset"),
    };
    let expected = match args[3] {
        v @ (Value::Ref(_) | Value::Null) => v,
        _ => panic!("compareAndSetReference: expected object or null"),
    };
    let new_value = match args[4] {
        Value::Ref(h) => h,
        _ => panic!("compareAndSetReference: expected object"),
    };

    let current = vm
        .heap_read()
        .read_field(object, offset, AllocationType::Reference)?;
    if current == expected {
        vm.heap_write().write_field(
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

fn jdk_internal_misc_unsafe_put_byte(
    vm: &mut VirtualMachine,
    _thread: &mut JavaThreadState,
    args: &[Value],
) -> NativeRet {
    let object = args
        .get(1)
        .ok_or(JvmError::Todo("putByte: missing 1 argument".to_string()))?
        .as_obj_ref()?;
    let offset = match args
        .get(2)
        .ok_or(JvmError::Todo("putByte: missing 2 argument".to_string()))?
    {
        Value::Long(l) if *l >= 0 => *l as usize,
        _ => panic!("putByte: expected non-negative offset"),
    };
    let value = args
        .get(3)
        .ok_or(JvmError::Todo("putByte: missing 3 argument".to_string()))?
        .as_int()?;

    vm.heap_write()
        .write_field(object, offset, Value::Integer(value), AllocationType::Byte)?;
    Ok(None)
}
