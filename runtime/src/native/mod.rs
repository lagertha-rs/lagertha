use crate::VirtualMachine;
use crate::heap::HeapObject;
use common::instruction::ArrayType;
use common::jtype::Value;
use std::collections::HashMap;
use tracing_log::log::debug;

//TODO: redesign, avoid string allocations here
#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct MethodKey {
    pub class: String,
    pub name: String,
    pub desc: String,
}

impl MethodKey {
    pub fn new(class: String, name: String, desc: String) -> Self {
        Self { class, name, desc }
    }
}

//TODO: redesign, avoid string allocations here
#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct InternalMethodKey {
    pub name: String,
    pub desc: String,
}

impl InternalMethodKey {
    pub fn new(name: String, desc: String) -> Self {
        Self { name, desc }
    }
}

pub type NativeRet = Option<Value>;
pub type NativeFn = fn(&mut VirtualMachine, &[Value]) -> NativeRet;

pub struct NativeRegistry {
    map: HashMap<MethodKey, NativeFn>,
    internal: HashMap<InternalMethodKey, NativeFn>,
}

impl NativeRegistry {
    pub fn new() -> Self {
        debug!("Initializing NativeRegistry...");
        let mut instance = Self {
            map: HashMap::new(),
            internal: HashMap::new(),
        };

        instance.register_internal(
            InternalMethodKey::new("clone".to_string(), "()Ljava/lang/Object;".to_string()),
            vm_internal_clone,
        );

        instance.register(
            MethodKey::new(
                "java/lang/System".to_string(),
                "registerNatives".to_string(),
                "()V".to_string(),
            ),
            java_lang_system_register_natives,
        );
        instance.register(
            MethodKey::new(
                "java/lang/Class".to_string(),
                "registerNatives".to_string(),
                "()V".to_string(),
            ),
            java_lang_class_register_natives,
        );
        instance.register(
            MethodKey::new(
                "java/lang/Class".to_string(),
                "desiredAssertionStatus0".to_string(),
                "(Ljava/lang/Class;)Z".to_string(),
            ),
            java_lang_class_desired_assertion_status_0,
        );
        instance.register(
            MethodKey::new(
                "jdk/internal/util/SystemProps$Raw".to_string(),
                "platformProperties".to_string(),
                "()[Ljava/lang/String;".to_string(),
            ),
            jdk_internal_util_system_props_raw_platform_properties,
        );
        instance.register(
            MethodKey::new(
                "jdk/internal/util/SystemProps$Raw".to_string(),
                "vmProperties".to_string(),
                "()[Ljava/lang/String;".to_string(),
            ),
            jdk_internal_util_system_props_raw_vm_properties,
        );
        instance.register(
            MethodKey::new(
                "java/lang/Class".to_string(),
                "getPrimitiveClass".to_string(),
                "(Ljava/lang/String;)Ljava/lang/Class;".to_string(),
            ),
            java_lang_class_get_primitive_class,
        );
        instance.register(
            MethodKey::new(
                "java/lang/Object".to_string(),
                "getClass".to_string(),
                "()Ljava/lang/Class;".to_string(),
            ),
            java_lang_object_get_class,
        );
        instance.register(
            MethodKey::new(
                "java/lang/Class".to_string(),
                "initClassName".to_string(),
                "()Ljava/lang/String;".to_string(),
            ),
            java_lang_object_init_class_name,
        );
        instance.register(
            MethodKey::new(
                "java/lang/System".to_string(),
                "arraycopy".to_string(),
                "(Ljava/lang/Object;ILjava/lang/Object;II)V".to_string(),
            ),
            java_lang_system_arraycopy,
        );
        instance.register(
            MethodKey::new(
                "jdk/internal/misc/Unsafe".to_string(),
                "registerNatives".to_string(),
                "()V".to_string(),
            ),
            jdk_internal_misc_unsafe_register_natives,
        );
        instance.register(
            MethodKey::new(
                "java/lang/Throwable".to_string(),
                "fillInStackTrace".to_string(),
                "(I)Ljava/lang/Throwable;".to_string(),
            ),
            java_lang_throwable_fill_in_stack_trace,
        );
        instance.register(
            MethodKey::new(
                "java/lang/Class".to_string(),
                "isPrimitive".to_string(),
                "()Z".to_string(),
            ),
            java_lang_class_is_primitive,
        );
        instance.register(
            MethodKey::new(
                "jdk/internal/misc/Unsafe".to_string(),
                "arrayBaseOffset0".to_string(),
                "(Ljava/lang/Class;)I".to_string(),
            ),
            jdk_internal_misc_unsafe_array_base_offset_0,
        );
        instance.register(
            MethodKey::new(
                "jdk/internal/misc/Unsafe".to_string(),
                "arrayIndexScale0".to_string(),
                "(Ljava/lang/Class;)I".to_string(),
            ),
            jdk_internal_misc_unsafe_array_index_scale_0,
        );
        instance.register(
            MethodKey::new(
                "java/lang/Float".to_string(),
                "floatToRawIntBits".to_string(),
                "(F)I".to_string(),
            ),
            java_lang_float_float_to_raw_int_bits,
        );
        instance.register(
            MethodKey::new(
                "java/lang/Double".to_string(),
                "doubleToRawLongBits".to_string(),
                "(D)J".to_string(),
            ),
            java_lang_double_double_to_raw_long_bits,
        );
        instance.register(
            MethodKey::new(
                "java/lang/Object".to_string(),
                "hashCode".to_string(),
                "()I".to_string(),
            ),
            java_lang_object_hash_code,
        );
        instance.register(
            MethodKey::new(
                "jdk/internal/misc/VM".to_string(),
                "initialize".to_string(),
                "()V".to_string(),
            ),
            jdk_internal_misc_vm_initialize,
        );
        instance.register(
            MethodKey::new(
                "java/lang/Runtime".to_string(),
                "maxMemory".to_string(),
                "()J".to_string(),
            ),
            java_lang_runtime_max_memory,
        );
        instance.register(
            MethodKey::new(
                "java/lang/Runtime".to_string(),
                "availableProcessors".to_string(),
                "()I".to_string(),
            ),
            java_lang_runtime_available_processors,
        );
        instance.register(
            MethodKey::new(
                "jdk/internal/misc/Unsafe".to_string(),
                "objectFieldOffset1".to_string(),
                "(Ljava/lang/Class;Ljava/lang/String;)J".to_string(),
            ),
            jdk_internal_misc_unsafe_object_field_offset_1,
        );
        instance.register(
            MethodKey::new(
                "jdk/internal/misc/Unsafe".to_string(),
                "fullFence".to_string(),
                "()V".to_string(),
            ),
            jdk_internal_misc_unsafe_full_fence,
        );
        instance.register(
            MethodKey::new(
                "jdk/internal/misc/Unsafe".to_string(),
                "compareAndSetInt".to_string(),
                "(Ljava/lang/Object;JII)Z".to_string(),
            ),
            jdk_internal_misc_unsafe_compare_and_set_int,
        );
        instance.register(
            MethodKey::new(
                "jdk/internal/misc/Unsafe".to_string(),
                "getReferenceVolatile".to_string(),
                "(Ljava/lang/Object;J)Ljava/lang/Object;".to_string(),
            ),
            jdk_internal_misc_unsafe_get_reference_volatile,
        );
        instance.register(
            MethodKey::new(
                "jdk/internal/misc/Unsafe".to_string(),
                "compareAndSetReference".to_string(),
                "(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z".to_string(),
            ),
            jdk_internal_misc_unsafe_compare_and_set_reference,
        );
        instance.register(
            MethodKey::new(
                "jdk/internal/misc/Unsafe".to_string(),
                "compareAndSetLong".to_string(),
                "(Ljava/lang/Object;JJJ)Z".to_string(),
            ),
            jdk_internal_misc_unsafe_compare_and_set_long,
        );
        instance.register(
            MethodKey::new(
                "java/io/FileInputStream".to_string(),
                "initIDs".to_string(),
                "()V".to_string(),
            ),
            java_io_file_input_stream_init_ids,
        );
        instance.register(
            MethodKey::new(
                "java/io/FileDescriptor".to_string(),
                "initIDs".to_string(),
                "()V".to_string(),
            ),
            java_io_file_descriptor_init_ids,
        );
        instance.register(
            MethodKey::new(
                "java/io/FileDescriptor".to_string(),
                "getHandle".to_string(),
                "(I)J".to_string(),
            ),
            java_io_file_descriptor_get_handle,
        );
        instance.register(
            MethodKey::new(
                "java/io/FileDescriptor".to_string(),
                "getAppend".to_string(),
                "(I)Z".to_string(),
            ),
            java_io_file_descriptor_get_append,
        );
        instance.register(
            MethodKey::new(
                "java/io/FileOutputStream".to_string(),
                "initIDs".to_string(),
                "()V".to_string(),
            ),
            java_io_file_output_stream_init_ids,
        );
        instance.register(
            MethodKey::new(
                "java/lang/System".to_string(),
                "setIn0".to_string(),
                "(Ljava/io/InputStream;)V".to_string(),
            ),
            java_lang_system_set_in_0,
        );
        instance.register(
            MethodKey::new(
                "java/lang/System".to_string(),
                "setOut0".to_string(),
                "(Ljava/io/PrintStream;)V".to_string(),
            ),
            java_lang_system_set_out_0,
        );
        instance.register(
            MethodKey::new(
                "java/lang/System".to_string(),
                "setErr0".to_string(),
                "(Ljava/io/PrintStream;)V".to_string(),
            ),
            java_lang_system_set_err_0,
        );
        instance.register(
            MethodKey::new(
                "jdk/internal/misc/ScopedMemoryAccess".to_string(),
                "registerNatives".to_string(),
                "()V".to_string(),
            ),
            jdk_internal_misc_scoped_memory_access_register_natives,
        );
        instance.register(
            MethodKey::new(
                "jdk/internal/misc/Signal".to_string(),
                "findSignal0".to_string(),
                "(Ljava/lang/String;)I".to_string(),
            ),
            jdk_internal_misc_signal_find_signal_0,
        );
        instance.register(
            MethodKey::new(
                "jdk/internal/misc/Signal".to_string(),
                "handle0".to_string(),
                "(IJ)J".to_string(),
            ),
            jdk_internal_misc_signal_handle_0,
        );
        instance.register(
            MethodKey::new(
                "jdk/internal/misc/CDS".to_string(),
                "getCDSConfigStatus".to_string(),
                "()I".to_string(),
            ),
            jdk_internal_misc_cds_get_cds_config_status,
        );
        instance.register(
            MethodKey::new(
                "jdk/internal/misc/CDS".to_string(),
                "initializeFromArchive".to_string(),
                "(Ljava/lang/Class;)V".to_string(),
            ),
            jdk_internal_misc_cds_initialize_from_archive,
        );
        instance.register(
            MethodKey::new(
                "java/lang/Object".to_string(),
                "notifyAll".to_string(),
                "()V".to_string(),
            ),
            java_lang_object_notify_all,
        );
        instance.register(
            MethodKey::new(
                "java/io/FileOutputStream".to_string(),
                "writeBytes".to_string(),
                "([BIIZ)V".to_string(),
            ),
            java_io_file_output_stream_write_bytes,
        );
        instance.register(
            MethodKey::new(
                "java/lang/System".to_string(),
                "identityHashCode".to_string(),
                "(Ljava/lang/Object;)I".to_string(),
            ),
            java_lang_system_identity_hash_code,
        );
        instance.register(
            MethodKey::new(
                "java/lang/StackTraceElement".to_string(),
                "initStackTraceElements".to_string(),
                "([Ljava/lang/StackTraceElement;Ljava/lang/Object;I)V".to_string(),
            ),
            java_lang_stack_trace_element_init_stack_trace_elements,
        );

        instance
    }

