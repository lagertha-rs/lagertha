use crate::class_loader::ClassLoaderErr;
use crate::method_area::MethodArea;
use crate::rt::class::LinkageError;
use crate::rt::constant_pool::error::RuntimePoolError;
use common::utils::cursor::CursorError;
use once_cell::sync::OnceCell;
use thiserror::Error;
use tracing_log::log::debug;

mod class_loader;
mod heap;
mod method_area;
mod native_registry;
pub mod rt;
mod stack;
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
pub static VM_CONFIGURATION: OnceCell<VmConfig> = OnceCell::new();

// TODO: right now static and global, probably should be dealt differently later
pub static METHOD_AREA: OnceCell<MethodArea> = OnceCell::new();

pub fn start(main_class: Vec<u8>, config: VmConfig) -> Result<(), JvmError> {
    debug!("Starting VM with config: {:?}", config);
    config.validate();
    VM_CONFIGURATION.set(config).expect("TODO: panic message");
    METHOD_AREA
        .set(MethodArea::try_with_main(main_class)?)
        .expect("TODO: panic message");
    debug!("VM started successfully");

    let main = METHOD_AREA.get().unwrap().get_main();
    Ok(())
}
