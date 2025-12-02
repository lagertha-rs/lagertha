use crate::error::JvmError;
use crate::keys::{FieldKey, MethodKey};
use crate::{Symbol, throw_exception};
use once_cell::sync::OnceCell;

pub(crate) struct Utf8Entry {
    pub value: String,
    pub utf8_sym: OnceCell<Symbol>,
}

impl Utf8Entry {
    pub fn new(value: String) -> Self {
        Self {
            value,
            utf8_sym: OnceCell::new(),
        }
    }
}

pub(crate) struct ClassEntry {
    pub name_idx: u16,
    pub name_sym: OnceCell<Symbol>,
}

impl ClassEntry {
    pub fn new(name_idx: u16) -> Self {
        Self {
            name_idx,
            name_sym: OnceCell::new(),
        }
    }

    pub fn get_name_sym(&self) -> Result<Symbol, JvmError> {
        if let Some(name_sym) = self.name_sym.get() {
            Ok(*name_sym)
        } else {
            throw_exception!(InternalError, "ClassEntry name_sym not initialized")
        }
    }
}

pub(crate) struct StringEntry {
    pub string_idx: u16,
    pub string_sym: OnceCell<Symbol>,
}

impl StringEntry {
    pub fn new(string_idx: u16) -> Self {
        Self {
            string_idx,
            string_sym: OnceCell::new(),
        }
    }

    pub fn get_string_sym(&self) -> Result<Symbol, JvmError> {
        if let Some(str_sym) = self.string_sym.get() {
            Ok(*str_sym)
        } else {
            throw_exception!(InternalError, "StringEntry string_sym not initialized")
        }
    }
}

pub(crate) struct MethodEntry {
    pub class_idx: u16,
    pub nat_idx: u16,
    pub class_sym: OnceCell<Symbol>,
}

impl MethodEntry {
    pub fn new(class_idx: u16, nat_idx: u16) -> Self {
        Self {
            class_idx,
            nat_idx,
            class_sym: OnceCell::new(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct MethodEntryView {
    pub class_sym: Symbol,
    pub name_and_type: NameAndTypeEntryView,
}

impl MethodEntryView {
    pub fn new(class_sym: Symbol, name_and_type: NameAndTypeEntryView) -> Self {
        Self {
            class_sym,
            name_and_type,
        }
    }
}

impl From<NameAndTypeEntryView> for MethodKey {
    fn from(value: NameAndTypeEntryView) -> Self {
        MethodKey {
            name: value.name_sym,
            desc: value.descriptor_sym,
        }
    }
}

pub(crate) struct FieldEntry {
    pub class_idx: u16,
    pub nat_idx: u16,
    pub class_sym: OnceCell<Symbol>,
}

impl FieldEntry {
    pub fn new(class_idx: u16, nat_idx: u16) -> Self {
        Self {
            class_idx,
            nat_idx,
            class_sym: OnceCell::new(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct FieldEntryView {
    pub class_sym: Symbol,
    pub name_and_type: NameAndTypeEntryView,
}

#[derive(Copy, Clone)]
pub enum MethodHandleEntryView {
    GetField(FieldEntryView),
    GetStatic(FieldEntryView),
    PutField(FieldEntryView),
    PutStatic(FieldEntryView),
    InvokeVirtual(MethodEntryView),
    InvokeStatic(MethodEntryView),
    InvokeSpecial(MethodEntryView),
    NewInvokeSpecial(MethodEntryView),
    InvokeInterface(MethodEntryView),
}

impl FieldEntryView {
    pub fn new(class_sym: Symbol, name_and_type: NameAndTypeEntryView) -> Self {
        Self {
            class_sym,
            name_and_type,
        }
    }
}

pub(crate) struct InvokeDynamicEntry {
    pub bootstrap_idx: u16,
    pub nat_idx: u16,
}

impl InvokeDynamicEntry {
    pub fn new(bootstrap_idx: u16, nat_idx: u16) -> Self {
        Self {
            bootstrap_idx,
            nat_idx,
        }
    }
}

pub(crate) struct NameAndTypeEntry {
    pub name_idx: u16,
    pub descriptor_idx: u16,
    pub name_sym: OnceCell<Symbol>,
    pub descriptor_sym: OnceCell<Symbol>,
}

impl NameAndTypeEntry {
    pub fn new(name_idx: u16, descriptor_idx: u16) -> Self {
        Self {
            name_idx,
            descriptor_idx,
            name_sym: OnceCell::new(),
            descriptor_sym: OnceCell::new(),
        }
    }
}

//#[derive(Copy, Clone)]
#[derive(Clone)]
pub struct InvokeDynamicEntryView {
    pub method_handle: MethodHandleEntryView,
    pub bootstrap_arguments: Vec<u16>, // TODO: need better representation
    pub nat_view: NameAndTypeEntryView,
}

impl InvokeDynamicEntryView {
    pub fn new(
        method_handle: MethodHandleEntryView,
        bootstrap_arguments: Vec<u16>,
        nat_view: NameAndTypeEntryView,
    ) -> Self {
        Self {
            method_handle,
            bootstrap_arguments,
            nat_view,
        }
    }
}

#[derive(Copy, Clone)]
pub struct NameAndTypeEntryView {
    pub name_sym: Symbol,
    pub descriptor_sym: Symbol,
}

impl NameAndTypeEntryView {
    pub fn new(name_sym: Symbol, descriptor_sym: Symbol) -> Self {
        Self {
            name_sym,
            descriptor_sym,
        }
    }
}

impl From<NameAndTypeEntryView> for FieldKey {
    fn from(value: NameAndTypeEntryView) -> Self {
        FieldKey {
            name: value.name_sym,
            desc: value.descriptor_sym,
        }
    }
}
