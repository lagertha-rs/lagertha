use crate::ClassFileErr;
use crate::constant::pool::ConstantPool;
use common::pretty_try;
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
    InterfaceMethodRef(ReferenceInfo),
    NameAndType(NameAndTypeInfo),
    Dynamic(DynamicInfo),
    InvokeDynamic(DynamicInfo),
    MethodHandle(MethodHandleInfo),
    MethodType(u16),
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

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.4.10
#[cfg_attr(test, derive(Serialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DynamicInfo {
    pub bootstrap_method_attr_index: u16,
    pub name_and_type_index: u16,
}

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.4.8
#[cfg_attr(test, derive(Serialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodHandleInfo {
    pub reference_kind: u8,
    pub reference_index: u16,
}

#[cfg_attr(test, derive(Serialize))]
#[derive(Debug, Clone, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum MethodHandleKind {
    GetField = 1,
    GetStatic = 2,
    PutField = 3,
    PutStatic = 4,
    InvokeVirtual = 5,
    InvokeStatic = 6,
    InvokeSpecial = 7,
    NewInvokeSpecial = 8,
    InvokeInterface = 9,
}

impl MethodHandleInfo {
    pub fn new(reference_kind: u8, reference_index: u16) -> Self {
        Self {
            reference_kind,
            reference_index,
        }
    }

    pub fn get_kind(&self) -> Result<MethodHandleKind, ClassFileErr> {
        MethodHandleKind::try_from_primitive(self.reference_kind)
            .map_err(|_| ClassFileErr::InvalidMethodHandleKind(self.reference_kind))
    }
}

