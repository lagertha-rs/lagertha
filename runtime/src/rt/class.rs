use crate::heap::method_area::MethodArea;
use crate::rt::constant_pool::RuntimeConstantPool;
use crate::rt::field::{InstanceField, StaticField};
use crate::rt::method::Method;
use crate::rt::{ClassState, JvmClass};
use crate::{ClassId, FieldKey, MethodId, MethodKey, Symbol};
use common::error::{JavaExceptionFromJvm, JvmError};
use common::jtype::{HeapRef, Value};
use jclass::ClassFile;
use jclass::constant::pool::ConstantPool;
use jclass::field::FieldInfo;
use jclass::flags::ClassFlags;
use jclass::method::MethodInfo;
use log::warn;
use once_cell::sync::OnceCell;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

//TODO: I guess hotspot doesn't split class and interface classes. Right now we do the same
// but probably it would be better to have separate InterfaceClass struct
pub struct InstanceClass {
    pub name: Symbol,
    pub flags: ClassFlags,

    pub cp: RuntimeConstantPool,
    pub super_id: Option<ClassId>,
    state: RefCell<ClassState>,
    mirror_ref: OnceCell<HeapRef>,
    interfaces: OnceCell<HashSet<ClassId>>,

    pub declared_method_index: OnceCell<HashMap<MethodKey, MethodId>>,
    pub vtable: OnceCell<Vec<MethodId>>,
    pub vtable_index: OnceCell<HashMap<MethodKey, u16>>,
    pub itable: OnceCell<HashMap<MethodKey, MethodId>>,

    // TODO: review if we need both offset maps
    pub instance_fields: OnceCell<Vec<InstanceField>>,
    pub instance_fields_offset_map: OnceCell<HashMap<FieldKey, u16>>,
    pub instance_fields_name_offset_map: OnceCell<HashMap<Symbol, u16>>,

    pub static_fields: OnceCell<HashMap<FieldKey, StaticField>>,
}

impl InstanceClass {
    fn load(
        flags: ClassFlags,
        cp: ConstantPool,
        method_area: &mut MethodArea,
        super_id: Option<ClassId>,
        this_class: u16,
    ) -> Result<ClassId, JvmError> {
        let cp = RuntimeConstantPool::new(cp.inner);
        let name = cp.get_class_sym(&this_class, method_area.interner())?;

        let class = JvmClass::Instance(Box::new(Self {
            cp,
            name,
            flags,
            super_id,
            state: RefCell::new(ClassState::Loaded),
            interfaces: OnceCell::new(),
            declared_method_index: OnceCell::new(),
            vtable: OnceCell::new(),
            vtable_index: OnceCell::new(),
            itable: OnceCell::new(),
            instance_fields: OnceCell::new(),
            instance_fields_offset_map: OnceCell::new(),
            instance_fields_name_offset_map: OnceCell::new(),
            static_fields: OnceCell::new(),
            mirror_ref: OnceCell::new(),
        }));

        Ok(method_area.push_class(class))
    }

