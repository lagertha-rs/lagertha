use crate::native::{NativeRegistry, NativeRet};
use crate::{FullyQualifiedMethodKey, ThreadId, VirtualMachine};
use common::jtype::Value;
use tracing_log::log::debug;

pub(super) fn do_register_java_lang_preregistered_natives(native_registry: &mut NativeRegistry) {
    native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "java/lang/Object",
            "getClass",
            "()Ljava/lang/Class;",
            &native_registry.string_interner,
        ),
        java_lang_object_get_class,
    );

    native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "java/lang/Throwable",
            "fillInStackTrace",
            "(I)Ljava/lang/Throwable;",
            &native_registry.string_interner,
        ),
        java_lang_throwable_fill_in_stack_trace,
    );

    native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "java/lang/Float",
            "floatToRawIntBits",
            "(F)I",
            &native_registry.string_interner,
        ),
        java_lang_float_float_to_raw_int_bits,
    );
    native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "java/lang/Double",
            "doubleToRawLongBits",
            "(D)J",
            &native_registry.string_interner,
        ),
        java_lang_double_double_to_raw_long_bits,
    );
    native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "java/lang/Object",
            "hashCode",
            "()I",
            &native_registry.string_interner,
        ),
        java_lang_object_hash_code,
    );

    native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "java/lang/Runtime",
            "maxMemory",
            "()J",
            &native_registry.string_interner,
        ),
        java_lang_runtime_max_memory,
    );
    native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "java/lang/Runtime",
            "availableProcessors",
            "()I",
            &native_registry.string_interner,
        ),
        java_lang_runtime_available_processors,
    );
    native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "java/lang/Object",
            "notifyAll",
            "()V",
            &native_registry.string_interner,
        ),
        java_lang_object_notify_all,
    );
    native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "java/lang/StackTraceElement",
            "initStackTraceElements",
            "([Ljava/lang/StackTraceElement;Ljava/lang/Object;I)V",
            &native_registry.string_interner,
        ),
        java_lang_stack_trace_element_init_stack_trace_elements,
    );
    native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "java/lang/Float",
            "intBitsToFloat",
            "(I)F",
            &native_registry.string_interner,
        ),
        java_lang_float_int_bits_to_float,
    );
    native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "java/lang/NullPointerException",
            "getExtendedNPEMessage",
            "()Ljava/lang/String;",
            &native_registry.string_interner,
        ),
        java_lang_null_pointer_exception_get_extended_npe_message,
    );
    native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "java/lang/System",
            "arraycopy",
            "(Ljava/lang/Object;ILjava/lang/Object;II)V",
            &native_registry.string_interner,
        ),
        java_lang_system_arraycopy,
    );
}

fn java_lang_system_arraycopy(
    vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    args: &[Value],
) -> NativeRet {
    let src_addr = args[0].as_obj_ref()?;
    let src_pos = args[1].as_int()?;
    let dest_addr = args[2].as_obj_ref()?;
    let dest_pos = args[3].as_int()?;
    let length = args[4].as_int()?;

    if length == 0 {
        return Ok(None);
    }

    vm.heap
        .copy_primitive_slice(src_addr, src_pos, dest_addr, dest_pos, length)?;
    Ok(None)
}

fn java_lang_object_get_class(
    vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: java.lang.Class.getClass");
    if let Value::Ref(h) = &args[0] {
        let target_class_id = vm.heap.get_class_id(h)?;
        let res = vm
            .method_area
            .get_mirror_ref_or_create(target_class_id, &mut vm.heap)?;
        Ok(Some(Value::Ref(res)))
    } else {
        panic!("java.lang.Class.getClass: expected object as argument");
    }
}

fn java_lang_object_hash_code(
    _vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: java.lang.Object.hashCode");
    if let Value::Ref(h) = &args[0] {
        Ok(Some(Value::Integer(*h as i32)))
    } else {
        panic!("java.lang.Object.hashCode: expected object as argument");
    }
}

