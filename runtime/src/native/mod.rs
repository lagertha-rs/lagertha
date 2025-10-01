use crate::VirtualMachine;
use crate::error::JvmError;
use crate::heap::{Heap, HeapObject};
use crate::method_area::MethodArea;
use common::jtype::{HeapAddr, Value};
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
        let src_array_len;
        let slice: &[Value] = match heap.get(src) {
            Some(HeapObject::PrimitiveArray { elements, .. }) => {
                src_array_len = elements.len();
                elements
            }
            Some(HeapObject::RefArray { elements, .. }) => {
                src_array_len = elements.len();
                elements
            }
            _ => panic!("java.lang.System.arraycopy: source is not an array"),
        };
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
        let dest_array_len;
        let dest_slice: &mut [Value] = match heap.get_mut(dest) {
            HeapObject::PrimitiveArray { elements, .. } => {
                dest_array_len = elements.len();
                elements
            }
            HeapObject::RefArray { elements, .. } => {
                dest_array_len = elements.len();
                elements
            }
            _ => panic!("java.lang.System.arraycopy: destination is not an array"),
        };
        if dest_pos
            .checked_add(length)
            .map_or(true, |end| end > dest_array_len)
        {
            panic!("java.lang.System.arraycopy: destination index out of bounds");
        }

        for i in 0..length {
            dest_slice[dest_pos + i] = tmp[i].clone();
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
                HeapObject::Instance(instance) => instance.class().name().unwrap().to_string(),
                HeapObject::PrimitiveArray { class, .. } => class.descriptor().to_string(),
                HeapObject::RefArray { .. } => {
                    panic!("java.lang.Class.getClass: array types not supported yet")
                }
            }
        } else {
            panic!("java.lang.Class.getClass: invalid heap address");
        };
        let res = vm
            .method_area
            .get_mirror_addr_by_name(&target_class)
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
            .unwrap()
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
    let h = vm.heap().borrow_mut().alloc_ref_array(string_class, 0);
    Value::Array(Some(h))
}

fn jdk_internal_misc_unsafe_register_natives(_vm: &mut VirtualMachine, _args: &[Value]) -> Value {
    debug!("TODO: Stub: jdk.internal.misc.Unsafe.registerNatives");
    Value::Object(None)
}

fn java_lang_throwable_fill_in_stack_trace(vm: &mut VirtualMachine, args: &[Value]) -> Value {
    debug!("TODO: Stub: java.lang.Throwable.fillInStackTrace");
    let frames = vm.frame_stack.frames();
    let class_idx = vm
        .heap
        .borrow_mut()
        .alloc_primitive_array(frames.len(), common::instruction::ArrayType::Int);
    let name_idx = vm
        .heap
        .borrow_mut()
        .alloc_primitive_array(frames.len(), common::instruction::ArrayType::Int);
    let descriptor_idx = vm
        .heap
        .borrow_mut()
        .alloc_primitive_array(frames.len(), common::instruction::ArrayType::Int);
    for frame in frames {
        let mut heap = vm.heap.borrow_mut();
        {
            let class_idx_ref = heap.get_mut(class_idx);
            if let HeapObject::PrimitiveArray { elements, .. } = class_idx_ref {
                elements.push(Value::Integer(frame.method().class_id().unwrap() as i32));
            }
        }
        {
            let name_idx_ref = heap.get_mut(name_idx);
            if let HeapObject::PrimitiveArray { elements, .. } = name_idx_ref {
                elements.push(Value::Integer(frame.method().name_idx() as i32));
            }
        }
        {
            let descriptor_idx_ref = heap.get_mut(descriptor_idx);
            if let HeapObject::PrimitiveArray { elements, .. } = descriptor_idx_ref {
                elements.push(Value::Integer(frame.method().descriptor().idx() as i32));
            }
        }
    }
    let obj_class = vm.method_area.get_class("java/lang/Object").unwrap();
    let backtrace_addr = vm.heap.borrow_mut().alloc_ref_array(obj_class, 3);
    if let HeapObject::RefArray { elements, .. } = vm.heap.borrow_mut().get_mut(backtrace_addr) {
        elements.push(Value::Array(Some(class_idx)));
        elements.push(Value::Array(Some(name_idx)));
        elements.push(Value::Array(Some(descriptor_idx)));
    }
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
