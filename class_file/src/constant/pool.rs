use crate::constant::{ConstantInfo, ConstantTag, NameAndTypeInfo, ReferenceInfo};
use crate::error::ClassFileErr;

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.4
#[derive(Debug, Clone, PartialEq)]
pub struct ConstantPool {
    pub cp: Vec<ConstantInfo>,
}

impl ConstantPool {
    pub fn get_utf8(&self, idx: u16) -> Result<&str, ClassFileErr> {
        self.cp
            .get(idx as usize)
            .ok_or(ClassFileErr::ConstantNotFound(idx))
            .and_then(|entry| match entry {
                ConstantInfo::Utf8(value) => Ok(value.as_str()),
                e => Err(ClassFileErr::TypeError(idx, ConstantTag::Utf8, e.get_tag())),
            })
    }
}

#[cfg(feature = "pretty_print")]
/// Getters that are useful only for pretty printing
impl ConstantPool {
    pub fn get_raw(&self, idx: u16) -> Result<&ConstantInfo, ClassFileErr> {
        self.cp
            .get(idx as usize)
            .ok_or(ClassFileErr::ConstantNotFound(idx))
    }

    pub fn get_class(&self, idx: u16) -> Result<u16, ClassFileErr> {
        self.cp
            .get(idx as usize)
            .ok_or(ClassFileErr::ConstantNotFound(idx))
            .and_then(|entry| match entry {
                ConstantInfo::Class(name_index) => Ok(*name_index),
                e => Err(ClassFileErr::TypeError(
                    idx,
                    ConstantTag::Class,
                    e.get_tag(),
                )),
            })
    }

    pub fn get_class_name(&self, idx: u16) -> Result<&str, ClassFileErr> {
        let name_index = self.get_class(idx)?;
        self.get_utf8(name_index)
    }

    pub fn get_methodref(&self, idx: u16) -> Result<&ReferenceInfo, ClassFileErr> {
        self.cp
            .get(idx as usize)
            .ok_or(ClassFileErr::ConstantNotFound(idx))
            .and_then(|entry| match entry {
                ConstantInfo::MethodRef(ref_info) => Ok(ref_info),
                e => Err(ClassFileErr::TypeError(
                    idx,
                    ConstantTag::MethodRef,
                    e.get_tag(),
                )),
            })
    }

    pub fn get_name_and_type(&self, idx: u16) -> Result<&NameAndTypeInfo, ClassFileErr> {
        self.cp
            .get(idx as usize)
            .ok_or(ClassFileErr::ConstantNotFound(idx))
            .and_then(|entry| match entry {
                ConstantInfo::NameAndType(ref_info) => Ok(ref_info),
                e => Err(ClassFileErr::TypeError(
                    idx,
                    ConstantTag::NameAndType,
                    e.get_tag(),
                )),
            })
    }

    pub fn get_nat_name(&self, nat: &NameAndTypeInfo) -> Result<&str, ClassFileErr> {
        self.get_utf8(nat.name_index)
    }

    pub fn get_nat_descriptor(&self, nat: &NameAndTypeInfo) -> Result<&str, ClassFileErr> {
        self.get_utf8(nat.descriptor_index)
    }

    pub fn get_method_class_name(&self, method_ref: &ReferenceInfo) -> Result<&str, ClassFileErr> {
        self.get_class_name(method_ref.class_index)
    }

    pub fn get_method_name(&self, method_ref: &ReferenceInfo) -> Result<&str, ClassFileErr> {
        let nat_index = method_ref.name_and_type_index;
        let nat = self.get_name_and_type(nat_index)?;
        self.get_nat_name(nat)
    }

    pub fn get_method_descriptor(&self, method_ref: &ReferenceInfo) -> Result<&str, ClassFileErr> {
        let nat_index = method_ref.name_and_type_index;
        let desc_index = self.get_name_and_type(nat_index)?;
        self.get_nat_descriptor(desc_index)
    }
}
