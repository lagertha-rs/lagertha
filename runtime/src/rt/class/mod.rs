use crate::rt::constant_pool::error::RuntimePoolError;
use common::cursor::CursorError;
use thiserror::Error;

pub mod class;
pub mod descriptor;
pub mod field;
pub mod instruction_set;
pub mod jtype;
pub mod method;

#[derive(Debug, Error)]
pub enum LoadingError {
    #[error("Unsupported opcode `{0:#04X}`")]
    UnsupportedOpCode(u8),
    #[error("")]
    DuplicatedCodeAttr,
    //TODO: confused 4.7.13. The LocalVariableTable Attribute
    //#[error("")]
    //DuplicatedLocalVariableTableAttr,
    #[error("")]
    DuplicatedSignatureAttr,
    #[error("")]
    DuplicatedStackMapTable,
    #[error("")]
    DuplicatedExceptionAttribute,
    #[error("")]
    DuplicatedRuntimeVisibleAnnotationsAttr,
    // TODO: only for non native?
    #[error("")]
    MissingCodeAttr,
    #[error("")]
    CodeAttrIsAmbiguousForNative,
    #[error(transparent)]
    RuntimeConstantPool(#[from] RuntimePoolError),
    #[error(transparent)]
    Cursor(#[from] CursorError),
}
