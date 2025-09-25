use crate::MethodKey;
use crate::heap::Heap;
use crate::method_area::MethodArea;
use common::jtype::{HeapAddr, Value};
use std::collections::HashMap;
use tracing_log::log::debug;

//TODO: right now JNIEnv owns MethodArea and Heap for simplicity, but it should be references instead
#[cfg_attr(test, derive(serde::Serialize))]
pub struct JNIEnv {
    #[cfg_attr(test, serde(skip_serializing))]
    pub native_registry: NativeRegistry,
    #[cfg_attr(test, serde(skip_serializing))]
    method_area: MethodArea,
    heap: Heap,
}

impl JNIEnv {
    pub fn new(heap: Heap, method_area: MethodArea) -> Self {
        debug!("Creating new JNIEnv");
        Self {
            heap,
            method_area,
            native_registry: NativeRegistry::new(),
        }
    }

    pub fn get_mirror(&mut self, class_name: &str) -> Result<HeapAddr, crate::JvmError> {
        self.method_area.get_mirror(class_name, &mut self.heap)
    }

    pub fn heap(&mut self) -> &mut Heap {
        &mut self.heap
    }

    pub fn method_area(&self) -> &MethodArea {
        &self.method_area
    }
}

pub type NativeFn = fn(&mut JNIEnv, &[Value]) -> Value;

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

fn java_lang_system_register_natives(_env: &mut JNIEnv, _args: &[Value]) -> Value {
    debug!("TODO: Stub: Registering java.lang.System native methods");
    Value::Object(None)
}

fn java_lang_class_register_natives(_env: &mut JNIEnv, _args: &[Value]) -> Value {
    debug!("TODO: Stub: Registering java.lang.Class native methods");
    Value::Object(None)
}

fn java_lang_class_desired_assertion_status_0(_env: &mut JNIEnv, _args: &[Value]) -> Value {
    debug!("TODO: Stub: java.lang.Class.desiredAssertionStatus0");
    Value::Integer(1)
}

fn jdk_internal_util_system_props_raw_platform_properties(
    _env: &mut JNIEnv,
    _args: &[Value],
) -> Value {
    debug!("TODO: Stub: jdk.internal.util.SystemProps$Raw.platformProperties");
    Value::Array(None)
}

fn jdk_internal_util_system_props_raw_vm_properties(env: &mut JNIEnv, _args: &[Value]) -> Value {
    debug!("TODO: Stub: jdk.internal.util.SystemProps$Raw.vmProperties");
    let string_class = env.method_area().get_class("java/lang/String").unwrap();
    let h = env.heap().alloc_array_ref(string_class, 0);
    Value::Array(Some(h))
}
