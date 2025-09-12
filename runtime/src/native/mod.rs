use crate::{MethodKey, Value};
use std::collections::HashMap;
use tracing_log::log::debug;

pub struct JNIEnv {
    native_registry: NativeRegistry,
}

pub type NativeFn = fn(&mut JNIEnv, &[Value]) -> Option<Value>;

pub struct NativeRegistry {
    map: HashMap<MethodKey, NativeFn>,
}

impl NativeRegistry {
    pub fn new() -> Self {
        let mut instance = Self {
            map: HashMap::new(),
        };

        instance.register(
            MethodKey::new("java/lang/System", "registerNatives", "()V"),
            register_system_methods,
        );
        instance
    }

    pub fn register(&mut self, key: MethodKey, f: NativeFn) {
        self.map.insert(key, f);
    }

    pub fn get(&self, key: &MethodKey) -> Option<&NativeFn> {
        self.map.get(key)
    }
}

fn register_system_methods(_env: &mut JNIEnv, _args: &[Value]) -> Option<Value> {
    debug!("Stub: Registering java.lang.System native methods");
    None
}