    fn link_methods(
        methods: Vec<MethodInfo>,
        this_id: ClassId,
        super_id: Option<ClassId>,
        method_area: &mut MethodArea,
    ) -> Result<(), JvmError> {
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
                declared_index.insert(method_key, method_id);
            }
        }

        let this = method_area.get_instance_class(&this_id)?;
        this.set_declared_methods(declared_index);
        this.set_vtable(vtable);
        this.set_vtable_index(vtable_index);
        Ok(())
    }

    fn link_fields(
        fields: Vec<FieldInfo>,
        this_id: ClassId,
        super_id: Option<ClassId>,
        method_area: &mut MethodArea,
    ) -> Result<(), JvmError> {
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
        let mut instance_fields_name_offset_map = if let Some(super_id) = super_id {
            method_area
                .get_instance_class(&super_id)?
                .get_instance_fields_name_offset_map()
                .clone()
        } else {
            HashMap::new()
        };
        let mut static_fields = HashMap::new();

        for field in fields {
            let field_key = {
                let cp = &method_area.get_instance_class(&this_id)?.cp;
                FieldKey {
                    name: cp.get_utf8_sym(&field.name_index, method_area.interner())?,
                    desc: cp.get_utf8_sym(&field.descriptor_index, method_area.interner())?,
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
                    declaring_class: this_id,
                });
                instance_fields_offset_map.insert(field_key, cur_offset);
                instance_fields_name_offset_map.insert(field_key.name, cur_offset);
            }
        }

        let this = method_area.get_instance_class(&this_id)?;
        this.set_instance_fields(instance_fields);
        this.set_instance_fields_offset_map(instance_fields_offset_map);
        this.set_instance_fields_name_offset_map(instance_fields_name_offset_map);
        this.set_static_fields(static_fields);
        Ok(())
    }

    fn link_interfaces(
        interfaces: Vec<u16>,
        this_id: ClassId,
        super_id: Option<ClassId>,
        method_area: &mut MethodArea,
    ) -> Result<(), JvmError> {
        let mut interface_ids = if let Some(super_id) = super_id {
            method_area
                .get_instance_class(&super_id)?
                .get_interfaces()
                .clone()
        } else {
            HashSet::new()
        };

        for interface in interfaces {
            let cp = &method_area.get_instance_class(&this_id)?.cp;
            let interface_name = cp.get_class_sym(&interface, method_area.interner())?;
            let interface_id = method_area.get_class_id_or_load(interface_name)?;
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
        this.interfaces.set(interface_ids).unwrap();
        Ok(())
    }

    fn link_itable(
        this_id: ClassId,
        super_id: Option<ClassId>,
        method_area: &mut MethodArea,
    ) -> Result<(), JvmError> {
        let mut itable = if let Some(super_id) = super_id {
            method_area
                .get_instance_class(&super_id)?
                .get_itable()
                .clone()
        } else {
            HashMap::new()
        };

        for interface in method_area.get_instance_class(&this_id)?.get_interfaces() {
            let interface_class = method_area.get_interface_class(interface)?;
            let interface_methods = interface_class.get_methods();
            for (method_key, method_id) in interface_methods {
                if !method_area.get_method(method_id).is_abstract() {
                    warn!(
                        "Skipping non-abstract interface method in itable linking, not supported yet"
                    );
                    continue;
                }
                let impl_method_id = method_area
                    .get_instance_class(&this_id)?
                    .get_vtable_method_id(method_key)?;
                if method_area.get_method(&impl_method_id).is_abstract() {
                    if !method_area
                        .get_instance_class(&this_id)?
                        .flags
                        .is_abstract()
                    {
                        Err(JvmError::Todo(format!(
                            "Class {} does not implement interface method {} {}",
                            method_area
                                .interner()
                                .resolve(&method_area.get_instance_class(&this_id)?.name),
                            method_area.interner().resolve(&method_key.name),
                            method_area.interner().resolve(&method_key.desc)
                        )))?;
                    }
                    continue;
                }
                itable.insert(*method_key, impl_method_id);
            }
        }

        let this = method_area.get_instance_class(&this_id)?;
        this.set_itable(itable);
        Ok(())
    }

    pub fn load_and_link(
        cf: ClassFile,
        method_area: &mut MethodArea,
        super_id: Option<ClassId>,
    ) -> Result<ClassId, JvmError> {
        let this_id = Self::load(cf.access_flags, cf.cp, method_area, super_id, cf.this_class)?;
        let debug_name = method_area
            .interner()
            .resolve(&method_area.get_instance_class(&this_id)?.name);

        Self::link_fields(cf.fields, this_id, super_id, method_area)?;
        Self::link_methods(cf.methods, this_id, super_id, method_area)?;
        Self::link_interfaces(cf.interfaces, this_id, super_id, method_area)?;
        Self::link_itable(this_id, super_id, method_area)?;

        let this = method_area.get_instance_class(&this_id)?;
        this.set_linked();
        Ok(this_id)
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

    pub fn get_interface_method_id(&self, key: &MethodKey) -> Result<MethodId, JvmError> {
        self.get_itable()
            .get(key)
            .copied()
            .ok_or(JvmError::JavaException(
                JavaExceptionFromJvm::NoSuchMethodError(None),
            ))
    }

    pub fn print_vtable(&self, method_area: &MethodArea) {
        println!(
            "VTable for class {}",
            method_area.interner().resolve(&self.name)
        );
        for (method_key, pos) in self.get_vtable_index() {
            let method_id = self.get_vtable()[*pos as usize];
            println!(
                "  {} {} -> Method ID: {:?}",
                method_area.interner().resolve(&method_key.name),
                method_area.interner().resolve(&method_key.desc),
                method_id
            );
        }
    }

    pub fn print_itable(&self, method_area: &MethodArea) {
        println!(
            "ITable for class {}",
            method_area.interner().resolve(&self.name)
        );
        for (method_key, method_id) in self.get_itable() {
            println!(
                "  {} {} -> Method ID: {:?}",
                method_area.interner().resolve(&method_key.name),
                method_area.interner().resolve(&method_key.desc),
                method_id
            );
        }
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

    fn set_interfaces(&self, interfaces: HashSet<ClassId>) {
        self.interfaces.set(interfaces).unwrap()
    }

    fn get_interfaces(&self) -> &HashSet<ClassId> {
        self.interfaces.get().unwrap()
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

    fn get_instance_fields_name_offset_map(&self) -> &HashMap<Symbol, u16> {
        self.instance_fields_name_offset_map.get().unwrap()
    }

    fn set_instance_fields_name_offset_map(
        &self,
        instance_fields_name_offset_map: HashMap<Symbol, u16>,
    ) {
        self.instance_fields_name_offset_map
            .set(instance_fields_name_offset_map)
            .unwrap()
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

    pub fn get_instance_field_offset_by_name(&self, field_name: &Symbol) -> Result<u16, JvmError> {
        self.get_instance_fields_name_offset_map()
            .get(field_name)
            .copied()
            .ok_or(JvmError::Todo("No such field".to_string()))
    }

    fn set_declared_methods(&self, declared: HashMap<MethodKey, MethodId>) {
        self.declared_method_index.set(declared).unwrap()
    }

    fn get_declared_methods(&self) -> &HashMap<MethodKey, MethodId> {
        self.declared_method_index.get().unwrap()
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

    fn get_itable(&self) -> &HashMap<MethodKey, MethodId> {
        self.itable.get().unwrap()
    }

    fn set_itable(&self, itable: HashMap<MethodKey, MethodId>) {
        self.itable.set(itable).unwrap()
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
        if let Some(method_id) = self.get_declared_methods().get(key) {
            return Ok(*method_id);
        }
        if let Some(method_id) = self.get_vtable_index().get(key) {
            return Ok(self.get_vtable()[*method_id as usize]);
        }
        Err(JvmError::JavaException(
            JavaExceptionFromJvm::NoSuchMethodError(None),
        ))
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
