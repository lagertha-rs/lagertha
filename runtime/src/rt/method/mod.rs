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
