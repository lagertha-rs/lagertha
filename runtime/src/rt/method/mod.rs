use crate::rt::method::java::Method;
use crate::rt::method::native::NativeMethod;

pub mod java;
pub mod native;

pub enum StaticMethodType {
    Java(Method),
    Native(NativeMethod),
}

pub enum VirtualMethodType {
    Abstract(Method),
    Java(Method),
    Native(NativeMethod),
}

impl VirtualMethodType {
    pub fn set_class(
        &self,
        class: std::sync::Arc<crate::rt::class::class::Class>,
    ) -> Result<(), crate::rt::class::LinkageError> {
        match self {
            VirtualMethodType::Abstract(m) => m.set_class(class),
            VirtualMethodType::Java(m) => m.set_class(class),
            VirtualMethodType::Native(m) => m.set_class(class),
        }
    }
}
