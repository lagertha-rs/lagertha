#[macro_export]
macro_rules! build_exception {
    ($kind:ident, $fmt:literal $(, $args:expr)* $(,)?) => {
        crate::error::JvmError::JavaExceptionDescriptor(
            crate::error::JavaExceptionDescriptor::with_message(
                crate::error::JavaExceptionKind::$kind,
                format!($fmt $(, $args)*),
            )
        )
    };
    ($kind:ident, $msg:expr) => {
        crate::error::JvmError::JavaExceptionDescriptor(
            crate::error::JavaExceptionDescriptor::with_message(
                crate::error::JavaExceptionKind::$kind,
                $msg,
            )
        )
    };
    ($kind:ident) => {
        crate::error::JvmError::JavaExceptionDescriptor(
            crate::error::JavaExceptionDescriptor::new(
                crate::error::JavaExceptionKind::$kind,
            )
        )
    };
    ($kind:ident, method_key: $mk:expr, class_sym: $class_sym:expr) => {{
        let interner = $crate::VirtualMachine::global().interner();
        let desc_str = interner.resolve(&$mk.desc);
        let class_name = interner.resolve(&$class_sym);
        let method_name = interner.resolve(&$mk.name);
        let msg = lvm_common::descriptor::MethodDescriptor::try_from(desc_str)
            .unwrap()
            .to_java_signature(class_name, method_name);
        crate::error::JvmError::JavaExceptionDescriptor(
            crate::error::JavaExceptionDescriptor::with_message(
                crate::error::JavaExceptionKind::$kind,
                msg,
            )
        )
    }};
    ($kind:ident, pool_idx: $pool_idx:expr, expected: $expected:expr, actual: $actual:expr) => {
        crate::error::JvmError::JavaExceptionDescriptor(
            crate::error::JavaExceptionDescriptor::with_message(
                crate::error::JavaExceptionKind::$kind,
                format!(
                    "Incompatible class change at runtime constant pool index {}: expected {}, found {}",
                    $pool_idx, $expected, $actual
                ),
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
