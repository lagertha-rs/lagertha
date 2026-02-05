use crate::constant_pool::types::{Dynamic, MethodHandle, NameAndType, Reference};
#[cfg(feature = "javap_print")]
use crate::constant_pool::ConstantPool;
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

    //TODO: check, don't want to spend too much time here, it is AI generated
    #[cfg(feature = "javap_print")]
    fn format_double_minimal_javap(x: f64) -> String {
        let bits = x.to_bits();
        if bits == 0x0000_0000_0000_0001 {
            return "4.9E-324d".into();
        }
        if bits == 0x8000_0000_0000_0001 {
            return "-4.9E-324d".into();
        }
        if x.is_nan() {
            return "NaNd".into();
        }
        if x.is_infinite() {
            return if x.is_sign_negative() {
                "-Infinityd".into()
            } else {
                "Infinityd".into()
            };
        }
        if x == 0.0 {
            return if x.is_sign_negative() {
                "-0.0d".into()
            } else {
                "0.0d".into()
            };
        }

        let abs = x.abs();
        let mut s = x.to_string();

        if s.contains('e') || s.contains('E') {
            s = s.replace('e', "E");
            return format!("{s}d");
        }

        if !(1e-3..1e7).contains(&abs) {
            let neg = x.is_sign_negative();
            let s = s.trim_start_matches('-');

            let (int_part, frac_part) = match s.find('.') {
                Some(p) => (&s[..p], &s[p + 1..]),
                None => (s, ""),
            };

            let int_no_lead = int_part.trim_start_matches('0');

            let (digits, exp): (String, i32) = if !int_no_lead.is_empty() {
                let mut d = String::with_capacity(int_no_lead.len() + frac_part.len());
                d.push_str(int_no_lead);
                d.push_str(frac_part);
                (d, int_no_lead.len() as i32 - 1)
            } else {
                let mut k = 0usize;
                for ch in frac_part.chars() {
                    if ch == '0' {
                        k += 1;
                    } else {
                        break;
                    }
                }
                let d = frac_part[k..].to_string();
                (d, -(k as i32 + 1))
            };

            let mut chars = digits.chars();
            let first = chars.next().unwrap(); // safe: x != 0
            let mut rest: String = chars.collect();
            while rest.ends_with('0') {
                rest.pop();
            }

            let mantissa = if rest.is_empty() {
                format!("{first}.0")
            } else {
                format!("{first}.{rest}")
            };

            let sign = if neg { "-" } else { "" };
            return format!("{sign}{mantissa}E{exp}d");
        }

        if !s.contains('.') {
            s.push_str(".0");
        }
        format!("{s}d")
    }

    //TODO: check, don't want to spend too much time here, it is AI generated
    #[cfg(feature = "javap_print")]
    pub fn format_float_minimal_javap(x: f32) -> String {
        use std::f32;

        if x.is_nan() {
            return "NaNf".to_string();
        }
        if x.is_infinite() {
            return if x.is_sign_positive() {
                "Infinityf".to_string()
            } else {
                "-Infinityf".to_string()
            };
        }

        if x == 0.0 {
            return if x.is_sign_negative() {
                "-0.0f".to_string()
            } else {
                "0.0f".to_string()
            };
        }

        let abs_x = x.abs();

        if !(1e-3..1e7).contains(&abs_x) {
            let formatted = if abs_x < f32::MIN_POSITIVE {
                format!("{:.1E}", x)
            } else {
                format!("{:.7E}", x)
            };

            return format!("{}f", formatted);
        }

        let mut s = x.to_string();

        if !s.contains(['.', 'e', 'E']) {
            s.push_str(".0");
        }

        format!("{}f", s)
    }

    #[cfg(feature = "javap_print")]
    pub(crate) fn javap_fmt(
        &self,
        ind: &mut common::utils::indent_write::Indented<'_>,
        cp: &ConstantPool,
    ) -> std::fmt::Result {
        use common::{try_javap_print, try_javap_print_method_name};
        use std::fmt::Write as _;
        let op_w = 16;
        match self {
            ConstantEntry::Utf8(s) => writeln!(ind, "{}", s.escape_debug()),
            ConstantEntry::Integer(i) => writeln!(ind, "{i}"),
            ConstantEntry::Float(fl) => writeln!(ind, "{}", Self::format_float_minimal_javap(*fl)),
            ConstantEntry::Long(l) => writeln!(ind, "{l}l"),
            ConstantEntry::Double(d) => writeln!(ind, "{}", Self::format_double_minimal_javap(*d)),
            ConstantEntry::String(index) => writeln!(
                ind,
                "{:<op_w$} // {}",
                format!("#{index}"),
                try_javap_print!(ind, cp.get_printable_utf8(index)),
                op_w = op_w
            ),
            ConstantEntry::Class(index) => writeln!(
                ind,
                "{:<op_w$} // {}",
                format!("#{index}"),
                try_javap_print!(ind, cp.get_javap_class_name_utf8(index)),
                op_w = op_w
            ),
            ConstantEntry::MethodRef(ref_info) | ConstantEntry::InterfaceMethodRef(ref_info) => {
                writeln!(
                    ind,
                    "{:<op_w$} // {}.{}:{}",
                    format!(
                        "#{}.#{}",
                        ref_info.class_index, ref_info.name_and_type_index
                    ),
                    try_javap_print!(ind, cp.get_method_or_field_class_name(ref_info)),
                    try_javap_print_method_name!(ind, cp.get_method_or_field_name(ref_info)),
                    try_javap_print!(ind, cp.get_method_or_field_descriptor(ref_info)),
                    op_w = op_w
                )
            }
            ConstantEntry::NameAndType(nat) => writeln!(
                ind,
                "{:<op_w$} // {}:{}",
                format!("#{}:#{}", nat.name_index, nat.descriptor_index),
                try_javap_print_method_name!(ind, cp.get_nat_name(nat)),
                try_javap_print!(ind, cp.get_nat_descriptor(nat)),
                op_w = op_w
            ),
            ConstantEntry::FieldRef(ref_info) => writeln!(
                ind,
                "{:<op_w$} // {}.{}:{}",
                format!(
                    "#{}.#{}",
                    ref_info.class_index, ref_info.name_and_type_index
                ),
                try_javap_print!(ind, cp.get_class_name(&ref_info.class_index)),
                try_javap_print!(ind, cp.get_method_or_field_name(ref_info)),
                try_javap_print!(ind, cp.get_method_or_field_descriptor(ref_info)),
                op_w = op_w
            ),
            ConstantEntry::Dynamic(dyn_info) | ConstantEntry::InvokeDynamic(dyn_info) => {
                writeln!(
                    ind,
                    "{:<op_w$} // #{}:{}:{}",
                    format!(
                        "#{}:#{}",
                        dyn_info.bootstrap_method_attr_index, dyn_info.name_and_type_index
                    ),
                    dyn_info.bootstrap_method_attr_index,
                    try_javap_print!(ind, cp.get_dyn_info_name(dyn_info)),
                    try_javap_print!(ind, cp.get_dyn_info_descriptor(dyn_info)),
                    op_w = op_w
                )
            }
            ConstantEntry::MethodType(idx) => writeln!(
                ind,
                "{:<op_w$} // {}",
                format!("#{}", idx),
                try_javap_print!(ind, cp.get_utf8(idx)),
                op_w = op_w
            ),
            ConstantEntry::MethodHandle(handle_info) => {
                let handle_kind = try_javap_print!(ind, handle_info.get_kind());
                let method_ref =
                    try_javap_print!(ind, cp.get_methodref(&handle_info.reference_index));
                writeln!(
                    ind,
                    "{:<op_w$} // {} {}.{}:{}",
                    format!(
                        "{}:#{}",
                        handle_info.reference_kind, handle_info.reference_index
                    ),
                    try_javap_print!(ind, handle_kind.get_javap_value()),
                    try_javap_print!(ind, cp.get_method_or_field_class_name(method_ref)),
                    try_javap_print_method_name!(ind, cp.get_method_or_field_name(method_ref)),
                    try_javap_print!(ind, cp.get_method_or_field_descriptor(method_ref)),
                    op_w = op_w
                )
            }
            e => todo!("Pretty print not implemented for {e:?}"),
        }
    }

    #[cfg(feature = "javap_print")]
    pub(crate) fn get_javap_value(
        &self,
        cp: &ConstantPool,
        this_class_name: &u16,
    ) -> Result<String, ClassFormatErr> {
        Ok(match self {
            ConstantEntry::Integer(i) => format!("{}", i),
            ConstantEntry::Class(index) => cp.get_javap_class_name_utf8(index)?.to_string(),
            _ => self.get_javap_type_and_value(cp, this_class_name)?,
        })
    }

    #[cfg(feature = "javap_print")]
    pub(crate) fn get_javap_type_and_value(
        &self,
        cp: &ConstantPool,
        this_class_name: &u16,
    ) -> Result<String, ClassFormatErr> {
        Ok(match self {
            ConstantEntry::Utf8(s) => format!("utf8 {}", s),
            ConstantEntry::Integer(i) => format!("int {}", i),
            ConstantEntry::Float(fl) => format!("float {}", Self::format_float_minimal_javap(*fl)),
            ConstantEntry::Long(l) => format!("long {}l", l),
            ConstantEntry::Double(d) => {
                format!("double {}", Self::format_double_minimal_javap(*d))
            }
            ConstantEntry::Class(index) => {
                format!("class {}", cp.get_javap_class_name_utf8(index)?)
            }
            ConstantEntry::String(index) => format!("String {}", cp.get_printable_utf8(index)?),
            ConstantEntry::MethodRef(ref_info) => {
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
            ConstantEntry::FieldRef(ref_info) => {
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
            ConstantEntry::InterfaceMethodRef(ref_info) => {
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
            ConstantEntry::InvokeDynamic(dyn_info) => {
                format!(
                    "InvokeDynamic #{}:{}:{}",
                    dyn_info.bootstrap_method_attr_index,
                    cp.get_dyn_info_name(dyn_info)?,
                    cp.get_dyn_info_descriptor(dyn_info)?
                )
            }
            ConstantEntry::MethodHandle(handle_info) => {
                let handle_kind = handle_info.get_kind()?;
                let method_ref = cp.get_methodref(&handle_info.reference_index)?;
                let method_name = match cp.get_method_or_field_name(method_ref)? {
                    "<init>" => "\"<init>\"".to_owned(),
                    other => other.to_owned(),
                };
                format!(
                    "{} {}.{}:{}",
                    handle_kind.get_javap_value()?,
                    cp.get_method_or_field_class_name(method_ref)?,
                    method_name,
                    cp.get_method_or_field_descriptor(method_ref)?,
                )
            }
            ConstantEntry::MethodType(idx) => cp.get_utf8(idx)?.to_string(),
            ConstantEntry::Dynamic(_) => "Dynamic (details omitted)".to_owned(),
            e => todo!("Pretty print not implemented for {e:?}"),
        })
    }
}
