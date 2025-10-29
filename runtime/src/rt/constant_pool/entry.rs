use crate::Sym;
use once_cell::sync::OnceCell;

pub(super) struct Utf8Entry {
    pub value: String,
    pub utf8_sym: OnceCell<Sym>,
}

impl Utf8Entry {
    pub fn new(value: String) -> Self {
        Self {
            value,
            utf8_sym: OnceCell::new(),
        }
    }
}

pub(super) struct ClassEntry {
    pub name_idx: u16,
    pub name_sym: OnceCell<Sym>,
}

impl ClassEntry {
    pub fn new(name_idx: u16) -> Self {
        Self {
            name_idx,
            name_sym: OnceCell::new(),
        }
    }
}

pub(super) struct StringEntry {
    pub string_idx: u16,
    pub string_sym: OnceCell<Sym>,
}

impl StringEntry {
    pub fn new(string_idx: u16) -> Self {
        Self {
            string_idx,
            string_sym: OnceCell::new(),
        }
    }
}

pub(super) struct MethodEntry {
    pub class_idx: u16,
    pub nat_idx: u16,
    pub class_sym: OnceCell<Sym>,
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
    pub class_sym: Sym,
    pub name_and_type: NameAndTypeEntryView,
}

impl MethodEntryView {
    pub fn new(class_sym: Sym, name_and_type: NameAndTypeEntryView) -> Self {
        Self {
            class_sym,
            name_and_type,
        }
    }
}

pub(super) struct FieldEntry {
    pub class_idx: u16,
    pub nat_idx: u16,
    pub class_sym: OnceCell<Sym>,
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

pub struct FieldEntryView {
    pub class_sym: Sym,
    pub name_and_type: NameAndTypeEntryView,
}

impl FieldEntryView {
    pub fn new(class_sym: Sym, name_and_type: NameAndTypeEntryView) -> Self {
        Self {
            class_sym,
            name_and_type,
        }
    }
}

pub(super) struct NameAndTypeEntry {
    pub name_idx: u16,
    pub descriptor_idx: u16,
    pub name_sym: OnceCell<Sym>,
    pub descriptor_sym: OnceCell<Sym>,
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

#[derive(Copy, Clone)]
pub struct NameAndTypeEntryView {
    pub name_sym: Sym,
    pub descriptor_sym: Sym,
}

impl NameAndTypeEntryView {
    pub fn new(name_sym: Sym, descriptor_sym: Sym) -> Self {
        Self {
            name_sym,
            descriptor_sym,
        }
    }
}