    fn register(&mut self, key: MethodKey, f: NativeFn) {
        debug!("Registering native method: {:?}", key);
        self.map.insert(key, f);
    }

    fn register_internal(&mut self, key: InternalMethodKey, f: NativeFn) {
        debug!("Registering internal native method: {:?}", key);
        self.internal.insert(key, f);
    }

    pub fn get(&self, key: &MethodKey) -> Option<&NativeFn> {
        if key.class.starts_with("[") {
            //TODO: redesign, avoid string allocations here
            let internal_key = InternalMethodKey::new(key.name.clone(), key.desc.clone());
            self.internal.get(&internal_key)
        } else {
            self.map.get(key)
        }
    }
}

fn java_lang_system_register_natives(_vm: &mut VirtualMachine, _args: &[Value]) -> NativeRet {
    debug!("TODO: Stub: Registering java.lang.System native methods");
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
        let heap = vm.heap.borrow();
        let arr = heap.get_array(&src);
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
        let mut heap = vm.heap.borrow_mut();
        let dest_array_len = heap.get_array(&dest).length();
        if dest_pos
            .checked_add(length)
            .map_or(true, |end| end > dest_array_len)
        {
            panic!("java.lang.System.arraycopy: destination index out of bounds");
        }

        for i in 0..length {
            heap.write_array_element(dest, dest_pos + i, tmp[i].clone())
                .unwrap();
        }
    }
    None
}

