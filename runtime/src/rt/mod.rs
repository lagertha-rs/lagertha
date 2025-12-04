use crate::error::JvmError;
use crate::heap::HeapRef;
use crate::keys::{ClassId, FieldKey, MethodKey};
use crate::rt::array::{ObjectArrayClass, PrimitiveArrayClass};
use crate::rt::class::InstanceClass;
use crate::rt::constant_pool::RuntimeConstantPool;
use crate::rt::field::{InstanceField, StaticField};
use crate::rt::interface::InterfaceClass;
use crate::vm::Value;
use crate::{MethodId, Symbol};
use common::jtype::PrimitiveType;
use jclass::flags::ClassFlags;
use once_cell::sync::OnceCell;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;

pub mod array;
pub mod class;
pub mod constant_pool;
mod field;
pub mod interface;
pub mod method;

pub trait ClassLike {
    fn base(&self) -> &BaseClass;
    fn get_clinit_method_id(&self) -> Option<&MethodId> {
        self.base().clinit.get()
    }

    fn has_clinit(&self) -> bool {
        self.base().clinit.get().is_some()
    }

    fn name(&self) -> Symbol {
        self.base().name
    }

    fn flags(&self) -> ClassFlags {
        self.base().flags
    }

    fn set_mirror_ref(&self, heap_ref: HeapRef) -> Result<(), JvmError> {
        self.base()
            .mirror_ref
            .set(heap_ref)
            .map_err(|_| JvmError::Todo("Mirror ref already set".to_string()))
    }

    fn get_mirror_ref(&self) -> Option<HeapRef> {
        self.base().mirror_ref.get().copied()
    }

    fn get_super(&self) -> Option<ClassId> {
        self.base().super_id
    }

    fn get_source_file(&self) -> Option<Symbol> {
        self.base().source_file
    }

    fn has_static_field(&self, field_key: &FieldKey) -> Result<bool, JvmError> {
        self.base()
            .get_static_fields()
            .map(|map| map.contains_key(field_key))
    }

    fn set_static_field_value(&self, field_key: &FieldKey, value: Value) -> Result<(), JvmError> {
        let static_field = self
            .base()
            .get_static_fields()?
            .get(field_key)
            .ok_or(JvmError::Todo("No such field".to_string()))?;
        *static_field.value.borrow_mut() = value;
        Ok(())
    }

    fn get_static_field_value(&self, field_key: &FieldKey) -> Result<Value, JvmError> {
        let static_field = self
            .base()
            .get_static_fields()?
            .get(field_key)
            .ok_or(JvmError::Todo("No such field".to_string()))?;
        Ok(*static_field.value.borrow())
    }

    fn get_interfaces(&self) -> Result<&HashSet<ClassId>, JvmError> {
        self.base().get_interfaces()
    }

    fn set_linked(&self) {
        *self.base().state.borrow_mut() = ClassState::Linked;
    }

    fn is_initializing(&self) -> bool {
        matches!(*self.base().state.borrow(), ClassState::Initializing)
    }

    fn set_initializing(&self) {
        *self.base().state.borrow_mut() = ClassState::Initializing;
    }

    fn set_initialized(&self) {
        *self.base().state.borrow_mut() = ClassState::Initialized;
    }

    fn is_initialized_or_initializing(&self) -> bool {
        matches!(
            *self.base().state.borrow(),
            ClassState::Initialized | ClassState::Initializing
        )
    }
}

pub struct BaseClass {
    name: Symbol,
    flags: ClassFlags,
    super_id: Option<ClassId>,
    state: RefCell<ClassState>,
    mirror_ref: OnceCell<HeapRef>,
    interfaces: OnceCell<HashSet<ClassId>>,
    static_fields: OnceCell<HashMap<FieldKey, StaticField>>,
    clinit: OnceCell<MethodId>,
    source_file: Option<Symbol>,
}

impl BaseClass {
    pub fn new(
        name: Symbol,
        flags: ClassFlags,
        super_id: Option<ClassId>,
        source_file: Option<Symbol>,
    ) -> Self {
        Self {
            name,
            flags,
            super_id,
            source_file,
            state: RefCell::new(ClassState::Loaded),
            mirror_ref: OnceCell::new(),
            interfaces: OnceCell::new(),
            static_fields: OnceCell::new(),
            clinit: OnceCell::new(),
        }
    }

    // Internal getters and setters for "lazy" initialized fields
    // mostly because I need to know this class id during linking

    fn set_clinit(&self, method_id: MethodId) -> Result<(), JvmError> {
        self.clinit
            .set(method_id)
            .map_err(|_| JvmError::Todo("BaseClass clinit already set".to_string()))
    }

    fn get_interfaces(&self) -> Result<&HashSet<ClassId>, JvmError> {
        self.interfaces
            .get()
            .ok_or(JvmError::Todo("BaseClass interfaces not set".to_string()))
    }

    fn set_interfaces(&self, interfaces: HashSet<ClassId>) -> Result<(), JvmError> {
        self.interfaces
            .set(interfaces)
            .map_err(|_| JvmError::Todo("BaseClass interfaces already set".to_string()))
    }

    fn set_static_fields(
        &self,
        static_fields: HashMap<FieldKey, StaticField>,
    ) -> Result<(), JvmError> {
        self.static_fields
            .set(static_fields)
            .map_err(|_| JvmError::Todo("BaseClass static_fields already set".to_string()))
    }

