use crate::{ClassId, TypeDescriptorId};
use common::jtype::Value;
use jclass::flags::FieldFlags;
use std::cell::RefCell;

#[derive(Debug, Copy, Clone)]
pub struct InstanceField {
    pub flags: FieldFlags,
    pub descriptor_id: TypeDescriptorId,
    pub offset: u16,
    pub declaring_class: ClassId,
}

#[derive(Debug)]
pub struct StaticField {
    pub flags: FieldFlags,
    pub descriptor: TypeDescriptorId,
    pub value: RefCell<Value>,
}

/*
impl Field {
    pub fn new(
        field_info: FieldInfo,
        cp: &RuntimeConstantPoolDeprecated,
    ) -> Result<Self, JvmError> {
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

    pub fn name_arc(&self) -> Arc<str> {
        self.name.clone()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn descriptor(&self) -> &Arc<FieldDescriptorReferenceDeprecated> {
        &self.descriptor
    }
}

impl StaticField {
    pub fn new(
        field_info: FieldInfo,
        cp: &RuntimeConstantPoolDeprecated,
    ) -> Result<Self, JvmError> {
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

    pub fn descriptor(&self) -> &Arc<FieldDescriptorReferenceDeprecated> {
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

 */
