extern crate core;

use crate::utils::cursor::CursorError;
use thiserror::Error;

pub mod access;
pub mod descriptor;
pub mod instruction;
pub mod jtype;
pub mod utils;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum DescriptorErr {
    #[error("")]
    ShouldStartWithParentheses,
    #[error("")]
    MissingClosingParenthesis,
    #[error("")]
    UnexpectedEnd,
    #[error("")]
    InvalidType,
    #[error("")]
    TrailingCharacters,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum InstructionErr {
    #[error("Unsupported opcode `{0:#04X}`")]
    UnsupportedOpCode(u8),
    #[error(transparent)]
    Cursor(#[from] CursorError),
}
