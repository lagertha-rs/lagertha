use crate::heap::method_area::MethodArea;
use crate::rt::constant_pool::RuntimeConstantPool;
use crate::rt::field::StaticField;
use crate::rt::method::Method;
use crate::rt::{BaseClass, ClassLike, JvmClass};
use crate::{ClassId, FieldKey, MethodId, MethodKey};
use common::error::JvmError;
use jclass::ClassFile;
use jclass::constant::pool::ConstantPool;
use jclass::field::FieldInfo;
use jclass::flags::ClassFlags;
use jclass::method::MethodInfo;
use once_cell::sync::OnceCell;
use std::cell::RefCell;
use std::collections::HashMap;

//TODO: I guess hotspot doesn't split class and interface classes. Right now we do the same
// but probably it would be better to have separate InterfaceClass struct
pub struct InterfaceClass {
    pub base: BaseClass,
    pub cp: RuntimeConstantPool,
    pub methods: OnceCell<HashMap<MethodKey, MethodId>>,
}

impl InterfaceClass {
    fn load(
        flags: ClassFlags,
        cp: ConstantPool,
        method_area: &mut MethodArea,
        super_id: Option<ClassId>,
        this_class: u16,
    ) -> Result<ClassId, JvmError> {
        let cp = RuntimeConstantPool::new(cp.inner);
        let name = cp.get_class_sym(&this_class, method_area.interner())?;

        //TODO: source file name? etc
        let class = JvmClass::Interface(Box::new(Self {
            base: BaseClass::new(name, flags, super_id, None),
            cp,
            methods: OnceCell::new(),
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
            if method_key.name == method_area.br().clinit_sym {
                method_area
                    .get_interface_class(&this_id)?
                    .base
                    .set_clinit(method_id)?;
            } else {
                declared_index.insert(method_key, method_id);
            }
        }

        let this = method_area.get_interface_class(&this_id)?;
        this.set_methods(declared_index);

        Ok(())
    }

    fn link_fields(
        fields: Vec<FieldInfo>,
        this_id: ClassId,
        method_area: &mut MethodArea,
    ) -> Result<(), JvmError> {
        let mut static_fields = HashMap::new();

        for field in fields {
            //TODO: assert is static?
            let field_key = {
                let cp = &method_area.get_interface_class(&this_id)?.cp;
                FieldKey {
                    name: cp.get_utf8_sym(&field.name_index, method_area.interner())?,
                    desc: cp.get_utf8_sym(&field.descriptor_index, method_area.interner())?,
                }
            };

            let descriptor_id = method_area.get_or_new_type_descriptor_id(field_key.desc)?;
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
        }

        let this = method_area.get_interface_class(&this_id)?;
        this.base.set_static_fields(static_fields)?;
        Ok(())
    }

    // TODO: copied from class.rs, can be definetely be refactored to avoid code duplication
    fn link_interfaces(
        interfaces: Vec<u16>,
        this_id: ClassId,
        super_id: Option<ClassId>,
        method_area: &mut MethodArea,
    ) -> Result<(), JvmError> {
        let mut interface_ids = super_id
            .map(|id| method_area.get_class_like(&id))
            .transpose()?
            .map(|class| class.get_interfaces().cloned())
            .transpose()?
            .unwrap_or_default();

        for interface in interfaces {
            let cp = &method_area.get_interface_class(&this_id)?.cp;
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
        let this = method_area.get_interface_class(&this_id)?;
        this.base.set_interfaces(interface_ids)?;
        Ok(())
    }

    pub fn load_and_link(
        cf: ClassFile,
        method_area: &mut MethodArea,
        super_id: Option<ClassId>,
    ) -> Result<ClassId, JvmError> {
        let this_id = Self::load(cf.access_flags, cf.cp, method_area, super_id, cf.this_class)?;
        let dbg = method_area
            .interner()
            .resolve(&method_area.get_class(&this_id).get_name());

        Self::link_methods(cf.methods, this_id, method_area)?;
        Self::link_fields(cf.fields, this_id, method_area)?;
        Self::link_interfaces(cf.interfaces, this_id, super_id, method_area)?;

        Ok(this_id)
    }

    pub fn set_methods(&self, methods: HashMap<MethodKey, MethodId>) {
        self.methods.set(methods).unwrap()
    }

    pub fn get_methods(&self) -> &HashMap<MethodKey, MethodId> {
        self.methods.get().unwrap()
    }
}

impl ClassLike for InterfaceClass {
    fn base(&self) -> &BaseClass {
        &self.base
    }
}
