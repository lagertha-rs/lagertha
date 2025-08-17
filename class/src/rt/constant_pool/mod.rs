use crate::class_file::constant_pool::{ConstantInfo, ReferenceInfo};
use crate::rt::constant_pool::error::RuntimePoolError;
use crate::rt::constant_pool::reference::{
    ClassReference, FieldDescriptorReference, FieldReference, MethodDescriptorReference,
    MethodReference, NameAndTypeReference, StringReference,
};
use crate::rt::descriptor::MethodDescriptor;
use crate::rt::jtype::Type;
use dashmap::DashMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

pub mod error;
pub mod reference;

#[derive(Debug, Clone)]
pub enum RuntimeConstantType {
    Dummy,
    Utf8,
    Integer,
    Float,
    Long,
    Double,
    Class,
    String,
    MethodRef,
    FieldRef,
    InterfaceRef,
    NameAndType,
    MethodNameAndType,
    FieldNameAndType,
}

#[derive(Debug, Clone)]
pub enum RuntimeConstant {
    Dummy,
    Utf8(Rc<String>),
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Class(Rc<ClassReference>),
    String(Rc<StringReference>),
    MethodRef(Rc<MethodReference>),
    FieldRef(Rc<FieldReference>),
    InterfaceRef(ReferenceInfo),
    NameAndType(Rc<NameAndTypeReference>),
}

impl RuntimeConstant {
    pub fn get_type(&self) -> RuntimeConstantType {
        match self {
            RuntimeConstant::Dummy => RuntimeConstantType::Dummy,
            RuntimeConstant::Utf8(_) => RuntimeConstantType::Utf8,
            RuntimeConstant::Integer(_) => RuntimeConstantType::Integer,
            RuntimeConstant::Float(_) => RuntimeConstantType::Float,
            RuntimeConstant::Long(_) => RuntimeConstantType::Long,
            RuntimeConstant::Double(_) => RuntimeConstantType::Double,
            RuntimeConstant::Class(_) => RuntimeConstantType::Class,
            RuntimeConstant::String(_) => RuntimeConstantType::String,
            RuntimeConstant::MethodRef(_) => RuntimeConstantType::MethodRef,
            RuntimeConstant::FieldRef(_) => RuntimeConstantType::FieldRef,
            RuntimeConstant::InterfaceRef(_) => RuntimeConstantType::InterfaceRef,
            RuntimeConstant::NameAndType(_) => RuntimeConstantType::NameAndType,
        }
    }
}

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-5.html#jvms-5.1
#[derive(Debug)]
pub struct RuntimeConstantPool {
    entries: Vec<RuntimeConstant>,
    method_descriptors: DashMap<u16, Rc<MethodDescriptorReference>>,
    field_descriptors: DashMap<u16, Rc<FieldDescriptorReference>>,
}

//TODO: change u16 idx as param to &u16
impl RuntimeConstantPool {
    pub fn new(entries: Vec<ConstantInfo>) -> Self {
        Self {
            entries: entries
                .into_iter()
                .enumerate()
                .map(|(i, c)| match c {
                    ConstantInfo::Dummy => RuntimeConstant::Dummy,
                    ConstantInfo::Utf8(val) => RuntimeConstant::Utf8(Rc::new(val)),
                    ConstantInfo::Integer(val) => RuntimeConstant::Integer(val),
                    ConstantInfo::Float(val) => RuntimeConstant::Float(val),
                    ConstantInfo::Long(val) => RuntimeConstant::Long(val),
                    ConstantInfo::Double(val) => RuntimeConstant::Double(val),
                    ConstantInfo::Class(idx) => {
                        RuntimeConstant::Class(Rc::new(ClassReference::new(i as u16, idx)))
                    }
                    ConstantInfo::String(idx) => {
                        RuntimeConstant::String(Rc::new(StringReference::new(i as u16, idx)))
                    }
                    ConstantInfo::MethodRef(method_ref) => {
                        RuntimeConstant::MethodRef(Rc::new(MethodReference::new(
                            i as u16,
                            method_ref.class_index,
                            method_ref.name_and_type_index,
                        )))
                    }
                    ConstantInfo::FieldRef(field_ref) => {
                        RuntimeConstant::FieldRef(Rc::new(FieldReference::new(
                            i as u16,
                            field_ref.class_index,
                            field_ref.name_and_type_index,
                        )))
                    }
                    ConstantInfo::InterfaceRef(v) => RuntimeConstant::InterfaceRef(v),
                    ConstantInfo::NameAndType(nat) => RuntimeConstant::NameAndType(Rc::new(
                        NameAndTypeReference::new(i as u16, nat.name_index, nat.descriptor_index),
                    )),
                })
                .collect(),
            method_descriptors: DashMap::new(),
            field_descriptors: DashMap::new(),
        }
    }