fn java_lang_class_register_natives(_vm: &mut VirtualMachine, _args: &[Value]) -> NativeRet {
    debug!("TODO: Stub: Registering java.lang.Class native methods");
    None
}

fn java_lang_class_desired_assertion_status_0(
    _vm: &mut VirtualMachine,
    _args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: java.lang.Class.desiredAssertionStatus0");
    Some(Value::Integer(1))
}

fn java_lang_class_is_primitive(vm: &mut VirtualMachine, args: &[Value]) -> NativeRet {
    debug!("TODO: Stub: java.lang.Class.isPrimitive");
    if let Value::Ref(h) = &args[0] {
        let is_primitive = vm.method_area().addr_is_primitive(h);
        Some(Value::Integer(if is_primitive { 1 } else { 0 }))
    } else {
        panic!("java.lang.Class.isPrimitive: expected object");
    }
}

fn java_lang_class_get_primitive_class(vm: &mut VirtualMachine, args: &[Value]) -> NativeRet {
    debug!("TODO: Stub: java.lang.Class.getPrimitiveClass");
    if let Value::Ref(h) = &args[0] {
        let v = vm.get_primitive_mirror_addr(h);
        Some(Value::Ref(v))
    } else {
        panic!("java.lang.Class.getPrimitiveClass: expected object");
    }
}

