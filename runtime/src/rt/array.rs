use crate::error::JvmError;
use crate::heap::HeapRef;
use crate::keys::{ClassId, MethodKey};
use crate::{MethodId, Symbol, build_exception};
use common::jtype::PrimitiveType;
use once_cell::sync::OnceCell;
use std::collections::HashMap;

pub struct PrimitiveArrayClass {
    pub name: Symbol,
    pub super_id: ClassId,
    pub element_type: PrimitiveType,
    pub vtable: Vec<MethodId>,
    pub vtable_index: HashMap<MethodKey, u16>,
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

    pub fn get_vtable_method_id(&self, key: &MethodKey) -> Result<MethodId, JvmError> {
        let pos =
            self.vtable_index.get(key).copied().ok_or(
                build_exception!(NoSuchMethodError, method_key: *key, class_sym: self.name),
            )?;
        Ok(self.vtable[pos as usize])
    }
}

pub struct ObjectArrayClass {
    pub name: Symbol,
    pub super_id: ClassId,
    pub element_class_id: ClassId,
    pub vtable: Vec<MethodId>,
    pub vtable_index: HashMap<MethodKey, u16>,
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

    pub fn get_vtable_method_id(&self, key: &MethodKey) -> Result<MethodId, JvmError> {
        let pos =
            self.vtable_index.get(key).copied().ok_or(
                build_exception!(NoSuchMethodError, method_key: *key, class_sym: self.name),
            )?;
        Ok(self.vtable[pos as usize])
    }
}