/// Fills the backtrace and depth fields of the Throwable object, it contains the VM internal information
/// about the current stack frames. The backtrace format isn't strictly defined.
/// My backtrace is an array of three arrays:
/// - an int array with the class ids of the classes in the stack frames
/// - an int array with the name indexes of the methods in the stack frames
/// - an int array with the line pc of the methods in the stack frames
fn java_lang_throwable_fill_in_stack_trace(
    vm: &mut VirtualMachine,
    thread_id: ThreadId,
    args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: java.lang.Throwable.fillInStackTrace");
    let mut frames: Vec<_> = vm
        .get_stack(&thread_id)?
        .frames()
        .iter()
        .filter(|frame| {
            let cur_method = vm.method_area.get_method(&frame.method_id());
            cur_method.name != vm.method_area.br().init_sym
                && !vm.method_area.instance_of(
                    cur_method.class_id(),
                    vm.method_area.br().java_lang_throwable_sym,
                )
        })
        .cloned() // TODO: very bad clone
        .collect();
    frames.reverse();
    let int_arr_class = vm
        .method_area
        .load_array_class(vm.method_area.br().int_array_desc)?;
    let class_id_array =
        vm.heap
            .alloc_array_with_default_value(int_arr_class, Value::Integer(0), frames.len())?;
    let method_id_array =
        vm.heap
            .alloc_array_with_default_value(int_arr_class, Value::Integer(0), frames.len())?;
    let line_nbr_array =
        vm.heap
            .alloc_array_with_default_value(int_arr_class, Value::Integer(0), frames.len())?;
    for (pos, frame) in frames.iter().enumerate() {
        let method = vm.method_area.get_method(&frame.method_id());
        vm.heap.write_array_element(
            class_id_array,
            pos,
            Value::Integer(method.class_id().to_i32()),
        )?;
        vm.heap.write_array_element(
            method_id_array,
            pos,
            Value::Integer(frame.method_id().to_i32()),
        )?;
        vm.heap
            .write_array_element(line_nbr_array, pos, Value::Integer(frame.pc() as i32))?;
    }
    let backtrace_addr = vm.heap.alloc_array_with_default_value(
        vm.method_area.br().get_java_lang_object_id()?,
        Value::Null,
        3,
    )?;
    vm.heap
        .write_array_element(backtrace_addr, 0, Value::Ref(class_id_array))?;
    vm.heap
        .write_array_element(backtrace_addr, 1, Value::Ref(method_id_array))?;
    vm.heap
        .write_array_element(backtrace_addr, 2, Value::Ref(line_nbr_array))?;
    let throwable_addr = match args[0] {
        Value::Ref(h) => h,
        _ => panic!("java.lang.Throwable.fillInStackTrace: expected object"),
    };
    let throwable_class_id = vm.heap.get_class_id(&throwable_addr)?;
    let backtrace_field_offset = vm
        .method_area
        .get_instance_class(&throwable_class_id)?
        .get_instance_field_offset(&vm.method_area.br().throwable_backtrace_fk)?;
    let depth_field_offset = vm
        .method_area
        .get_instance_class(&throwable_class_id)?
        .get_instance_field_offset(&vm.method_area.br().throwable_depth_fk)?;
    vm.heap.write_instance_field(
        throwable_addr,
        backtrace_field_offset as usize,
        Value::Ref(backtrace_addr),
    )?;
    vm.heap
        .write_instance_field(
            throwable_addr,
            depth_field_offset as usize,
            Value::Integer(frames.len() as i32),
        )
        .unwrap();

    Ok(Some(Value::Ref(throwable_addr)))
}

fn java_lang_float_float_to_raw_int_bits(
    _vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: java.lang.Float.floatToRawIntBits");
    if let Value::Float(f) = args[0] {
        Ok(Some(Value::Integer(f.to_bits() as i32)))
    } else {
        panic!("java.lang.Float.floatToRawIntBits: expected float argument");
    }
}

fn java_lang_double_double_to_raw_long_bits(
    _vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: java.lang.Double.doubleToRawLongBits");
    if let Value::Double(d) = args[0] {
        Ok(Some(Value::Long(d.to_bits() as i64)))
    } else {
        panic!("java.lang.Double.doubleToRawLongBits: expected double argument");
    }
}

fn java_lang_runtime_max_memory(
    vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    _args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: java.lang.Runtime.maxMemory");
    Ok(Some(Value::Long(vm.config.max_heap_size as i64)))
}

fn java_lang_runtime_available_processors(
    _vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    _args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: java.lang.Runtime.availableProcessors");
    Ok(Some(Value::Integer(1)))
}

