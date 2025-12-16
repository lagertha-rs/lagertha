use crate::error::JvmError;
use crate::heap::method_area::MethodArea;
use crate::keys::{ClassId, FieldKey, MethodKey, ThreadId};
use crate::rt::constant_pool::RuntimeConstantPool;
use crate::rt::field::{InstanceField, StaticField};
use crate::rt::method::Method;
use crate::rt::{BaseClass, ClassLike, JvmClass};
use crate::{MethodId, Symbol, build_exception, throw_exception};
use jclass::ClassFile;
use jclass::attribute::class::ClassAttr;
use jclass::constant::pool::ConstantPool;
use jclass::field::FieldInfo;
use jclass::flags::ClassFlags;
use jclass::method::MethodInfo;
use once_cell::sync::OnceCell;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::RwLock;

pub struct InstanceClass {
    base: BaseClass,

    pub cp: RuntimeConstantPool,

    pub declared_method_index: OnceCell<HashMap<MethodKey, MethodId>>,
    pub vtable: OnceCell<Vec<MethodId>>,
    pub vtable_index: OnceCell<HashMap<MethodKey, u16>>,
    pub itable: OnceCell<HashMap<MethodKey, MethodId>>,

    // TODO: review if we need both offset maps
    pub instance_fields: OnceCell<Vec<InstanceField>>,
    pub instance_fields_offset_map: OnceCell<HashMap<FieldKey, usize>>,
    pub instance_fields_name_offset_map: OnceCell<HashMap<Symbol, usize>>,

    instance_size: OnceCell<usize>,
}

impl InstanceClass {
    fn load(
        super_id: Option<ClassId>,
        method_area: &mut MethodArea,
        flags: ClassFlags,
        cp: RuntimeConstantPool,
        this_class: u16,
        attributes: Vec<ClassAttr>,
    ) -> Result<ClassId, JvmError> {
        let name = cp.get_class_sym(&this_class, method_area.interner())?;

        //TODO: clean up
        let mut source_file = None;
        for attr in &attributes {
            if let ClassAttr::SourceFile(sourcefile_index) = attr {
                source_file = Some(cp.get_utf8_sym(sourcefile_index, method_area.interner())?);
                break;
            }
        }

        let class = JvmClass::Instance(Box::new(Self {
            base: BaseClass::new(name, flags, super_id, source_file),
            cp,
            declared_method_index: OnceCell::new(),
            vtable: OnceCell::new(),
            vtable_index: OnceCell::new(),
            itable: OnceCell::new(),
            instance_fields: OnceCell::new(),
            instance_fields_offset_map: OnceCell::new(),
            instance_fields_name_offset_map: OnceCell::new(),
            instance_size: OnceCell::new(),
        }));

        Ok(method_area.push_class(class))
    }

    // TODO: needs clean up
    fn prepare_methods(
        methods: Vec<MethodInfo>,
        this_id: ClassId,
        super_id: Option<ClassId>,
        method_area: &mut MethodArea,
    ) -> Result<(Vec<MethodId>, HashMap<MethodKey, u16>), JvmError> {
        let mut declared_index = HashMap::new();
        let (mut vtable, mut vtable_index) = super_id
            .map(|id| method_area.get_instance_class(&id))
            .transpose()?
            .map(|class| -> Result<_, JvmError> {
                Ok((
                    class.get_vtable()?.clone(),
                    class.get_vtable_index()?.clone(),
                ))
            })
            .transpose()?
            .unwrap_or_default();

        for method in methods {
            let method_key = {
                let cp = &method_area.get_instance_class(&this_id)?.cp;
                MethodKey {
                    name: cp.get_utf8_sym(&method.name_index, method_area.interner())?,
                    desc: cp.get_utf8_sym(&method.descriptor_index, method_area.interner())?,
                }
            };
            let descriptor_id = method_area
                .get_or_new_method_descriptor_id(&method_key.desc)
                .unwrap();
            let method = Method::new(
                method,
                this_id,
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
                if method_key.name == method_area.br().clinit_sym {
                    method_area
                        .get_instance_class(&this_id)?
                        .base
                        .set_clinit(method_id)?;
                } else {
                    declared_index.insert(method_key, method_id);
                }
            }
        }

        let this = method_area.get_instance_class(&this_id)?;
        this.set_declared_methods(declared_index)?;
        Ok((vtable, vtable_index))
    }

