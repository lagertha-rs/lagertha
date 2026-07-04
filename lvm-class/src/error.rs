use crate::flags::ClassFlags;
use lvm_common::error::{MethodDescriptorErr, SignatureErr, TypeDescriptorErr};
use lvm_common::utils::cursor::CursorError;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstructionErr {
    UnsupportedOpCode(u8),
    UnknownArrayType(u8),
    Cursor(CursorError),
    UnexpectedEof,
}

impl Display for InstructionErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InstructionErr::UnsupportedOpCode(opcode) => {
                write!(f, "Unsupported opcode: {:#X}", opcode)
            }
            InstructionErr::UnknownArrayType(atype) => {
                write!(f, "Unknown array type: {}", atype)
            }
            InstructionErr::Cursor(err) => write!(f, "Cursor error: {}", err),
            InstructionErr::UnexpectedEof => write!(f, "Unexpected end of instruction stream"),
        }
    }
}

impl From<CursorError> for InstructionErr {
    fn from(value: CursorError) -> Self {
        InstructionErr::Cursor(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClassFormatErr {
    IllegalClassFlags(ClassFlags),
    // TODO: custom errors, not tested and not compared with the oracle's jvm
    Cursor(CursorError),
    WrongMagic(u32),
    TrailingBytes,
    UnknownTag(u8),
    /// First u16 is index, second is expected type, third is actual type
    TypeError(u16, String, String),
    ConstantNotFound(u16),
    UnknownStackFrameType(u8),
    UnknownAttribute(String),
    AttributeIsNotShared(String),
    InvalidMethodHandleKind(u8),
    Signature(SignatureErr),
    MethodDescriptor(MethodDescriptorErr),
    Instruction(InstructionErr),
    TypeDescriptor(TypeDescriptorErr),
    Format(std::fmt::Error),
}

impl Display for ClassFormatErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "java.lang.ClassFormatError: ")?;
        match self {
            ClassFormatErr::IllegalClassFlags(flags) => {
                write!(f, "Illegal class modifiers: {:x}", flags.get_raw())
            }
            ClassFormatErr::Cursor(err) => write!(f, "Cursor error: {}", err),
            ClassFormatErr::WrongMagic(magic) => write!(f, "Wrong magic number: {:#X}", magic),
            ClassFormatErr::TrailingBytes => write!(f, "Trailing bytes after class file"),
            ClassFormatErr::UnknownTag(tag) => write!(f, "Unknown constant pool tag: {}", tag),
            ClassFormatErr::TypeError(index, expected, actual) => write!(
                f,
                "Type error at index {}: expected {}, got {}",
                index, expected, actual
            ),
            ClassFormatErr::ConstantNotFound(index) => {
                write!(f, "Constant not found at index {}", index)
            }
            ClassFormatErr::UnknownStackFrameType(frame_type) => {
                write!(f, "Unknown stack frame type: {}", frame_type)
            }
            ClassFormatErr::UnknownAttribute(name) => write!(f, "Unknown attribute: {}", name),
            ClassFormatErr::AttributeIsNotShared(name) => {
                write!(f, "Attribute is not shared: {}", name)
            }
            ClassFormatErr::InvalidMethodHandleKind(kind) => {
                write!(f, "Invalid method handle kind: {}", kind)
            }
            ClassFormatErr::Signature(err) => write!(f, "Signature error: {}", err),
            ClassFormatErr::MethodDescriptor(err) => {
                write!(f, "Method descriptor error: {}", err)
            }
            ClassFormatErr::Instruction(err) => write!(f, "Instruction error: {}", err),
            ClassFormatErr::TypeDescriptor(err) => {
                write!(f, "Type descriptor error: {}", err)
            }
            ClassFormatErr::Format(err) => write!(f, "Format error: {}", err),
        }
    }
}

impl From<CursorError> for ClassFormatErr {
    fn from(value: CursorError) -> Self {
        ClassFormatErr::Cursor(value)
    }
}

impl From<SignatureErr> for ClassFormatErr {
    fn from(value: SignatureErr) -> Self {
        ClassFormatErr::Signature(value)
    }
}

impl From<MethodDescriptorErr> for ClassFormatErr {
    fn from(value: MethodDescriptorErr) -> Self {
        ClassFormatErr::MethodDescriptor(value)
    }
}

impl From<InstructionErr> for ClassFormatErr {
    fn from(value: InstructionErr) -> Self {
        ClassFormatErr::Instruction(value)
    }
}

impl From<TypeDescriptorErr> for ClassFormatErr {
    fn from(value: TypeDescriptorErr) -> Self {
        ClassFormatErr::TypeDescriptor(value)
    }
}

impl From<std::fmt::Error> for ClassFormatErr {
    fn from(value: std::fmt::Error) -> Self {
        ClassFormatErr::Format(value)
    }
}
