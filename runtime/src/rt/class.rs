use crate::heap::method_area::MethodArea;
use crate::rt::JvmClass;
use crate::rt::constant_pool::RuntimeConstantPool;
use crate::rt::field::{InstanceField, StaticField};
use crate::rt::method::Method;
use crate::{ClassId, FieldKey, MethodId, MethodKey, Symbol};
use common::error::{JavaExceptionFromJvm, JvmError};
use common::jtype::{HeapRef, Value};
use jclass::ClassFile;
use once_cell::sync::OnceCell;
use std::cell::RefCell;
use std::collections::HashMap;

// TODO: something like that...
pub enum ClassState {
    Loaded,       // Parsed, superclass loaded
    Linked,       // Verified, prepared
    Initializing, // <clinit> in progress
    Initialized,  // <clinit> executed
}

pub struct InstanceClass {
    pub name: Symbol,

    pub cp: RuntimeConstantPool,
    pub super_id: Option<ClassId>,
    state: RefCell<ClassState>,
    mirror_ref: OnceCell<HeapRef>,

    pub declared_method_index: OnceCell<HashMap<MethodKey, MethodId>>,
    pub vtable: OnceCell<Vec<MethodId>>,
    pub vtable_index: OnceCell<HashMap<MethodKey, u16>>,

    pub instance_fields: OnceCell<Vec<InstanceField>>,
    pub instance_fields_offset_map: OnceCell<HashMap<FieldKey, u16>>,

    pub static_fields: OnceCell<HashMap<FieldKey, StaticField>>,
}

impl InstanceClass {
    pub fn load_and_link(
        cf: ClassFile,
        method_area: &mut MethodArea,
        super_id: Option<ClassId>,
    ) -> Result<ClassId, JvmError> {
        let cp = RuntimeConstantPool::new(cf.cp.inner);
        // class state = Loading
        let name = cp.get_class_sym(&cf.this_class, method_area.interner())?;

        let class = JvmClass::Instance(Box::new(Self {
            cp,
            name,
            super_id,
            state: RefCell::new(ClassState::Loaded),
            declared_method_index: OnceCell::new(),
            vtable: OnceCell::new(),
            vtable_index: OnceCell::new(),
            instance_fields: OnceCell::new(),
            instance_fields_offset_map: OnceCell::new(),
            static_fields: OnceCell::new(),
            mirror_ref: OnceCell::new(),
        }));
        let class_id = method_area.push_class(class);
        let mut declared_index = HashMap::new();

        let (mut vtable, mut vtable_index) = {
            if let Some(super_id) = super_id {
                let super_class = method_area.get_instance_class(&super_id)?;
                (
                    super_class.get_vtable().clone(),
                    super_class.get_vtable_index().clone(),
                )
            } else {
                (Vec::new(), HashMap::new())
            }
        };

        for method in cf.methods {
            let method_key = {
                let cp = &method_area.get_instance_class(&class_id)?.cp;
                MethodKey {
                    name: cp.get_utf8_sym(&method.name_index, &method_area.interner())?,
                    desc: cp.get_utf8_sym(&method.descriptor_index, &method_area.interner())?,
                }
            };
            let descriptor_id = method_area
                .get_or_new_method_descriptor_id(&method_key.desc)
                .unwrap();
            let method = Method::new(
                method,
                class_id,
                descriptor_id,
                method_key.name,
                method_key.desc,
            );
            let is_static = method.is_static();
            let is_constructor = method_key.name == method_area.br().init_sym
                || method_key.name == method_area.br().clinit_sym;
            let method_id = method_area.push_method(method);

            // TODO: need to think about private as well. Private methods should not be in vtable
            // but it can be called with invokevirtual from the same class...
            if !is_static && !is_constructor {
                if let Some(pos) = vtable_index.get(&method_key) {
                    vtable[*pos as usize] = method_id;
                } else {
                    vtable_index.insert(method_key, vtable.len() as u16);
                    vtable.push(method_id);
                }
            } else {
                declared_index.insert(method_key, method_id);
            }
        }

        {
            let this = method_area.get_instance_class(&class_id)?;
            this.set_declared_methods(declared_index);
            this.set_vtable(vtable);
            this.set_vtable_index(vtable_index);
        }

        let mut instance_fields = if let Some(super_id) = super_id {
            method_area
                .get_instance_class(&super_id)?
                .get_instance_fields()
                .clone()
        } else {
            Vec::new()
        };
        let mut instance_fields_offset_map = if let Some(super_id) = super_id {
            method_area
                .get_instance_class(&super_id)?
                .get_instance_fields_offset_map()
                .clone()
        } else {
            HashMap::new()
        };
        let mut static_fields = HashMap::new();

        for field in cf.fields {
            let field_key = {
                let cp = &method_area.get_instance_class(&class_id)?.cp;
                FieldKey {
                    name: cp.get_utf8_sym(&field.name_index, &method_area.interner())?,
                    desc: cp.get_utf8_sym(&field.descriptor_index, &method_area.interner())?,
                }
            };

            let descriptor_id = method_area.get_or_new_type_descriptor_id(field_key.desc)?;
            if field.access_flags.is_static() {
                let static_field = StaticField {
                    flags: field.access_flags,
                    value: RefCell::new(
                        method_area
                            .get_type_descriptor(&descriptor_id)
                            .get_default_value(),
                    ),
                    descriptor: descriptor_id,
                };
                static_fields.insert(field_key, static_field);
            } else {
                let cur_offset = instance_fields.len() as u16;
                instance_fields.push(InstanceField {
                    flags: field.access_flags,
                    descriptor_id,
                    offset: cur_offset,
                    declaring_class: class_id,
                });
                instance_fields_offset_map.insert(field_key, cur_offset);
            }
        }

        {
            let this = method_area.get_instance_class(&class_id)?;
            this.set_instance_fields(instance_fields);
            this.set_instance_fields_offset_map(instance_fields_offset_map);
            this.set_static_fields(static_fields);
            this.set_linked();
        }

        Ok(class_id)
    }

