use class_file::descriptor::MethodDescriptor;
use class_file::jtype::Type;
use class_file::ClassFileErr;
use common::CursorError;
use std::rc::Rc;
use thiserror::Error;

pub mod access;
pub mod class;
pub mod field;
pub mod instruction_set;
pub mod method;
pub mod runtime_constant_pool;

#[derive(Debug, Error)]
pub enum JvmError {
    #[error(transparent)]
    ClassFile(#[from] ClassFileErr),
    #[error(transparent)]
    Cursor(#[from] CursorError),
    #[error("")]
    MissingAttributeInConstantPoll,
    #[error("")]
    ConstantNotFoundInRuntimePool,
    #[error("")]
    TrailingBytes,
    #[error("")]
    TypeError,
}

type OnceCell<I> = once_cell::unsync::OnceCell<I>;
//type OnceCell<I> = once_cell::sync::OnceCell<I>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ClassReference {
    pub name_index: u16,
    pub name: OnceCell<Rc<String>>,
}

impl ClassReference {
    pub fn new(name_index: u16) -> Self {
        Self {
            name_index,
            name: OnceCell::new(),
        }
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
    pub descriptor_index: u16,
    pub name: OnceCell<Rc<String>>,
    pub raw_descriptor: OnceCell<Rc<String>>,
    pub resolved_field: OnceCell<Rc<Type>>,
    pub resolved_method: OnceCell<Rc<MethodDescriptor>>,
}

impl NameAndTypeReference {
    pub fn new(name_index: u16, descriptor_index: u16) -> Self {
        Self {
            name_index,
            descriptor_index,
            name: OnceCell::new(),
            raw_descriptor: OnceCell::new(),
            resolved_field: OnceCell::new(),
            resolved_method: OnceCell::new(),
        }
    }
}
