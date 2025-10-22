use crate::ClassId;
use crate::class_loader::ClassLoaderErr;
use crate::rt::LinkageError;
use crate::rt::constant_pool::error::RuntimePoolError;
use common::jtype::HeapAddr;
use common::utils::cursor::CursorError;
use thiserror::Error;

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
    #[error("ClassNotFoundException: {0}")]
    ClassNotFound2(ClassId),
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
    #[error("NegativeArraySizeException")]
    NegativeArraySizeException,
    #[error("ArrayIndexOutOfBoundsException")]
    ArrayIndexOutOfBoundsException,
    #[error("Method is not expecting to be abstract `{0}`")]
    MethodIsAbstract(String),
    #[error("ArithmeticException `{0}`")]
    ArithmeticException(String),
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
}

impl JavaLangError {
    pub fn as_reference(&self) -> JavaExceptionReference {
        match self {
            JavaLangError::ArithmeticException(_) => JavaExceptionReference {
                class: "java/lang/ArithmeticException",
                name: "<init>",
                descriptor: "(Ljava/lang/String;)V",
            },
        }
    }

    pub fn get_message(&self) -> &String {
        match self {
            JavaLangError::ArithmeticException(msg) => msg,
        }
    }
}