    pub fn set_static_field_value(
        &self,
        field_key: &FieldKey,
        value: Value,
    ) -> Result<(), JvmError> {
        let static_fields = self.static_fields.get().unwrap();
        let static_field = static_fields
            .get(field_key)
            .ok_or(JvmError::Todo("No such field".to_string()))?;
        *static_field.value.borrow_mut() = value;
        Ok(())
    }

    pub fn get_static_field_value(&self, field_key: &FieldKey) -> Result<Value, JvmError> {
        let static_fields = self.static_fields.get().unwrap();
        let static_field = static_fields
            .get(field_key)
            .ok_or(JvmError::Todo("No such field".to_string()))?;
        Ok(*static_field.value.borrow())
    }

    pub fn get_super_id(&self) -> Option<ClassId> {
        self.super_id
    }

    fn set_linked(&self) {
        *self.state.borrow_mut() = ClassState::Linked;
    }

    pub fn is_initializing(&self) -> bool {
        matches!(*self.state.borrow(), ClassState::Initializing)
    }

    pub fn set_initializing(&self) {
        *self.state.borrow_mut() = ClassState::Initializing;
    }

    pub fn set_initialized(&self) {
        *self.state.borrow_mut() = ClassState::Initialized;
    }

    pub fn is_initialized_or_initializing(&self) -> bool {
        matches!(
            *self.state.borrow(),
            ClassState::Initialized | ClassState::Initializing
        )
    }

    fn set_static_fields(&self, static_fields: HashMap<FieldKey, StaticField>) {
        self.static_fields.set(static_fields).unwrap()
    }

    fn set_instance_fields(&self, instance_fields: Vec<InstanceField>) {
        self.instance_fields.set(instance_fields).unwrap()
    }

    fn set_instance_fields_offset_map(&self, instance_fields_offset_map: HashMap<FieldKey, u16>) {
        self.instance_fields_offset_map
            .set(instance_fields_offset_map)
            .unwrap()
    }
    fn get_instance_fields_offset_map(&self) -> &HashMap<FieldKey, u16> {
        self.instance_fields_offset_map.get().unwrap()
    }

    pub fn get_instance_fields(&self) -> &Vec<InstanceField> {
        self.instance_fields.get().unwrap()
    }

    pub fn get_instance_field_offset(&self, field_key: &FieldKey) -> Result<u16, JvmError> {
        self.get_instance_fields_offset_map()
            .get(field_key)
            .copied()
            .ok_or(JvmError::Todo("No such field".to_string()))
    }

    fn set_declared_methods(&self, declared: HashMap<MethodKey, MethodId>) {
        self.declared_method_index.set(declared).unwrap()
    }

    fn set_vtable(&self, vtable: Vec<MethodId>) {
        self.vtable.set(vtable).unwrap()
    }

    fn set_vtable_index(&self, vtable_index: HashMap<MethodKey, u16>) {
        self.vtable_index.set(vtable_index).unwrap()
    }

    fn get_vtable(&self) -> &Vec<MethodId> {
        self.vtable.get().unwrap()
    }

    fn get_vtable_index(&self) -> &HashMap<MethodKey, u16> {
        self.vtable_index.get().unwrap()
    }

    pub fn get_vtable_method_id(&self, key: &MethodKey) -> Result<MethodId, JvmError> {
        let vtable_index = self.get_vtable_index();
        let pos = vtable_index
            .get(key)
            .copied()
            .ok_or(JvmError::JavaException(
                JavaExceptionFromJvm::NoSuchMethodError(None),
            ))?;
        Ok(self.get_vtable()[pos as usize])
    }

    pub fn get_special_method_id(&self, key: &MethodKey) -> Result<MethodId, JvmError> {
        self.declared_method_index
            .get()
            .unwrap()
            .get(key)
            .copied()
            .ok_or(JvmError::JavaException(
                JavaExceptionFromJvm::NoSuchMethodError(None),
            ))
    }

    pub fn is_child_of(&self, other: &ClassId) -> bool {
        if let Some(sup) = &self.super_id {
            sup == other
        } else {
            false
        }
    }

    pub fn get_mirror_ref(&self) -> Option<HeapRef> {
        self.mirror_ref.get().copied()
    }

    pub fn set_mirror_ref(&self, heap_ref: HeapRef) -> Result<(), JvmError> {
        self.mirror_ref
            .set(heap_ref)
            .map_err(|_| JvmError::Todo("Mirror ref already set".to_string()))
    }
}
