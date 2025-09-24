use crate::MethodKey;
use common::jtype::Value;
use std::collections::HashMap;
use tracing_log::log::debug;

pub struct JNIEnv {
    pub native_registry: NativeRegistry,
}

impl JNIEnv {
    pub fn new() -> Self {
        debug!("Creating new JNIEnv");
        Self {
            native_registry: NativeRegistry::new(),
        }
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

fn jdk_internal_util_system_props_raw_vm_properties(_env: &mut JNIEnv, _args: &[Value]) -> Value {
    debug!("TODO: Stub: jdk.internal.util.SystemProps$Raw.vmProperties");
    Value::Array(None)
}
