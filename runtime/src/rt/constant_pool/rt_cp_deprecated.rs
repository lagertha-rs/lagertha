use crate::rt::constant_pool::reference_deprecated::{
    ClassReferenceDeprecated, FieldDescriptorReferenceDeprecated, FieldReferenceDeprecated,
    InvokeDynamicReferenceDeprecated, MethodDescriptorReferenceDeprecated,
    MethodHandleReferenceDeprecated, MethodReferenceDeprecated, MethodTypeReferenceDeprecated,
    NameAndTypeReferenceDeprecated, StringReferenceDeprecated,
};
use common::descriptor::MethodDescriptor;
use common::error::RuntimePoolError;
use common::jtype::DescriptorType;
use dashmap::DashMap;
use jclass::constant::ConstantInfo;
use std::fmt::Display;
use std::sync::Arc;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RuntimeConstantTypeDeprecated {
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
    InvokeDynamicRef,
    InterfaceMethodRef,
    NameAndType,
    MethodNameAndType,
    FieldNameAndType,
    MethodTypeRef,
    MethodHandleRef,
}

impl Display for RuntimeConstantTypeDeprecated {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_str = match self {
            RuntimeConstantTypeDeprecated::Unused => "Unused",
            RuntimeConstantTypeDeprecated::Utf8 => "Utf8",
            RuntimeConstantTypeDeprecated::Integer => "Integer",
            RuntimeConstantTypeDeprecated::Float => "Float",
            RuntimeConstantTypeDeprecated::Long => "Long",
            RuntimeConstantTypeDeprecated::Double => "Double",
            RuntimeConstantTypeDeprecated::Class => "Class",
            RuntimeConstantTypeDeprecated::String => "String",
            RuntimeConstantTypeDeprecated::MethodRef => "MethodRef",
            RuntimeConstantTypeDeprecated::FieldRef => "FieldRef",
            RuntimeConstantTypeDeprecated::InvokeDynamicRef => "InvokeDynamicRef",
            RuntimeConstantTypeDeprecated::InterfaceMethodRef => "InterfaceMethodRef",
            RuntimeConstantTypeDeprecated::NameAndType => "NameAndType",
            RuntimeConstantTypeDeprecated::MethodNameAndType => "MethodNameAndType",
            RuntimeConstantTypeDeprecated::FieldNameAndType => "FieldNameAndType",
            RuntimeConstantTypeDeprecated::MethodTypeRef => "MethodTypeRef",
            RuntimeConstantTypeDeprecated::MethodHandleRef => "MethodHandleRef",
        };
        write!(f, "{}", type_str)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeConstantDeprecated {
    Unused,
    Utf8(Arc<str>),
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Class(Arc<ClassReferenceDeprecated>),
    String(Arc<StringReferenceDeprecated>),
    MethodRef(Arc<MethodReferenceDeprecated>),
    FieldRef(Arc<FieldReferenceDeprecated>),
    InvokeDynamicRef(Arc<InvokeDynamicReferenceDeprecated>),
    InterfaceMethodRef(Arc<MethodReferenceDeprecated>),
    NameAndType(Arc<NameAndTypeReferenceDeprecated>),
    MethodType(Arc<MethodTypeReferenceDeprecated>),
    MethodHandleRef(Arc<MethodHandleReferenceDeprecated>),
}

impl RuntimeConstantDeprecated {
    pub fn get_type(&self) -> RuntimeConstantTypeDeprecated {
        match self {
            RuntimeConstantDeprecated::Unused => RuntimeConstantTypeDeprecated::Unused,
            RuntimeConstantDeprecated::Utf8(_) => RuntimeConstantTypeDeprecated::Utf8,
            RuntimeConstantDeprecated::Integer(_) => RuntimeConstantTypeDeprecated::Integer,
            RuntimeConstantDeprecated::Float(_) => RuntimeConstantTypeDeprecated::Float,
            RuntimeConstantDeprecated::Long(_) => RuntimeConstantTypeDeprecated::Long,
            RuntimeConstantDeprecated::Double(_) => RuntimeConstantTypeDeprecated::Double,
            RuntimeConstantDeprecated::Class(_) => RuntimeConstantTypeDeprecated::Class,
            RuntimeConstantDeprecated::String(_) => RuntimeConstantTypeDeprecated::String,
            RuntimeConstantDeprecated::MethodRef(_) => RuntimeConstantTypeDeprecated::MethodRef,
            RuntimeConstantDeprecated::FieldRef(_) => RuntimeConstantTypeDeprecated::FieldRef,
            RuntimeConstantDeprecated::InterfaceMethodRef(_) => {
                RuntimeConstantTypeDeprecated::InterfaceMethodRef
            }
            RuntimeConstantDeprecated::NameAndType(_) => RuntimeConstantTypeDeprecated::NameAndType,
            RuntimeConstantDeprecated::InvokeDynamicRef(_) => {
                RuntimeConstantTypeDeprecated::InvokeDynamicRef
            }
            RuntimeConstantDeprecated::MethodType(_) => {
                RuntimeConstantTypeDeprecated::MethodTypeRef
            }
            RuntimeConstantDeprecated::MethodHandleRef(_) => {
                RuntimeConstantTypeDeprecated::MethodHandleRef
            }
        }
    }
}

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-5.html#jvms-5.1
#[derive(Debug)]
pub struct RuntimeConstantPoolDeprecated {
    entries: Vec<RuntimeConstantDeprecated>,
    method_descriptors: DashMap<u16, Arc<MethodDescriptorReferenceDeprecated>>,
    field_descriptors: DashMap<u16, Arc<FieldDescriptorReferenceDeprecated>>,
}

//TODO: change u16 idx as param to &u16
impl RuntimeConstantPoolDeprecated {
    pub fn new(entries: Vec<ConstantInfo>) -> Self {
        Self {
            entries: entries
                .into_iter()
                .enumerate()
                .map(|(i, c)| match c {
                    ConstantInfo::Unused => RuntimeConstantDeprecated::Unused,
                    ConstantInfo::Utf8(val) => RuntimeConstantDeprecated::Utf8(Arc::from(val)),
                    ConstantInfo::Integer(val) => RuntimeConstantDeprecated::Integer(val),
                    ConstantInfo::Float(val) => RuntimeConstantDeprecated::Float(val),
                    ConstantInfo::Long(val) => RuntimeConstantDeprecated::Long(val),
                    ConstantInfo::Double(val) => RuntimeConstantDeprecated::Double(val),
                    ConstantInfo::Class(idx) => RuntimeConstantDeprecated::Class(Arc::new(
                        ClassReferenceDeprecated::new(i as u16, idx),
                    )),
                    ConstantInfo::String(idx) => RuntimeConstantDeprecated::String(Arc::new(
                        StringReferenceDeprecated::new(i as u16, idx),
                    )),
                    ConstantInfo::MethodRef(method_ref) => RuntimeConstantDeprecated::MethodRef(
                        Arc::new(MethodReferenceDeprecated::new(
                            i as u16,
                            method_ref.class_index,
                            method_ref.name_and_type_index,
                        )),
                    ),
                    ConstantInfo::FieldRef(field_ref) => RuntimeConstantDeprecated::FieldRef(
                        Arc::new(FieldReferenceDeprecated::new(
                            i as u16,
                            field_ref.class_index,
                            field_ref.name_and_type_index,
                        )),
                    ),
                    ConstantInfo::NameAndType(nat) => RuntimeConstantDeprecated::NameAndType(
                        Arc::new(NameAndTypeReferenceDeprecated::new(
                            i as u16,
                            nat.name_index,
                            nat.descriptor_index,
                        )),
                    ),
                    ConstantInfo::InterfaceMethodRef(method_ref) => {
                        RuntimeConstantDeprecated::InterfaceMethodRef(Arc::new(
                            MethodReferenceDeprecated::new(
                                i as u16,
                                method_ref.class_index,
                                method_ref.name_and_type_index,
                            ),
                        ))
                    }
                    ConstantInfo::InvokeDynamic(dyn_info) => {
                        RuntimeConstantDeprecated::InvokeDynamicRef(Arc::new(
                            InvokeDynamicReferenceDeprecated::new(
                                dyn_info.bootstrap_method_attr_index,
                                dyn_info.name_and_type_index,
                            ),
                        ))
                    }
                    ConstantInfo::MethodType(descriptor_index) => {
                        RuntimeConstantDeprecated::MethodType(Arc::new(
                            MethodTypeReferenceDeprecated::new(descriptor_index),
                        ))
                    }
                    ConstantInfo::MethodHandle(handle) => {
                        RuntimeConstantDeprecated::MethodHandleRef(Arc::new(
                            MethodHandleReferenceDeprecated::new(
                                handle.reference_kind,
                                handle.reference_index,
                            ),
                        ))
                    }
                    other => unimplemented!("{:?}", other),
                })
                .collect(),
            method_descriptors: DashMap::new(),
            field_descriptors: DashMap::new(),
        }
    }