    fn entry(&self, idx: &u16) -> Result<&RuntimeConstant, RuntimePoolError> {
        self.entries
            .get(*idx as usize)
            .ok_or(RuntimePoolError::WrongIndex(*idx))
    }

    /*TODO: as far as I understand it will be needed for ldc opcode, now use only for display.
     * get are called to resolve references before return it. does it make sense to have separate
     * resolve method, instead of using getters?
     * And seems I can't resolve nat here, cause IDK it is field or method, so probably it should
     * not be named as get, but more specific for ldc opcode etc...
     */
    pub fn get(&self, idx: &u16) -> Result<&RuntimeConstant, RuntimePoolError> {
        let entry = self.entry(idx)?;
        match entry {
            RuntimeConstant::Class(_) => {
                self.get_class(idx)?;
            }
            RuntimeConstant::String(_) => {
                self.get_string(idx)?;
            }
            RuntimeConstant::MethodRef(_) => {
                self.get_methodref(idx)?;
            }
            RuntimeConstant::FieldRef(_) => {
                self.get_fieldref(idx)?;
            }
            RuntimeConstant::NameAndType(_) => unimplemented!(),
            _ => {}
        };
        Ok(entry)
    }

    pub fn get_utf8(&self, idx: &u16) -> Result<&Rc<String>, RuntimePoolError> {
        match self.entry(idx)? {
            RuntimeConstant::Utf8(string) => Ok(string),
            other => Err(RuntimePoolError::TypeError(
                *idx,
                RuntimeConstantType::Utf8,
                other.get_type(),
            )),
        }
    }

    pub fn get_string(&self, idx: &u16) -> Result<&Rc<StringReference>, RuntimePoolError> {
        match self.entry(idx)? {
            RuntimeConstant::String(str_ref) => {
                str_ref.value.get_or_try_init::<_, RuntimePoolError>(|| {
                    Ok(self.get_utf8(str_ref.string_index())?.clone())
                })?;
                Ok(str_ref)
            }
            other => Err(RuntimePoolError::TypeError(
                *idx,
                RuntimeConstantType::String,
                other.get_type(),
            )),
        }
    }

    pub fn get_class(&self, idx: &u16) -> Result<&Rc<ClassReference>, RuntimePoolError> {
        match self.entry(idx)? {
            RuntimeConstant::Class(class_ref) => {
                class_ref.name.get_or_try_init::<_, RuntimePoolError>(|| {
                    Ok(self.get_utf8(class_ref.name_index())?.clone())
                })?;
                Ok(class_ref)
            }
            other => Err(RuntimePoolError::TypeError(
                *idx,
                RuntimeConstantType::Class,
                other.get_type(),
            )),
        }
    }

    //TODO: all other getters return ref, but this one can't cause of dash map
    pub fn get_method_descriptor(
        &self,
        idx: &u16,
    ) -> Result<Rc<MethodDescriptorReference>, RuntimePoolError> {
        if let Some(descriptor) = self.method_descriptors.get(&idx) {
            return Ok(descriptor.clone());
        }
        let raw = self.get_utf8(idx)?.clone();
        let resolved = MethodDescriptor::try_from(raw.as_str())?;
        let reference = Rc::new(MethodDescriptorReference::new(*idx, raw, resolved));
        self.method_descriptors.insert(*idx, reference.clone());
        Ok(reference)
    }

    //TODO: all other getters return ref, but this one can't cause of dash map
    pub fn get_field_descriptor(
        &self,
        idx: &u16,
    ) -> Result<Rc<FieldDescriptorReference>, RuntimePoolError> {
        if let Some(descriptor) = self.field_descriptors.get(&idx) {
            return Ok(descriptor.clone());
        }
        let raw = self.get_utf8(idx)?.clone();
        let resolved = Type::try_from(raw.as_str())?;
        let reference = Rc::new(FieldDescriptorReference::new(*idx, raw, resolved));
        self.field_descriptors.insert(*idx, reference.clone());
        Ok(reference)
    }

