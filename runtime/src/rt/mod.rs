use once_cell::sync::OnceCell;
use common::error::JvmError;
use common::jtype::{HeapRef, PrimitiveType};
use crate::rt::class::InstanceClass;
use crate::{ClassId, TypeDescriptorId, Symbol};
use crate::rt::field::InstanceField;

pub mod class;
pub mod class_deprecated;
pub mod constant_pool;
mod field;
pub mod field_deprecated;
pub mod method;
pub mod method_deprecated;

pub enum JvmClass {
    Instance(InstanceClass),
    PrimitiveArray(PrimitiveArrayClass),
    InstanceArray(ObjectArrayClass),
}

impl JvmClass {
    pub fn get_name(&self) -> &Symbol {
        match self {
            JvmClass::Instance(ic) => &ic.name,
            JvmClass::PrimitiveArray(pac) => &pac.name,
            JvmClass::InstanceArray(oac) => &oac.name,
        }
    }

    pub fn get_instance_fields(&self) -> &[InstanceField] {
        match self {
            JvmClass::Instance(ic) => {
                ic.instance_fields
                    .get()
                    .map_or(&[], |fields_vec| fields_vec.as_slice())
            }
            _ => &[],
        }
    }

    pub fn get_mirror_ref(&self) -> Option<HeapRef> {
        match self {
            JvmClass::Instance(ic) => ic.get_mirror_ref(),
            JvmClass::PrimitiveArray(pac) => pac.get_mirror_ref(),
            JvmClass::InstanceArray(oac) => oac.get_mirror_ref(),
        }
    }

    pub fn set_mirror_ref(&self, mirror: HeapRef) -> Result<(), JvmError> {
        match self {
            JvmClass::Instance(ic) => ic.set_mirror_ref(mirror),
            JvmClass::PrimitiveArray(pac) => pac.set_mirror_ref(mirror),
            JvmClass::InstanceArray(oac) => oac.set_mirror_ref(mirror),
        }
    }

    pub fn is_child_of(&self, other: &ClassId) -> bool {
        match self {
            JvmClass::Instance(i) => i.is_child_of(other),
            JvmClass::PrimitiveArray(arr) => &arr.super_id == other,
            JvmClass::InstanceArray(arr) => &arr.super_id == other,
        }
    }

    pub fn get_super_idb(&self) -> Option<ClassId> {
        match self {
            JvmClass::Instance(i) => i.super_id,
            JvmClass::PrimitiveArray(arr) => Some(arr.super_id),
            JvmClass::InstanceArray(arr) =>  Some(arr.super_id)
        }
    }
}

pub struct PrimitiveArrayClass {
    pub name: Symbol,
    pub super_id: ClassId,
    pub element_descriptor: TypeDescriptorId,
    pub element_type: PrimitiveType,
    pub(crate) mirror_ref: OnceCell<HeapRef>,

}

impl PrimitiveArrayClass {
    pub fn get_mirror_ref(&self) -> Option<HeapRef> {
        self.mirror_ref.get().copied()
    }

    pub fn set_mirror_ref(&self, mirror: HeapRef) -> Result<(), JvmError> {
         self.mirror_ref.set(mirror).map_err(|_| {
            JvmError::Todo("PrimitiveArrayClass mirror_ref already set".to_string())
    })}

}



pub struct ObjectArrayClass {
    pub name: Symbol,
    pub super_id: ClassId,
    pub element_descriptor: TypeDescriptorId,
    pub element_class_id: ClassId,
    pub(crate) mirror_ref: OnceCell<HeapRef>,

}

impl ObjectArrayClass {
    pub fn get_mirror_ref(&self) -> Option<HeapRef> {
        self.mirror_ref.get().copied()
    }

    pub fn set_mirror_ref(&self, mirror: HeapRef) -> Result<(), JvmError> {
         self.mirror_ref.set(mirror).map_err(|_| {
            JvmError::Todo("ObjectArrayClass mirror_ref already set".to_string())
    })}

}
