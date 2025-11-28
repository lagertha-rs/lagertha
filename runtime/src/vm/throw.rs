#[macro_export]
macro_rules! build_exception {
    ($kind:ident, $fmt:literal $(, $args:expr)* $(,)?) => {
        common::error::JvmError::JavaException(
            common::error::JavaExceptionFromJvm::with_message(
                common::error::JavaExceptionKind::$kind,
                format!($fmt $(, $args)*),
            )
        )
    };
    ($kind:ident, $msg:expr) => {
        common::error::JvmError::JavaException(
            common::error::JavaExceptionFromJvm::with_message(
                common::error::JavaExceptionKind::$kind,
                $msg,
            )
        )
    };
    ($kind:ident) => {
        common::error::JvmError::JavaException(
            common::error::JavaExceptionFromJvm::new(
                common::error::JavaExceptionKind::$kind,
            )
        )
    };
}

#[macro_export]
macro_rules! throw_exception {
    ($($args:tt)*) => {
        Err($crate::build_exception!($($args)*))
    };
}
