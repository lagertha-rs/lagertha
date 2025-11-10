mod preregistered;
mod registrable;

use crate::native_deprecated::preregistered::preregister_natives;
use crate::native_deprecated::registrable::add_registrable_natives;
use crate::rt::BaseClass;
use crate::rt::class::InstanceClass;
use crate::stack_deprecated::FrameStackDeprecated;
use crate::{FullyQualifiedMethodKey, VirtualMachineDeprecated};
use common::error::JvmError;
use common::jtype::Value;
use jclass::flags::ClassFlags;
use lasso::ThreadedRodeo;
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::sync::Arc;
use tracing_log::log::debug;

pub type NativeRetDeprecated = Result<Option<Value>, JvmError>;
pub type NativeFnDeprecated = fn(
    &mut VirtualMachineDeprecated,
    frame_stack: &FrameStackDeprecated,
    &[Value],
) -> NativeRetDeprecated;

pub struct NativeRegistryDeprecated {
    map: HashMap<FullyQualifiedMethodKey, NativeFnDeprecated>,
    string_interner: Arc<ThreadedRodeo>,
}

impl NativeRegistryDeprecated {
    pub fn new(string_interner: Arc<ThreadedRodeo>) -> Self {
        debug!("Initializing NativeRegistryDeprecated...");
        let mut instance = Self {
            map: HashMap::new(),
            string_interner,
        };

        preregister_natives(&mut instance);
        add_registrable_natives(&mut instance);

        instance
    }

    fn register(&mut self, key: FullyQualifiedMethodKey, f: NativeFnDeprecated) {
        self.map.insert(key, f);
    }

    pub fn get(&self, key: &FullyQualifiedMethodKey) -> Option<&NativeFnDeprecated> {
        self.map.get(key)
    }
}
