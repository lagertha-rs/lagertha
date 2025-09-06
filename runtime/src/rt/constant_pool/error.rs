use crate::rt::constant_pool::RuntimeConstantType;
use common::{MethodDescriptorErr, TypeDescriptorErr};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RuntimePoolError {
    #[error(transparent)]
    MethodDescriptor(#[from] MethodDescriptorErr),
    #[error(transparent)]
    TypeDescriptor(#[from] TypeDescriptorErr),
    #[error("WrongIndex")]
    WrongIndex(u16),
    #[error("TypeError")]
    TypeError(u16, RuntimeConstantType, RuntimeConstantType), //TODO: named args?
    #[error("TryingToAccessUnresolved")]
    TryingToAccessUnresolved(u16, RuntimeConstantType),
}
