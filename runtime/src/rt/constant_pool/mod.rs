use crate::rt::constant_pool::error::RuntimePoolError;
use crate::rt::constant_pool::reference::{
    ClassReference, FieldDescriptorReference, FieldReference, MethodDescriptorReference,
    MethodReference, NameAndTypeReference, StringReference,
};
use class_file::constant::{ConstantInfo, ReferenceInfo};
use common::descriptor::MethodDescriptor;
use common::jtype::Type;
use dashmap::DashMap;
use std::sync::Arc;

pub mod error;
pub mod reference;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RuntimeConstantType {
    Unused,
    Utf8,
    Integer,
    Float,
    Long,
    Double,
    Class,
    String,
    MethodRef,
    FieldRef,
    InterfaceMethodRef,
    NameAndType,
    MethodNameAndType,
    FieldNameAndType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeConstant {
    Unused,
    Utf8(Arc<String>),
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Class(Arc<ClassReference>),
    String(Arc<StringReference>),
    MethodRef(Arc<MethodReference>),
    FieldRef(Arc<FieldReference>),
    InterfaceMethodRef(Arc<ReferenceInfo>), //TODO: stub
    NameAndType(Arc<NameAndTypeReference>),
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
            RuntimeConstant::MethodRef(_) => RuntimeConstantType::MethodRef,
            RuntimeConstant::FieldRef(_) => RuntimeConstantType::FieldRef,
            RuntimeConstant::InterfaceMethodRef(_) => RuntimeConstantType::InterfaceMethodRef,
            RuntimeConstant::NameAndType(_) => RuntimeConstantType::NameAndType,
        }
    }
}

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-5.html#jvms-5.1
#[derive(Debug)]
pub struct RuntimeConstantPool {
    entries: Vec<RuntimeConstant>,
    method_descriptors: DashMap<u16, Arc<MethodDescriptorReference>>,
    field_descriptors: DashMap<u16, Arc<FieldDescriptorReference>>,
}

//TODO: change u16 idx as param to &u16
impl RuntimeConstantPool {
    pub fn new(entries: Vec<ConstantInfo>) -> Self {
        Self {
            entries: entries
                .into_iter()
                .enumerate()
                .map(|(i, c)| match c {
                    ConstantInfo::Unused => RuntimeConstant::Unused,
                    ConstantInfo::Utf8(val) => RuntimeConstant::Utf8(Arc::new(val)),
                    ConstantInfo::Integer(val) => RuntimeConstant::Integer(val),
                    ConstantInfo::Float(val) => RuntimeConstant::Float(val),
                    ConstantInfo::Long(val) => RuntimeConstant::Long(val),
                    ConstantInfo::Double(val) => RuntimeConstant::Double(val),
                    ConstantInfo::Class(idx) => {
                        RuntimeConstant::Class(Arc::new(ClassReference::new(i as u16, idx)))
                    }
                    ConstantInfo::String(idx) => {
                        RuntimeConstant::String(Arc::new(StringReference::new(i as u16, idx)))
                    }
                    ConstantInfo::MethodRef(method_ref) => {
                        RuntimeConstant::MethodRef(Arc::new(MethodReference::new(
                            i as u16,
                            method_ref.class_index,
                            method_ref.name_and_type_index,
                        )))
                    }
                    ConstantInfo::FieldRef(field_ref) => {
                        RuntimeConstant::FieldRef(Arc::new(FieldReference::new(
                            i as u16,
                            field_ref.class_index,
                            field_ref.name_and_type_index,
                        )))
                    }
                    ConstantInfo::NameAndType(nat) => RuntimeConstant::NameAndType(Arc::new(
                        NameAndTypeReference::new(i as u16, nat.name_index, nat.descriptor_index),
                    )),
                    ConstantInfo::InterfaceMethodRef(ref_info) => {
                        RuntimeConstant::InterfaceMethodRef(Arc::new(ref_info))
                    }
                    other => unimplemented!("{:?}", other),
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

    pub fn get_utf8(&self, idx: &u16) -> Result<&Arc<String>, RuntimePoolError> {
        match self.entry(idx)? {
            RuntimeConstant::Utf8(string) => Ok(string),
            other => Err(RuntimePoolError::TypeError(
                *idx,
                RuntimeConstantType::Utf8,
                other.get_type(),
            )),
        }
    }

    pub fn get_string(&self, idx: &u16) -> Result<&Arc<StringReference>, RuntimePoolError> {
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

    pub fn get_class(&self, idx: &u16) -> Result<&Arc<ClassReference>, RuntimePoolError> {
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

    pub fn get_class_name(&self, idx: &u16) -> Result<&Arc<String>, RuntimePoolError> {
        Ok(self.get_class(idx)?.name()?)
    }

    //TODO: all other getters return ref, but this one can't cause of dash map
    pub fn get_method_descriptor(
        &self,
        idx: &u16,
    ) -> Result<Arc<MethodDescriptorReference>, RuntimePoolError> {
        if let Some(descriptor) = self.method_descriptors.get(&idx) {
            return Ok(descriptor.clone());
        }
        let raw = self.get_utf8(idx)?.clone();
        let resolved = MethodDescriptor::try_from(raw.as_str())?;
        let reference = Arc::new(MethodDescriptorReference::new(*idx, raw, resolved));
        self.method_descriptors.insert(*idx, reference.clone());
        Ok(reference)
    }

    //TODO: all other getters return ref, but this one can't cause of dash map
    pub fn get_field_descriptor(
        &self,
        idx: &u16,
    ) -> Result<Arc<FieldDescriptorReference>, RuntimePoolError> {
        if let Some(descriptor) = self.field_descriptors.get(&idx) {
            return Ok(descriptor.clone());
        }
        let raw = self.get_utf8(idx)?.clone();
        let resolved = Type::try_from(raw.as_str())?;
        let reference = Arc::new(FieldDescriptorReference::new(*idx, raw, resolved));
        self.field_descriptors.insert(*idx, reference.clone());
        Ok(reference)
    }

    pub fn get_method_nat(
        &self,
        idx: &u16,
    ) -> Result<&Arc<NameAndTypeReference>, RuntimePoolError> {
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

    pub fn get_field_nat(&self, idx: &u16) -> Result<&Arc<NameAndTypeReference>, RuntimePoolError> {
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

    pub fn get_methodref(&self, idx: &u16) -> Result<&Arc<MethodReference>, RuntimePoolError> {
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

    pub fn get_fieldref(&self, idx: &u16) -> Result<&Arc<FieldReference>, RuntimePoolError> {
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
