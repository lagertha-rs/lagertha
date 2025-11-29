use crate::utils::cursor::CursorError;

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MethodDescriptorErr {
    ShouldStartWithParentheses(String),
    MissingClosingParenthesis(String),
    TrailingCharacters,
    Type(String, TypeDescriptorErr),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstructionErr {
    UnsupportedOpCode(u8),
    UnknownArrayType(u8),
    Cursor(CursorError),
    UnexpectedEof,
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
    WrongIndex(u16),
    TypeError(u16, String, String),
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
