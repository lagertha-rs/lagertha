use crate::rt::constant_pool::RuntimeConstantType;
use common::{MethodDescriptorErr, TypeDescriptorErr};
use std::fmt;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RuntimePoolError {
    #[error(transparent)]
    MethodDescriptor(#[from] MethodDescriptorErr),
    #[error(transparent)]
    TypeDescriptor(#[from] TypeDescriptorErr),
    #[error("")]
    WrongIndex(u16),
    #[error("")]
    TypeError(u16, RuntimeConstantType, RuntimeConstantType), //TODO: named args?
    #[error("")]
    TryingToAccessUnresolved(u16, RuntimeConstantType),
}

impl Into<fmt::Error> for RuntimePoolError {
    fn into(self) -> fmt::Error {
        fmt::Error
    }
}
