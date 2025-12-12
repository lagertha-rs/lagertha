mod preregistered;
mod registrable;

use crate::VirtualMachine;
use crate::error::JvmError;
use crate::keys::FullyQualifiedMethodKey;
use crate::native::preregistered::preregister_natives;
use crate::native::registrable::add_registrable_natives;
use crate::thread::JavaThreadState;
use crate::vm::Value;
use dashmap::DashMap;
use lasso::ThreadedRodeo;
use std::collections::HashMap;
use std::sync::Arc;
use tracing_log::log::debug;

pub type NativeRet = Result<Option<Value>, JvmError>;
pub type NativeFn = fn(&VirtualMachine, thread: &mut JavaThreadState, &[Value]) -> NativeRet;

pub struct NativeRegistry {
    map: DashMap<FullyQualifiedMethodKey, NativeFn>,
    string_interner: Arc<ThreadedRodeo>,
}

impl NativeRegistry {
    pub fn new(string_interner: Arc<ThreadedRodeo>) -> Self {
        debug!("Initializing NativeRegistry...");
        let mut instance = Self {
            map: DashMap::new(),
            string_interner,
        };

        preregister_natives(&mut instance);
        add_registrable_natives(&mut instance);

        instance
    }

    fn register(&self, key: FullyQualifiedMethodKey, f: NativeFn) {
        self.map.insert(key, f);
    }

    pub fn get(&self, key: &FullyQualifiedMethodKey) -> Option<NativeFn> {
        self.map.get(key).map(|entry| *entry.value())
    }
}