fn java_lang_object_get_class(vm: &mut VirtualMachine, args: &[Value]) -> NativeRet {
    debug!("TODO: Stub: java.lang.Class.getClass");
    if let Value::Ref(h) = &args[0] {
        let target_class = if let Some(obj) = vm.heap().borrow().get(*h) {
            match obj {
                HeapObject::Instance(instance) => instance.class().clone(),
                HeapObject::Array(array) => array.class().clone(),
            }
        } else {
            panic!("java.lang.Class.getClass: invalid heap address");
        };
        let res = vm.get_mirror_addr_by_class(&target_class).unwrap();
        Some(Value::Ref(res))
    } else {
        panic!("java.lang.Class.getClass: expected object as argument");
    }
}

fn java_lang_object_hash_code(_vm: &mut VirtualMachine, args: &[Value]) -> NativeRet {
    debug!("TODO: Stub: java.lang.Object.hashCode");
    if let Value::Ref(h) = &args[0] {
        Some(Value::Integer(*h as i32))
    } else {
        panic!("java.lang.Object.hashCode: expected object as argument");
    }
}

fn java_lang_system_identity_hash_code(_vm: &mut VirtualMachine, args: &[Value]) -> NativeRet {
    debug!("TODO: Stub: java.lang.System.identityHashCode");
    if let Value::Ref(h) = &args[0] {
        Some(Value::Integer(*h as i32))
    } else {
        panic!("java.lang.System.identityHashCode: expected object as argument");
    }
}

fn java_lang_object_init_class_name(vm: &mut VirtualMachine, args: &[Value]) -> NativeRet {
    debug!("TODO: Stub: java.lang.Class.initClassName");
    if let Value::Ref(h) = &args[0] {
        let class_name = vm
            .method_area()
            .get_class_by_mirror(h)
            .unwrap()
            .name()
            .replace('/', ".");
        let val = Value::Ref(vm.heap().borrow_mut().get_or_new_string(&class_name));
        vm.heap()
            .borrow_mut()
            .write_instance_field(*h, "name", "Ljava/lang/String;", val)
            .unwrap();
        Some(val)
    } else {
        panic!("java.lang.Class.initClassName: expected object as argument");
    }
}

fn jdk_internal_util_system_props_raw_platform_properties(
    vm: &mut VirtualMachine,
    _args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: jdk.internal.util.SystemProps$Raw.platformProperties");
    let string_class = vm.method_area().get_class("java/lang/String").unwrap();
    let mut heap = vm.heap.borrow_mut();
    let empty_string_stub = heap.get_or_new_string("");
    let h = heap.alloc_array_with_value(string_class, 40, Value::Ref(empty_string_stub));
    let enc = heap.get_or_new_string("UTF-8");
    let line_separator_value = heap.get_or_new_string("\n");
    heap.write_array_element(h, 19, Value::Ref(line_separator_value))
        .unwrap();
    heap.write_array_element(h, 27, Value::Ref(enc)).unwrap();
    heap.write_array_element(h, 28, Value::Ref(enc)).unwrap();
    heap.write_array_element(h, 34, Value::Ref(enc)).unwrap();

    Some(Value::Ref(h))
}