    fn link_fields(
        fields: Vec<FieldInfo>,
        this_id: ClassId,
        super_id: Option<ClassId>,
        method_area: &mut MethodArea,
    ) -> Result<(), JvmError> {
        let mut instance_fields = super_id
            .map(|id| method_area.get_instance_class(&id))
            .transpose()?
            .map(|class| class.get_instance_fields().cloned())
            .transpose()?
            .unwrap_or_default();
        let mut instance_fields_offset_map = super_id
            .map(|id| method_area.get_instance_class(&id))
            .transpose()?
            .map(|class| class.get_instance_fields_offset_map().cloned())
            .transpose()?
            .unwrap_or_default();
        let mut instance_fields_name_offset_map = super_id
            .map(|id| method_area.get_instance_class(&id))
            .transpose()?
            .map(|class| class.get_instance_fields_name_offset_map().cloned())
            .transpose()?
            .unwrap_or_default();
        let mut instance_size = super_id
            .map(|id| method_area.get_instance_class(&id))
            .transpose()?
            .map(|class| class.get_instance_size())
            .transpose()?
            .unwrap_or_default();
        let mut static_fields = HashMap::new();

        for field in fields {
            let field_key = {
                let cp = &method_area.get_instance_class(&this_id)?.cp;
                FieldKey {
                    name: cp.get_utf8_sym(&field.name_index, method_area.interner())?,
                    desc: cp.get_utf8_sym(&field.descriptor_index, method_area.interner())?,
                }
            };

            let descriptor_id = method_area.get_or_new_field_descriptor_id(field_key.desc)?;
            let descriptor = method_area.get_field_descriptor(&descriptor_id);

            if field.access_flags.is_static() {
                let static_field = StaticField {
                    flags: field.access_flags,
                    value: RwLock::new(descriptor.into()),
                    descriptor: descriptor_id,
                };
                static_fields.insert(field_key, static_field);
            } else {
                let size = descriptor.as_allocation_type().byte_size();
                instance_size = (instance_size + size - 1) & !(size - 1);

                let instance_offset = instance_size;
                let position = instance_fields.len();

                instance_size += size;

                instance_fields.push(InstanceField {
                    flags: field.access_flags,
                    descriptor_id,
                    offset: instance_offset,
                    declaring_class: this_id,
                });
                instance_fields_offset_map.insert(field_key, position);
                instance_fields_name_offset_map.insert(field_key.name, position);
            }
        }

        let this = method_area.get_instance_class(&this_id)?;
        this.set_instance_fields(instance_fields)?;
        this.set_instance_fields_offset_map(instance_fields_offset_map)?;
        this.set_instance_fields_name_offset_map(instance_fields_name_offset_map)?;
        this.set_instance_size(instance_size)?;
        this.base.set_static_fields(static_fields)?;
        Ok(())
    }

    fn link_interfaces(
        interfaces: Vec<u16>,
        this_id: ClassId,
        super_id: Option<ClassId>,
        method_area: &mut MethodArea,
        thread_id: ThreadId,
    ) -> Result<(), JvmError> {
        let mut interface_ids = super_id
            .map(|id| method_area.get_instance_class(&id))
            .transpose()?
            .map(|class| class.base.get_interfaces().cloned())
            .transpose()?
            .unwrap_or_default();

        for interface in interfaces {
            let cp = &method_area.get_instance_class(&this_id)?.cp;
            let interface_name = cp.get_class_sym(&interface, method_area.interner())?;
            let interface_id = method_area.get_class_id_or_load(interface_name, thread_id)?;
            interface_ids.insert(interface_id);

            /* TODO: probably need to handle superinterfaces as well
                something like:
                if let Ok(interface_class) = method_area.get_interface_class(&interface_id) {
                for super_interface_id in interface_class.get_super_interfaces() {
                    interface_ids.insert(*super_interface_id);
                }
            }
                 */
        }
        let this = method_area.get_instance_class(&this_id)?;
        this.base.set_interfaces(interface_ids)?;
        Ok(())
    }

