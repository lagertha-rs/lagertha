use crate::rt::constant_pool::rt_cp_deprecated::RuntimeConstantTypeDeprecated;
use common::descriptor::MethodDescriptor;
use common::error::RuntimePoolError;
use common::jtype::Type;
use std::sync::Arc;

type OnceCell<I> = once_cell::sync::OnceCell<I>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ClassReferenceDeprecated {
    cp_index: u16,
    name_index: u16,
    pub(super) name: OnceCell<Arc<str>>,
}

impl ClassReferenceDeprecated {
    pub fn new(cp_index: u16, name_index: u16) -> Self {
        Self {
            cp_index,
            name_index,
            name: OnceCell::new(),
        }
    }

    pub fn cp_index(&self) -> &u16 {
        &self.cp_index
    }

    pub fn name_index(&self) -> &u16 {
        &self.name_index
    }

    pub fn name(&self) -> Result<&str, RuntimePoolError> {
        self.name
            .get()
            .map(AsRef::as_ref)
            .ok_or(RuntimePoolError::TryingToAccessUnresolved(
                self.cp_index,
                RuntimeConstantTypeDeprecated::Class.to_string(),
            ))
    }

    pub fn name_arc(&self) -> Result<Arc<str>, RuntimePoolError> {
        self.name().map(|s| Arc::from(s))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StringReferenceDeprecated {
    cp_index: u16,
    string_index: u16,
    pub(super) value: OnceCell<Arc<str>>,
}

impl StringReferenceDeprecated {
    pub fn new(cp_index: u16, string_index: u16) -> Self {
        Self {
            cp_index,
            string_index,
            value: OnceCell::new(),
        }
    }

    pub fn string_index(&self) -> &u16 {
        &self.string_index
    }

    pub fn value(&self) -> Result<&str, RuntimePoolError> {
        self.value
            .get()
            .map(AsRef::as_ref)
            .ok_or(RuntimePoolError::TryingToAccessUnresolved(
                self.cp_index,
                RuntimeConstantTypeDeprecated::String.to_string(),
            ))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MethodReferenceDeprecated {
    cp_index: u16,
    class_index: u16,
    name_and_type_index: u16,
    pub(super) class: OnceCell<Arc<ClassReferenceDeprecated>>,
    pub(super) name_and_type: OnceCell<Arc<NameAndTypeReferenceDeprecated>>,
}

impl MethodReferenceDeprecated {
    pub fn new(cp_index: u16, class_index: u16, name_and_type_index: u16) -> Self {
        Self {
            cp_index,
            class_index,
            name_and_type_index,
            class: OnceCell::new(),
            name_and_type: OnceCell::new(),
        }
    }

    pub fn class_index(&self) -> &u16 {
        &self.class_index
    }

    pub fn name_and_type_index(&self) -> &u16 {
        &self.name_and_type_index
    }

    pub fn class_ref(&self) -> Result<&Arc<ClassReferenceDeprecated>, RuntimePoolError> {
        self.class
            .get()
            .ok_or(RuntimePoolError::TryingToAccessUnresolved(
                self.cp_index,
                RuntimeConstantTypeDeprecated::MethodRef.to_string(),
            ))
    }

    pub fn name_and_type_ref(
        &self,
    ) -> Result<&Arc<NameAndTypeReferenceDeprecated>, RuntimePoolError> {
        self.name_and_type
            .get()
            .ok_or(RuntimePoolError::TryingToAccessUnresolved(
                self.cp_index,
                RuntimeConstantTypeDeprecated::MethodRef.to_string(),
            ))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FieldReferenceDeprecated {
    cp_index: u16,
    class_index: u16,
    name_and_type_index: u16,
    pub(super) class: OnceCell<Arc<ClassReferenceDeprecated>>,
    pub(super) name_and_type: OnceCell<Arc<NameAndTypeReferenceDeprecated>>,
}

impl FieldReferenceDeprecated {
    pub fn new(cp_index: u16, class_index: u16, name_and_type_index: u16) -> Self {
        Self {
            cp_index,
            class_index,
            name_and_type_index,
            class: OnceCell::new(),
            name_and_type: OnceCell::new(),
        }
    }

    pub fn class_index(&self) -> &u16 {
        &self.class_index
    }

    pub fn name_and_type_index(&self) -> &u16 {
        &self.name_and_type_index
    }

    pub fn class_ref(&self) -> Result<&Arc<ClassReferenceDeprecated>, RuntimePoolError> {
        self.class
            .get()
            .ok_or(RuntimePoolError::TryingToAccessUnresolved(
                self.cp_index,
                RuntimeConstantTypeDeprecated::FieldRef.to_string(),
            ))
    }

    pub fn name_and_type_ref(
        &self,
    ) -> Result<&Arc<NameAndTypeReferenceDeprecated>, RuntimePoolError> {
        self.name_and_type
            .get()
            .ok_or(RuntimePoolError::TryingToAccessUnresolved(
                self.cp_index,
                RuntimeConstantTypeDeprecated::FieldRef.to_string(),
            ))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NameAndTypeReferenceDeprecated {
    cp_index: u16,
    name_index: u16,
    pub(super) name: OnceCell<Arc<str>>,
    descriptor_index: u16,
    // TODO: either method, either field. find elegant solution
    pub(super) method_descriptor: OnceCell<Arc<MethodDescriptorReferenceDeprecated>>,
    pub(super) field_descriptor: OnceCell<Arc<FieldDescriptorReferenceDeprecated>>,
}

impl NameAndTypeReferenceDeprecated {
    pub fn new(cp_index: u16, name_index: u16, descriptor_index: u16) -> Self {
        Self {
            cp_index,
            name_index,
            descriptor_index,
            name: OnceCell::new(),
            method_descriptor: OnceCell::new(),
            field_descriptor: OnceCell::new(),
        }
    }

    pub fn name_index(&self) -> &u16 {
        &self.name_index
    }

    pub fn descriptor_index(&self) -> &u16 {
        &self.descriptor_index
    }

    pub fn name(&self) -> Result<&str, RuntimePoolError> {
        self.name
            .get()
            .map(AsRef::as_ref)
            .ok_or(RuntimePoolError::TryingToAccessUnresolved(
                self.cp_index,
                RuntimeConstantTypeDeprecated::NameAndType.to_string(),
            ))
    }

    pub fn method_descriptor_ref(
        &self,
    ) -> Result<&Arc<MethodDescriptorReferenceDeprecated>, RuntimePoolError> {
        self.method_descriptor
            .get()
            .ok_or(RuntimePoolError::TryingToAccessUnresolved(
                self.cp_index,
                RuntimeConstantTypeDeprecated::NameAndType.to_string(),
            ))
    }

    pub fn field_descriptor_ref(
        &self,
    ) -> Result<&Arc<FieldDescriptorReferenceDeprecated>, RuntimePoolError> {
        self.field_descriptor
            .get()
            .ok_or(RuntimePoolError::TryingToAccessUnresolved(
                self.cp_index,
                RuntimeConstantTypeDeprecated::NameAndType.to_string(),
            ))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MethodDescriptorReferenceDeprecated {
    idx: u16,
    raw: Arc<str>,
    resolved: MethodDescriptor,
}

impl MethodDescriptorReferenceDeprecated {
    pub fn new(idx: u16, raw: Arc<str>, resolved: MethodDescriptor) -> Self {
        Self { idx, raw, resolved }
    }

    pub fn idx(&self) -> u16 {
        self.idx
    }

    pub fn raw(&self) -> &str {
        &self.raw
    }

    pub fn raw_arc(&self) -> Arc<str> {
        self.raw.clone()
    }

    pub fn resolved(&self) -> &MethodDescriptor {
        &self.resolved
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FieldDescriptorReferenceDeprecated {
    idx: u16,
    raw: Arc<str>,
    resolved: Type,
}

impl FieldDescriptorReferenceDeprecated {
    pub fn new(idx: u16, raw: Arc<str>, resolved: Type) -> Self {
        Self { idx, raw, resolved }
    }

    pub fn idx(&self) -> u16 {
        self.idx
    }

    pub fn raw(&self) -> &str {
        &self.raw
    }

    pub fn raw_arc(&self) -> Arc<str> {
        self.raw.clone()
    }

    pub fn resolved(&self) -> &Type {
        &self.resolved
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct InvokeDynamicReferenceDeprecated {
    bootstrap_method_attr_index: u16,
    name_and_type_index: u16,
    pub(super) name_and_type: OnceCell<Arc<NameAndTypeReferenceDeprecated>>,
}

impl InvokeDynamicReferenceDeprecated {
    pub fn new(bootstrap_method_attr_index: u16, name_and_type_index: u16) -> Self {
        Self {
            bootstrap_method_attr_index,
            name_and_type_index,
            name_and_type: OnceCell::new(),
        }
    }

    pub fn bootstrap_method_idx(&self) -> u16 {
        self.bootstrap_method_attr_index
    }

    pub fn name_and_type_index(&self) -> u16 {
        self.name_and_type_index
    }

    pub fn name_and_type_ref(
        &self,
    ) -> Result<&Arc<NameAndTypeReferenceDeprecated>, RuntimePoolError> {
        self.name_and_type
            .get()
            .ok_or(RuntimePoolError::TryingToAccessUnresolved(
                self.name_and_type_index,
                RuntimeConstantTypeDeprecated::NameAndType.to_string(),
            ))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MethodTypeReferenceDeprecated {
    descriptor_index: u16,
    pub(super) descriptor: OnceCell<Arc<MethodDescriptorReferenceDeprecated>>,
}

impl MethodTypeReferenceDeprecated {
    pub fn new(descriptor_index: u16) -> Self {
        Self {
            descriptor_index,
            descriptor: OnceCell::new(),
        }
    }

    pub fn descriptor_index(&self) -> u16 {
        self.descriptor_index
    }

    pub fn descriptor_ref(
        &self,
    ) -> Result<&Arc<MethodDescriptorReferenceDeprecated>, RuntimePoolError> {
        self.descriptor
            .get()
            .ok_or(RuntimePoolError::TryingToAccessUnresolved(
                self.descriptor_index,
                RuntimeConstantTypeDeprecated::MethodTypeRef.to_string(),
            ))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MethodHandleReferenceDeprecated {
    reference_kind: u8,
    reference_index: u16,
}

impl MethodHandleReferenceDeprecated {
    pub fn new(reference_kind: u8, reference_index: u16) -> Self {
        Self {
            reference_kind,
            reference_index,
        }
    }

    pub fn reference_kind(&self) -> u8 {
        self.reference_kind
    }

    pub fn reference_index(&self) -> u16 {
        self.reference_index
    }
}
