use crate::ClassFileErr;
use common::cursor::ByteCursor;
#[cfg(feature = "pretty_print")]
use common::indent_write::Indented;
use common::pretty_try;
use num_enum::TryFromPrimitive;
#[cfg(feature = "pretty_print")]
use std::fmt::Write as _;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub struct ConstantPool {
    pub cp: Vec<ConstantInfo>,
}

impl ConstantPool {
    pub fn get_utf8(&self, idx: u16) -> Result<&str, ClassFileErr> {
        self.cp
            .get(idx as usize)
            .ok_or(ClassFileErr::ConstantNotFound(idx))
            .and_then(|entry| match entry {
                ConstantInfo::Utf8(value) => Ok(value.as_str()),
                e => Err(ClassFileErr::TypeError(idx, ConstantTag::Utf8, e.get_tag())),
            })
    }

    #[cfg(feature = "pretty_print")]
    pub fn get_class(&self, idx: u16) -> Result<u16, ClassFileErr> {
        self.cp
            .get(idx as usize)
            .ok_or(ClassFileErr::ConstantNotFound(idx))
            .and_then(|entry| match entry {
                ConstantInfo::Class(name_index) => Ok(*name_index),
                e => Err(ClassFileErr::TypeError(
                    idx,
                    ConstantTag::Class,
                    e.get_tag(),
                )),
            })
    }

    #[cfg(feature = "pretty_print")]
    pub fn get_class_name(&self, idx: u16) -> Result<&str, ClassFileErr> {
        let name_index = self.get_class(idx)?;
        self.get_utf8(name_index)
    }

    #[cfg(feature = "pretty_print")]
    pub fn get_methodref(&self, idx: u16) -> Result<&ReferenceInfo, ClassFileErr> {
        self.cp
            .get(idx as usize)
            .ok_or(ClassFileErr::ConstantNotFound(idx))
            .and_then(|entry| match entry {
                ConstantInfo::MethodRef(ref_info) => Ok(ref_info),
                e => Err(ClassFileErr::TypeError(
                    idx,
                    ConstantTag::MethodRef,
                    e.get_tag(),
                )),
            })
    }

    #[cfg(feature = "pretty_print")]
    pub fn get_name_and_type(&self, idx: u16) -> Result<&NameAndTypeInfo, ClassFileErr> {
        self.cp
            .get(idx as usize)
            .ok_or(ClassFileErr::ConstantNotFound(idx))
            .and_then(|entry| match entry {
                ConstantInfo::NameAndType(ref_info) => Ok(ref_info),
                e => Err(ClassFileErr::TypeError(
                    idx,
                    ConstantTag::NameAndType,
                    e.get_tag(),
                )),
            })
    }

    #[cfg(feature = "pretty_print")]
    pub fn get_nat_name(&self, nat: &NameAndTypeInfo) -> Result<&str, ClassFileErr> {
        self.get_utf8(nat.name_index)
    }
    #[cfg(feature = "pretty_print")]
    pub fn get_nat_descriptor(&self, nat: &NameAndTypeInfo) -> Result<&str, ClassFileErr> {
        self.get_utf8(nat.descriptor_index)
    }
    #[cfg(feature = "pretty_print")]
    pub fn get_method_class_name(&self, method_ref: &ReferenceInfo) -> Result<&str, ClassFileErr> {
        self.get_class_name(method_ref.class_index)
    }

    #[cfg(feature = "pretty_print")]
    pub fn get_method_name(&self, method_ref: &ReferenceInfo) -> Result<&str, ClassFileErr> {
        let nat_index = method_ref.name_and_type_index;
        let nat = self.get_name_and_type(nat_index)?;
        self.get_nat_name(nat)
    }

