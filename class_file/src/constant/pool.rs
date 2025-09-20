use crate::constant::{ConstantInfo, ConstantTag};
use crate::error::ClassFormatErr;

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.4
#[derive(Debug, Clone, PartialEq)]
pub struct ConstantPool {
    pub inner: Vec<ConstantInfo>,
}

impl ConstantPool {
    pub fn get_utf8(&self, idx: &u16) -> Result<&str, ClassFormatErr> {
        self.inner
            .get(*idx as usize)
            .ok_or(ClassFormatErr::ConstantNotFound(*idx))
            .and_then(|entry| match entry {
                ConstantInfo::Utf8(value) => Ok(value.as_str()),
                e => Err(ClassFormatErr::TypeError(
                    *idx,
                    ConstantTag::Utf8,
                    e.get_tag(),
                )),
            })
    }
}

#[cfg(feature = "pretty_print")]
/// Getters that are useful only for pretty printing
impl ConstantPool {
    pub fn get_printable_utf8(&self, idx: &u16) -> Result<String, ClassFormatErr> {
        self.get_utf8(idx).map(|raw| raw.escape_debug().to_string())
    }

    pub fn get_raw(&self, idx: &u16) -> Result<&ConstantInfo, ClassFormatErr> {
        self.inner
            .get(*idx as usize)
            .ok_or(ClassFormatErr::ConstantNotFound(*idx))
    }

    pub fn get_integer(&self, idx: &u16) -> Result<&i32, ClassFormatErr> {
        self.inner
            .get(*idx as usize)
            .ok_or(ClassFormatErr::ConstantNotFound(*idx))
            .and_then(|entry| match entry {
                ConstantInfo::Integer(value) => Ok(value),
                e => Err(ClassFormatErr::TypeError(
                    *idx,
                    ConstantTag::Integer,
                    e.get_tag(),
                )),
            })
    }

    pub fn get_class(&self, idx: &u16) -> Result<u16, ClassFormatErr> {
        self.inner
            .get(*idx as usize)
            .ok_or(ClassFormatErr::ConstantNotFound(*idx))
            .and_then(|entry| match entry {
                ConstantInfo::Class(name_index) => Ok(*name_index),
                e => Err(ClassFormatErr::TypeError(
                    *idx,
                    ConstantTag::Class,
                    e.get_tag(),
                )),
            })
    }

    pub fn get_class_name(&self, idx: &u16) -> Result<&str, ClassFormatErr> {
        let name_index = self.get_class(idx)?;
        self.get_utf8(&name_index)
    }

    //TODO: There is a macro to do that? replace?
    pub fn get_pretty_class_name(&self, idx: &u16) -> Result<String, ClassFormatErr> {
        let name_index = self.get_class(idx)?;
        self.get_utf8(&name_index).map(|raw| raw.replace('/', "."))
    }

    pub fn get_pretty_class_name_utf8(&self, idx: &u16) -> Result<String, ClassFormatErr> {
        self.get_utf8(idx).map(|raw| {
            if raw.starts_with('[') {
                format!("\"{raw}\"")
            } else {
                raw.to_string()
            }
        })
    }

    pub fn get_pretty_class_name_for_cp_print(&self, idx: &u16) -> Result<String, ClassFormatErr> {
        let name_index = self.get_class(idx)?;
        self.get_pretty_class_name_utf8(&name_index)
    }

    pub fn get_methodref(
        &self,
        idx: &u16,
    ) -> Result<&crate::constant::ReferenceInfo, ClassFormatErr> {
        self.inner
            .get(*idx as usize)
            .ok_or(ClassFormatErr::ConstantNotFound(*idx))
            .and_then(|entry| match entry {
                ConstantInfo::MethodRef(ref_info) => Ok(ref_info),
                ConstantInfo::InterfaceMethodRef(ref_info) => Ok(ref_info),
                e => Err(ClassFormatErr::TypeError(
                    *idx,
                    ConstantTag::MethodRef,
                    e.get_tag(),
                )),
            })
    }

    pub fn get_name_and_type(
        &self,
        idx: &u16,
    ) -> Result<&crate::constant::NameAndTypeInfo, ClassFormatErr> {
        self.inner
            .get(*idx as usize)
            .ok_or(ClassFormatErr::ConstantNotFound(*idx))
            .and_then(|entry| match entry {
                ConstantInfo::NameAndType(ref_info) => Ok(ref_info),
                e => Err(ClassFormatErr::TypeError(
                    *idx,
                    ConstantTag::NameAndType,
                    e.get_tag(),
                )),
            })
    }

    pub fn get_nat_name(
        &self,
        nat: &crate::constant::NameAndTypeInfo,
    ) -> Result<&str, ClassFormatErr> {
        self.get_utf8(&nat.name_index)
    }

    pub fn get_nat_descriptor(
        &self,
        nat: &crate::constant::NameAndTypeInfo,
    ) -> Result<&str, ClassFormatErr> {
        self.get_utf8(&nat.descriptor_index)
    }

    pub fn get_dyn_info_name(
        &self,
        dyn_info: &crate::constant::DynamicInfo,
    ) -> Result<&str, ClassFormatErr> {
        let nat = self.get_name_and_type(&dyn_info.name_and_type_index)?;
        self.get_nat_name(nat)
    }

    pub fn get_dyn_info_descriptor(
        &self,
        dyn_info: &crate::constant::DynamicInfo,
    ) -> Result<&str, ClassFormatErr> {
        let nat = self.get_name_and_type(&dyn_info.name_and_type_index)?;
        self.get_nat_descriptor(nat)
    }

    pub fn get_method_or_field_class_name(
        &self,
        method_ref: &crate::constant::ReferenceInfo,
    ) -> Result<String, ClassFormatErr> {
        self.get_pretty_class_name_for_cp_print(&method_ref.class_index)
    }

    pub fn get_method_or_field_name(
        &self,
        method_ref: &crate::constant::ReferenceInfo,
    ) -> Result<&str, ClassFormatErr> {
        let nat_index = method_ref.name_and_type_index;
        let nat = self.get_name_and_type(&nat_index)?;
        self.get_nat_name(nat)
    }

    pub fn get_method_or_field_name_by_nat_idx(
        &self,
        nat_index: &u16,
    ) -> Result<&str, ClassFormatErr> {
        let nat = self.get_name_and_type(nat_index)?;
        self.get_nat_name(nat)
    }

    pub fn get_method_or_field_descriptor(
        &self,
        ref_info: &crate::constant::ReferenceInfo,
    ) -> Result<&str, ClassFormatErr> {
        let nat_index = ref_info.name_and_type_index;
        let desc_index = self.get_name_and_type(&nat_index)?;
        self.get_nat_descriptor(desc_index)
    }
}
