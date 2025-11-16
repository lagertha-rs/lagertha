use crate::{ClassId, FieldDescriptorId};
use common::Value;
use jclass::flags::FieldFlags;
use std::cell::RefCell;

#[derive(Debug, Copy, Clone)]
pub struct InstanceField {
    pub flags: FieldFlags,
    pub descriptor_id: FieldDescriptorId,
    pub offset: u16,
    pub declaring_class: ClassId,
}

#[derive(Debug)]
pub struct StaticField {
    pub flags: FieldFlags,
    pub descriptor: FieldDescriptorId,
    pub value: RefCell<Value>,
}
