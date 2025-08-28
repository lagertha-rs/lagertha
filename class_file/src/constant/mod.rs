use crate::ClassFileErr;
use crate::constant::pool::ConstantPool;
use common::utils::cursor::ByteCursor;
use num_enum::TryFromPrimitive;
#[cfg(test)]
use serde::Serialize;
use std::fmt::{Display, Formatter};

pub mod pool;

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.4-210
/// Table 4.4-B. Constant pool tags (by tag)
#[cfg_attr(test, derive(Serialize))]
#[derive(Debug, Clone, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum ConstantTag {
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

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.4-140
/// Each entry is as described in section column of Table 4.4-A. Constant pool tags (by section)
#[cfg_attr(test, derive(Serialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum ConstantInfo {
    Unused,
    Utf8(String),
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Class(u16),
    String(u16),
    MethodRef(ReferenceInfo),
    FieldRef(ReferenceInfo),
    InterfaceRef(ReferenceInfo),
    NameAndType(NameAndTypeInfo),
}

#[cfg_attr(test, derive(Serialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReferenceInfo {
    pub class_index: u16,
    pub name_and_type_index: u16,
}

#[cfg_attr(test, derive(Serialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NameAndTypeInfo {
    pub name_index: u16,
    pub descriptor_index: u16,
}

impl ReferenceInfo {
    pub fn new(class_index: u16, name_and_type_index: u16) -> Self {
        Self {
            class_index,
            name_and_type_index,
        }
    }
}

impl NameAndTypeInfo {
    pub fn new(name_index: u16, descriptor_index: u16) -> Self {
        Self {
            name_index,
            descriptor_index,
        }
    }
}

