use crate::rt::constant_pool::RuntimeConstantType;
use crate::rt::constant_pool::error::RuntimePoolError;
use common::descriptor::MethodDescriptor;
use common::jtype::Type;
use std::rc::Rc;

type OnceCell<I> = once_cell::sync::OnceCell<I>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ClassReference {
    cp_index: u16,
    name_index: u16,
    pub(super) name: OnceCell<Rc<String>>,
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

    pub fn name(&self) -> Result<&Rc<String>, RuntimePoolError> {
        self.name
            .get()
            .ok_or(RuntimePoolError::TryingToAccessUnresolved(
                self.cp_index,
                RuntimeConstantType::Class,
            ))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StringReference {
    cp_index: u16,
    string_index: u16,
    pub(super) value: OnceCell<Rc<String>>,
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

    pub fn value(&self) -> Result<&Rc<String>, RuntimePoolError> {
        self.value
            .get()
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
    pub(super) class: OnceCell<Rc<ClassReference>>,
    pub(super) name_and_type: OnceCell<Rc<NameAndTypeReference>>,
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

    pub fn class(&self) -> Result<&Rc<ClassReference>, RuntimePoolError> {
        self.class
            .get()
            .ok_or(RuntimePoolError::TryingToAccessUnresolved(
                self.cp_index,
                RuntimeConstantType::MethodRef,
            ))
    }

    pub fn name_and_type(&self) -> Result<&Rc<NameAndTypeReference>, RuntimePoolError> {
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
    pub(super) class: OnceCell<Rc<ClassReference>>,
    pub(super) name_and_type: OnceCell<Rc<NameAndTypeReference>>,
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

    pub fn class(&self) -> Result<&Rc<ClassReference>, RuntimePoolError> {
        self.class
            .get()
            .ok_or(RuntimePoolError::TryingToAccessUnresolved(
                self.cp_index,
                RuntimeConstantType::FieldRef,
            ))
    }

    pub fn name_and_type(&self) -> Result<&Rc<NameAndTypeReference>, RuntimePoolError> {
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
    pub(super) name: OnceCell<Rc<String>>,
    descriptor_index: u16,
    // TODO: either method, either field. find elegant solution
    pub(super) method_descriptor: OnceCell<Rc<MethodDescriptorReference>>,
    pub(super) field_descriptor: OnceCell<Rc<FieldDescriptorReference>>,
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

    pub fn name(&self) -> Result<&Rc<String>, RuntimePoolError> {
        self.name
            .get()
            .ok_or(RuntimePoolError::TryingToAccessUnresolved(
                self.cp_index,
                RuntimeConstantType::NameAndType,
            ))
    }

    pub fn method_descriptor(&self) -> Result<&Rc<MethodDescriptorReference>, RuntimePoolError> {
        self.method_descriptor
            .get()
            .ok_or(RuntimePoolError::TryingToAccessUnresolved(
                self.cp_index,
                RuntimeConstantType::NameAndType,
            ))
    }

    pub fn field_descriptor(&self) -> Result<&Rc<FieldDescriptorReference>, RuntimePoolError> {
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
    raw: Rc<String>,
    resolved: MethodDescriptor,
}

impl MethodDescriptorReference {
    pub fn new(idx: u16, raw: Rc<String>, resolved: MethodDescriptor) -> Self {
        Self { idx, raw, resolved }
    }

    pub fn idx(&self) -> u16 {
        self.idx
    }

    pub fn raw(&self) -> &Rc<String> {
        &self.raw
    }

    pub fn resolved(&self) -> &MethodDescriptor {
        &self.resolved
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FieldDescriptorReference {
    idx: u16,
    raw: Rc<String>,
    resolved: Type,
}

impl FieldDescriptorReference {
    pub fn new(idx: u16, raw: Rc<String>, resolved: Type) -> Self {
        Self { idx, raw, resolved }
    }

    pub fn idx(&self) -> u16 {
        self.idx
    }

    pub fn raw(&self) -> &Rc<String> {
        &self.raw
    }

    pub fn resolved(&self) -> &Type {
        &self.resolved
    }
}
