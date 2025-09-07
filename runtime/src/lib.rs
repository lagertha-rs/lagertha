use crate::class_loader::ClassLoaderErr;
use crate::method_area::MethodArea;
use crate::rt::class::LinkageError;
use crate::rt::constant_pool::error::RuntimePoolError;
use crate::stack::ThreadStack;
use common::utils::cursor::CursorError;
use once_cell::sync::OnceCell;
use std::sync::Arc;
use thiserror::Error;
use tracing_log::log::debug;

mod class_loader;
mod executor;
mod heap;
mod method_area;
mod native_registry;
pub mod rt;
pub mod stack;
mod string_pool;

type HeapAddr = usize;

#[derive(Clone, Debug)]
pub enum Value {
    Null,
    Ref(HeapAddr),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct ClassId(pub usize);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct MethodId(pub usize);

#[derive(Debug, Eq, PartialEq)]
pub enum ClassState {
    Loaded,
    Initialized,
}

#[derive(Debug, Error)]
pub enum JvmError {
    #[error("LinkageError: {0}")]
    Linkage(#[from] LinkageError),
    #[error(transparent)]
    Cursor(#[from] CursorError),
    #[error(transparent)]
    RuntimePool(#[from] RuntimePoolError),
    #[error(transparent)]
    ClassLoader(#[from] ClassLoaderErr),
    #[error("MissingAttributeInConstantPoll")]
    MissingAttributeInConstantPoll,
    #[error("ConstantNotFoundInRuntimePool")]
    ConstantNotFoundInRuntimePool,
    #[error("TrailingBytes")]
    TrailingBytes,
    #[error("TypeError")]
    TypeError,
    #[error("ClassNotFoundException: {0}")]
    ClassNotFound(String),
    #[error("stack overflow")]
    StackOverflow,
    #[error("Stack is empty")]
    StackIsEmpty,
    #[error("OutOfMemory")]
    OutOfMemory,
    #[error("Could not find or load main class {0}")]
    NoMainClassFound(String),
}

#[derive(Debug)]
pub struct VmConfig {
    pub home: String,
    pub version: String,
    pub initial_heap_size: usize,
    pub max_heap_size: usize,
    pub stack_size_per_thread: usize,
}

//TODO: make it better
impl VmConfig {
    pub fn validate(&self) {
        if self.version != "24.0.2" {
            panic!(
                "Unsupported Java version: {}. Only 24.0.2 is supported.",
                self.version
            );
        }
    }
}

pub fn start(main_class: Vec<u8>, config: VmConfig) -> Result<(), JvmError> {
    debug!("Starting VM with config: {:?}", config);
    config.validate();

    let vm_config = Arc::new(config);
    let method_area = Arc::new(MethodArea::new(vm_config.clone())?);

    let executor = executor::Executor::new(&vm_config, method_area);
    executor.start(main_class)
}