    fn link_itable_and_vtable(
        this_id: ClassId,
        super_id: Option<ClassId>,
        method_area: &mut MethodArea,
        mut vtable: Vec<MethodId>,
        mut vtable_index: HashMap<MethodKey, u16>,
    ) -> Result<(), JvmError> {
        let mut itable = super_id
            .map(|id| method_area.get_instance_class(&id))
            .transpose()?
            .map(|class| class.get_itable().cloned())
            .transpose()?
            .unwrap_or_default();

        for interface in method_area
            .get_instance_class(&this_id)?
            .base
            .get_interfaces()?
        {
            let interface_class = method_area.get_interface_class(interface)?;
            let interface_methods = interface_class.get_methods();
            for (method_key, method_id) in interface_methods {
                let impl_method_id = match vtable_index.get(method_key) {
                    Some(&idx) => vtable[idx as usize],
                    None => {
                        // not in vtable yet
                        if !method_area.get_method(method_id).is_abstract() {
                            // Default method - add to vtable
                            let idx = vtable.len() as u16;
                            vtable_index.insert(*method_key, idx);
                            vtable.push(*method_id);
                            *method_id
                        } else {
                            // Abstract - class must be abstract or it's an error
                            if !method_area
                                .get_instance_class(&this_id)?
                                .base
                                .flags
                                .is_abstract()
                            {
                                return Err(JvmError::Todo(format!(
                                    "Concrete class {} does not implement interface method {} {}",
                                    method_area.interner().resolve(
                                        &method_area.get_instance_class(&this_id)?.base.name
                                    ),
                                    method_area.interner().resolve(&method_key.name),
                                    method_area.interner().resolve(&method_key.desc)
                                )));
                            }
                            continue;
                        }
                    }
                };
                itable.insert(*method_key, impl_method_id);
            }
        }

        let this = method_area.get_instance_class(&this_id)?;
        this.set_itable(itable)?;
        this.set_vtable(vtable)?;
        this.set_vtable_index(vtable_index)?;
        Ok(())
    }

    fn prepare_cp(cp: ConstantPool, attr: &mut Vec<ClassAttr>) -> RuntimeConstantPool {
        let methods = attr
            .iter()
            .position(|a| matches!(a, ClassAttr::BootstrapMethods(_)))
            .map(|pos| match attr.remove(pos) {
                ClassAttr::BootstrapMethods(m) => m,
                _ => unreachable!(),
            })
            .unwrap_or_default();

        RuntimeConstantPool::new(cp.inner, methods)
    }

    pub fn load_and_link(
        mut cf: ClassFile,
        method_area: &mut MethodArea,
        super_id: Option<ClassId>,
        thread_id: ThreadId,
    ) -> Result<ClassId, JvmError> {
        let runtime_cp = Self::prepare_cp(cf.cp, &mut cf.attributes);
        let this_id = Self::load(
            super_id,
            method_area,
            cf.access_flags,
            runtime_cp,
            cf.this_class,
            cf.attributes,
        )?;

        Self::link_fields(cf.fields, this_id, super_id, method_area)?;
        let (vtable, vtable_index) =
            Self::prepare_methods(cf.methods, this_id, super_id, method_area)?;
        Self::link_interfaces(cf.interfaces, this_id, super_id, method_area, thread_id)?;
        Self::link_itable_and_vtable(this_id, super_id, method_area, vtable, vtable_index)?;

        let this = method_area.get_instance_class(&this_id)?;
        this.set_linked();
        Ok(this_id)
    }

    /** TODO: need to check actually if it is correct, seems yes for example:
     * Map.of().getClass();
     * it uses invokeinterface, but the method is in vtable inherited from Object
     */
    pub fn get_interface_method_id(&self, key: &MethodKey) -> Result<MethodId, JvmError> {
        if let Some(interface_method_id) = self.get_itable()?.get(key) {
            return Ok(*interface_method_id);
        }
        self.get_vtable_method_id(key)
    }

    pub fn get_instance_field(&self, field_key: &FieldKey) -> Result<&InstanceField, JvmError> {
        let idx = self
            .get_instance_fields_offset_map()?
            .get(field_key)
            .copied()
            .ok_or(JvmError::Todo("No such field".to_string()))?;
        Ok(&self.get_instance_fields()?[idx])
    }

    pub fn get_instance_field_by_name(
        &self,
        field_name: &Symbol,
    ) -> Result<&InstanceField, JvmError> {
        let idx = self
            .get_instance_fields_name_offset_map()?
            .get(field_name)
            .copied()
            .ok_or(JvmError::Todo("No such field".to_string()))?;
        Ok(&self.get_instance_fields()?[idx])
    }

    pub fn get_vtable_method_id(&self, key: &MethodKey) -> Result<MethodId, JvmError> {
        let vtable_index = self.get_vtable_index()?;
        let pos = vtable_index
            .get(key)
            .copied()
            .ok_or(build_exception!(NoSuchMethodError, method_key: *key, class_sym: self.name()))?;
        Ok(self.get_vtable()?[pos as usize])
    }

    pub fn get_special_method_id(&self, key: &MethodKey) -> Result<MethodId, JvmError> {
        if let Some(id) = self.get_special_method_id_opt(key) {
            return Ok(id);
        }
        throw_exception!(NoSuchMethodError, method_key: *key, class_sym: self.name())
    }

    pub fn get_special_method_id_opt(&self, key: &MethodKey) -> Option<MethodId> {
        if let Some(method_id) = self.get_declared_methods().ok()?.get(key) {
            return Some(*method_id);
        }
        if let Some(method_id) = self.get_vtable_index().ok()?.get(key) {
            return Some(self.get_vtable().ok()?[*method_id as usize]);
        }
        None
    }

