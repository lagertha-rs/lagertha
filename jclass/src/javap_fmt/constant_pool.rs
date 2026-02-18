use crate::constant_pool::{ConstantEntry, ConstantKind, ConstantPool};
use crate::constant_pool::{Dynamic, MethodHandleKind, NameAndType, Reference};
use crate::javap_fmt::fmt_method_name;
use common::error::ClassFormatErr;
use common::utils::indent_write::Indented;
use std::fmt::Write as _;

impl ConstantPool {
    pub fn get_printable_utf8(&self, idx: &u16) -> Result<String, ClassFormatErr> {
        self.get_utf8(idx).map(|raw| raw.escape_debug().to_string())
    }

    pub fn get_raw(&self, idx: &u16) -> Result<&ConstantEntry, ClassFormatErr> {
        self.inner
            .get(*idx as usize)
            .ok_or(ClassFormatErr::ConstantNotFound(*idx))
    }

    pub fn get_integer(&self, idx: &u16) -> Result<&i32, ClassFormatErr> {
        self.inner
            .get(*idx as usize)
            .ok_or(ClassFormatErr::ConstantNotFound(*idx))
            .and_then(|entry| match entry {
                ConstantEntry::Integer(value) => Ok(value),
                e => Err(ClassFormatErr::TypeError(
                    *idx,
                    ConstantKind::Integer.to_string(),
                    e.get_kind().to_string(),
                )),
            })
    }

    //TODO: There is a macro to do that? replace?
    pub fn get_javap_class_name(&self, idx: &u16) -> Result<String, ClassFormatErr> {
        let name_index = self.get_class(idx)?;
        self.get_utf8(&name_index).map(|raw| raw.replace('/', "."))
    }

    pub fn get_javap_class_name_utf8(&self, idx: &u16) -> Result<String, ClassFormatErr> {
        self.get_utf8(idx).map(|raw| {
            if raw.starts_with('[') {
                format!("\"{raw}\"")
            } else {
                raw.to_string()
            }
        })
    }

    pub fn get_javap_class_name_for_cp_print(&self, idx: &u16) -> Result<String, ClassFormatErr> {
        let name_index = self.get_class(idx)?;
        self.get_javap_class_name_utf8(&name_index)
    }

    pub fn get_methodref(&self, idx: &u16) -> Result<&Reference, ClassFormatErr> {
        self.inner
            .get(*idx as usize)
            .ok_or(ClassFormatErr::ConstantNotFound(*idx))
            .and_then(|entry| match entry {
                ConstantEntry::MethodRef(ref_info) => Ok(ref_info),
                ConstantEntry::InterfaceMethodRef(ref_info) => Ok(ref_info),
                e => Err(ClassFormatErr::TypeError(
                    *idx,
                    ConstantKind::MethodRef.to_string(),
                    e.get_kind().to_string(),
                )),
            })
    }

    pub fn get_name_and_type(&self, idx: &u16) -> Result<&NameAndType, ClassFormatErr> {
        self.inner
            .get(*idx as usize)
            .ok_or(ClassFormatErr::ConstantNotFound(*idx))
            .and_then(|entry| match entry {
                ConstantEntry::NameAndType(ref_info) => Ok(ref_info),
                e => Err(ClassFormatErr::TypeError(
                    *idx,
                    ConstantKind::NameAndType.to_string(),
                    e.get_kind().to_string(),
                )),
            })
    }

    pub fn get_nat_name(&self, nat: &NameAndType) -> Result<&str, ClassFormatErr> {
        self.get_utf8(&nat.name_index)
    }

    pub fn get_nat_descriptor(&self, nat: &NameAndType) -> Result<&str, ClassFormatErr> {
        self.get_utf8(&nat.descriptor_index)
    }

    pub fn get_dyn_info_name(&self, dyn_info: &Dynamic) -> Result<&str, ClassFormatErr> {
        let nat = self.get_name_and_type(&dyn_info.name_and_type_index)?;
        self.get_nat_name(nat)
    }

    pub fn get_dyn_info_descriptor(&self, dyn_info: &Dynamic) -> Result<&str, ClassFormatErr> {
        let nat = self.get_name_and_type(&dyn_info.name_and_type_index)?;
        self.get_nat_descriptor(nat)
    }

    pub fn get_method_or_field_class_name(
        &self,
        method_ref: &Reference,
    ) -> Result<String, ClassFormatErr> {
        self.get_javap_class_name_for_cp_print(&method_ref.class_index)
    }

    pub fn get_method_or_field_name(&self, method_ref: &Reference) -> Result<&str, ClassFormatErr> {
        let nat_index = method_ref.name_and_type_index;
        let nat = self.get_name_and_type(&nat_index)?;
        self.get_nat_name(nat)
    }

