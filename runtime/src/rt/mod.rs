use crate::rt::class::InstanceClass;
use crate::rt::field::InstanceField;
use crate::rt::interface::InterfaceClass;
use crate::{ClassId, Symbol};
use common::error::JvmError;
use common::jtype::{DescriptorPrimitiveType, HeapRef, PrimitiveType};
use once_cell::sync::OnceCell;

pub mod class;
pub mod class_deprecated;
pub mod constant_pool;
mod field;
pub mod field_deprecated;
pub mod interface;
pub mod method;
pub mod method_deprecated;

// TODO: something like that...
pub enum ClassState {
    Loaded,       // Parsed, superclass loaded
    Linked,       // Verified, prepared
    Initializing, // <clinit> in progress
    Initialized,  // <clinit> executed
}

pub enum JvmClass {
    Instance(Box<InstanceClass>),
    Interface(Box<InterfaceClass>),
    Primitive(PrimitiveClass),
    PrimitiveArray(PrimitiveArrayClass),
    InstanceArray(ObjectArrayClass),
}

impl JvmClass {
    pub fn get_name(&self) -> &Symbol {
        match self {
            JvmClass::Instance(ic) => &ic.name,
            JvmClass::PrimitiveArray(pac) => &pac.name,
            JvmClass::InstanceArray(oac) => &oac.name,
            JvmClass::Primitive(pc) => &pc.name,
            JvmClass::Interface(i) => &i.name,
        }
    }

    pub fn get_instance_fields(&self) -> &[InstanceField] {
        match self {
            JvmClass::Instance(ic) => ic
                .instance_fields
                .get()
                .map_or(&[], |fields_vec| fields_vec.as_slice()),
            _ => &[],
        }
    }

    pub fn get_mirror_ref(&self) -> Option<HeapRef> {
        match self {
            JvmClass::Instance(ic) => ic.get_mirror_ref(),
            JvmClass::PrimitiveArray(pac) => pac.get_mirror_ref(),
            JvmClass::InstanceArray(oac) => oac.get_mirror_ref(),
            JvmClass::Primitive(pc) => pc.get_mirror_ref(),
            JvmClass::Interface(i) => i.get_mirror_ref(),
        }
    }

    pub fn set_mirror_ref(&self, mirror: HeapRef) -> Result<(), JvmError> {
        match self {
            JvmClass::Instance(ic) => ic.set_mirror_ref(mirror),
            JvmClass::PrimitiveArray(pac) => pac.set_mirror_ref(mirror),
            JvmClass::InstanceArray(oac) => oac.set_mirror_ref(mirror),
            JvmClass::Primitive(pc) => pc.set_mirror_ref(mirror),
            JvmClass::Interface(i) => i.set_mirror_ref(mirror),
        }
    }

    pub fn get_super_id(&self) -> Option<ClassId> {
        match self {
            JvmClass::Instance(i) => i.super_id,
            JvmClass::PrimitiveArray(arr) => Some(arr.super_id),
            JvmClass::InstanceArray(arr) => Some(arr.super_id),
            JvmClass::Primitive(_) => None,
            JvmClass::Interface(i) => i.get_super_id(),
        }
    }
}

pub struct PrimitiveClass {
    pub name: Symbol,
    pub primitive_type: PrimitiveType,
    pub(crate) mirror_ref: OnceCell<HeapRef>,
}

impl PrimitiveClass {
    pub fn new(name: Symbol, primitive_type: PrimitiveType) -> Self {
        Self {
            name,
            primitive_type,
            mirror_ref: OnceCell::new(),
        }
    }
    pub fn get_mirror_ref(&self) -> Option<HeapRef> {
        self.mirror_ref.get().copied()
    }

    pub fn set_mirror_ref(&self, mirror: HeapRef) -> Result<(), JvmError> {
        self.mirror_ref
            .set(mirror)
            .map_err(|_| JvmError::Todo("PrimitiveClass mirror_ref already set".to_string()))
    }
}

pub struct PrimitiveArrayClass {
    pub name: Symbol,
    pub super_id: ClassId,
    pub element_type: DescriptorPrimitiveType,
    pub(crate) mirror_ref: OnceCell<HeapRef>,
}

impl PrimitiveArrayClass {
    pub fn get_mirror_ref(&self) -> Option<HeapRef> {
        self.mirror_ref.get().copied()
    }

    pub fn set_mirror_ref(&self, mirror: HeapRef) -> Result<(), JvmError> {
        self.mirror_ref
            .set(mirror)
            .map_err(|_| JvmError::Todo("PrimitiveArrayClass mirror_ref already set".to_string()))
    }
}

pub struct ObjectArrayClass {
    pub name: Symbol,
    pub super_id: ClassId,
    pub element_class_id: ClassId,
    pub(crate) mirror_ref: OnceCell<HeapRef>,
}

impl ObjectArrayClass {
    pub fn get_mirror_ref(&self) -> Option<HeapRef> {
        self.mirror_ref.get().copied()
    }

    pub fn set_mirror_ref(&self, mirror: HeapRef) -> Result<(), JvmError> {
        self.mirror_ref
            .set(mirror)
            .map_err(|_| JvmError::Todo("ObjectArrayClass mirror_ref already set".to_string()))
    }
}