    #[cfg(feature = "pretty_print")]
    pub fn get_method_descriptor(&self, method_ref: &ReferenceInfo) -> Result<&str, ClassFileErr> {
        let nat_index = method_ref.name_and_type_index;
        let desc_index = self.get_name_and_type(nat_index)?;
        self.get_nat_descriptor(desc_index)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum ConstantTag {
    Dummy = 0,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReferenceInfo {
    pub class_index: u16,
    pub name_and_type_index: u16,
}

impl ReferenceInfo {
    pub fn new(class_index: u16, name_and_type_index: u16) -> Self {
        Self {
            class_index,
            name_and_type_index,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NameAndTypeInfo {
    pub name_index: u16,
    pub descriptor_index: u16,
}

impl NameAndTypeInfo {
    pub fn new(name_index: u16, descriptor_index: u16) -> Self {
        Self {
            name_index,
            descriptor_index,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConstantInfo {
    Dummy,
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

impl<'a> ConstantInfo {
    pub(crate) fn read(cursor: &mut ByteCursor<'a>) -> Result<Self, ClassFileErr> {
        let raw_tag = cursor.u8()?;
        let tag = ConstantTag::try_from_primitive(raw_tag)
            .map_err(|_| ClassFileErr::UnknownTag(raw_tag))?;
        let const_info = match tag {
            ConstantTag::Dummy => {
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
            ConstantInfo::Dummy => ConstantTag::Dummy,
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
    pub(crate) fn fmt_pretty(&self, ind: &mut Indented<'_>, cp: &ConstantPool) -> std::fmt::Result {
        let op_w = 16;
        match self {
            ConstantInfo::Utf8(s) => {
                writeln!(ind, "{s}")
            }
            ConstantInfo::Integer(i) => writeln!(ind, "{}", i),
            ConstantInfo::Float(fl) => writeln!(ind, "{}", fl),
            ConstantInfo::Long(l) => writeln!(ind, "{}", l),
            ConstantInfo::Double(d) => writeln!(ind, "{}", d),
            ConstantInfo::Class(index) => writeln!(
                ind,
                "{:<op_w$} // {}",
                format!("#{index}"),
                pretty_try!(ind, cp.get_utf8(*index)),
                op_w = op_w
            ),
            ConstantInfo::String(index) => writeln!(
                ind,
                "{:<op_w$} // {}",
                format!("#{index}"),
                pretty_try!(ind, cp.get_utf8(*index)),
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
                pretty_try!(ind, cp.get_method_name(ref_info)),
                pretty_try!(ind, cp.get_method_descriptor(ref_info)),
                op_w = op_w
            ),
            ConstantInfo::FieldRef(ref_info) => {
                writeln!(
                    ind,
                    "FieldRef(class: {}, name_and_type: {})",
                    ref_info.class_index, ref_info.name_and_type_index
                )
            }
            ConstantInfo::InterfaceRef(ref_info) => {
                writeln!(
                    ind,
                    "InterfaceRef(class: {}, name_and_type: {})",
                    ref_info.class_index, ref_info.name_and_type_index
                )
            }
            ConstantInfo::NameAndType(nat) => writeln!(
                ind,
                "{:<op_w$} // {}:{}",
                format!("#{}:#{}", nat.name_index, nat.descriptor_index),
                pretty_try!(ind, cp.get_nat_name(nat)),
                pretty_try!(ind, cp.get_nat_descriptor(nat)),
                op_w = op_w
            ),
            e => {
                writeln!(ind, "GGGG {e:?}")
            }
        }
    }
}

#[cfg(feature = "pretty_print")]
impl Display for ConstantTag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ConstantTag::Dummy => "Dummy",
            ConstantTag::Utf8 => "Utf8",
            ConstantTag::Integer => "Integer",
            ConstantTag::Float => "Float",
            ConstantTag::Long => "Long",
            ConstantTag::Double => "Double",
            ConstantTag::Class => "Class",
            ConstantTag::String => "String",
            ConstantTag::FieldRef => "FieldRef",
            ConstantTag::MethodRef => "MethodRef",
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
