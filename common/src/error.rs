use crate::utils::cursor::CursorError;
use std::fmt::{Display, Formatter};

// TODO: looks like a trash bin, needs refactoring
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignatureErr {
    UnexpectedEnd,
    MissingParamsOpenParen,
    MissingParamsCloseParen,
    TrailingCharacters,
    InvalidIdentifier,
    MissingSuper,
    InvalidBound,
    Type(TypeDescriptorErr),
    InvalidSuperClassType,
}

impl Display for SignatureErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SignatureErr::UnexpectedEnd => write!(f, "Unexpected end of signature"),
            SignatureErr::MissingParamsOpenParen => {
                write!(f, "Missing opening parenthesis for method parameters")
            }
            SignatureErr::MissingParamsCloseParen => {
                write!(f, "Missing closing parenthesis for method parameters")
            }
            SignatureErr::TrailingCharacters => write!(f, "Trailing characters after signature"),
            SignatureErr::InvalidIdentifier => write!(f, "Invalid identifier in signature"),
            SignatureErr::MissingSuper => write!(f, "Missing 'super' in class signature"),
            SignatureErr::InvalidBound => write!(f, "Invalid bound in type variable"),
            SignatureErr::Type(err) => write!(f, "Type descriptor error: {}", err),
            SignatureErr::InvalidSuperClassType => {
                write!(f, "Invalid superclass type in class signature")
            }
        }
    }
}

impl From<TypeDescriptorErr> for SignatureErr {
    fn from(value: TypeDescriptorErr) -> Self {
        SignatureErr::Type(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeDescriptorErr {
    UnexpectedEnd,
    InvalidType(char),
    InvalidObjectRef,
}

impl Display for TypeDescriptorErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeDescriptorErr::UnexpectedEnd => write!(f, "Unexpected end of type descriptor"),
            TypeDescriptorErr::InvalidType(c) => write!(f, "Invalid type character: {}", c),
            TypeDescriptorErr::InvalidObjectRef => write!(f, "Invalid object reference type"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MethodDescriptorErr {
    ShouldStartWithParentheses(String),
    MissingClosingParenthesis(String),
    TrailingCharacters,
    Type(String, TypeDescriptorErr),
}

impl Display for MethodDescriptorErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MethodDescriptorErr::ShouldStartWithParentheses(desc) => {
                write!(f, "Method descriptor should start with '(': {}", desc)
            }
            MethodDescriptorErr::MissingClosingParenthesis(desc) => {
                write!(
                    f,
                    "Missing closing parenthesis in method descriptor: {}",
                    desc
                )
            }
            MethodDescriptorErr::TrailingCharacters => {
                write!(f, "Trailing characters after method descriptor")
            }
            MethodDescriptorErr::Type(desc, err) => {
                write!(f, "Type descriptor error in '{}': {}", desc, err)
            }
        }
    }
}

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

#[derive(Debug)]
pub enum LinkageError {
    Instruction(InstructionErr),
    UnsupportedOpCode(u8),
    DuplicatedCodeAttr,
    //TODO: confused 4.7.13. The LocalVariableTable Attribute
    //DuplicatedLocalVariableTableAttr,
    DuplicatedSignatureAttr,
    DuplicatedStackMapTable,
    DuplicatedExceptionAttribute,
    DuplicatedRuntimeVisibleAnnotationsAttr,
    DuplicatedRuntimeInvisibleAnnotationsAttr,
    CodeAttrIsAmbiguousForNative,
    RuntimeConstantPool(RuntimePoolError),
    Cursor(CursorError),
    ClassFile(ClassFormatErr),
    DuplicatedClassInMethod,
    MethodClassIsNotSet,
}

impl From<InstructionErr> for LinkageError {
    fn from(value: InstructionErr) -> Self {
        LinkageError::Instruction(value)
    }
}

impl From<CursorError> for LinkageError {
    fn from(value: CursorError) -> Self {
        LinkageError::Cursor(value)
    }
}

impl From<RuntimePoolError> for LinkageError {
    fn from(value: RuntimePoolError) -> Self {
        LinkageError::RuntimeConstantPool(value)
    }
}

impl From<ClassFormatErr> for LinkageError {
    fn from(value: ClassFormatErr) -> Self {
        LinkageError::ClassFile(value)
    }
}

#[derive(Debug)]
pub enum RuntimePoolError {
    MethodDescriptor(MethodDescriptorErr),
    TypeDescriptor(TypeDescriptorErr),
    TryingToAccessUnresolved(u16, String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClassFormatErr {
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
}

impl Display for ClassFormatErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
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
