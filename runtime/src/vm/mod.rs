use crate::error::JvmError;
use crate::heap::HeapRef;
use crate::throw_exception;
use common::jtype::{JavaType, PrimitiveType};

pub mod bootstrap_registry;
pub mod stack;
pub mod throw;

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
    pub fn as_nullable_obj_ref(&self) -> Result<Option<HeapRef>, JvmError> {
        match self {
            Value::Ref(addr) => Ok(Some(*addr)),
            Value::Null => Ok(None),
            _ => Err(JvmError::Todo(
                "Value::as_nullable_obj_ref called on non-reference value".to_string(),
            )),
        }
    }

    pub fn as_obj_ref(&self) -> Result<HeapRef, JvmError> {
        match self {
            Value::Ref(addr) => Ok(*addr),
            Value::Null => throw_exception!(NullPointerException),
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

    pub fn as_long(&self) -> Result<i64, JvmError> {
        match self {
            Value::Long(v) => Ok(*v),
            _ => Err(JvmError::Todo(
                "Value::as_long called on non-long value".to_string(),
            )),
        }
    }

    pub fn as_double(&self) -> Result<f64, JvmError> {
        match self {
            Value::Double(v) => Ok(*v),
            _ => Err(JvmError::Todo(
                "Value::as_double called on non-double value".to_string(),
            )),
        }
    }
}

impl From<&PrimitiveType> for Value {
    fn from(value: &PrimitiveType) -> Self {
        match value {
            PrimitiveType::Byte
            | PrimitiveType::Char
            | PrimitiveType::Short
            | PrimitiveType::Int
            | PrimitiveType::Boolean => Value::Integer(0),
            PrimitiveType::Double => Value::Double(0.0),
            PrimitiveType::Float => Value::Float(0.0),
            PrimitiveType::Long => Value::Long(0),
        }
    }
}

impl From<&JavaType> for Value {
    fn from(jtype: &JavaType) -> Self {
        match jtype {
            JavaType::Primitive(prim) => Value::from(prim),
            JavaType::Instance(_)
            | JavaType::GenericInstance(_)
            | JavaType::TypeVar(_)
            | JavaType::Array(_) => Value::Null,
        }
    }
}
