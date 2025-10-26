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