fn java_lang_stack_trace_element_init_stack_trace_elements(
    vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    _args: &[Value],
) -> NativeRet {
    /*
    debug!("TODO: Stub: java.lang.StackTraceElement.initStackTraceElements");
    let elements_array = match &_args[0] {
        Value::Ref(h) => *h,
        _ => panic!("java.lang.StackTraceElement.initStackTraceElements: expected array"),
    };
    let object = match &_args[1] {
        Value::Ref(h) => *h,
        _ => panic!("java.lang.StackTraceElement.initStackTraceElements: expected object"),
    };
    let depth = match _args[2] {
        Value::Integer(i) if i >= 0 => i as usize,
        _ => panic!(
            "java.lang.StackTraceElement.initStackTraceElements: expected non-negative depth"
        ),
    };

    // TODO: obviously need to clean this up
    for i in 0..depth {
        let i = i as i32;
        let class_id = ClassIdDeprecated::try_from_usize(
            vm.heap
                .get_array(
                    &vm.heap
                        .get_array(&object)?
                        .get_element(0)
                        .unwrap()
                        .as_obj_ref()
                        .unwrap(),
                )?
                .get_element(i)
                .unwrap()
                .as_int()
                .unwrap() as usize,
        )
        .unwrap();
        let method_id = vm
            .heap
            .get_array(
                &vm.heap
                    .get_array(&object)?
                    .get_element(1)
                    .unwrap()
                    .as_obj_ref()
                    .unwrap(),
            )?
            .get_element(i)
            .unwrap()
            .as_int()
            .unwrap() as usize;
        let cp = vm
            .heap
            .get_array(
                &vm.heap
                    .get_array(&object)?
                    .get_element(2)
                    .unwrap()
                    .as_obj_ref()
                    .unwrap(),
            )?
            .get_element(i)
            .unwrap()
            .as_int()
            .unwrap();
        let class = vm
            .method_area_deprecated
            .get_class_by_id(&class_id)
            .unwrap();
        let declaring_class_object = vm.heap.get_mirror_addr(class).unwrap();
        let method = class.get_method_by_id(&method_id).unwrap();
        let class_name = vm.heap.get_or_new_string(&class.name().replace('/', "."));
        let method_name = vm.heap.get_or_new_string(method.name());
        let source = vm.heap.get_or_new_string(class.source_file().unwrap());
        let line_nbr = method.get_line_number_by_cp(cp).unwrap();
        let cur_stack_trace_entry = vm
            .heap
            .get_array(&elements_array)?
            .get_element(i)
            .unwrap()
            .as_obj_ref()
            .unwrap();

        let stack_trace_class_id = vm.heap.get_class_id(&cur_stack_trace_entry);
        let stack_trace_class = vm
            .method_area_deprecated
            .get_class_by_id(&stack_trace_class_id)
            .unwrap();

        vm.heap
            .write_instance_field(
                cur_stack_trace_entry,
                stack_trace_class.get_field_index("declaringClass").unwrap(),
                Value::Ref(class_name),
            )
            .unwrap();
        vm.heap
            .write_instance_field(
                cur_stack_trace_entry,
                stack_trace_class.get_field_index("methodName").unwrap(),
                Value::Ref(method_name),
            )
            .unwrap();
        vm.heap
            .write_instance_field(
                cur_stack_trace_entry,
                stack_trace_class.get_field_index("fileName").unwrap(),
                Value::Ref(source),
            )
            .unwrap();
        vm.heap
            .write_instance_field(
                cur_stack_trace_entry,
                stack_trace_class.get_field_index("lineNumber").unwrap(),
                Value::Integer(line_nbr as i32),
            )
            .unwrap();
        vm.heap
            .write_instance_field(
                cur_stack_trace_entry,
                stack_trace_class
                    .get_field_index("declaringClassObject")
                    .unwrap(),
                Value::Ref(declaring_class_object),
            )
            .unwrap();
    }
    Ok(None)
     */
    todo!()
}

fn java_lang_object_notify_all(
    _vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    _args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: java.lang.Object.notifyAll");
    Ok(None)
}

fn java_lang_float_int_bits_to_float(
    _vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: java.lang.Float.intBitsToFloat");
    if let Value::Integer(i) = args[0] {
        Ok(Some(Value::Float(f32::from_bits(i as u32))))
    } else {
        panic!("java.lang.Float.intBitsToFloat: expected int argument");
    }
}

fn java_lang_null_pointer_exception_get_extended_npe_message(
    _vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    _args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: java.lang.NullPointerException.getExtendedNPEMessage");
    // For now, just return null, later:
    // https://bugs.openjdk.org/browse/JDK-8218628
    Ok(Some(Value::Null))
}
