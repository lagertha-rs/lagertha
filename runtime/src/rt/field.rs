use crate::{ClassId, TypeDescriptorId};
use common::jtype::Value;
use jclass::flags::FieldFlags;
use std::cell::RefCell;

#[derive(Debug, Copy, Clone)]
pub struct InstanceField {
    pub flags: FieldFlags,
    pub descriptor_id: TypeDescriptorId,
    pub offset: usize,
    pub declaring_class: ClassId,
}

#[derive(Debug)]
pub struct StaticField {
    pub flags: FieldFlags,
    pub descriptor: TypeDescriptorId,
    pub value: RefCell<Value>,
}
