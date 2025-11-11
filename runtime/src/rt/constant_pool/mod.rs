use crate::Symbol;
use crate::rt::constant_pool::entry::{
    ClassEntry, FieldEntry, FieldEntryView, MethodEntry, MethodEntryView, NameAndTypeEntry,
    NameAndTypeEntryView, StringEntry, Utf8Entry,
};
use common::error::{JvmError, RuntimePoolError};
use jclass::constant::ConstantInfo;
use lasso::ThreadedRodeo;
use std::fmt::Display;

pub mod bootstrap_registry;
pub mod entry;

pub enum RuntimeConstantType {
    Unused,
    Utf8,
    Integer,
    Float,
    Long,
    Double,
    Class,
    String,
    Method,
    Field,
    InvokeDynamic,
    InterfaceMethod,
    NameAndType,
    MethodNameAndType,
    FieldNameAndType,
    MethodType,
    MethodHandle,
}

impl Display for RuntimeConstantType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_str = match self {
            RuntimeConstantType::Unused => "Unused",
            RuntimeConstantType::Utf8 => "Utf8",
            RuntimeConstantType::Integer => "Integer",
            RuntimeConstantType::Float => "Float",
            RuntimeConstantType::Long => "Long",
            RuntimeConstantType::Double => "Double",
            RuntimeConstantType::Class => "Class",
            RuntimeConstantType::String => "String",
            RuntimeConstantType::Method => "Method",
            RuntimeConstantType::Field => "Field",
            RuntimeConstantType::InvokeDynamic => "InvokeDynamic",
            RuntimeConstantType::InterfaceMethod => "InterfaceMethod",
            RuntimeConstantType::NameAndType => "NameAndType",
            RuntimeConstantType::MethodNameAndType => "MethodNameAndType",
            RuntimeConstantType::FieldNameAndType => "FieldNameAndType",
            RuntimeConstantType::MethodType => "MethodType",
            RuntimeConstantType::MethodHandle => "MethodHandle",
        };
        write!(f, "{}", type_str)
    }
}

pub enum RuntimeConstant {
    Unused,
    Utf8(Utf8Entry),
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Class(ClassEntry),
    String(StringEntry),
    Method(MethodEntry),
    Field(FieldEntry),
    InvokeDynamic,                /*(Arc<InvokeDynamicReferenceDeprecated>)*/
    InterfaceMethod(MethodEntry), /*(Arc<MethodReferenceDeprecated>)*/
    NameAndType(NameAndTypeEntry),
    MethodType,   /*(Arc<MethodTypeReferenceDeprecated>)*/
    MethodHandle, /*(Arc<MethodHandleReferenceDeprecated>)*/
}

impl RuntimeConstant {
    pub fn get_type(&self) -> RuntimeConstantType {
        match self {
            RuntimeConstant::Unused => RuntimeConstantType::Unused,
            RuntimeConstant::Utf8(_) => RuntimeConstantType::Utf8,
            RuntimeConstant::Integer(_) => RuntimeConstantType::Integer,
            RuntimeConstant::Float(_) => RuntimeConstantType::Float,
            RuntimeConstant::Long(_) => RuntimeConstantType::Long,
            RuntimeConstant::Double(_) => RuntimeConstantType::Double,
            RuntimeConstant::Class(_) => RuntimeConstantType::Class,
            RuntimeConstant::String(_) => RuntimeConstantType::String,
            RuntimeConstant::Method(_) => RuntimeConstantType::Method,
            RuntimeConstant::Field(_) => RuntimeConstantType::Field,
            RuntimeConstant::InterfaceMethod(_) => RuntimeConstantType::InterfaceMethod,
            RuntimeConstant::NameAndType(_) => RuntimeConstantType::NameAndType,
            RuntimeConstant::InvokeDynamic => RuntimeConstantType::InvokeDynamic,
            RuntimeConstant::MethodType => RuntimeConstantType::MethodType,
            RuntimeConstant::MethodHandle => RuntimeConstantType::MethodHandle,
        }
    }
}

