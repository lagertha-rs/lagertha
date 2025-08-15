use crate::rt::descriptor::MethodDescriptor;
use crate::rt::jtype::Type;
use crate::JvmError;
use crate::JvmError::TryingAccessUninitializedRuntimeConstant;
use std::fmt;
use std::fmt::Formatter;
use std::rc::Rc;

pub mod access;
pub mod class;
pub mod descriptor;
pub mod field;
pub mod instruction_set;
pub mod jtype;
pub mod method;
pub mod runtime_constant_pool;

type OnceCell<I> = once_cell::unsync::OnceCell<I>;
//type OnceCell<I> = once_cell::sync::OnceCell<I>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ClassReference {
    name_index: u16,
    name: OnceCell<Rc<String>>,
}

impl ClassReference {
    pub fn new(name_index: u16) -> Self {
        Self {
            name_index,
            name: OnceCell::new(),
        }
    }

    pub fn get_name(&self) -> Result<&Rc<String>, JvmError> {
        self.name
            .get()
            .ok_or(TryingAccessUninitializedRuntimeConstant(
                "class",
                self.name_index,
            ))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StringReference {
    pub string_index: u16,
    pub value: OnceCell<Rc<String>>,
}

impl StringReference {
    pub fn new(string_index: u16) -> Self {
        Self {
            string_index,
            value: OnceCell::new(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MethodReference {
    pub class_index: u16,
    pub name_and_type_index: u16,
    pub class: OnceCell<Rc<ClassReference>>,
    pub name_and_type: OnceCell<Rc<NameAndTypeReference>>,
}

impl fmt::Display for MethodReference {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

impl MethodReference {
    pub fn new(class_index: u16, name_and_type_index: u16) -> Self {
        Self {
            class_index,
            name_and_type_index,
            class: OnceCell::new(),
            name_and_type: OnceCell::new(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FieldReference {
    pub class_index: u16,
    pub name_and_type_index: u16,
    pub class: OnceCell<Rc<ClassReference>>,
    pub name_and_type: OnceCell<Rc<NameAndTypeReference>>,
}

impl FieldReference {
    pub fn new(class_index: u16, name_and_type_index: u16) -> Self {
        Self {
            class_index,
            name_and_type_index,
            class: OnceCell::new(),
            name_and_type: OnceCell::new(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NameAndTypeReference {
    pub name_index: u16,
    pub name: OnceCell<Rc<String>>,
    pub descriptor_index: u16,
    // TODO: either method, either field. find elegant solution
    pub method_descriptor: OnceCell<Rc<MethodDescriptorReference>>,
    pub field_descriptor: OnceCell<Rc<FieldDescriptorReference>>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MethodDescriptorReference {
    pub idx: u16,
    pub raw: Rc<String>,
    pub resolved: MethodDescriptor,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FieldDescriptorReference {
    pub idx: u16,
    pub raw: Rc<String>,
    pub resolved: Type,
}

impl NameAndTypeReference {
    pub fn new(name_index: u16, descriptor_index: u16) -> Self {
        Self {
            name_index,
            descriptor_index,
            name: OnceCell::new(),
            method_descriptor: OnceCell::new(),
            field_descriptor: OnceCell::new(),
        }
    }
}