fn jdk_internal_util_system_props_raw_vm_properties(
    vm: &mut VirtualMachine,
    _args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: jdk.internal.util.SystemProps$Raw.vmProperties");
    let string_class = vm.method_area().get_class("java/lang/String").unwrap();
    let h = vm.heap().borrow_mut().alloc_array(string_class, 4);
    let mut heap = vm.heap.borrow_mut();
    let java_home_key = heap.get_or_new_string("java.home");
    let java_home_value = heap.get_or_new_string(&vm.config.home);
    let sun_page_align_stub = heap.get_or_new_string("sun.nio.PageAlignDirectMemory");
    let false_str = heap.get_or_new_string("false");
    heap.write_array_element(h, 0, Value::Ref(java_home_key))
        .unwrap();
    heap.write_array_element(h, 1, Value::Ref(java_home_value))
        .unwrap();
    heap.write_array_element(h, 2, Value::Ref(sun_page_align_stub))
        .unwrap();
    heap.write_array_element(h, 3, Value::Ref(false_str))
        .unwrap();
    Some(Value::Ref(h))
}

fn jdk_internal_misc_unsafe_register_natives(
    _vm: &mut VirtualMachine,
    _args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: jdk.internal.misc.Unsafe.registerNatives");
    None
}

/// Fills the backtrace and depth fields of the Throwable object, it contains the VM internal information
/// about the current stack frames. The backtrace format isn't strictly defined.
/// My backtrace is an array of three arrays:
/// - an int array with the class ids of the classes in the stack frames
/// - an int array with the name indexes of the methods in the stack frames
/// - an int array with the line pc of the methods in the stack frames
fn java_lang_throwable_fill_in_stack_trace(vm: &mut VirtualMachine, args: &[Value]) -> NativeRet {
    debug!("TODO: Stub: java.lang.Throwable.fillInStackTrace");
    let mut frames: Vec<_> = vm
        .frame_stack
        .frames()
        .iter()
        .filter(|frame| {
            frame.method().name() != "<init>"
                && !frame
                    .method()
                    .class()
                    .unwrap()
                    .is_subclass_of("java/lang/Throwable")
        })
        .collect();
    let mut heap = vm.heap.borrow_mut();
    let int_class = vm
        .method_area
        .get_class(ArrayType::Int.descriptor())
        .unwrap();
    let class_id_array = heap.alloc_array(int_class.clone(), frames.len());
    let method_id_array = heap.alloc_array(int_class.clone(), frames.len());
    let line_nbr_array = heap.alloc_array(int_class, frames.len());
    for (pos, frame) in frames.iter().enumerate() {
        heap.write_array_element(
            class_id_array,
            pos,
            Value::Integer(frame.method().class_id().unwrap() as i32),
        )
        .unwrap();
        heap.write_array_element(
            method_id_array,
            pos,
            Value::Integer(frame.method().id().unwrap() as i32),
        )
        .unwrap();
        heap.write_array_element(line_nbr_array, pos, Value::Integer(frame.pc() as i32))
            .unwrap();
    }
    let obj_class = vm.method_area.get_class("java/lang/Object").unwrap();
    let backtrace_addr = heap.alloc_array(obj_class, 3);
    heap.write_array_element(backtrace_addr, 0, Value::Ref(class_id_array))
        .unwrap();
    heap.write_array_element(backtrace_addr, 1, Value::Ref(method_id_array))
        .unwrap();
    heap.write_array_element(backtrace_addr, 2, Value::Ref(line_nbr_array))
        .unwrap();
    let throwable_addr = match args[0] {
        Value::Ref(h) => h,
        _ => panic!("java.lang.Throwable.fillInStackTrace: expected object"),
    };
    heap.write_instance_field(
        throwable_addr,
        "backtrace",
        "Ljava/lang/Object;",
        Value::Ref(backtrace_addr),
    )
    .unwrap();
    heap.write_instance_field(
        throwable_addr,
        "depth",
        "I",
        Value::Integer(frames.len() as i32),
    )
    .unwrap();

    Some(Value::Ref(throwable_addr))
}

