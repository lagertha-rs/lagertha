use crate::heap::HeapRef;
use crate::keys::{MethodKey, Symbol};
use crate::rt::constant_pool::RuntimeConstantType;
use common::descriptor::MethodDescriptor;
use common::error::{InstructionErr, LinkageError, RuntimePoolError, TypeDescriptorErr};
use common::utils::cursor::CursorError;
use lasso::ThreadedRodeo;
use std::fmt::Display;

#[derive(Debug)]
pub enum JvmError {
    MainClassNotFound(String),
    Linkage(LinkageError),
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
    JavaExceptionThrown(HeapRef),
    Uninitialized,
    WrongHeapAddress(HeapRef),
    Todo(String),
    NotAJavaInstanceTodo(String),
    JavaException(JavaExceptionFromJvm),
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

impl From<JavaExceptionFromJvm> for JvmError {
    fn from(value: JavaExceptionFromJvm) -> Self {
        JvmError::JavaException(value)
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
            JvmError::JavaException(ex) => {
                let mut result = ex.kind.class_name_dot();
                if let Some(message) = ex.message {
                    let resolved_message = message.into_resolved(interner);
                    result.push_str(": ");
                    result.push_str(&resolved_message);
                }
                if let Some(cause) = ex.cause {
                    result.push_str(&format!(
                        "\nCaused by: {}",
                        JvmError::JavaException(*cause).into_pretty_string(interner)
                    ));
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

#[derive(Debug, Clone)]
pub enum ExceptionMessage {
    Resolved(String),
    MethodNotFound(MethodKey, Symbol),
    IncompatibleClassChangeRuntimePool {
        pool_idx: u16,
        expected: RuntimeConstantType,
        actual: RuntimeConstantType,
    },
}

impl ExceptionMessage {
    pub fn into_resolved(self, interner: &ThreadedRodeo) -> String {
        match self {
            ExceptionMessage::Resolved(s) => s,
            ExceptionMessage::MethodNotFound(method_key, class_sym) => {
                let desc_str = interner.resolve(&method_key.desc);
                let class_name = interner.resolve(&class_sym);
                let method_name = interner.resolve(&method_key.name);
                MethodDescriptor::try_from(desc_str)
                    .unwrap()
                    .to_java_signature(class_name, method_name)
            }
            ExceptionMessage::IncompatibleClassChangeRuntimePool {
                pool_idx,
                expected,
                actual,
            } => {
                format!(
                    "Incompatible class change at runtime constant pool index {}: expected {}, found {}",
                    pool_idx, expected, actual
                )
            }
        }
    }
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
pub struct JavaExceptionFromJvm {
    pub kind: JavaExceptionKind,
    pub message: Option<ExceptionMessage>,
    pub cause: Option<Box<JavaExceptionFromJvm>>,
}

impl JavaExceptionFromJvm {
    const CONSTRUCTOR_NAME: &'static str = "<init>";
    const STRING_PARAM_CONSTRUCTOR: &'static str = "(Ljava/lang/String;)V";
    const NO_PARAM_CONSTRUCTOR: &'static str = "()V";

    pub fn new(kind: JavaExceptionKind) -> Self {
        Self {
            kind,
            message: None,
            cause: None,
        }
    }

    pub fn with_message(kind: JavaExceptionKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: Some(ExceptionMessage::Resolved(message.into())),
            cause: None,
        }
    }

    pub fn with_method_not_found(
        kind: JavaExceptionKind,
        key: MethodKey,
        class_sym: Symbol,
    ) -> Self {
        Self {
            kind,
            message: Some(ExceptionMessage::MethodNotFound(key, class_sym)),
            cause: None,
        }
    }

    pub fn with_runtime_pool_incompatible_class_change(
        kind: JavaExceptionKind,
        pool_idx: u16,
        expected: RuntimeConstantType,
        actual: RuntimeConstantType,
    ) -> Self {
        Self {
            kind,
            message: Some(ExceptionMessage::IncompatibleClassChangeRuntimePool {
                pool_idx,
                expected,
                actual,
            }),
            cause: None,
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
