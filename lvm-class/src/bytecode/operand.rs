//! Bytecode instruction operand types.
//!
//! Contains data structures for complex instruction operands like switch tables
//! and array type codes.

use num_enum::TryFromPrimitive;
use std::fmt::Formatter;

/// Operand data for the `lookupswitch` instruction.
///
/// https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.lookupswitch
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LookupSwitchData {
    pub padding: u8,
    pub default_offset: i32,
    pub pairs: Vec<(i32, i32)>,
}

/// Operand data for the `tableswitch` instruction.
///
/// https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-6.html#jvms-6.5.tableswitch
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableSwitchData {
    pub padding: u8,
    pub default_offset: i32,
    pub low: i32,
    pub high: i32,
    pub offsets: Vec<i32>,
}

/// Array element type codes for the `newarray` instruction.
///
/// https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.newarray
/// Table 6.5-A. Newarray type codes
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum ArrayType {
    Boolean = 4,
    Char = 5,
    Float = 6,
    Double = 7,
    Byte = 8,
    Short = 9,
    Int = 10,
    Long = 11,
}

impl ArrayType {
    pub fn get_byte_size(&self) -> u8 {
        match self {
            ArrayType::Boolean | ArrayType::Byte => 1,
            ArrayType::Char | ArrayType::Short => 2,
            ArrayType::Int | ArrayType::Float => 4,
            ArrayType::Long | ArrayType::Double => 8,
        }
    }

    pub fn descriptor(&self) -> &str {
        match self {
            ArrayType::Boolean => "[Z",
            ArrayType::Byte => "[B",
            ArrayType::Char => "[C",
            ArrayType::Short => "[S",
            ArrayType::Int => "[I",
            ArrayType::Long => "[J",
            ArrayType::Float => "[F",
            ArrayType::Double => "[D",
        }
    }

    pub const fn as_str(&self) -> &'static str {
        match self {
            ArrayType::Boolean => "boolean",
            ArrayType::Char => "char",
            ArrayType::Float => "float",
            ArrayType::Double => "double",
            ArrayType::Byte => "byte",
            ArrayType::Short => "short",
            ArrayType::Int => "int",
            ArrayType::Long => "long",
        }
    }
}

impl TryFrom<&str> for ArrayType {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "[Z" => Ok(ArrayType::Boolean),
            "[B" => Ok(ArrayType::Byte),
            "[C" => Ok(ArrayType::Char),
            "[S" => Ok(ArrayType::Short),
            "[I" => Ok(ArrayType::Int),
            "[J" => Ok(ArrayType::Long),
            "[F" => Ok(ArrayType::Float),
            "[D" => Ok(ArrayType::Double),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for ArrayType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
