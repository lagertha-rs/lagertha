use crate::jtype::HeapAddr;
use crate::utils::cursor::CursorError;
use thiserror::Error;

// TODO: looks like a trash bin, needs refactoring
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SignatureErr {
    #[error("Unexpected end of signature")]
    UnexpectedEnd,
    #[error("Expected '(' after formal type parameters")]
    MissingParamsOpenParen,
    #[error("Expected ')' to close parameter list")]
    MissingParamsCloseParen,
    #[error("Trailing characters after method signature")]
    TrailingCharacters,
    #[error("Invalid identifier for type parameter")]
    InvalidIdentifier,
    #[error("Missing superclass signature")]
    MissingSuper,
    #[error("Invalid type after ':' in bound")]
    InvalidBound,
    #[error("Type parse error: {0}")]
    Type(#[from] TypeDescriptorErr),
    #[error("Superclass must be a class type signature")]
    InvalidSuperClassType,
}

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
    #[error("Unexpected end of instruction stream")]
    UnexpectedEof,
}

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

#[derive(Debug, Error)]
pub enum RuntimePoolError {
    #[error(transparent)]
    MethodDescriptor(#[from] MethodDescriptorErr),
    #[error(transparent)]
    TypeDescriptor(#[from] TypeDescriptorErr),
    #[error("WrongIndex")]
    WrongIndex(u16),
    #[error("TypeError at index {0}: expected {1} but found {2}")]
    TypeError(u16, String, String),
    #[error("TryingToAccessUnresolved: index {0} of type {1}")]
    TryingToAccessUnresolved(u16, String),
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ClassFormatErr {
    #[error(transparent)]
    Cursor(#[from] CursorError),
    #[error("Incompatible magic value: {0}")]
    WrongMagic(u32),
    #[error("Expected end of file but found trailing bytes.")]
    TrailingBytes,
    #[error("TrailingBytes")]
    UnknownTag(u8),
    #[error("Expected type `{1}` with index `{0}` but found `{2}`")]
    /// First u16 is index, second is expected type, third is actual type
    TypeError(u16, String, String),
    #[error("Constant with index `{0}` isn't found in constant constant.")]
    ConstantNotFound(u16),
    #[error("Unknown stack frame type {0}.")]
    UnknownStackFrameType(u8),
    #[error("Unknown attribute `{0}.")]
    UnknownAttribute(String),
    #[error("Can't build shared attribute, the `{0}` attribute isn't shared.")]
    AttributeIsNotShared(String),
    #[error("Invalid method handle kind {0}.")]
    InvalidMethodHandleKind(u8),
    #[error(transparent)]
    Signature(#[from] SignatureErr),
    #[error(transparent)]
    MethodDescriptor(#[from] MethodDescriptorErr),
}

#[derive(Debug, Error)]
pub enum ClassLoaderErr {
    #[error("JavaHomeIsNotSet")]
    JavaHomeIsNotSet,
    #[error("CanNotAccessSource")]
    CanNotAccessSource,
    #[error("ClassNotFoundException: {0}")]
    ClassNotFound(String),
    #[error("ArchiveErr")]
    ArchiveErr,
}

#[derive(Debug, Error)]
pub enum JvmError {
    #[error("LinkageError: {0}")]
    Linkage(#[from] LinkageError),
    #[error(transparent)]
    Cursor(#[from] CursorError),
    #[error("RuntimeConstantPoolError: {0}")]
    RuntimePool(#[from] RuntimePoolError),
    #[error(transparent)]
    ClassLoader(#[from] ClassLoaderErr),
    #[error("MissingAttributeInConstantPoll")]
    MissingAttributeInConstantPoll,
    #[error("ConstantNotFoundInRuntimePool")]
    ConstantNotFoundInRuntimePool,
    #[error("TrailingBytes")]
    TrailingBytes,
    #[error("ClassNotFoundException: {0}")]
    ClassNotFound(String),
    #[error("stack overflow")]
    StackOverflow,
    #[error("Frame stack is empty")]
    FrameStackIsEmpty,
    #[error("Operand stack is empty")]
    OperandStackIsEmpty,
    #[error("OutOfMemory")]
    OutOfMemory,
    #[error("Could not find or load main class {0}")]
    NoMainClassFound(String),
    #[error("NoSuchMethod: {0}")]
    NoSuchMethod(String),
    #[error("NoSuchField: {0}")]
    NoSuchFieldError(String),
    #[error("LocalVariableNotFound: {0}")]
    LocalVariableNotFound(u8),
    #[error("LocalVariableNotInitialized: {0}")]
    LocalVariableNotInitialized(u8),
    #[error("TypeDescriptorErr: {0}")]
    TypeDescriptorErr(#[from] TypeDescriptorErr),
    #[error("NullPointerException")]
    NullPointerException,
    #[error("InstructionErr: {0}")]
    InstructionErr(#[from] InstructionErr),
    #[error("ClassMirrorIsAlreadyCreated")]
    ClassMirrorIsAlreadyCreated,
    #[error("Method is not expecting to be abstract `{0}`")]
    MethodIsAbstract(String),
    #[error("UnexpectedType: `{0}`")]
    UnexpectedType(String),
    #[error("JavaExceptionThrown: `{0}`")]
    JavaExceptionThrown(HeapAddr),
    #[error("Uninitialized")]
    Uninitialized,
    #[error("WrongHeapAddress: `{0}`")]
    WrongHeapAddress(HeapAddr),
    #[error("TODO map to correct error: `{0}`")]
    Todo(String),
    #[error("JavaLangError: {0}")]
    JavaException(#[from] JavaExceptionFromJvm),
}

// TODO: everything below needs to be refactored
#[derive(Debug, Error)]
pub enum JavaExceptionFromJvm {
    #[error("LinkageError: {0}")]
    JavaLang(JavaLangError),
}

impl JavaExceptionFromJvm {
    pub fn as_reference(&self) -> JavaExceptionReference {
        match self {
            JavaExceptionFromJvm::JavaLang(err) => err.as_reference(),
        }
    }

    pub fn get_message(&self) -> Option<&String> {
        match self {
            JavaExceptionFromJvm::JavaLang(err) => err.get_message(),
        }
    }
}

pub struct JavaExceptionReference {
    pub class: &'static str,
    pub name: &'static str,
    pub descriptor: &'static str,
}

#[derive(Debug, Error)]
pub enum JavaLangError {
    #[error("java.lang.ArithmeticException: {0}")]
    ArithmeticException(String),
    #[error("java.lang.UnsupportedOperationException: {0}")]
    UnsupportedOperationException(String),
    #[error("java.lang.ArrayIndexOutOfBoundsException: {0}")]
    ArrayIndexOutOfBoundsException(String),
    #[error("java.lang.NegativeArraySizeException: {0}")]
    NegativeArraySizeException(String),
    #[error("java.lang.NullPointerException")]
    NullPointerException,
}

impl JavaLangError {
    pub fn as_reference(&self) -> JavaExceptionReference {
        match self {
            JavaLangError::ArithmeticException(_) => JavaExceptionReference {
                class: "java/lang/ArithmeticException",
                name: "<init>",
                descriptor: "(Ljava/lang/String;)V",
            },
            JavaLangError::UnsupportedOperationException(_) => JavaExceptionReference {
                class: "java/lang/UnsupportedOperationException",
                name: "<init>",
                descriptor: "(Ljava/lang/String;)V",
            },
            JavaLangError::ArrayIndexOutOfBoundsException(_) => JavaExceptionReference {
                class: "java/lang/ArrayIndexOutOfBoundsException",
                name: "<init>",
                descriptor: "(Ljava/lang/String;)V",
            },
            JavaLangError::NegativeArraySizeException(_) => JavaExceptionReference {
                class: "java/lang/NegativeArraySizeException",
                name: "<init>",
                descriptor: "(Ljava/lang/String;)V",
            },
            JavaLangError::NullPointerException => JavaExceptionReference {
                class: "java/lang/NullPointerException",
                name: "<init>",
                descriptor: "()V",
            },
        }
    }

    pub fn get_message(&self) -> Option<&String> {
        match self {
            JavaLangError::ArithmeticException(msg) => Some(msg),
            JavaLangError::UnsupportedOperationException(msg) => Some(msg),
            JavaLangError::ArrayIndexOutOfBoundsException(msg) => Some(msg),
            JavaLangError::NegativeArraySizeException(msg) => Some(msg),
            JavaLangError::NullPointerException => None,
        }
    }
}
