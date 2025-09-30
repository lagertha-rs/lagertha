use crate::rt::method::java::Method;
use crate::rt::method::native::NativeMethod;
use std::sync::Arc;

pub mod java;
pub mod native;

pub enum StaticMethodType {
    Java(Arc<Method>),
    Native(NativeMethod),
}

impl StaticMethodType {
    pub fn set_class(
        &self,
        class: Arc<crate::rt::class::class::Class>,
    ) -> Result<(), crate::rt::class::LinkageError> {
        match self {
            StaticMethodType::Java(m) => m.set_class(class),
            StaticMethodType::Native(m) => m.set_class(class),
        }
    }
}

pub enum VirtualMethodType {
    Abstract(Arc<Method>),
    Java(Arc<Method>),
    Native(NativeMethod),
}

impl VirtualMethodType {
    pub fn set_class(
        &self,
        class: Arc<crate::rt::class::class::Class>,
    ) -> Result<(), crate::rt::class::LinkageError> {
        match self {
            VirtualMethodType::Abstract(m) => m.set_class(class),
            VirtualMethodType::Java(m) => m.set_class(class),
            VirtualMethodType::Native(m) => m.set_class(class),
        }
    }
}