fn jdk_internal_misc_unsafe_array_base_offset_0(
    _vm: &mut VirtualMachine,
    _args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: jdk.internal.misc.Unsafe.arrayBaseOffset0");
    Some(Value::Integer(0))
}

fn jdk_internal_misc_unsafe_compare_and_set_int(
    vm: &mut VirtualMachine,
    args: &[Value],
) -> NativeRet {
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
    let mut heap = vm.heap.borrow_mut();
    let instance = heap.get_instance_mut(object);
    let field = instance.get_field_mut(offset);
    if let Value::Integer(current_value) = field {
        if *current_value == expected {
            *field = Value::Integer(new_value);
            Some(Value::Integer(1))
        } else {
            Some(Value::Integer(0))
        }
    } else {
        panic!("jdk.internal.misc.Unsafe.compareAndSetLong: field at offset is not long");
    }
}

fn jdk_internal_misc_unsafe_compare_and_set_long(
    vm: &mut VirtualMachine,
    args: &[Value],
) -> NativeRet {
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
    let mut heap = vm.heap.borrow_mut();
    let instance = heap.get_instance_mut(object);
    let field = instance.get_field_mut(offset);
    if let Value::Long(current_value) = field {
        if *current_value == expected {
            *field = Value::Long(new_value);
            Some(Value::Integer(1))
        } else {
            Some(Value::Integer(0))
        }
    } else {
        panic!("jdk.internal.misc.Unsafe.compareAndSetLong: field at offset is not long");
    }
}

fn jdk_internal_misc_unsafe_get_reference_volatile(
    vm: &mut VirtualMachine,
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
    let heap = vm.heap.borrow();
    match heap.get(base) {
        Some(HeapObject::Instance(instance)) => {
            let field = instance.get_field(off as usize);
            match field {
                Value::Ref(_) => Some(*field),
                _ => panic!("Unsafe.getReferenceVolatile field is not an object"),
            }
        }
        Some(HeapObject::Array(array)) => {
            let idx = off as usize;
            if idx >= array.length() {
                panic!("Unsafe.getReferenceVolatile array index out of bounds");
            }
            let element = &array.elements()[idx];
            match element {
                Value::Ref(_) | Value::Null => Some(*element),
                _ => panic!("Unsafe.getReferenceVolatile array element is not an object"),
            }
        }
        None => panic!("Unsafe.getReferenceVolatile base address is invalid"),
    }
}

fn jdk_internal_misc_unsafe_object_field_offset_1(
    vm: &mut VirtualMachine,
    args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: jdk.internal.misc.Unsafe.objectFieldOffset");
    let class_addr = match &args[1] {
        Value::Ref(h) => h,
        _ => panic!("jdk.internal.misc.Unsafe.objectFieldOffset: expected class object"),
    };
    let field_name = match &args[2] {
        Value::Ref(h) => {
            let heap = vm.heap.borrow();
            let s = heap.get_string(*h).unwrap();
            s.to_string()
        }
        _ => panic!("jdk.internal.misc.Unsafe.objectFieldOffset: expected field name string"),
    };
    let class = vm.method_area().get_class_by_mirror(class_addr).unwrap();
    let offset = class.get_field_index_by_name(&field_name).unwrap();
    Some(Value::Long(offset as i64))
}

fn jdk_internal_misc_unsafe_array_index_scale_0(
    _vm: &mut VirtualMachine,
    _args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: jdk.internal.misc.Unsafe.arrayIndexScale0");
    Some(Value::Integer(1))
}

fn jdk_internal_misc_unsafe_full_fence(_vm: &mut VirtualMachine, _args: &[Value]) -> NativeRet {
    debug!("TODO: Stub: jdk.internal.misc.Unsafe.fullFence");
    None
}

// TODO: pure mess
fn jdk_internal_misc_unsafe_compare_and_set_reference(
    vm: &mut VirtualMachine,
    args: &[Value],
) -> NativeRet {
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
    let mut heap = vm.heap.borrow_mut();
    let new_value = match args[4] {
        Value::Ref(h) => h,
        _ => panic!("jdk.internal.misc.Unsafe.compareAndSetReference: expected long new value"),
    };
    match heap.get_mut(*object) {
        HeapObject::Array(array) => {
            if offset >= array.length() {
                panic!(
                    "jdk.internal.misc.Unsafe.compareAndSetReference: array index out of bounds"
                );
            }
            let field = &mut array.elements_mut()[offset];
            if *field == expected {
                *field = Value::Ref(new_value);
                Some(Value::Integer(1))
            } else {
                Some(Value::Integer(0))
            }
        }
        HeapObject::Instance(instance) => {
            let field = instance.get_field_mut(offset);
            if *field == expected {
                *field = Value::Ref(new_value);
                Some(Value::Integer(1))
            } else {
                Some(Value::Integer(0))
            }
        }
    }
}

