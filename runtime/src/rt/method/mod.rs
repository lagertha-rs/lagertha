use crate::rt::method::java::Method;
use crate::rt::method::native::NativeMethod;

pub mod java;
pub mod native;

#[derive(Debug)]
pub enum StaticMethodType {
    Java(Method),
    Native(NativeMethod),
}

#[derive(Debug)]
pub enum VirtualMethodType {
    Java(Method),
    Native(NativeMethod),
}
