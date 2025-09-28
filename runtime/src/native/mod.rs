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
        if let Some(v) = vm.method_area.get_primitive_mirror(h) {
            Value::Object(Some(v))
        } else {
            panic!("java.lang.Class.getPrimitiveClass: expected to be a primitive type name");
        }
    } else {
        panic!("java.lang.Class.getPrimitiveClass: expected String object");
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
