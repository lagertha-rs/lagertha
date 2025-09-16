extern crate core;

use crate::utils::cursor::CursorError;
use thiserror::Error;

pub mod descriptor;
pub mod instruction;
pub mod jtype;
pub mod signature;
pub mod utils;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum TypeDescriptorErr {
    #[error("Unexpected end of descriptor")]
    UnexpectedEnd,
    #[error("Invalid descriptor type `{0}`")]
    InvalidType(char),
    #[error("Invalid object type")]
    InvalidObjectRef,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum MethodDescriptorErr {
    #[error("Descriptor must start with '('. Descriptor: \"{0}\"")]
    ShouldStartWithParentheses(String),
    #[error("Descriptor must contain ')'. Descriptor: \"{0}\"")]
    MissingClosingParenthesis(String),
    #[error("TrailingCharacters")]
    TrailingCharacters,
    #[error("Method descriptor error in \"{0}\": {1}")]
    Type(String, TypeDescriptorErr),
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum InstructionErr {
    #[error("Unsupported opcode `{0:#04X}`")]
    UnsupportedOpCode(u8),
    #[error("Unknown array type `{0:#04X}`")]
    UnknownArrayType(u8),
    #[error(transparent)]
    Cursor(#[from] CursorError),
}
