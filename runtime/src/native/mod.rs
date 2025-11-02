mod preregistered;
mod registrable;

use crate::native::preregistered::preregister_natives;
use crate::native::registrable::add_registrable_natives;
use crate::stack_deprecated::FrameStackDeprecated;
use crate::{FullyQualifiedMethodKey, ThreadId, VirtualMachine};
use common::error::JvmError;
use common::jtype::Value;
use lasso::ThreadedRodeo;
use std::collections::HashMap;
use std::sync::Arc;
use tracing_log::log::debug;

pub type NativeRet = Result<Option<Value>, JvmError>;
pub type NativeFn = fn(&mut VirtualMachine, thread_id: ThreadId, &[Value]) -> NativeRet;

pub struct NativeRegistry {
    map: HashMap<FullyQualifiedMethodKey, NativeFn>,
    string_interner: Arc<ThreadedRodeo>,
}

impl NativeRegistry {
    pub fn new(string_interner: Arc<ThreadedRodeo>) -> Self {
        debug!("Initializing NativeRegistry...");
        let mut instance = Self {
            map: HashMap::new(),
            string_interner,
        };

        preregister_natives(&mut instance);
        add_registrable_natives(&mut instance);

        instance
    }

    fn register(&mut self, key: FullyQualifiedMethodKey, f: NativeFn) {
        self.map.insert(key, f);
    }

    pub fn get(&self, key: &FullyQualifiedMethodKey) -> Option<&NativeFn> {
        self.map.get(key)
    }
}
