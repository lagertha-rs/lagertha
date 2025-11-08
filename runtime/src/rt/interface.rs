use crate::heap::method_area::MethodArea;
use crate::rt::JvmClass;
use crate::rt::constant_pool::RuntimeConstantPool;
use crate::rt::field::StaticField;
use crate::rt::method::Method;
use crate::{ClassId, FieldKey, MethodId, MethodKey, Symbol};
use common::error::JvmError;
use common::jtype::HeapRef;
use jclass::ClassFile;
use jclass::constant::pool::ConstantPool;
use jclass::method::MethodInfo;
use once_cell::sync::OnceCell;
use std::collections::HashMap;

//TODO: I guess hotspot doesn't split class and interface classes. Right now we do the same
// but probably it would be better to have separate InterfaceClass struct
pub struct InterfaceClass {
    pub name: Symbol,
    pub cp: RuntimeConstantPool,
    pub super_id: Option<ClassId>,
    mirror_ref: OnceCell<HeapRef>,

    pub methods: OnceCell<HashMap<MethodKey, MethodId>>,
    pub static_fields: OnceCell<HashMap<FieldKey, StaticField>>,
}

impl InterfaceClass {
    fn load(
        cp: ConstantPool,
        method_area: &mut MethodArea,
        super_id: Option<ClassId>,
        this_class: u16,
    ) -> Result<ClassId, JvmError> {
        let cp = RuntimeConstantPool::new(cp.inner);
        let name = cp.get_class_sym(&this_class, method_area.interner())?;

        let class = JvmClass::Interface(Box::new(Self {
            cp,
            name,
            super_id,
            methods: OnceCell::new(),
            static_fields: OnceCell::new(),
            mirror_ref: OnceCell::new(),
        }));

        Ok(method_area.push_class(class))
    }

    fn link_methods(
        methods: Vec<MethodInfo>,
        this_id: ClassId,
        method_area: &mut MethodArea,
    ) -> Result<(), JvmError> {
        let mut declared_index = HashMap::new();
        for method in methods {
            // TODO: can be extracted to a common function
            let method_key = {
                let cp = &method_area.get_interface_class(&this_id)?.cp;
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
            let method_id = method_area.push_method(method);
            declared_index.insert(method_key, method_id);
        }

        let this = method_area.get_interface_class(&this_id)?;
        this.set_methods(declared_index);

        Ok(())
    }

    pub fn load_and_link(
        cf: ClassFile,
        method_area: &mut MethodArea,
        super_id: Option<ClassId>,
    ) -> Result<ClassId, JvmError> {
        let this_id = Self::load(cf.cp, method_area, super_id, cf.this_class)?;
        Self::link_methods(cf.methods, this_id, method_area)?;

        Ok(this_id)
    }

    pub fn get_super_id(&self) -> Option<ClassId> {
        self.super_id
    }

    fn set_methods(&self, methods: HashMap<MethodKey, MethodId>) {
        self.methods.set(methods).unwrap()
    }

    pub(crate) fn get_methods(&self) -> &HashMap<MethodKey, MethodId> {
        self.methods.get().unwrap()
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
