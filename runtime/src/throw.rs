#[macro_export]
macro_rules! build_exception {
        ($variant:ident, $fmt:literal $(, $args:expr)* $(,)?) => {
        common::error::JvmError::JavaException(
            common::error::JavaExceptionFromJvm::$variant(Some(format!($fmt $(, $args)*))),
        )
    };
    ($variant:ident, $msg:expr) => {
        common::error::JvmError::JavaException(
            common::error::JavaExceptionFromJvm::$variant(Some($msg.into())),
        )
    };
    ($variant:ident) => {
        common::error::JvmError::JavaException(
            common::error::JavaExceptionFromJvm::$variant(None),
        )
    };
}

#[macro_export]
macro_rules! throw_exception {
    ($variant:ident, $fmt:literal $(, $args:expr)* $(,)?) => {
        Err(crate::build_exception!($variant, $fmt $(, $args)*))
    };
    ($variant:ident, $msg:expr) => {
        Err(crate::build_exception!($variant, $msg))
    };
    ($variant:ident) => {
        Err(crate::build_exception!($variant))
    };
}
