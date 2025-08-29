use crate::attribute::AttributeType;
use crate::constant::ConstantTag;
use common::utils::cursor::CursorError;
#[cfg(test)]
use serde::Serialize;
use thiserror::Error;

#[cfg_attr(test, derive(Serialize))]
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ClassFileErr {
    #[error(transparent)]
    Cursor(#[from] CursorError),
    #[error("")]
    WrongMagic,
    #[error("Expected end of file but found trailing bytes.")]
    TrailingBytes,
    #[error("")]
    UnknownTag(u8),
    #[error("Expected type `{1}` with index `{0}` but found `{2}`")]
    /// First u16 is index, second is expected type, third is actual type
    TypeError(u16, ConstantTag, ConstantTag),
    #[error("Constant with index `{0}` isn't found in constant constant.")]
    ConstantNotFound(u16),
    #[error("Unknown stack frame type {0}.")]
    UnknownStackFrameType(u8),
    #[error("Unknown attribute `{0}.")]
    UnknownAttribute(String),
    #[error("Can't build shared attribute, the `{0}` attribute isn't shared.")]
    AttributeIsNotShared(AttributeType),
    #[error("Invalid method handle kind {0}.")]
    InvalidMethodHandleKind(u8),
}