pub struct RuntimeConstantPool {
    entries: Vec<RuntimeConstant>,
}

impl RuntimeConstantPool {
    pub fn new(entries: Vec<ConstantInfo>) -> Self {
        let mut rt_entries = Vec::with_capacity(entries.len());
        for entry in entries {
            let rt_entry = match entry {
                ConstantInfo::Unused => RuntimeConstant::Unused,
                ConstantInfo::Utf8(utf8) => RuntimeConstant::Utf8(Utf8Entry::new(utf8)),
                ConstantInfo::Integer(v) => RuntimeConstant::Integer(v),
                ConstantInfo::Float(v) => RuntimeConstant::Float(v),
                ConstantInfo::Long(v) => RuntimeConstant::Long(v),
                ConstantInfo::Double(v) => RuntimeConstant::Double(v),
                ConstantInfo::Class(idx) => RuntimeConstant::Class(ClassEntry::new(idx)),
                ConstantInfo::String(idx) => RuntimeConstant::String(StringEntry::new(idx)),
                ConstantInfo::MethodRef(ref_info) => RuntimeConstant::Method(MethodEntry::new(
                    ref_info.class_index,
                    ref_info.name_and_type_index,
                )),
                ConstantInfo::FieldRef(ref_info) => RuntimeConstant::Field(FieldEntry::new(
                    ref_info.class_index,
                    ref_info.name_and_type_index,
                )),
                ConstantInfo::NameAndType(nat_info) => RuntimeConstant::NameAndType(
                    NameAndTypeEntry::new(nat_info.name_index, nat_info.descriptor_index),
                ),
                ConstantInfo::InterfaceMethodRef(ref_info) => RuntimeConstant::InterfaceMethod(
                    MethodEntry::new(ref_info.class_index, ref_info.name_and_type_index),
                ),
                ConstantInfo::InvokeDynamic(_) => RuntimeConstant::InvokeDynamic,
                ConstantInfo::MethodType(_) => RuntimeConstant::MethodType,
                ConstantInfo::MethodHandle(_) => RuntimeConstant::MethodHandle,
                other => unimplemented!("{:?} not implemented yet", other),
            };
            rt_entries.push(rt_entry);
        }
        Self {
            entries: rt_entries,
        }
    }

    pub fn get_constant(
        &self,
        idx: &u16,
        interner: &ThreadedRodeo,
    ) -> Result<&RuntimeConstant, JvmError> {
        let entry = self.entry(idx)?;
        match entry {
            RuntimeConstant::Class(_) => {
                self.get_class_sym(idx, interner)?;
            }
            RuntimeConstant::String(_) => {
                self.get_string_sym(idx, interner)?;
            }
            RuntimeConstant::Method(_) => {
                self.get_method_view(idx, interner)?;
            }
            RuntimeConstant::Field(_) => {
                self.get_field_view(idx, interner)?;
            }
            _ => {}
        };
        Ok(entry)
    }

    fn entry(&self, idx: &u16) -> Result<&RuntimeConstant, RuntimePoolError> {
        self.entries
            .get(*idx as usize)
            .ok_or(RuntimePoolError::WrongIndex(*idx))
    }

    pub fn get_utf8_sym(
        &self,
        idx: &u16,
        interner: &ThreadedRodeo,
    ) -> Result<Symbol, RuntimePoolError> {
        match self.entry(idx)? {
            RuntimeConstant::Utf8(entry) => Ok(*entry
                .utf8_sym
                .get_or_init(|| interner.get_or_intern(&entry.value))),
            other => Err(RuntimePoolError::TypeError(
                *idx,
                RuntimeConstantType::Utf8.to_string(),
                other.get_type().to_string(),
            )),
        }
    }