    fn get_static_fields(&self) -> Result<&HashMap<FieldKey, StaticField>, JvmError> {
        self.static_fields.get().ok_or(JvmError::Todo(
            "BaseClass static_fields not set".to_string(),
        ))
    }
}

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

impl Display for JvmClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JvmClass::Instance(_) => write!(f, "InstanceClass"),
            JvmClass::Interface(_) => write!(f, "InterfaceClass"),
            JvmClass::Primitive(_) => write!(f, "PrimitiveClass"),
            JvmClass::PrimitiveArray(_) => write!(f, "PrimitiveArrayClass"),
            JvmClass::InstanceArray(_) => write!(f, "ObjectArrayClass"),
        }
    }
}

//TODO: there is right now some code duplication between InstanceClass, InterfaceClass and JvmClass methods. refactor
impl JvmClass {
    const BUILTIN_CLASS_FLAGS: i32 = 0x411; // public, super, final

    pub fn as_class_like(&self) -> Result<&dyn ClassLike, JvmError> {
        match self {
            JvmClass::Instance(inst) => Ok(inst.as_ref()),
            JvmClass::Interface(i) => Ok(i.as_ref()),
            _ => Err(JvmError::Todo(
                "as_class_like not implemented for this JvmClass variant".to_string(),
            )),
        }
    }

    pub fn get_source_file(&self) -> Option<Symbol> {
        match self {
            JvmClass::Instance(inst) => inst.get_source_file(),
            JvmClass::Interface(i) => i.get_source_file(),
            _ => None,
        }
    }

    pub fn get_cp(&self) -> Result<&RuntimeConstantPool, JvmError> {
        match self {
            JvmClass::Instance(inst) => Ok(&inst.cp),
            JvmClass::Interface(i) => Ok(&i.cp),
            _ => Err(JvmError::Todo(
                "get_cp not implemented for this JvmClass variant".to_string(),
            )),
        }
    }

    pub fn get_static_field_value(&self, field_key: &FieldKey) -> Result<Value, JvmError> {
        match self {
            JvmClass::Instance(inst) => inst.get_static_field_value(field_key),
            JvmClass::Interface(i) => i.get_static_field_value(field_key),
            JvmClass::PrimitiveArray(_) => Err(JvmError::Todo(
                "PrimitiveArrayClass has no static fields".to_string(),
            )),
            JvmClass::InstanceArray(_) => Err(JvmError::Todo(
                "ObjectArrayClass has no static fields".to_string(),
            )),
            JvmClass::Primitive(_) => Err(JvmError::Todo(
                "PrimitiveClass has no static fields".to_string(),
            )),
        }
    }

    pub fn get_interfaces(&self) -> Result<&HashSet<ClassId>, JvmError> {
        match self {
            JvmClass::Instance(inst) => inst.get_interfaces(),
            JvmClass::Interface(i) => i.get_interfaces(),
            other => unimplemented!("{other}"),
        }
    }

    pub fn get_vtable_method_id(&self, key: &MethodKey) -> Result<MethodId, JvmError> {
        match self {
            JvmClass::Instance(inst) => inst.get_vtable_method_id(key),
            JvmClass::Interface(_) => todo!(),
            JvmClass::Primitive(_) => todo!(),
            JvmClass::PrimitiveArray(arr) => arr.get_vtable_method_id(key),
            JvmClass::InstanceArray(arr) => arr.get_vtable_method_id(key),
        }
    }

    // TODO: it is more like a stub right now, no guarantees that method is actually static
    pub fn get_static_method_id(&self, key: &MethodKey) -> Result<MethodId, JvmError> {
        match self {
            JvmClass::Instance(inst) => inst.get_special_method_id(key),
            JvmClass::Interface(i) => i.get_methods().get(key).copied().ok_or(JvmError::Todo(
                "No such method in InterfaceClass".to_string(),
            )),
            _ => Err(JvmError::Todo(
                "get_static_method_id not implemented for this JvmClass variant".to_string(),
            )),
        }
    }
    pub fn get_name(&self) -> Symbol {
        match self {
            JvmClass::Instance(ic) => ic.name(),
            JvmClass::Interface(i) => i.name(),
            JvmClass::PrimitiveArray(pac) => pac.name,
            JvmClass::InstanceArray(oac) => oac.name,
            JvmClass::Primitive(pc) => pc.name,
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
            JvmClass::Interface(i) => i.get_mirror_ref(),
            JvmClass::PrimitiveArray(pac) => pac.get_mirror_ref(),
            JvmClass::InstanceArray(oac) => oac.get_mirror_ref(),
            JvmClass::Primitive(pc) => pc.get_mirror_ref(),
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
            JvmClass::Instance(i) => i.get_super(),
            JvmClass::Interface(i) => i.get_super(),
            JvmClass::PrimitiveArray(arr) => Some(arr.super_id),
            JvmClass::InstanceArray(arr) => Some(arr.super_id),
            JvmClass::Primitive(_) => None,
        }
    }

    pub fn is_primitive(&self) -> bool {
        matches!(self, JvmClass::Primitive(_))
    }

    pub fn is_array(&self) -> bool {
        matches!(
            self,
            JvmClass::PrimitiveArray(_) | JvmClass::InstanceArray(_)
        )
    }

    pub fn is_interface(&self) -> bool {
        matches!(self, JvmClass::Interface(_))
    }

    pub fn get_raw_flags(&self) -> i32 {
        match self {
            JvmClass::Instance(ic) => ic.flags().get_raw_i32(),
            JvmClass::Interface(i) => i.flags().get_raw_i32(),
            _ => Self::BUILTIN_CLASS_FLAGS,
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