    fn entry(&self, idx: &u16) -> Result<&RuntimeConstantDeprecated, RuntimePoolError> {
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
    pub fn get(&self, idx: &u16) -> Result<&RuntimeConstantDeprecated, RuntimePoolError> {
        let entry = self.entry(idx)?;
        match entry {
            RuntimeConstantDeprecated::Class(_) => {
                self.get_class(idx)?;
            }
            RuntimeConstantDeprecated::String(_) => {
                self.get_string(idx)?;
            }
            RuntimeConstantDeprecated::MethodRef(_) => {
                self.get_methodref(idx)?;
            }
            RuntimeConstantDeprecated::FieldRef(_) => {
                self.get_fieldref(idx)?;
            }
            RuntimeConstantDeprecated::NameAndType(_) => unimplemented!(),
            _ => {}
        };
        Ok(entry)
    }

    pub fn get_utf8(&self, idx: &u16) -> Result<&str, RuntimePoolError> {
        match self.entry(idx)? {
            RuntimeConstantDeprecated::Utf8(string) => Ok(string),
            other => Err(RuntimePoolError::TypeError(
                *idx,
                RuntimeConstantTypeDeprecated::Utf8.to_string(),
                other.get_type().to_string(),
            )),
        }
    }

    pub fn get_utf8_arc(&self, idx: &u16) -> Result<Arc<str>, RuntimePoolError> {
        match self.entry(idx)? {
            RuntimeConstantDeprecated::Utf8(string) => Ok(string.clone()),
            other => Err(RuntimePoolError::TypeError(
                *idx,
                RuntimeConstantTypeDeprecated::Utf8.to_string(),
                other.get_type().to_string(),
            )),
        }
    }

    pub fn get_string(
        &self,
        idx: &u16,
    ) -> Result<&Arc<StringReferenceDeprecated>, RuntimePoolError> {
        match self.entry(idx)? {
            RuntimeConstantDeprecated::String(str_ref) => {
                str_ref.value.get_or_try_init::<_, RuntimePoolError>(|| {
                    self.get_utf8_arc(str_ref.string_index())
                })?;
                Ok(str_ref)
            }
            other => Err(RuntimePoolError::TypeError(
                *idx,
                RuntimeConstantTypeDeprecated::String.to_string(),
                other.get_type().to_string(),
            )),
        }
    }

    pub fn get_class(&self, idx: &u16) -> Result<&Arc<ClassReferenceDeprecated>, RuntimePoolError> {
        match self.entry(idx)? {
            RuntimeConstantDeprecated::Class(class_ref) => {
                class_ref.name.get_or_try_init::<_, RuntimePoolError>(|| {
                    self.get_utf8_arc(class_ref.name_index())
                })?;
                Ok(class_ref)
            }
            other => Err(RuntimePoolError::TypeError(
                *idx,
                RuntimeConstantTypeDeprecated::Class.to_string(),
                other.get_type().to_string(),
            )),
        }
    }

    pub fn get_class_name(&self, idx: &u16) -> Result<&str, RuntimePoolError> {
        self.get_class(idx)?.name()
    }

    //TODO: all other getters return ref, but this one can't cause of dash map
    pub fn get_method_descriptor(
        &self,
        idx: &u16,
    ) -> Result<Arc<MethodDescriptorReferenceDeprecated>, RuntimePoolError> {
        if let Some(descriptor) = self.method_descriptors.get(&idx) {
            return Ok(descriptor.clone());
        }
        let raw = self.get_utf8_arc(idx)?.clone();
        let resolved = MethodDescriptor::try_from(raw.as_ref())?;
        let reference = Arc::new(MethodDescriptorReferenceDeprecated::new(
            *idx, raw, resolved,
        ));
        self.method_descriptors.insert(*idx, reference.clone());
        Ok(reference)
    }

    //TODO: all other getters return ref, but this one can't cause of dash map
    pub fn get_field_descriptor(
        &self,
        idx: &u16,
    ) -> Result<Arc<FieldDescriptorReferenceDeprecated>, RuntimePoolError> {
        if let Some(descriptor) = self.field_descriptors.get(&idx) {
            return Ok(descriptor.clone());
        }
        let raw = self.get_utf8_arc(idx)?;
        let resolved = DescriptorType::try_from(raw.as_ref())?;
        let reference = Arc::new(FieldDescriptorReferenceDeprecated::new(*idx, raw, resolved));
        self.field_descriptors.insert(*idx, reference.clone());
        Ok(reference)
    }

    pub fn get_method_nat(
        &self,
        idx: &u16,
    ) -> Result<&Arc<NameAndTypeReferenceDeprecated>, RuntimePoolError> {
        match self.entry(idx)? {
            RuntimeConstantDeprecated::NameAndType(method_nat) => {
                method_nat.name.get_or_try_init::<_, RuntimePoolError>(|| {
                    self.get_utf8_arc(method_nat.name_index())
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
                RuntimeConstantTypeDeprecated::MethodNameAndType.to_string(),
                other.get_type().to_string(),
            )),
        }
    }

    pub fn get_field_nat(
        &self,
        idx: &u16,
    ) -> Result<&Arc<NameAndTypeReferenceDeprecated>, RuntimePoolError> {
        match self.entry(idx)? {
            RuntimeConstantDeprecated::NameAndType(field_nat) => {
                field_nat.name.get_or_try_init::<_, RuntimePoolError>(|| {
                    self.get_utf8_arc(field_nat.name_index())
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
                RuntimeConstantTypeDeprecated::FieldNameAndType.to_string(),
                other.get_type().to_string(),
            )),
        }
    }

    pub fn get_methodref(
        &self,
        idx: &u16,
    ) -> Result<&Arc<MethodReferenceDeprecated>, RuntimePoolError> {
        match self.entry(idx)? {
            RuntimeConstantDeprecated::MethodRef(method_ref) => {
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
                RuntimeConstantTypeDeprecated::MethodRef.to_string(),
                other.get_type().to_string(),
            )),
        }
    }

    pub fn get_interface_methodref(
        &self,
        idx: &u16,
    ) -> Result<&Arc<MethodReferenceDeprecated>, RuntimePoolError> {
        match self.entry(idx)? {
            RuntimeConstantDeprecated::InterfaceMethodRef(method_ref) => {
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
                RuntimeConstantTypeDeprecated::InterfaceMethodRef.to_string(),
                other.get_type().to_string(),
            )),
        }
    }

    pub fn get_fieldref(
        &self,
        idx: &u16,
    ) -> Result<&Arc<FieldReferenceDeprecated>, RuntimePoolError> {
        match self.entry(idx)? {
            RuntimeConstantDeprecated::FieldRef(field_ref) => {
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
                RuntimeConstantTypeDeprecated::FieldRef.to_string(),
                other.get_type().to_string(),
            )),
        }
    }
}
