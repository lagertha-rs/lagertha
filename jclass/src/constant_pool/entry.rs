use crate::constant_pool::types::{Dynamic, MethodHandle, NameAndType, Reference};
use common::error::ClassFormatErr;
use common::utils::cursor::ByteCursor;
use num_enum::TryFromPrimitive;
use std::fmt::{Display, Formatter};

/// Tag identifying the type of a constant pool entry.
///
/// https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4-210
/// Table 4.4-B. Constant pool tags (by tag)
#[derive(Debug, Clone, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum ConstantKind {
    Unused = 0,
    Utf8 = 1,
    Integer = 3,
    Float = 4,
    Long = 5,
    Double = 6,
    Class = 7,
    String = 8,
    FieldRef = 9,
    MethodRef = 10,
    InterfaceMethodRef = 11,
    NameAndType = 12,
    MethodHandle = 15,
    MethodType = 16,
    Dynamic = 17,
    InvokeDynamic = 18,
    Module = 19,
    Package = 20,
}

impl Display for ConstantKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ConstantKind::Unused => "Unused",
            ConstantKind::Utf8 => "Utf8",
            ConstantKind::Integer => "Integer",
            ConstantKind::Float => "Float",
            ConstantKind::Long => "Long",
            ConstantKind::Double => "Double",
            ConstantKind::Class => "Class",
            ConstantKind::String => "String",
            ConstantKind::FieldRef => "Fieldref",
            ConstantKind::MethodRef => "Methodref",
            ConstantKind::InterfaceMethodRef => "InterfaceMethodref",
            ConstantKind::NameAndType => "NameAndType",
            ConstantKind::MethodHandle => "MethodHandle",
            ConstantKind::MethodType => "MethodType",
            ConstantKind::Dynamic => "Dynamic",
            ConstantKind::InvokeDynamic => "InvokeDynamic",
            ConstantKind::Module => "Module",
            ConstantKind::Package => "Package",
        };
        f.pad(s)
    }
}

/// A single entry in the constant pool.
///
/// https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4-140
/// Each entry is as described in section column of Table 4.4-A. Constant pool tags (by section)
#[derive(Debug, Clone, PartialEq)]
pub enum ConstantEntry {
    Unused,
    Utf8(String),
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Class(u16),
    String(u16),
    MethodRef(Reference),
    FieldRef(Reference),
    InterfaceMethodRef(Reference),
    NameAndType(NameAndType),
    Dynamic(Dynamic),
    InvokeDynamic(Dynamic),
    MethodHandle(MethodHandle),
    MethodType(u16),
}

impl<'a> ConstantEntry {
    pub(crate) fn read(cursor: &mut ByteCursor<'a>) -> Result<Self, ClassFormatErr> {
        let raw_tag = cursor.u8()?;
        let tag = ConstantKind::try_from_primitive(raw_tag)
            .map_err(|_| ClassFormatErr::UnknownTag(raw_tag))?;
        let const_info = match tag {
            ConstantKind::Unused => {
                unreachable!() // TODO: Sure?
            }
            ConstantKind::Utf8 => {
                let len = cursor.u16()?;
                let bytes = cursor.bytes(len as usize)?;
                Self::Utf8(String::from_utf8_lossy(bytes).to_string())
            }
            ConstantKind::Integer => {
                let value = cursor.i32()?;
                Self::Integer(value)
            }
            ConstantKind::Float => {
                let value = cursor.f32()?;
                Self::Float(value)
            }
            ConstantKind::Long => {
                let value = cursor.i64()?;
                Self::Long(value)
            }
            ConstantKind::Double => {
                let value = cursor.f64()?;
                Self::Double(value)
            }
            ConstantKind::Class => Self::Class(cursor.u16()?),
            ConstantKind::String => Self::String(cursor.u16()?),
            ConstantKind::FieldRef => Self::FieldRef(Reference::new(cursor.u16()?, cursor.u16()?)),
            ConstantKind::MethodRef => {
                Self::MethodRef(Reference::new(cursor.u16()?, cursor.u16()?))
            }
            ConstantKind::InterfaceMethodRef => {
                Self::InterfaceMethodRef(Reference::new(cursor.u16()?, cursor.u16()?))
            }
            ConstantKind::NameAndType => {
                Self::NameAndType(NameAndType::new(cursor.u16()?, cursor.u16()?))
            }
            ConstantKind::Dynamic => Self::Dynamic(Dynamic::new(cursor.u16()?, cursor.u16()?)),
            ConstantKind::InvokeDynamic => {
                Self::InvokeDynamic(Dynamic::new(cursor.u16()?, cursor.u16()?))
            }
            ConstantKind::Module => todo!(),
            ConstantKind::Package => todo!(),
            ConstantKind::MethodHandle => {
                Self::MethodHandle(MethodHandle::new(cursor.u8()?, cursor.u16()?))
            }
            ConstantKind::MethodType => Self::MethodType(cursor.u16()?),
        };
        Ok(const_info)
    }

    pub fn get_kind(&self) -> ConstantKind {
        match self {
            ConstantEntry::Unused => ConstantKind::Unused,
            ConstantEntry::Utf8(_) => ConstantKind::Utf8,
            ConstantEntry::Integer(_) => ConstantKind::Integer,
            ConstantEntry::Float(_) => ConstantKind::Float,
            ConstantEntry::Long(_) => ConstantKind::Long,
            ConstantEntry::Double(_) => ConstantKind::Double,
            ConstantEntry::Class(_) => ConstantKind::Class,
            ConstantEntry::String(_) => ConstantKind::String,
            ConstantEntry::MethodRef(_) => ConstantKind::MethodRef,
            ConstantEntry::FieldRef(_) => ConstantKind::FieldRef,
            ConstantEntry::InterfaceMethodRef(_) => ConstantKind::InterfaceMethodRef,
            ConstantEntry::NameAndType(_) => ConstantKind::NameAndType,
            ConstantEntry::Dynamic(_) => ConstantKind::Dynamic,
            ConstantEntry::InvokeDynamic(_) => ConstantKind::InvokeDynamic,
            ConstantEntry::MethodHandle(_) => ConstantKind::MethodHandle,
            ConstantEntry::MethodType(_) => ConstantKind::MethodType,
        }
    }
}
