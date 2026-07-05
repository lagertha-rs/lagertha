use crate::heap::HeapRef;
use lasso::ThreadedRodeo;
use lvm_class::{ClassFormatErr, InstructionErr};
use lvm_common::error::{MethodDescriptorErr, TypeDescriptorErr};
use lvm_common::utils::cursor::CursorError;
use std::fmt::Display;

#[derive(Debug)]
pub enum JvmError {
    MainClassNotFound(String),
    Cursor(CursorError),
    RuntimePool(RuntimePoolError),
    MissingAttributeInConstantPoll,
    ConstantNotFoundInRuntimePool,
    TrailingBytes,
    StackOverflow,
    FrameStackIsEmpty,
    OperandStackIsEmpty,
    OutOfMemory,
    NoMainClassFound(String),
    NoSuchFieldError(String),
    LocalVariableNotFound(u8),
    LocalVariableNotInitialized(u8),
    TypeDescriptorErr(TypeDescriptorErr),
    InstructionErr(InstructionErr),
    ClassMirrorIsAlreadyCreated,
    MethodIsAbstract(String),
    UnexpectedType(String),
    Uninitialized,
    WrongHeapAddress(HeapRef),
    Todo(String),
    NotAJavaInstanceTodo(String),

    // TODO: to be refactored next
    Linkage(LinkageError),

    // Exception that is not mapped yet
    JavaExceptionDescriptor(JavaExceptionDescriptor),

    // Mapped java exception
    JavaException(HeapRef),
}

impl From<CursorError> for JvmError {
    fn from(value: CursorError) -> Self {
        JvmError::Cursor(value)
    }
}

impl From<TypeDescriptorErr> for JvmError {
    fn from(value: TypeDescriptorErr) -> Self {
        JvmError::TypeDescriptorErr(value)
    }
}

impl From<InstructionErr> for JvmError {
    fn from(value: InstructionErr) -> Self {
        JvmError::InstructionErr(value)
    }
}

impl From<RuntimePoolError> for JvmError {
    fn from(value: RuntimePoolError) -> Self {
        JvmError::RuntimePool(value)
    }
}

impl From<LinkageError> for JvmError {
    fn from(value: LinkageError) -> Self {
        JvmError::Linkage(value)
    }
}

impl From<JavaExceptionDescriptor> for JvmError {
    fn from(value: JavaExceptionDescriptor) -> Self {
        JvmError::JavaExceptionDescriptor(value)
    }
}

impl Display for JvmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl JvmError {
    pub fn into_pretty_string(self, interner: &ThreadedRodeo) -> String {
        match self {
            JvmError::JavaExceptionDescriptor(desc) => {
                let mut result = desc.kind.class_name_dot();
                if let Some(message) = desc.message {
                    result.push_str(": ");
                    result.push_str(&message);
                }
                result
            }
            _ => format!("{:?}", self),
        }
    }
}

pub struct JavaExceptionReference {
    pub class: &'static str,
    pub name: &'static str,
    pub descriptor: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JavaExceptionKind {
    ArithmeticException,
    UnsupportedOperationException,
    ArrayIndexOutOfBoundsException,
    NegativeArraySizeException,
    NullPointerException,
    ArrayStoreException,
    InternalError,
    NoSuchMethodError,
    ClassNotFoundException,
    UnsatisfiedLinkError,
    IncompatibleClassChangeError,
    ClassFormatError,
    IOException,
}

impl JavaExceptionKind {
    pub const fn class_name(self) -> &'static str {
        match self {
            Self::ArithmeticException => "java/lang/ArithmeticException",
            Self::UnsupportedOperationException => "java/lang/UnsupportedOperationException",
            Self::ArrayIndexOutOfBoundsException => "java/lang/ArrayIndexOutOfBoundsException",
            Self::NegativeArraySizeException => "java/lang/NegativeArraySizeException",
            Self::NullPointerException => "java/lang/NullPointerException",
            Self::ArrayStoreException => "java/lang/ArrayStoreException",
            Self::InternalError => "java/lang/InternalError",
            Self::NoSuchMethodError => "java/lang/NoSuchMethodError",
            Self::ClassNotFoundException => "java/lang/ClassNotFoundException",
            Self::UnsatisfiedLinkError => "java/lang/UnsatisfiedLinkError",
            Self::IncompatibleClassChangeError => "java/lang/IncompatibleClassChangeError",
            Self::ClassFormatError => "java/lang/ClassFormatError",
            Self::IOException => "java/io/IOException",
        }
    }

    pub fn class_name_dot(self) -> String {
        self.class_name().replace('/', ".")
    }
}

#[derive(Debug, Clone)]
pub struct JavaExceptionDescriptor {
    pub kind: JavaExceptionKind,
    pub message: Option<String>,
}

impl JavaExceptionDescriptor {
    const CONSTRUCTOR_NAME: &'static str = "<init>";
    const STRING_PARAM_CONSTRUCTOR: &'static str = "(Ljava/lang/String;)V";
    const NO_PARAM_CONSTRUCTOR: &'static str = "()V";

    pub fn new(kind: JavaExceptionKind) -> Self {
        Self {
            kind,
            message: None,
        }
    }

    pub fn with_message(kind: JavaExceptionKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: Some(message.into()),
        }
    }

    pub fn as_reference(&self) -> JavaExceptionReference {
        JavaExceptionReference {
            class: self.kind.class_name(),
            name: Self::CONSTRUCTOR_NAME,
            descriptor: if self.message.is_some() {
                Self::STRING_PARAM_CONSTRUCTOR
            } else {
                Self::NO_PARAM_CONSTRUCTOR
            },
        }
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
    ClassFormat(ClassFormatErr, String),
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

#[derive(Debug)]
pub enum RuntimePoolError {
    MethodDescriptor(MethodDescriptorErr),
    TypeDescriptor(TypeDescriptorErr),
    TryingToAccessUnresolved(u16, String),
}