    pub fn get_method_or_field_name_by_nat_idx(
        &self,
        nat_index: &u16,
    ) -> Result<&str, ClassFormatErr> {
        let nat = self.get_name_and_type(nat_index)?;
        self.get_nat_name(nat)
    }

    pub fn get_method_or_field_descriptor(
        &self,
        ref_info: &Reference,
    ) -> Result<&str, ClassFormatErr> {
        let nat_index = ref_info.name_and_type_index;
        let desc_index = self.get_name_and_type(&nat_index)?;
        self.get_nat_descriptor(desc_index)
    }
}

impl ConstantEntry {
    //TODO: check, don't want to spend too much time here, it is AI generated
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

    pub(crate) fn javap_fmt(
        &self,
        ind: &mut Indented,
        cp: &ConstantPool,
    ) -> Result<(), ClassFormatErr> {
        let op_w = 16;
        match self {
            ConstantEntry::Utf8(s) => writeln!(ind, "{}", s.escape_debug())?,
            ConstantEntry::Integer(i) => writeln!(ind, "{i}")?,
            ConstantEntry::Float(fl) => writeln!(ind, "{}", Self::format_float_minimal_javap(*fl))?,
            ConstantEntry::Long(l) => writeln!(ind, "{l}l")?,
            ConstantEntry::Double(d) => writeln!(ind, "{}", Self::format_double_minimal_javap(*d))?,
            ConstantEntry::String(index) => writeln!(
                ind,
                "{:<op_w$} // {}",
                format!("#{index}"),
                cp.get_printable_utf8(index)?,
                op_w = op_w
            )?,
            ConstantEntry::Class(index) => writeln!(
                ind,
                "{:<op_w$} // {}",
                format!("#{index}"),
                cp.get_javap_class_name_utf8(index)?,
                op_w = op_w
            )?,
            ConstantEntry::MethodRef(ref_info) | ConstantEntry::InterfaceMethodRef(ref_info) => {
                writeln!(
                    ind,
                    "{:<op_w$} // {}.{}:{}",
                    format!(
                        "#{}.#{}",
                        ref_info.class_index, ref_info.name_and_type_index
                    ),
                    cp.get_method_or_field_class_name(ref_info)?,
                    fmt_method_name(cp.get_method_or_field_name(ref_info)?),
                    cp.get_method_or_field_descriptor(ref_info)?,
                    op_w = op_w
                )?
            }
            ConstantEntry::NameAndType(nat) => writeln!(
                ind,
                "{:<op_w$} // {}:{}",
                format!("#{}:#{}", nat.name_index, nat.descriptor_index),
                fmt_method_name(cp.get_nat_name(nat)?),
                cp.get_nat_descriptor(nat)?,
                op_w = op_w
            )?,
            ConstantEntry::FieldRef(ref_info) => writeln!(
                ind,
                "{:<op_w$} // {}.{}:{}",
                format!(
                    "#{}.#{}",
                    ref_info.class_index, ref_info.name_and_type_index
                ),
                cp.get_class_name(&ref_info.class_index)?,
                cp.get_method_or_field_name(ref_info)?,
                cp.get_method_or_field_descriptor(ref_info)?,
                op_w = op_w
            )?,
            ConstantEntry::Dynamic(dyn_info) | ConstantEntry::InvokeDynamic(dyn_info) => writeln!(
                ind,
                "{:<op_w$} // #{}:{}:{}",
                format!(
                    "#{}:#{}",
                    dyn_info.bootstrap_method_attr_index, dyn_info.name_and_type_index
                ),
                dyn_info.bootstrap_method_attr_index,
                cp.get_dyn_info_name(dyn_info)?,
                cp.get_dyn_info_descriptor(dyn_info)?,
                op_w = op_w
            )?,
            ConstantEntry::MethodType(idx) => writeln!(
                ind,
                "{:<op_w$} // {}",
                format!("#{}", idx),
                cp.get_utf8(idx)?,
                op_w = op_w
            )?,
            ConstantEntry::MethodHandle(handle_info) => {
                let handle_kind = handle_info.get_kind()?;
                let method_ref = cp.get_methodref(&handle_info.reference_index)?;
                writeln!(
                    ind,
                    "{:<op_w$} // {} {}.{}:{}",
                    format!(
                        "{}:#{}",
                        handle_info.reference_kind, handle_info.reference_index
                    ),
                    handle_kind.get_javap_value()?,
                    cp.get_method_or_field_class_name(method_ref)?,
                    fmt_method_name(cp.get_method_or_field_name(method_ref)?),
                    cp.get_method_or_field_descriptor(method_ref)?,
                    op_w = op_w
                )?
            }
            e => todo!("Pretty print not implemented for {e:?}"),
        }
        Ok(())
    }

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

impl MethodHandleKind {
    pub(crate) fn get_javap_value(&self) -> Result<&str, ClassFormatErr> {
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
