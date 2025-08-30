extern crate core;

use crate::utils::cursor::CursorError;
use thiserror::Error;

pub mod access;
pub mod descriptor;
pub mod instruction;
pub mod jtype;
pub mod utils;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum TypeDescriptorErr {
    #[error("Unexpected end of descriptor")]
    UnexpectedEnd,
    #[error("Invalid descriptor type `{0}`")]
    InvalidType(char),
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum MethodDescriptorErr {
    #[error("")]
    ShouldStartWithParentheses,
    #[error("")]
    MissingClosingParenthesis,
    #[error("")]
    TrailingCharacters,
    #[error("Method descriptor error in \"{0}\": {1}")]
    Type(String, TypeDescriptorErr),
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum InstructionErr {
    #[error("Unsupported opcode `{0:#04X}`")]
    UnsupportedOpCode(u8),
    #[error(transparent)]
    Cursor(#[from] CursorError),
}