impl<'a> ConstantInfo {
    pub(crate) fn read(cursor: &mut ByteCursor<'a>) -> Result<Self, ClassFileErr> {
        let raw_tag = cursor.u8()?;
        let tag = ConstantTag::try_from_primitive(raw_tag)
            .map_err(|_| ClassFileErr::UnknownTag(raw_tag))?;
        let const_info = match tag {
            ConstantTag::Unused => {
                unreachable!() // TODO: Sure?
            }
            ConstantTag::Utf8 => {
                let len = cursor.u16()?;
                let bytes = cursor.bytes(len as usize)?;
                Self::Utf8(String::from_utf8(bytes).unwrap())
            }
            ConstantTag::Integer => {
                let value = cursor.i32()?;
                Self::Integer(value)
            }
            ConstantTag::Float => todo!(),
            ConstantTag::Long => {
                let value = cursor.i64()?;
                Self::Long(value)
            }
            ConstantTag::Double => todo!(),
            ConstantTag::Class => Self::Class(cursor.u16()?),
            ConstantTag::String => Self::String(cursor.u16()?),
            ConstantTag::FieldRef => {
                Self::FieldRef(ReferenceInfo::new(cursor.u16()?, cursor.u16()?))
            }
            ConstantTag::MethodRef => {
                Self::MethodRef(ReferenceInfo::new(cursor.u16()?, cursor.u16()?))
            }
            ConstantTag::InterfaceMethodRef => {
                Self::InterfaceRef(ReferenceInfo::new(cursor.u16()?, cursor.u16()?))
            }
            ConstantTag::NameAndType => {
                Self::NameAndType(NameAndTypeInfo::new(cursor.u16()?, cursor.u16()?))
            }
            ConstantTag::Dynamic => todo!(),
            ConstantTag::InvokeDynamic => todo!(),
            ConstantTag::Module => todo!(),
            ConstantTag::Package => todo!(),
            ConstantTag::MethodHandle | ConstantTag::MethodType => todo!(),
        };
        Ok(const_info)
    }

    pub fn get_tag(&self) -> ConstantTag {
        match self {
            ConstantInfo::Unused => ConstantTag::Unused,
            ConstantInfo::Utf8(_) => ConstantTag::Utf8,
            ConstantInfo::Integer(_) => ConstantTag::Integer,
            ConstantInfo::Float(_) => ConstantTag::Float,
            ConstantInfo::Long(_) => ConstantTag::Long,
            ConstantInfo::Double(_) => ConstantTag::Double,
            ConstantInfo::Class(_) => ConstantTag::Class,
            ConstantInfo::String(_) => ConstantTag::String,
            ConstantInfo::MethodRef(_) => ConstantTag::MethodRef,
            ConstantInfo::FieldRef(_) => ConstantTag::FieldRef,
            ConstantInfo::InterfaceRef(_) => ConstantTag::InterfaceMethodRef,
            ConstantInfo::NameAndType(_) => ConstantTag::NameAndType,
        }
    }

    #[cfg(feature = "pretty_print")]
    pub(crate) fn fmt_pretty(
        &self,
        ind: &mut common::utils::indent_write::Indented<'_>,
        cp: &ConstantPool,
    ) -> std::fmt::Result {
        use common::{pretty_method_name_try, pretty_try};
        use std::fmt::Write as _;
        let op_w = 16;
        match self {
            ConstantInfo::Utf8(s) => writeln!(ind, "{s}"),
            ConstantInfo::Integer(i) => writeln!(ind, "{i}"),
            ConstantInfo::Float(fl) => writeln!(ind, "{fl}"),
            ConstantInfo::Long(l) => writeln!(ind, "{l}l"),
            ConstantInfo::Double(d) => writeln!(ind, "{d}"),
            ConstantInfo::Class(index) => writeln!(
                ind,
                "{:<op_w$} // {}",
                format!("#{index}"),
                pretty_try!(ind, cp.get_utf8(index)),
                op_w = op_w
            ),
            ConstantInfo::String(index) => writeln!(
                ind,
                "{:<op_w$} // {}",
                format!("#{index}"),
                pretty_try!(ind, cp.get_utf8(index)),
                op_w = op_w
            ),
            ConstantInfo::MethodRef(ref_info) => writeln!(
                ind,
                "{:<op_w$} // {}.{}:{}",
                format!(
                    "#{}.#{}",
                    ref_info.class_index, ref_info.name_and_type_index
                ),
                pretty_try!(ind, cp.get_method_class_name(ref_info)),
                pretty_method_name_try!(ind, cp.get_method_name(ref_info)),
                pretty_try!(ind, cp.get_method_descriptor(ref_info)),
                op_w = op_w
            ),
            ConstantInfo::NameAndType(nat) => writeln!(
                ind,
                "{:<op_w$} // {}:{}",
                format!("#{}:#{}", nat.name_index, nat.descriptor_index),
                pretty_method_name_try!(ind, cp.get_nat_name(nat)),
                pretty_try!(ind, cp.get_nat_descriptor(nat)),
                op_w = op_w
            ),
            e => todo!("Pretty print not implemented for {e:?}"),
        }
    }

    #[cfg(feature = "pretty_print")]
    pub(crate) fn get_pretty_value(
        &self,
        cp: &ConstantPool,
        this_class_name: &u16,
    ) -> Result<String, ClassFileErr> {
        Ok(match self {
            ConstantInfo::Utf8(s) => format!("utf8 {}", s),
            ConstantInfo::Integer(i) => format!("int {}", i),
            ConstantInfo::Float(fl) => format!("float {}", fl),
            ConstantInfo::Long(l) => format!("long {}l", l),
            ConstantInfo::Double(d) => format!("double {}", d),
            ConstantInfo::Class(index) => format!("class {}", cp.get_utf8(index)?),
            ConstantInfo::String(index) => format!("String {}", cp.get_utf8(index)?),
            ConstantInfo::MethodRef(ref_info) => {
                let method_name = match cp.get_method_name(ref_info)? {
                    "<init>" => "\"<init>\"".to_owned(),
                    other => other.to_owned(),
                };
                let name = cp.get_method_class_name(ref_info)?;
                let final_class_name = {
                    if name != cp.get_class_name(this_class_name)? {
                        format_args!("{}.", name)
                    } else {
                        format_args!("")
                    }
                };
                format!(
                    "Method {}{}:{}",
                    final_class_name,
                    method_name,
                    cp.get_method_descriptor(ref_info)?,
                )
            }
            e => todo!("Pretty print not implemented for {e:?}"),
        })
    }
}

impl Display for ConstantTag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ConstantTag::Unused => "Unused",
            ConstantTag::Utf8 => "Utf8",
            ConstantTag::Integer => "Integer",
            ConstantTag::Float => "Float",
            ConstantTag::Long => "Long",
            ConstantTag::Double => "Double",
            ConstantTag::Class => "Class",
            ConstantTag::String => "String",
            ConstantTag::FieldRef => "Fieldref",
            ConstantTag::MethodRef => "Methodref",
            ConstantTag::InterfaceMethodRef => "InterfaceMethodRef",
            ConstantTag::NameAndType => "NameAndType",
            ConstantTag::MethodHandle => "MethodHandle",
            ConstantTag::MethodType => "MethodType",
            ConstantTag::Dynamic => "Dynamic",
            ConstantTag::InvokeDynamic => "InvokeDynamic",
            ConstantTag::Module => "Module",
            ConstantTag::Package => "Package",
        };
        f.pad(s)
    }
}
