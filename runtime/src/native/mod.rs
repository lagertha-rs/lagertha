use crate::MethodKey;
use common::jtype::{ObjectRef, Value};
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
            register_system_methods,
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

fn register_system_methods(_env: &mut JNIEnv, _args: &[Value]) -> Value {
    debug!("Stub: Registering java.lang.System native methods");
    Value::Object(ObjectRef::Null)
}
