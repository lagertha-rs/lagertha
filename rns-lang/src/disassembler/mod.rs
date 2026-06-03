use common::error::{ClassFormatErr, InstructionErr};
use jclass::ClassFile;
use std::fmt;

mod attribute;
mod class;
mod constant_pool;
mod flags;
mod indent_write;
mod instruction;
mod method;

#[derive(Debug)]
pub enum DisasmError {
    Fmt(fmt::Error),
    ClassFormat(ClassFormatErr),
    Instruction(InstructionErr),
    ConstantNotFound(u16),
    UnsupportedConstant(String),
    UnsupportedMethodAttribute { method: String, attribute: String },
    UnsupportedCodeAttribute { method: String, attribute: &'static str },
    UnsupportedExceptionTable { method: String, handlers: usize },
}

pub type DisasmResult<T> = Result<T, DisasmError>;

impl fmt::Display for DisasmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fmt(err) => write!(f, "format error: {err}"),
            Self::ClassFormat(err) => write!(f, "class format error: {err}"),
            Self::Instruction(err) => write!(f, "instruction error: {err}"),
            Self::ConstantNotFound(idx) => write!(f, "constant not found at index {idx}"),
            Self::UnsupportedConstant(kind) => write!(f, "unsupported constant pool entry {kind}"),
            Self::UnsupportedMethodAttribute { method, attribute } => {
                write!(f, "method {method}: unsupported attribute {attribute}")
            }
            Self::UnsupportedCodeAttribute { method, attribute } => {
                write!(f, "method {method}: unsupported code attribute {attribute}")
            }
            Self::UnsupportedExceptionTable { method, handlers } => {
                write!(f, "method {method}: unsupported exception table ({handlers} handlers)")
            }
        }
    }
}

impl From<fmt::Error> for DisasmError {
    fn from(value: fmt::Error) -> Self {
        Self::Fmt(value)
    }
}

impl From<ClassFormatErr> for DisasmError {
    fn from(value: ClassFormatErr) -> Self {
        Self::ClassFormat(value)
    }
}

impl From<InstructionErr> for DisasmError {
    fn from(value: InstructionErr) -> Self {
        Self::Instruction(value)
    }
}

impl std::error::Error for DisasmError {}

pub fn disassemble(class: &ClassFile) -> DisasmResult<String> {
    class::fmt_rns(class)
}