    pub fn get_method_nat(&self, idx: &u16) -> Result<&Rc<NameAndTypeReference>, RuntimePoolError> {
        match self.entry(idx)? {
            RuntimeConstant::NameAndType(method_nat) => {
                method_nat.name.get_or_try_init::<_, RuntimePoolError>(|| {
                    Ok(self.get_utf8(method_nat.name_index())?.clone())
                })?;
                method_nat
                    .method_descriptor
                    .get_or_try_init::<_, RuntimePoolError>(|| {
                        self.get_method_descriptor(method_nat.descriptor_index())
                    })?;
                Ok(method_nat)
            }
            other => Err(RuntimePoolError::TypeError(
                *idx,
                RuntimeConstantType::MethodNameAndType,
                other.get_type(),
            )),
        }
    }

    pub fn get_field_nat(&self, idx: &u16) -> Result<&Rc<NameAndTypeReference>, RuntimePoolError> {
        match self.entry(idx)? {
            RuntimeConstant::NameAndType(field_nat) => {
                field_nat.name.get_or_try_init::<_, RuntimePoolError>(|| {
                    Ok(self.get_utf8(field_nat.name_index())?.clone())
                })?;
                field_nat
                    .field_descriptor
                    .get_or_try_init::<_, RuntimePoolError>(|| {
                        self.get_field_descriptor(field_nat.descriptor_index())
                    })?;
                Ok(field_nat)
            }
            other => Err(RuntimePoolError::TypeError(
                *idx,
                RuntimeConstantType::FieldNameAndType,
                other.get_type(),
            )),
        }
    }

    pub fn get_methodref(&self, idx: &u16) -> Result<&Rc<MethodReference>, RuntimePoolError> {
        match self.entry(idx)? {
            RuntimeConstant::MethodRef(method_ref) => {
                method_ref.class.get_or_try_init(|| {
                    Ok::<_, RuntimePoolError>(self.get_class(method_ref.class_index())?.clone())
                })?;
                method_ref
                    .name_and_type
                    .get_or_try_init::<_, RuntimePoolError>(|| {
                        Ok(self
                            .get_method_nat(method_ref.name_and_type_index())?
                            .clone())
                    })?;
                Ok(method_ref)
            }
            other => Err(RuntimePoolError::TypeError(
                *idx,
                RuntimeConstantType::MethodRef,
                other.get_type(),
            )),
        }
    }

    pub fn get_fieldref(&self, idx: &u16) -> Result<&Rc<FieldReference>, RuntimePoolError> {
        match self.entry(idx)? {
            RuntimeConstant::FieldRef(field_ref) => {
                field_ref.class.get_or_try_init::<_, RuntimePoolError>(|| {
                    Ok(self.get_class(field_ref.class_index())?.clone())
                })?;
                field_ref
                    .name_and_type
                    .get_or_try_init::<_, RuntimePoolError>(|| {
                        Ok(self.get_field_nat(field_ref.name_and_type_index())?.clone())
                    })?;
                Ok(field_ref)
            }
            other => Err(RuntimePoolError::TypeError(
                *idx,
                RuntimeConstantType::FieldRef,
                other.get_type(),
            )),
        }
    }
}

impl fmt::Display for RuntimeConstant {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeConstant::Dummy => {} //TODO
            RuntimeConstant::Utf8(val) => write!(f, "Utf8 {val}")?,
            RuntimeConstant::Integer(val) => write!(f, "Integer {val}")?,
            RuntimeConstant::Float(val) => write!(f, "Float {val}")?,
            RuntimeConstant::Long(val) => write!(f, "Long {val}")?,
            RuntimeConstant::Double(val) => write!(f, "Double {val}")?,
            RuntimeConstant::Class(_val) => write!(f, "Class")?, //TODO
            RuntimeConstant::String(val) => write!(f, "String {val}")?,
            RuntimeConstant::MethodRef(val) => write!(f, "Method {val}")?,
            RuntimeConstant::FieldRef(val) => write!(f, "Field {val}")?,
            RuntimeConstant::InterfaceRef(_val) => write!(f, "Interface")?, //TODO
            RuntimeConstant::NameAndType(_val) => write!(f, "NameAndType")?, //TODO
        }
        Ok(())
    }
}