fn jdk_internal_misc_vm_initialize(_vm: &mut VirtualMachine, _args: &[Value]) -> NativeRet {
    debug!("TODO: Stub: jdk.internal.misc.VM.initialize");
    None
}

fn java_lang_float_float_to_raw_int_bits(_vm: &mut VirtualMachine, args: &[Value]) -> NativeRet {
    debug!("TODO: Stub: java.lang.Float.floatToRawIntBits");
    if let Value::Float(f) = args[0] {
        Some(Value::Integer(f.to_bits() as i32))
    } else {
        panic!("java.lang.Float.floatToRawIntBits: expected float argument");
    }
}

fn java_lang_double_double_to_raw_long_bits(_vm: &mut VirtualMachine, args: &[Value]) -> NativeRet {
    debug!("TODO: Stub: java.lang.Double.doubleToRawLongBits");
    if let Value::Double(d) = args[0] {
        Some(Value::Long(d.to_bits() as i64))
    } else {
        panic!("java.lang.Double.doubleToRawLongBits: expected double argument");
    }
}

fn java_lang_runtime_max_memory(vm: &mut VirtualMachine, _args: &[Value]) -> NativeRet {
    debug!("TODO: Stub: java.lang.Runtime.maxMemory");
    Some(Value::Long(vm.config.max_heap_size as i64))
}

fn java_lang_runtime_available_processors(_vm: &mut VirtualMachine, _args: &[Value]) -> NativeRet {
    debug!("TODO: Stub: java.lang.Runtime.availableProcessors");
    Some(Value::Integer(1))
}

fn java_io_file_input_stream_init_ids(_vm: &mut VirtualMachine, _args: &[Value]) -> NativeRet {
    debug!("TODO: Stub: java.io.FileInputStream.initIDs");
    None
}

fn java_io_file_descriptor_init_ids(_vm: &mut VirtualMachine, _args: &[Value]) -> NativeRet {
    debug!("TODO: Stub: java.io.FileDescriptor.initIDs");
    None
}

fn java_io_file_descriptor_get_handle(_vm: &mut VirtualMachine, _args: &[Value]) -> NativeRet {
    debug!("TODO: Stub: java.io.FileDescriptor.getHandle");
    Some(Value::Long(0))
}

fn java_io_file_descriptor_get_append(_vm: &mut VirtualMachine, _args: &[Value]) -> NativeRet {
    debug!("TODO: Stub: java.io.FileDescriptor.getAppend");
    Some(Value::Integer(0))
}

fn java_io_file_output_stream_init_ids(_vm: &mut VirtualMachine, _args: &[Value]) -> NativeRet {
    debug!("TODO: Stub: java.io.FileInputStream.initIDs");
    None
}

fn java_lang_system_set_in_0(_vm: &mut VirtualMachine, _args: &[Value]) -> NativeRet {
    debug!("TODO: Stub: java.lang.System.setIn0");
    None
}

fn java_lang_system_set_out_0(vm: &mut VirtualMachine, args: &[Value]) -> NativeRet {
    debug!("TODO: Stub: java.lang.System.setIn0");
    let val = match &args[0] {
        Value::Ref(h) => *h,
        _ => panic!("java.lang.System.setOut0: expected PrintStream object"),
    };
    let system_class = vm.method_area().get_class("java/lang/System").unwrap();
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
    let system_class = vm.method_area().get_class("java/lang/System").unwrap();
    system_class
        .set_static_field("err", "Ljava/io/PrintStream;", Value::Ref(val))
        .unwrap();
    None
}

fn java_lang_stack_trace_element_init_stack_trace_elements(
    vm: &mut VirtualMachine,
    _args: &[Value],
) -> NativeRet {
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
    let h = vm.heap();
    let heap = h.borrow_mut();
    let array = heap.get_array(&elements_array);
    let obj = heap.get_array(&object);
    let class_info = heap.get_array(&obj.get_element(0).as_obj_ref().unwrap());
    let method_info = heap.get_array(&obj.get_element(1).as_obj_ref().unwrap());
    let cp_info = heap.get_array(&obj.get_element(2).as_obj_ref().unwrap());

    for i in 0..depth {
        let class_id = class_info.get_element(i).as_int().unwrap() as usize;
        let method_id = method_info.get_element(i).as_int().unwrap() as usize;
        let class = vm.method_area().get_class_by_id(class_id).unwrap();
        let method = class.get_method_by_id(&method_id).unwrap();
        print!("")
    }
    None
}

