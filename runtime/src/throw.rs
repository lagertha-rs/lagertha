#[macro_export]
macro_rules! throw_exception {
    ($variant:ident, $fmt:literal $(, $args:expr)* $(,)?) => {
        Err(common::error::JvmError::JavaException(
            common::error::JavaExceptionFromJvm::$variant(Some(format!($fmt $(, $args)*))),
        ))
    };
    ($variant:ident, $msg:expr) => {
        Err(common::error::JvmError::JavaException(
            common::error::JavaExceptionFromJvm::$variant(Some($msg.into())),
        ))
    };
    ($variant:ident) => {
        Err(common::error::JvmError::JavaException(
            common::error::JavaExceptionFromJvm::$variant(None),
        ))
    };
}
