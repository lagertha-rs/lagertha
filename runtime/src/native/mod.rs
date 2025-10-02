use crate::VirtualMachine;
use crate::error::JvmError;
use crate::heap::HeapObject;
use common::instruction::ArrayType;
use common::jtype::Value;
use std::collections::HashMap;
use std::ffi::c_char;
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

pub type NativeFn = fn(&mut VirtualMachine, &[Value]) -> Value;

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
            |vm, args| {
                java_lang_system_arraycopy(vm, args).unwrap();
                Value::Object(None)
            },
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

fn java_lang_system_register_natives(_vm: &mut VirtualMachine, _args: &[Value]) -> Value {
    debug!("TODO: Stub: Registering java.lang.System native methods");
    Value::Object(None)
}

fn java_lang_system_arraycopy(vm: &mut VirtualMachine, args: &[Value]) -> Result<(), JvmError> {
    debug!("TODO: Stub: java.lang.System.arraycopy");
    let src = match &args[0] {
        Value::Array(Some(h)) => *h,
        _ => panic!("java.lang.System.arraycopy: expected source array object"),
    };
    let src_pos = match args[1] {
        Value::Integer(i) if i >= 0 => i as usize,
        _ => panic!("java.lang.System.arraycopy: expected non-negative source position"),
    };
    let dest = match &args[2] {
        Value::Array(Some(h)) => *h,
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
            heap.write_array_element(dest, dest_pos + i, tmp[i].clone())?;
        }
    }
    Ok(())
}

fn java_lang_class_register_natives(_vm: &mut VirtualMachine, _args: &[Value]) -> Value {
    debug!("TODO: Stub: Registering java.lang.Class native methods");
    Value::Object(None)
}

fn java_lang_class_desired_assertion_status_0(_vm: &mut VirtualMachine, _args: &[Value]) -> Value {
    debug!("TODO: Stub: java.lang.Class.desiredAssertionStatus0");
    Value::Integer(1)
}

fn java_lang_class_is_primitive(vm: &mut VirtualMachine, args: &[Value]) -> Value {
    debug!("TODO: Stub: java.lang.Class.isPrimitive");
    if let Value::Object(Some(h)) = &args[0] {
        let is_primitive = vm.method_area().addr_is_primitive(h);
        Value::Integer(if is_primitive { 1 } else { 0 })
    } else {
        panic!("java.lang.Class.isPrimitive: expected object");
    }
}

fn java_lang_class_get_primitive_class(vm: &mut VirtualMachine, args: &[Value]) -> Value {
    debug!("TODO: Stub: java.lang.Class.getPrimitiveClass");
    if let Value::Object(Some(h)) = &args[0] {
        let v = vm.method_area().get_primitive_mirror_addr(h);
        Value::Object(Some(v))
    } else {
        panic!("java.lang.Class.getPrimitiveClass: expected object");
    }
}

fn java_lang_object_get_class(vm: &mut VirtualMachine, args: &[Value]) -> Value {
    debug!("TODO: Stub: java.lang.Class.getClass");
    if let Value::Object(Some(h)) | Value::Array(Some(h)) = &args[0] {
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
        Value::Object(Some(res))
    } else {
        panic!("java.lang.Class.getClass: expected object as argument");
    }
}

fn java_lang_object_init_class_name(vm: &mut VirtualMachine, args: &[Value]) -> Value {
    debug!("TODO: Stub: java.lang.Class.initClassName");
    if let Value::Object(Some(h)) = &args[0] {
        let class_name = vm
            .method_area()
            .get_class_by_mirror(h)
            .unwrap()
            .name()
            .replace('/', ".");
        let val = Value::Object(Some(vm.heap().borrow_mut().get_or_new_string(&class_name)));
        vm.heap()
            .borrow_mut()
            .write_instance_field(*h, "name", "Ljava/lang/String;", val)
            .unwrap();
        val
    } else {
        panic!("java.lang.Class.initClassName: expected object as argument");
    }
}

fn jdk_internal_util_system_props_raw_platform_properties(
    _vm: &mut VirtualMachine,
    _args: &[Value],
) -> Value {
    debug!("TODO: Stub: jdk.internal.util.SystemProps$Raw.platformProperties");
    Value::Array(None)
}

fn jdk_internal_util_system_props_raw_vm_properties(
    vm: &mut VirtualMachine,
    _args: &[Value],
) -> Value {
    debug!("TODO: Stub: jdk.internal.util.SystemProps$Raw.vmProperties");
    let string_class = vm.method_area().get_class("java/lang/String").unwrap();
    let h = vm.heap().borrow_mut().alloc_array(string_class, 0);
    Value::Array(Some(h))
}

fn jdk_internal_misc_unsafe_register_natives(_vm: &mut VirtualMachine, _args: &[Value]) -> Value {
    debug!("TODO: Stub: jdk.internal.misc.Unsafe.registerNatives");
    Value::Object(None)
}

fn java_lang_throwable_fill_in_stack_trace(vm: &mut VirtualMachine, args: &[Value]) -> Value {
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
        .write_array_element(backtrace_addr, 0, Value::Array(Some(class_idx)))
        .unwrap();
    vm.heap
        .borrow_mut()
        .write_array_element(backtrace_addr, 1, Value::Array(Some(name_idx)))
        .unwrap();
    vm.heap
        .borrow_mut()
        .write_array_element(backtrace_addr, 2, Value::Array(Some(descriptor_idx)))
        .unwrap();
    let throwable_addr = match args[0] {
        Value::Object(Some(h)) => h,
        _ => panic!("java.lang.Throwable.fillInStackTrace: expected object"),
    };
    vm.heap
        .borrow_mut()
        .write_instance_field(
            throwable_addr,
            "backtrace",
            "Ljava/lang/Object;",
            Value::Array(Some(backtrace_addr)),
        )
        .unwrap();

    Value::Object(Some(throwable_addr))
}
