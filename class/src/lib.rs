use crate::class_file::ClassFile;
use crate::rt::class::class::Class;
use crate::rt::constant_pool::error::RuntimePoolError;
use class_file::ClassFileErr;
use common::CursorError;
use std::fmt;
use thiserror::Error;

mod class_file;
pub mod rt;

#[derive(Debug, Error)]
pub enum JvmError {
    #[error(transparent)]
    ClassFile(#[from] ClassFileErr),
    #[error(transparent)]
    Cursor(#[from] CursorError),
    #[error(transparent)]
    RuntimePool(#[from] RuntimePoolError),
    #[error("")]
    MissingAttributeInConstantPoll,
    #[error("")]
    ConstantNotFoundInRuntimePool,
    #[error("")]
    TrailingBytes,
    #[error("")]
    TypeError,
}

impl Into<fmt::Error> for JvmError {
    fn into(self) -> fmt::Error {
        fmt::Error
    }
}

// TODO: returns only class right now, in future not sure
pub fn create_runtime(main_class: Vec<u8>) -> Result<Class, JvmError> {
    let class_file = ClassFile::try_from(main_class)?;
    Class::new(class_file)
}
