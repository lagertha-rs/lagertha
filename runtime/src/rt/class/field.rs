use crate::JvmError;
use crate::rt::constant_pool::RuntimeConstantPool;
use crate::rt::constant_pool::reference::FieldDescriptorReference;
use class_file::field::FieldInfo;
use class_file::flags::FieldFlags;
use common::jtype::Value;
use std::cell::{Ref, RefCell};
use std::ops::Deref;
use std::sync::Arc;

#[derive(Debug)]
pub struct Field {
    name: Arc<str>,
    flags: FieldFlags,
    descriptor: Arc<FieldDescriptorReference>,
}

#[derive(Debug)]
pub struct StaticField {
    name: Arc<str>,
    flags: FieldFlags,
    descriptor: Arc<FieldDescriptorReference>,
    value: RefCell<Value>,
}

impl Field {
    pub fn new(field_info: FieldInfo, cp: &RuntimeConstantPool) -> Result<Self, JvmError> {
        let name = cp.get_utf8_arc(&field_info.name_index)?;
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
    pub fn new(field_info: FieldInfo, cp: &RuntimeConstantPool) -> Result<Self, JvmError> {
        let name = cp.get_utf8_arc(&field_info.name_index)?;
        let flags = field_info.access_flags;
        let descriptor = cp.get_field_descriptor(&field_info.descriptor_index)?;
        let value = RefCell::new(descriptor.resolved().get_default_value());

        //TODO: field attributes

        Ok(Self {
            name,
            flags,
            descriptor,
            value,
        })
    }

    pub fn name_arc(&self) -> Arc<str> {
        self.name.clone()
    }

    pub fn descriptor(&self) -> &Arc<FieldDescriptorReference> {
        &self.descriptor
    }

    pub fn value(&self) -> Value {
        self.value.borrow().deref().clone()
    }

    pub fn set_value(&self, value: Value) -> Result<(), JvmError> {
        if !self.descriptor.resolved().is_compatible_with(&value) {
            unimplemented!()
        }
        self.value.replace(value);
        Ok(())
    }
}