impl fmt::Display for RuntimeConstantPool {
    // copying -v arg
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let digits = (self.entries.len().saturating_sub(1))
            .to_string()
            .len()
            .max(1);
        let idx_w = digits + 1;
        let kind_w = 16;
        let op_w = 16;

        macro_rules! try_map {
            ($e:expr) => {
                $e.map_err(|_| fmt::Error)
            };
        }

        for (pos, entry) in self.entries.iter().enumerate().skip(1) {
            write!(f, "  {p:>w$} = ", p = format!("#{}", pos), w = idx_w)?;

            match entry {
                RuntimeConstant::Utf8(s) => {
                    writeln!(
                        f,
                        "{:<kind_w$} {:<op_w$}",
                        "Utf8",
                        s,
                        kind_w = kind_w,
                        op_w = op_w
                    )?;
                }

                RuntimeConstant::Class(class) => {
                    let name = try_map!(self.get_utf8(class.name_index()))?;
                    writeln!(
                        f,
                        "{:<kind_w$} {:<op_w$} // {}",
                        "Class",
                        format!("#{}", class.name_index()),
                        name,
                        kind_w = kind_w,
                        op_w = op_w
                    )?;
                }

                RuntimeConstant::String(sr) => {
                    let val = try_map!(self.get_utf8(sr.string_index()))?;
                    writeln!(
                        f,
                        "{:<kind_w$} {:<op_w$} // {}",
                        "String",
                        format!("#{}", sr.string_index()),
                        val,
                        kind_w = kind_w,
                        op_w = op_w
                    )?;
                }

                RuntimeConstant::MethodRef(mr) => {
                    let class = try_map!(self.get_class(mr.class_index()))?;
                    let nat = try_map!(self.get_method_nat(mr.name_and_type_index()))?;
                    let cls_name = class.name.get().ok_or(fmt::Error)?;
                    let name = try_map!(self.get_utf8(nat.name_index()))?;
                    let desc = &nat.method_descriptor.get().ok_or(fmt::Error)?.raw();

                    writeln!(
                        f,
                        "{:<kind_w$} {:<op_w$} // {}.{}:{}",
                        "Methodref",
                        format!("#{}.#{}", mr.class_index(), mr.name_and_type_index()),
                        cls_name,
                        name,
                        desc,
                        kind_w = kind_w,
                        op_w = op_w
                    )?;
                }

                RuntimeConstant::FieldRef(fr) => {
                    let class = try_map!(self.get_class(fr.class_index()))?;
                    let nat = try_map!(self.get_field_nat(fr.name_and_type_index()))?;
                    let cls_name = class.name.get().ok_or(fmt::Error)?;
                    let name = try_map!(self.get_utf8(nat.name_index()))?;
                    let desc = &nat.field_descriptor.get().ok_or(fmt::Error)?.raw();
                    writeln!(
                        f,
                        "{:<kind_w$} {:<op_w$} // {}.{}:{}",
                        "Fieldref",
                        format!("#{}.#{}", fr.class_index(), fr.name_and_type_index()),
                        cls_name,
                        name,
                        desc,
                        kind_w = kind_w,
                        op_w = op_w
                    )?;
                }

                RuntimeConstant::NameAndType(nat) => {
                    let name = try_map!(self.get_utf8(nat.name_index()))?;
                    let desc = try_map!(self.get_utf8(nat.descriptor_index()))?;
                    writeln!(
                        f,
                        "{:<kind_w$} {:<op_w$} // {}:{}",
                        "NameAndType",
                        format!("#{}.#{}", nat.name_index(), nat.descriptor_index()),
                        name,
                        desc,
                        kind_w = kind_w,
                        op_w = op_w
                    )?;
                }

                other => {
                    writeln!(
                        f,
                        "{:<kind_w$} {:<op_w$}",
                        "/* TODO */",
                        format!("{:?}", other),
                        kind_w = kind_w,
                        op_w = op_w
                    )?;
                }
            }
        }
        Ok(())
    }
}
