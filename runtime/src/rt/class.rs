use crate::heap::method_area::MethodArea;
use crate::rt::constant_pool::RuntimeConstantPool;
use crate::rt::field::{InstanceField, StaticField};
use crate::rt::method::Method;
use crate::{ClassId, FieldId, FieldKey, MethodId, MethodKey};
use common::error::{JavaExceptionFromJvm, JvmError};
use jclass::ClassFile;
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::sync::Arc;

// TODO: something like that...
pub enum ClassState {
    Loading,     // Currently being loaded
    Loaded,      // Parsed, superclass loaded
    Linked,      // Verified, prepared
    Initialized, // <clinit> executed
}

pub struct Class {
    pub name: String, //TODO: debug, delete
    pub super_id: Option<ClassId>,
    pub declared_method_index: OnceCell<HashMap<MethodKey, MethodId>>,
    pub vtable: OnceCell<Vec<MethodId>>,
    pub vtable_index: OnceCell<HashMap<MethodKey, u16>>,

    pub instance_fields: OnceCell<Vec<InstanceField>>,
    pub instance_fields_offset_map: OnceCell<HashMap<FieldKey, u16>>,

    pub static_fields: OnceCell<HashMap<FieldKey, StaticField>>,
}

impl Class {
    pub fn load_and_link(
        cf: ClassFile,
        method_area: &mut MethodArea,
        super_id: Option<ClassId>,
    ) -> Result<ClassId, JvmError> {
        let cp = Arc::new(RuntimeConstantPool::new(cf.cp.inner));
        // class state = Loading
        let name = cp.get_class(&cf.this_class, &method_area.string_interner)?;
        let name = method_area.string_interner.resolve(&name).to_string();

        let class = Self {
            name: name.clone(),
            super_id,
            declared_method_index: OnceCell::new(),
            vtable: OnceCell::new(),
            vtable_index: OnceCell::new(),
            instance_fields: OnceCell::new(),
            instance_fields_offset_map: OnceCell::new(),
            static_fields: OnceCell::new(),
        };
        let class_id = method_area.push_class(class);
        let mut declared_index = HashMap::new();

        let (mut vtable, mut vtable_index) = super_id
            .map(|id| {
                let super_class = method_area.get_class(&id);
                (
                    super_class.get_vtable().clone(),
                    super_class.get_vtable_index().clone(),
                )
            })
            .unwrap_or_default();

        for method in cf.methods {
            let name = cp.get_utf8(&method.name_index, &method_area.string_interner)?;
            let method_key = MethodKey {
                name,
                desc: cp.get_utf8(&method.descriptor_index, &method_area.string_interner)?,
            };
            let method = Method::new(method, class_id);
            let is_static = method.is_static();
            let is_constructor = method_area.is_constructor_symbol(name);
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
            let this = method_area.get_class(&class_id);
            this.set_declared_methods(declared_index);
            this.set_vtable(vtable);
            this.set_vtable_index(vtable_index);
        }

        let mut instance_fields = super_id
            .map(|id| method_area.get_class(&id).get_instance_fields().clone())
            .unwrap_or_default();
        let mut instance_fields_offset_map = super_id
            .map(|id| {
                method_area
                    .get_class(&id)
                    .get_instance_fields_offset_map()
                    .clone()
            })
            .unwrap_or_default();
        let mut static_fields = HashMap::new();

        for field in cf.fields {
            let desc_sym = cp.get_utf8(&field.descriptor_index, &method_area.string_interner)?;
            let field_key = FieldKey {
                name: cp.get_utf8(&field.name_index, &method_area.string_interner)?,
                desc: desc_sym,
            };
            let descriptor_id = method_area.get_or_new_field_descriptor_id(&desc_sym)?;
            if field.access_flags.is_static() {
                let static_field = StaticField {
                    flags: field.access_flags,
                    value: std::cell::RefCell::new(
                        method_area
                            .get_field_descriptor(&descriptor_id)
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
            let this = method_area.get_class(&class_id);
            this.set_instance_fields(instance_fields);
            this.set_instance_fields_offset_map(instance_fields_offset_map);
            this.set_static_fields(static_fields);
        }

        Ok(class_id)
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

    fn get_instance_fields(&self) -> &Vec<InstanceField> {
        self.instance_fields.get().unwrap()
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
}
