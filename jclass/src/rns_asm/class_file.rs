use crate::ClassFile;
use crate::attribute::ClassAttribute;
use crate::constant_pool::ConstantPool;
use crate::flags::ClassFlags;
use crate::member::{FieldInfo, MethodInfo};
use crate::rns_asm::AttributeNameMap;

#[derive(Debug)]
pub struct ClassFileBuilder {
    minor_version: u16,
    major_version: u16,
    cp: ConstantPool,
    access_flags: ClassFlags,
    this_class: Option<u16>,
    super_class: Option<u16>,
    interfaces: Vec<u16>,
    fields: Vec<FieldInfo>,
    methods: Vec<MethodInfo>,
    attributes: Vec<ClassAttribute>,
    attribute_names: AttributeNameMap,
}

impl ClassFileBuilder {
    pub fn new(minor_version: u16, major_version: u16, cp: ConstantPool) -> Self {
        Self {
            minor_version,
            major_version,
            cp,
            access_flags: ClassFlags::new(0),
            this_class: None,
            super_class: None,
            interfaces: Vec::new(),
            fields: Vec::new(),
            methods: Vec::new(),
            attributes: Vec::new(),
            attribute_names: AttributeNameMap::new(),
        }
    }

    pub fn access_flags(mut self, access_flags: ClassFlags) -> Self {
        self.access_flags = access_flags;
        self
    }

    pub fn this_class(mut self, this_class: Option<u16>) -> Self {
        self.this_class = this_class;
        self
    }

    pub fn super_class(mut self, super_class: Option<u16>) -> Self {
        self.super_class = super_class;
        self
    }

    pub fn build(self) -> Option<ClassFile> {
        Some(ClassFile {
            minor_version: self.minor_version,
            major_version: self.major_version,
            cp: self.cp,
            access_flags: self.access_flags,
            this_class: self.this_class?,
            super_class: self.super_class?,
            interfaces: self.interfaces,
            fields: self.fields,
            methods: self.methods,
            attributes: self.attributes,
            attribute_names: self.attribute_names,
        })
    }
}