fn vm_internal_clone(vm: &mut VirtualMachine, args: &[Value]) -> NativeRet {
    debug!("TODO: Stub: internal clone");
    let obj = match &args[0] {
        Value::Ref(h) => *h,
        _ => panic!("internal clone: expected object"),
    };
    let mut borrowed_heap = vm.heap.borrow_mut();
    let cloned = borrowed_heap.clone_object(obj);
    Some(Value::Ref(cloned))
}

fn jdk_internal_misc_scoped_memory_access_register_natives(
    _vm: &mut VirtualMachine,
    _args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: jdk.internal.misc.ScopedMemoryAccess.registerNatives");
    None
}

fn jdk_internal_misc_signal_find_signal_0(vm: &mut VirtualMachine, args: &[Value]) -> NativeRet {
    debug!("TODO: Stub: jdk.internal.misc.Signal.findSignal0");
    let signal_name = match &args[0] {
        Value::Ref(h) => {
            let heap = vm.heap.borrow();
            let s = heap.get_string(*h).unwrap();
            s.to_string()
        }
        _ => panic!("jdk.internal.misc.Signal.findSignal0: expected signal name string"),
    };
    let signal_number = match signal_name.as_str() {
        "HUP" => 1,
        "INT" => 2,
        "QUIT" => 3,
        "ILL" => 4,
        "ABRT" => 6,
        "FPE" => 8,
        "KILL" => 9,
        "SEGV" => 11,
        "PIPE" => 13,
        "ALRM" => 14,
        "TERM" => 15,
        "USR1" => 10,
        "USR2" => 12,
        "CHLD" => 17,
        "CONT" => 18,
        "STOP" => 19,
        "TSTP" => 20,
        "TTIN" => 21,
        "TTOU" => 22,
        _ => -1,
    };
    Some(Value::Integer(signal_number))
}

fn jdk_internal_misc_signal_handle_0(_vm: &mut VirtualMachine, _args: &[Value]) -> NativeRet {
    debug!("TODO: Stub: jdk.internal.misc.Signal.handle0");
    Some(Value::Long(1))
}

fn jdk_internal_misc_cds_get_cds_config_status(
    _vm: &mut VirtualMachine,
    _args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: jdk.internal.misc.CDS.getCDSConfigStatus");
    Some(Value::Integer(0))
}

fn jdk_internal_misc_cds_initialize_from_archive(
    _vm: &mut VirtualMachine,
    _args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: jdk.internal.misc.CDS.initializeFromArchive");
    None
}

fn java_lang_object_notify_all(_vm: &mut VirtualMachine, _args: &[Value]) -> NativeRet {
    debug!("TODO: Stub: java.lang.Object.notifyAll");
    None
}

fn java_io_file_output_stream_write_bytes(vm: &mut VirtualMachine, args: &[Value]) -> NativeRet {
    debug!("TODO: Partial implementation: java.io.FileOutputStream.writeBytes");
    let fd_obj = match &args[0] {
        Value::Ref(h) => *h,
        _ => panic!("java.io.FileOutputStream.writeBytes: expected FileDescriptor object"),
    };
    let bytes_array = match &args[1] {
        Value::Ref(h) => *h,
        _ => panic!("java.io.FileOutputStream.writeBytes: expected byte array"),
    };
    let offset = match args[2] {
        Value::Integer(i) if i >= 0 => i as usize,
        _ => panic!("java.io.FileOutputStream.writeBytes: expected non-negative offset"),
    };
    let length = match args[3] {
        Value::Integer(i) if i >= 0 => i as usize,
        _ => panic!("java.io.FileOutputStream.writeBytes: expected non-negative length"),
    };

    let heap = vm.heap.borrow();
    let array = heap.get_array(&bytes_array);
    for i in offset..offset + length {
        let byte = match array.get_element(i) {
            Value::Integer(b) => b,
            _ => panic!("java.io.FileOutputStream.writeBytes: expected byte element"),
        };
        print!("{}", *byte as u8 as char);
    }
    None
}