    // Internal getters and setters for "lazy" initialized fields
    // mostly because I need to know this class id during linking

    // pub(crate) because array classes need to access Object class vtable
    pub(crate) fn get_vtable(&self) -> Result<&Vec<MethodId>, JvmError> {
        self.vtable
            .get()
            .ok_or(JvmError::Todo("Vtable not initialized yet".to_string()))
    }

    // pub(crate) because array classes need to access Object class vtable_index
    pub(crate) fn get_vtable_index(&self) -> Result<&HashMap<MethodKey, u16>, JvmError> {
        self.vtable_index.get().ok_or(JvmError::Todo(
            "Vtable index not initialized yet".to_string(),
        ))
    }

    fn set_declared_methods(
        &self,
        declared_index: HashMap<MethodKey, MethodId>,
    ) -> Result<(), JvmError> {
        self.declared_method_index
            .set(declared_index)
            .map_err(|_| JvmError::Todo("Declared methods already initialized".to_string()))
    }

    fn get_declared_methods(&self) -> Result<&HashMap<MethodKey, MethodId>, JvmError> {
        self.declared_method_index.get().ok_or(JvmError::Todo(
            "Declared methods not initialized yet".to_string(),
        ))
    }

    fn set_vtable(&self, vtable: Vec<MethodId>) -> Result<(), JvmError> {
        self.vtable
            .set(vtable)
            .map_err(|_| JvmError::Todo("Vtable already initialized".to_string()))
    }

    fn set_vtable_index(&self, vtable_index: HashMap<MethodKey, u16>) -> Result<(), JvmError> {
        self.vtable_index
            .set(vtable_index)
            .map_err(|_| JvmError::Todo("Vtable index already initialized".to_string()))
    }

    pub(crate) fn get_instance_fields(&self) -> Result<&Vec<InstanceField>, JvmError> {
        self.instance_fields.get().ok_or(JvmError::Todo(
            "Instance fields not initialized yet".to_string(),
        ))
    }

    pub fn get_instance_size(&self) -> Result<usize, JvmError> {
        self.instance_size.get().copied().ok_or(JvmError::Todo(
            "Instance size not initialized yet".to_string(),
        ))
    }

    fn set_instance_size(&self, size: usize) -> Result<(), JvmError> {
        self.instance_size
            .set(size)
            .map_err(|_| JvmError::Todo("Instance size already initialized".to_string()))
    }

    fn set_instance_fields(&self, instance_fields: Vec<InstanceField>) -> Result<(), JvmError> {
        self.instance_fields
            .set(instance_fields)
            .map_err(|_| JvmError::Todo("Instance fields already initialized".to_string()))
    }

    fn get_instance_fields_offset_map(&self) -> Result<&HashMap<FieldKey, usize>, JvmError> {
        self.instance_fields_offset_map.get().ok_or(JvmError::Todo(
            "Instance fields offset map not initialized yet".to_string(),
        ))
    }

    fn set_instance_fields_offset_map(
        &self,
        instance_fields_offset_map: HashMap<FieldKey, usize>,
    ) -> Result<(), JvmError> {
        self.instance_fields_offset_map
            .set(instance_fields_offset_map)
            .map_err(|_| {
                JvmError::Todo("Instance fields offset map already initialized".to_string())
            })
    }

    fn get_instance_fields_name_offset_map(&self) -> Result<&HashMap<Symbol, usize>, JvmError> {
        self.instance_fields_name_offset_map
            .get()
            .ok_or(JvmError::Todo(
                "Instance fields name offset map not initialized yet".to_string(),
            ))
    }

    fn set_instance_fields_name_offset_map(
        &self,
        instance_fields_name_offset_map: HashMap<Symbol, usize>,
    ) -> Result<(), JvmError> {
        self.instance_fields_name_offset_map
            .set(instance_fields_name_offset_map)
            .map_err(|_| {
                JvmError::Todo("Instance fields name offset map already initialized".to_string())
            })
    }

    fn get_itable(&self) -> Result<&HashMap<MethodKey, MethodId>, JvmError> {
        self.itable
            .get()
            .ok_or(JvmError::Todo("Itable not initialized yet".to_string()))
    }

    fn set_itable(&self, itable: HashMap<MethodKey, MethodId>) -> Result<(), JvmError> {
        self.itable
            .set(itable)
            .map_err(|_| JvmError::Todo("Itable already initialized".to_string()))
    }
}

impl ClassLike for InstanceClass {
    fn base(&self) -> &BaseClass {
        &self.base
    }
}
