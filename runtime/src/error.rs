use crate::ClassId;
use crate::class_loader::ClassLoaderErr;
use crate::rt::LinkageError;
use crate::rt::constant_pool::error::RuntimePoolError;
use common::jtype::HeapAddr;
use common::utils::cursor::CursorError;
use thiserror::Error;

//TODO: there is a way to not copy-paste macros? refactor later
#[macro_export]
macro_rules! throw_unsupported_operation {
    ($fmt:literal $(, $args:expr)* $(,)?) => {
        Err(JavaExceptionFromJvm::JavaLang(
            JavaLangError::UnsupportedOperationException(format!($fmt $(, $args)*)),
        ))
    };
    ($msg:expr) => {
        Err(JavaExceptionFromJvm::JavaLang(
            JavaLangError::UnsupportedOperationException($msg.into()),
        ))
    };
}

#[macro_export]
macro_rules! throw_arithmetic_exception {
    ($fmt:literal $(, $args:expr)* $(,)?) => {
        Err(JavaExceptionFromJvm::JavaLang(
            JavaLangError::ArithmeticException(format!($fmt $(, $args)*)),
        ))
    };
    ($msg:expr) => {
        Err(JavaExceptionFromJvm::JavaLang(
            JavaLangError::ArithmeticException($msg.into()),
        ))
    };
}

#[macro_export]
macro_rules! throw_negative_array_size_exception {
    ($fmt:literal $(, $args:expr)* $(,)?) => {
        Err(JavaExceptionFromJvm::JavaLang(
            JavaLangError::NegativeArraySizeException(format!($fmt $(, $args)*)),
        ))
    };
    ($msg:expr) => {
        Err(JavaExceptionFromJvm::JavaLang(
            JavaLangError::NegativeArraySizeException($msg.into()),
        ))
    };
}

#[macro_export]
// TODO: can have fixed message template
macro_rules! throw_index_out_of_bounds_exception {
    ($fmt:literal $(, $args:expr)* $(,)?) => {
        Err(JavaExceptionFromJvm::JavaLang(
            JavaLangError::ArrayIndexOutOfBoundsException(format!($fmt $(, $args)*)),
        ))
    };
    ($msg:expr) => {
        Err(JavaExceptionFromJvm::JavaLang(
            JavaLangError::ArrayIndexOutOfBoundsException($msg.into()),
        ))
    };
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
    TypeDescriptorErr(#[from] common::TypeDescriptorErr),
    #[error("NullPointerException")]
    NullPointerException,
    #[error("InstructionErr: {0}")]
    InstructionErr(#[from] common::InstructionErr),
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

    pub fn get_message(&self) -> &String {
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
        }
    }

    pub fn get_message(&self) -> &String {
        match self {
            JavaLangError::ArithmeticException(msg) => msg,
            JavaLangError::UnsupportedOperationException(msg) => msg,
            JavaLangError::ArrayIndexOutOfBoundsException(msg) => msg,
            JavaLangError::NegativeArraySizeException(msg) => msg,
        }
    }
}
