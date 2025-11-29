#[macro_export]
macro_rules! build_exception {
    ($kind:ident, $fmt:literal $(, $args:expr)* $(,)?) => {
        crate::error::JvmError::JavaException(
            crate::error::JavaExceptionFromJvm::with_message(
                crate::error::JavaExceptionKind::$kind,
                format!($fmt $(, $args)*),
            )
        )
    };
    ($kind:ident, $msg:expr) => {
        crate::error::JvmError::JavaException(
            crate::error::JavaExceptionFromJvm::with_message(
                crate::error::JavaExceptionKind::$kind,
                $msg,
            )
        )
    };
    ($kind:ident) => {
        crate::error::JvmError::JavaException(
            crate::error::JavaExceptionFromJvm::new(
                crate::error::JavaExceptionKind::$kind,
            )
        )
    };
    ($kind:ident, method_key: $mk:expr, class_sym: $class_sym:expr) => {
        crate::error::JvmError::JavaException(
            crate::error::JavaExceptionFromJvm::with_method_not_found(
                crate::error::JavaExceptionKind::$kind,
                $mk,
                $class_sym,
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
