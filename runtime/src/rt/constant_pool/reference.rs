use crate::rt::constant_pool::RuntimeConstantType;
use crate::rt::constant_pool::error::RuntimePoolError;
use common::descriptor::MethodDescriptor;
use common::jtype::Type;
use std::sync::Arc;

type OnceCell<I> = once_cell::sync::OnceCell<I>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ClassReference {
    cp_index: u16,
    name_index: u16,
    pub(super) name: OnceCell<Arc<str>>,
}

impl ClassReference {
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
                RuntimeConstantType::Class,
            ))
    }

    pub fn name_arc(&self) -> Result<Arc<str>, RuntimePoolError> {
        self.name().map(|s| Arc::from(s))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StringReference {
    cp_index: u16,
    string_index: u16,
    pub(super) value: OnceCell<Arc<str>>,
}

impl StringReference {
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
                RuntimeConstantType::String,
            ))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MethodReference {
    cp_index: u16,
    class_index: u16,
    name_and_type_index: u16,
    pub(super) class: OnceCell<Arc<ClassReference>>,
    pub(super) name_and_type: OnceCell<Arc<NameAndTypeReference>>,
}

impl MethodReference {
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

    pub fn class_ref(&self) -> Result<&Arc<ClassReference>, RuntimePoolError> {
        self.class
            .get()
            .ok_or(RuntimePoolError::TryingToAccessUnresolved(
                self.cp_index,
                RuntimeConstantType::MethodRef,
            ))
    }

    pub fn name_and_type_ref(&self) -> Result<&Arc<NameAndTypeReference>, RuntimePoolError> {
        self.name_and_type
            .get()
            .ok_or(RuntimePoolError::TryingToAccessUnresolved(
                self.cp_index,
                RuntimeConstantType::MethodRef,
            ))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FieldReference {
    cp_index: u16,
    class_index: u16,
    name_and_type_index: u16,
    pub(super) class: OnceCell<Arc<ClassReference>>,
    pub(super) name_and_type: OnceCell<Arc<NameAndTypeReference>>,
}

impl FieldReference {
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

    pub fn class_ref(&self) -> Result<&Arc<ClassReference>, RuntimePoolError> {
        self.class
            .get()
            .ok_or(RuntimePoolError::TryingToAccessUnresolved(
                self.cp_index,
                RuntimeConstantType::FieldRef,
            ))
    }

    pub fn name_and_type_ref(&self) -> Result<&Arc<NameAndTypeReference>, RuntimePoolError> {
        self.name_and_type
            .get()
            .ok_or(RuntimePoolError::TryingToAccessUnresolved(
                self.cp_index,
                RuntimeConstantType::FieldRef,
            ))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NameAndTypeReference {
    cp_index: u16,
    name_index: u16,
    pub(super) name: OnceCell<Arc<str>>,
    descriptor_index: u16,
    // TODO: either method, either field. find elegant solution
    pub(super) method_descriptor: OnceCell<Arc<MethodDescriptorReference>>,
    pub(super) field_descriptor: OnceCell<Arc<FieldDescriptorReference>>,
}

impl NameAndTypeReference {
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
                RuntimeConstantType::NameAndType,
            ))
    }

    pub fn method_descriptor_ref(
        &self,
    ) -> Result<&Arc<MethodDescriptorReference>, RuntimePoolError> {
        self.method_descriptor
            .get()
            .ok_or(RuntimePoolError::TryingToAccessUnresolved(
                self.cp_index,
                RuntimeConstantType::NameAndType,
            ))
    }

    pub fn field_descriptor_ref(&self) -> Result<&Arc<FieldDescriptorReference>, RuntimePoolError> {
        self.field_descriptor
            .get()
            .ok_or(RuntimePoolError::TryingToAccessUnresolved(
                self.cp_index,
                RuntimeConstantType::NameAndType,
            ))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MethodDescriptorReference {
    idx: u16,
    raw: Arc<str>,
    resolved: MethodDescriptor,
}

impl MethodDescriptorReference {
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
pub struct FieldDescriptorReference {
    idx: u16,
    raw: Arc<str>,
    resolved: Type,
}

impl FieldDescriptorReference {
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
pub struct InvokeDynamicReference {
    bootstrap_method_attr_index: u16,
    name_and_type_index: u16,
    pub(super) name_and_type: OnceCell<Arc<NameAndTypeReference>>,
}

impl InvokeDynamicReference {
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

    pub fn name_and_type_ref(&self) -> Result<&Arc<NameAndTypeReference>, RuntimePoolError> {
        self.name_and_type
            .get()
            .ok_or(RuntimePoolError::TryingToAccessUnresolved(
                self.name_and_type_index,
                RuntimeConstantType::NameAndType,
            ))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MethodTypeReference {
    descriptor_index: u16,
    pub(super) descriptor: OnceCell<Arc<MethodDescriptorReference>>,
}

impl MethodTypeReference {
    pub fn new(descriptor_index: u16) -> Self {
        Self {
            descriptor_index,
            descriptor: OnceCell::new(),
        }
    }

    pub fn descriptor_index(&self) -> u16 {
        self.descriptor_index
    }

    pub fn descriptor_ref(&self) -> Result<&Arc<MethodDescriptorReference>, RuntimePoolError> {
        self.descriptor
            .get()
            .ok_or(RuntimePoolError::TryingToAccessUnresolved(
                self.descriptor_index,
                RuntimeConstantType::MethodTypeRef,
            ))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MethodHandleReference {
    reference_kind: u8,
    reference_index: u16,
}

impl MethodHandleReference {
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
