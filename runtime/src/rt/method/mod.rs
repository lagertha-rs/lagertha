use crate::rt::method::java::Method;
use crate::rt::method::native::NativeMethod;

pub mod java;
pub mod native;

#[derive(Debug)]
pub enum StaticMethodType {
    Java(Method),
    Native(NativeMethod),
}

impl StaticMethodType {
    pub fn is_main(&self) -> bool {
        match self {
            StaticMethodType::Java(m) => m.is_main(),
            StaticMethodType::Native(_) => false,
        }
    }
}

#[derive(Debug)]
pub enum VirtualMethodType {
    Java(Method),
    Native(NativeMethod),
}
