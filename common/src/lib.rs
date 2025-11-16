extern crate core;

use crate::error::{JavaExceptionFromJvm, JvmError};

pub mod descriptor;
pub mod error;
pub mod instruction;
pub mod jtype;
pub mod signature;
pub mod utils;

pub type HeapRef = usize;

/// Used to represent stack operand, local variable, arguments and static field values.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    Integer(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Ref(HeapRef),
    Null,
}

impl Value {
    pub fn as_obj_ref(&self) -> Result<HeapRef, JvmError> {
        match self {
            Value::Ref(addr) => Ok(*addr),
            Value::Null => Err(JvmError::JavaException(
                JavaExceptionFromJvm::NullPointerException(None),
            )),
            _ => Err(JvmError::Todo(
                "Value::as_obj_ref called on non-reference value".to_string(),
            )),
        }
    }

    pub fn as_int(&self) -> Result<i32, JvmError> {
        match self {
            Value::Integer(v) => Ok(*v),
            _ => Err(JvmError::Todo(
                "Value::as_int called on non-integer value".to_string(),
            )),
        }
    }
}
