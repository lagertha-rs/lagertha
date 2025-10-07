use crate::VirtualMachine;
use crate::heap::HeapObject;
use common::instruction::ArrayType;
use common::jtype::Value;
use std::collections::HashMap;
use tracing_log::log::debug;

//TODO: avoid string allocations here
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

pub type NativeRet = Option<Value>;
pub type NativeFn = fn(&mut VirtualMachine, &[Value]) -> NativeRet;

pub struct NativeRegistry {
    map: HashMap<MethodKey, NativeFn>,
}

impl NativeRegistry {
    pub fn new() -> Self {
        debug!("Initializing NativeRegistry...");
        let mut instance = Self {
            map: HashMap::new(),
        };

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

        instance
    }

    pub fn register(&mut self, key: MethodKey, f: NativeFn) {
        debug!("Registering native method: {:?}", key);
        self.map.insert(key, f);
    }

    pub fn get(&self, key: &MethodKey) -> Option<&NativeFn> {
        self.map.get(key)
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
        let v = vm.method_area().get_primitive_mirror_addr(h);
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
        let res = vm
            .method_area
            .get_mirror_addr_by_class(&target_class)
            .unwrap();
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

fn java_lang_throwable_fill_in_stack_trace(vm: &mut VirtualMachine, args: &[Value]) -> NativeRet {
    debug!("TODO: Stub: java.lang.Throwable.fillInStackTrace");
    let frames = vm.frame_stack.frames();
    let int_class = vm
        .method_area
        .get_class(ArrayType::Int.descriptor())
        .unwrap();
    let class_idx = vm
        .heap
        .borrow_mut()
        .alloc_array(int_class.clone(), frames.len());
    let name_idx = vm
        .heap
        .borrow_mut()
        .alloc_array(int_class.clone(), frames.len());
    let descriptor_idx = vm.heap.borrow_mut().alloc_array(int_class, frames.len());
    for (pos, frame) in frames.iter().enumerate() {
        let mut heap = vm.heap.borrow_mut();
        heap.write_array_element(
            class_idx,
            pos,
            Value::Integer(frame.method().class_id().unwrap() as i32),
        )
        .unwrap();
        heap.write_array_element(
            name_idx,
            pos,
            Value::Integer(frame.method().name_idx() as i32),
        )
        .unwrap();
        heap.write_array_element(
            descriptor_idx,
            pos,
            Value::Integer(frame.method().descriptor().idx() as i32),
        )
        .unwrap();
    }
    let obj_class = vm.method_area.get_class("java/lang/Object").unwrap();
    let backtrace_addr = vm.heap.borrow_mut().alloc_array(obj_class, 3);
    vm.heap
        .borrow_mut()
        .write_array_element(backtrace_addr, 0, Value::Ref(class_idx))
        .unwrap();
    vm.heap
        .borrow_mut()
        .write_array_element(backtrace_addr, 1, Value::Ref(name_idx))
        .unwrap();
    vm.heap
        .borrow_mut()
        .write_array_element(backtrace_addr, 2, Value::Ref(descriptor_idx))
        .unwrap();
    let throwable_addr = match args[0] {
        Value::Ref(h) => h,
        _ => panic!("java.lang.Throwable.fillInStackTrace: expected object"),
    };
    vm.heap
        .borrow_mut()
        .write_instance_field(
            throwable_addr,
            "backtrace",
            "Ljava/lang/Object;",
            Value::Ref(backtrace_addr),
        )
        .unwrap();
    vm.heap
        .borrow_mut()
        .write_instance_field(
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