impl DynamicInfo {
    pub fn new(bootstrap_method_attr_index: u16, name_and_type_index: u16) -> Self {
        Self {
            bootstrap_method_attr_index,
            name_and_type_index,
        }
    }
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
                Self::InterfaceMethodRef(ReferenceInfo::new(cursor.u16()?, cursor.u16()?))
            }
            ConstantTag::NameAndType => {
                Self::NameAndType(NameAndTypeInfo::new(cursor.u16()?, cursor.u16()?))
            }
            ConstantTag::Dynamic => Self::Dynamic(DynamicInfo::new(cursor.u16()?, cursor.u16()?)),
            ConstantTag::InvokeDynamic => {
                Self::InvokeDynamic(DynamicInfo::new(cursor.u16()?, cursor.u16()?))
            }
            ConstantTag::Module => todo!(),
            ConstantTag::Package => todo!(),
            ConstantTag::MethodHandle => {
                Self::MethodHandle(MethodHandleInfo::new(cursor.u8()?, cursor.u16()?))
            }
            ConstantTag::MethodType => Self::MethodType(cursor.u16()?),
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
            ConstantInfo::InterfaceMethodRef(_) => ConstantTag::InterfaceMethodRef,
            ConstantInfo::NameAndType(_) => ConstantTag::NameAndType,
            ConstantInfo::Dynamic(_) => ConstantTag::Dynamic,
            ConstantInfo::InvokeDynamic(_) => ConstantTag::InvokeDynamic,
            ConstantInfo::MethodHandle(_) => ConstantTag::MethodHandle,
            ConstantInfo::MethodType(_) => ConstantTag::MethodType,
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
            ConstantInfo::Utf8(s) => writeln!(ind, "{}", s.escape_debug()),
            ConstantInfo::Integer(i) => writeln!(ind, "{i}"),
            ConstantInfo::Float(fl) => writeln!(ind, "{fl}"),
            ConstantInfo::Long(l) => writeln!(ind, "{l}l"),
            ConstantInfo::Double(d) => writeln!(ind, "{d}"),
            ConstantInfo::String(index) => writeln!(
                ind,
                "{:<op_w$} // {}",
                format!("#{index}"),
                pretty_try!(ind, cp.get_printable_utf8(index)),
                op_w = op_w
            ),
            ConstantInfo::Class(index) => writeln!(
                ind,
                "{:<op_w$} // {}",
                format!("#{index}"),
                pretty_try!(ind, cp.get_pretty_class_name_utf8(index)),
                op_w = op_w
            ),
            ConstantInfo::MethodRef(ref_info) | ConstantInfo::InterfaceMethodRef(ref_info) => {
                writeln!(
                    ind,
                    "{:<op_w$} // {}.{}:{}",
                    format!(
                        "#{}.#{}",
                        ref_info.class_index, ref_info.name_and_type_index
                    ),
                    pretty_try!(ind, cp.get_method_or_field_class_name(ref_info)),
                    pretty_method_name_try!(ind, cp.get_method_or_field_name(ref_info)),
                    pretty_try!(ind, cp.get_method_or_field_descriptor(ref_info)),
                    op_w = op_w
                )
            }
            ConstantInfo::NameAndType(nat) => writeln!(
                ind,
                "{:<op_w$} // {}:{}",
                format!("#{}:#{}", nat.name_index, nat.descriptor_index),
                pretty_method_name_try!(ind, cp.get_nat_name(nat)),
                pretty_try!(ind, cp.get_nat_descriptor(nat)),
                op_w = op_w
            ),
            ConstantInfo::FieldRef(ref_info) => writeln!(
                ind,
                "{:<op_w$} // {}.{}:{}",
                format!(
                    "#{}.#{}",
                    ref_info.class_index, ref_info.name_and_type_index
                ),
                pretty_try!(ind, cp.get_class_name(&ref_info.class_index)),
                pretty_try!(ind, cp.get_method_or_field_name(ref_info)),
                pretty_try!(ind, cp.get_method_or_field_descriptor(ref_info)),
                op_w = op_w
            ),
            ConstantInfo::Dynamic(dyn_info) | ConstantInfo::InvokeDynamic(dyn_info) => {
                writeln!(
                    ind,
                    "{:<op_w$} // #{}:{}:{}",
                    format!(
                        "#{}:#{}",
                        dyn_info.bootstrap_method_attr_index, dyn_info.name_and_type_index
                    ),
                    dyn_info.bootstrap_method_attr_index,
                    pretty_try!(ind, cp.get_dyn_info_name(dyn_info)),
                    pretty_try!(ind, cp.get_dyn_info_descriptor(dyn_info)),
                    op_w = op_w
                )
            }
            ConstantInfo::MethodType(idx) => writeln!(
                ind,
                "{:<op_w$} // {}",
                format!("#{}", idx),
                pretty_try!(ind, cp.get_utf8(idx)),
                op_w = op_w
            ),
            ConstantInfo::MethodHandle(handle_info) => {
                let handle_kind = pretty_try!(ind, handle_info.get_kind());
                let method_ref = pretty_try!(ind, cp.get_methodref(&handle_info.reference_index));
                writeln!(
                    ind,
                    "{:<op_w$} // {} {}.{}:{}",
                    format!(
                        "{}:#{}",
                        handle_info.reference_kind, handle_info.reference_index
                    ),
                    pretty_try!(ind, handle_kind.get_pretty_value()),
                    pretty_try!(ind, cp.get_method_or_field_class_name(method_ref)),
                    pretty_method_name_try!(ind, cp.get_method_or_field_name(method_ref)),
                    pretty_try!(ind, cp.get_method_or_field_descriptor(method_ref)),
                    op_w = op_w
                )
            }
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
            ConstantInfo::Class(index) => {
                format!("class {}", cp.get_pretty_class_name_utf8(index)?)
            }
            ConstantInfo::String(index) => format!("String {}", cp.get_printable_utf8(index)?),
            ConstantInfo::MethodRef(ref_info) => {
                let method_name = match cp.get_method_or_field_name(ref_info)? {
                    "<init>" => "\"<init>\"".to_owned(),
                    other => other.to_owned(),
                };
                let name = cp.get_method_or_field_class_name(ref_info)?;
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
                    cp.get_method_or_field_descriptor(ref_info)?,
                )
            }
            ConstantInfo::FieldRef(ref_info) => {
                let name = cp.get_method_or_field_class_name(ref_info)?;
                let final_class_name = {
                    if name != cp.get_class_name(this_class_name)? {
                        format_args!("{}.", name)
                    } else {
                        format_args!("")
                    }
                };
                format!(
                    "Field {}{}:{}",
                    final_class_name,
                    cp.get_method_or_field_name(ref_info)?,
                    cp.get_method_or_field_descriptor(ref_info)?,
                )
            }
            ConstantInfo::InterfaceMethodRef(ref_info) => {
                let name = cp.get_method_or_field_class_name(ref_info)?;
                let final_class_name = {
                    if name != cp.get_class_name(this_class_name)? {
                        format_args!("{}.", name)
                    } else {
                        format_args!("")
                    }
                };
                format!(
                    "InterfaceMethod {}{}:{}",
                    final_class_name,
                    cp.get_method_or_field_name(ref_info)?,
                    cp.get_method_or_field_descriptor(ref_info)?,
                )
            }
            ConstantInfo::InvokeDynamic(dyn_info) => {
                format!(
                    "InvokeDynamic #{}:{}:{}",
                    dyn_info.bootstrap_method_attr_index,
                    cp.get_dyn_info_name(dyn_info)?,
                    cp.get_dyn_info_descriptor(dyn_info)?
                )
            }
            ConstantInfo::Dynamic(_) => "Dynamic (details omitted)".to_owned(),
            e => todo!("Pretty print not implemented for {e:?}"),
        })
    }
}

impl MethodHandleKind {
    #[cfg(feature = "pretty_print")]
    pub(crate) fn get_pretty_value(&self) -> Result<&str, ClassFileErr> {
        Ok(match self {
            MethodHandleKind::GetField => "REF_getField",
            MethodHandleKind::GetStatic => "REF_getStatic",
            MethodHandleKind::PutField => "REF_putField",
            MethodHandleKind::PutStatic => "REF_putStatic",
            MethodHandleKind::InvokeVirtual => "REF_invokeVirtual",
            MethodHandleKind::InvokeStatic => "REF_invokeStatic",
            MethodHandleKind::InvokeSpecial => "REF_invokeSpecial",
            MethodHandleKind::NewInvokeSpecial => "REF_newInvokeSpecial",
            MethodHandleKind::InvokeInterface => "REF_invokeInterface",
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
            ConstantTag::InterfaceMethodRef => "InterfaceMethodref",
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
