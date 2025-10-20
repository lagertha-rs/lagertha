use crate::rt::constant_pool::error::RuntimePoolError;
use common::InstructionErr;
use common::utils::cursor::CursorError;
use jclass::error::ClassFormatErr;
use thiserror::Error;

pub mod class;
pub mod constant_pool;
pub mod field;
pub mod method;

#[derive(Debug, Error)]
pub enum LinkageError {
    #[error(transparent)]
    Instruction(#[from] InstructionErr),
    #[error("Unsupported opcode `{0:#04X}`")]
    UnsupportedOpCode(u8),
    #[error("")]
    DuplicatedCodeAttr,
    //TODO: confused 4.7.13. The LocalVariableTable Attribute
    //#[error("")]
    //DuplicatedLocalVariableTableAttr,
    #[error("DuplicatedSignatureAttr")]
    DuplicatedSignatureAttr,
    #[error("DuplicatedStackMapTable")]
    DuplicatedStackMapTable,
    #[error("DuplicatedExceptionAttribute")]
    DuplicatedExceptionAttribute,
    #[error("DuplicatedRuntimeVisibleAnnotationsAttr")]
    DuplicatedRuntimeVisibleAnnotationsAttr,
    #[error("DuplicatedRuntimeInvisibleAnnotationsAttr")]
    DuplicatedRuntimeInvisibleAnnotationsAttr,
    #[error("CodeAttrIsAmbiguousForNative")]
    CodeAttrIsAmbiguousForNative,
    #[error(transparent)]
    RuntimeConstantPool(#[from] RuntimePoolError),
    #[error(transparent)]
    Cursor(#[from] CursorError),
    #[error("java.lang.ClassFormatError: {0}")]
    ClassFile(#[from] ClassFormatErr),
    #[error("DuplicatedClassInMethod")]
    DuplicatedClassInMethod,
    #[error("MethodClassIsNotSet")]
    MethodClassIsNotSet,
}
