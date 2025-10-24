mod preregistered;
mod registrable;

use crate::VirtualMachine;
use crate::native::preregistered::preregister_natives;
use crate::native::registrable::add_registrable_natives;
use common::jtype::Value;
use lasso::{Spur, ThreadedRodeo};
use std::collections::HashMap;
use std::sync::Arc;
use tracing_log::log::debug;

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct MethodKey {
    pub class: Option<Spur>,
    pub name: Spur,
    pub desc: Spur,
}

impl MethodKey {
    pub fn new(class: Spur, name: Spur, desc: Spur) -> Self {
        Self {
            class: Some(class),
            name,
            desc,
        }
    }

    pub fn new_internal(name: Spur, desc: Spur) -> Self {
        Self {
            class: None,
            name,
            desc,
        }
    }

    pub fn new_internal_with_str(name: &str, desc: &str, interner: &ThreadedRodeo) -> Self {
        Self {
            class: None,
            name: interner.get_or_intern(name),
            desc: interner.get_or_intern(desc),
        }
    }

    pub fn new_with_str(class: &str, name: &str, desc: &str, interner: &ThreadedRodeo) -> Self {
        Self {
            class: Some(interner.get_or_intern(class)),
            name: interner.get_or_intern(name),
            desc: interner.get_or_intern(desc),
        }
    }
}

pub type NativeRet = Option<Value>;
pub type NativeFn = fn(&mut VirtualMachine, &[Value]) -> NativeRet;

pub struct NativeRegistry {
    map: HashMap<MethodKey, NativeFn>,
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

    fn register(&mut self, key: MethodKey, f: NativeFn) {
        self.map.insert(key, f);
    }

    pub fn get(&self, key: &MethodKey) -> Option<&NativeFn> {
        self.map.get(key)
    }
}
