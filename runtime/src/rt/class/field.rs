use crate::JvmError;
use crate::rt::constant_pool::RuntimeConstantPool;
use crate::rt::constant_pool::reference::FieldDescriptorReference;
use class_file::field::FieldInfo;
use class_file::flags::FieldFlags;
use common::jtype::TypeValue;
use std::sync::Arc;

#[derive(Debug)]
pub struct Field {
    pub name: Arc<String>,
    pub flags: FieldFlags,
    pub descriptor: Arc<FieldDescriptorReference>,
}

#[derive(Debug)]
pub struct StaticField {
    pub name: Arc<String>,
    pub flags: FieldFlags,
    pub descriptor: Arc<FieldDescriptorReference>,
    pub value: TypeValue,
}

impl Field {
    pub fn new(field_info: FieldInfo, cp: &Arc<RuntimeConstantPool>) -> Result<Self, JvmError> {
        let name = cp.get_utf8(&field_info.name_index)?.clone();
        let flags = field_info.access_flags;
        let descriptor = cp.get_field_descriptor(&field_info.descriptor_index)?;

        //TODO: field attributes

        Ok(Self {
            name,
            flags,
            descriptor,
        })
    }
}

impl StaticField {
    pub fn new(field_info: FieldInfo, cp: &Arc<RuntimeConstantPool>) -> Result<Self, JvmError> {
        let name = cp.get_utf8(&field_info.name_index)?.clone();
        let flags = field_info.access_flags;
        let descriptor = cp.get_field_descriptor(&field_info.descriptor_index)?;
        let value = descriptor.resolved().get_default_value();

        //TODO: field attributes

        Ok(Self {
            name,
            flags,
            descriptor,
            value,
        })
    }
}
