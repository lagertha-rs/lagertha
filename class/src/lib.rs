use crate::class_file::ClassFile;
use crate::rt::class::Class;
use class_file::ClassFileErr;
use common::CursorError;
use std::fmt;
use std::fmt::Error;
use thiserror::Error;

mod class_file;
pub mod rt;

#[derive(Debug, Error)]
pub enum JvmError {
    #[error(transparent)]
    ClassFile(#[from] ClassFileErr),
    #[error(transparent)]
    Cursor(#[from] CursorError),
    #[error("")]
    MissingAttributeInConstantPoll,
    #[error("")]
    ConstantNotFoundInRuntimePool,
    #[error("")]
    TrailingBytes,
    #[error("")]
    TypeError,
    #[error("")]
    TryingAccessUninitializedRuntimeConstant(&'static str, u16), //TODO: not str, but rather enum?
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