    pub fn get_nat_view(
        &self,
        idx: &u16,
        interner: &ThreadedRodeo,
    ) -> Result<NameAndTypeEntryView, RuntimePoolError> {
        match self.entry(idx)? {
            RuntimeConstant::NameAndType(entry) => {
                let name_sym = *entry.name_sym.get_or_try_init::<_, RuntimePoolError>(|| {
                    self.get_utf8_sym(&entry.name_idx, interner)
                })?;
                let descriptor_sym = *entry
                    .descriptor_sym
                    //TODO: delete explicit type?
                    .get_or_try_init::<_, RuntimePoolError>(|| {
                        self.get_utf8_sym(&entry.descriptor_idx, interner)
                    })?;
                Ok(NameAndTypeEntryView::new(name_sym, descriptor_sym))
            }
            other => Err(RuntimePoolError::TypeError(
                *idx,
                RuntimeConstantType::MethodNameAndType.to_string(),
                other.get_type().to_string(),
            )),
        }
    }

    pub fn get_method_view(
        &self,
        idx: &u16,
        interner: &ThreadedRodeo,
    ) -> Result<MethodEntryView, RuntimePoolError> {
        match self.entry(idx)? {
            RuntimeConstant::Method(entry) => {
                let class_sym = *entry
                    .class_sym
                    .get_or_try_init(|| self.get_class_sym(&entry.class_idx, interner))?;
                let nat_view = self.get_nat_view(&entry.nat_idx, interner)?;
                Ok(MethodEntryView::new(class_sym, nat_view))
            }
            other => Err(RuntimePoolError::TypeError(
                *idx,
                RuntimeConstantType::Method.to_string(),
                other.get_type().to_string(),
            )),
        }
    }

    pub fn get_interface_method_view(
        &self,
        idx: &u16,
        interner: &ThreadedRodeo,
    ) -> Result<MethodEntryView, RuntimePoolError> {
        match self.entry(idx)? {
            RuntimeConstant::InterfaceMethod(entry) => {
                let class_sym = *entry
                    .class_sym
                    .get_or_try_init(|| self.get_class_sym(&entry.class_idx, interner))?;
                let nat_view = self.get_nat_view(&entry.nat_idx, interner)?;
                Ok(MethodEntryView::new(class_sym, nat_view))
            }
            other => Err(RuntimePoolError::TypeError(
                *idx,
                RuntimeConstantType::InterfaceMethod.to_string(),
                other.get_type().to_string(),
            )),
        }
    }

    pub fn get_field_view(
        &self,
        idx: &u16,
        interner: &ThreadedRodeo,
    ) -> Result<FieldEntryView, RuntimePoolError> {
        match self.entry(idx)? {
            RuntimeConstant::Field(entry) => {
                let class_sym = *entry
                    .class_sym
                    .get_or_try_init(|| self.get_class_sym(&entry.class_idx, interner))?;
                let nat_view = self.get_nat_view(&entry.nat_idx, interner)?;
                Ok(FieldEntryView::new(class_sym, nat_view))
            }
            other => Err(RuntimePoolError::TypeError(
                *idx,
                RuntimeConstantType::Field.to_string(),
                other.get_type().to_string(),
            )),
        }
    }

    pub fn get_string_sym(
        &self,
        idx: &u16,
        interner: &ThreadedRodeo,
    ) -> Result<Symbol, RuntimePoolError> {
        match self.entry(idx)? {
            RuntimeConstant::String(entry) => entry
                .string_sym
                .get_or_try_init::<_, RuntimePoolError>(|| {
                    self.get_utf8_sym(&entry.string_idx, interner)
                })
                .copied(),
            other => Err(RuntimePoolError::TypeError(
                *idx,
                RuntimeConstantType::String.to_string(),
                other.get_type().to_string(),
            )),
        }
    }

    pub fn get_class_sym(
        &self,
        idx: &u16,
        interner: &ThreadedRodeo,
    ) -> Result<Symbol, RuntimePoolError> {
        match self.entry(idx)? {
            RuntimeConstant::Class(entry) => entry
                .name_sym
                .get_or_try_init::<_, RuntimePoolError>(|| {
                    self.get_utf8_sym(&entry.name_idx, interner)
                })
                .copied(),
            other => Err(RuntimePoolError::TypeError(
                *idx,
                RuntimeConstantType::Class.to_string(),
                other.get_type().to_string(),
            )),
        }
    }
}
