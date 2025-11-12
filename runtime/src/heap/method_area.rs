use crate::bootstrap_registry::BootstrapRegistry;
use crate::class_loader::ClassLoader;
use crate::heap::Heap;
use crate::rt::array::{ObjectArrayClass, PrimitiveArrayClass};
use crate::rt::class::InstanceClass;
use crate::rt::constant_pool::RuntimeConstantPool;
use crate::rt::interface::InterfaceClass;
use crate::rt::method::Method;
use crate::rt::{ClassLike, JvmClass, PrimitiveClass};
use crate::{
    ClassId, FieldKey, FullyQualifiedMethodKey, MethodDescriptorId, MethodId, Symbol,
    TypeDescriptorId, VmConfig, debug_log,
};
use common::descriptor::MethodDescriptor;
use common::error::{JvmError, LinkageError, MethodDescriptorErr};
use common::jtype::{DescriptorType, HeapRef, PrimitiveType, Value};
use jclass::ClassFile;
use lasso::{Spur, ThreadedRodeo};
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::sync::Arc;

pub struct MethodArea {
    bootstrap_class_loader: ClassLoader,
    class_name_to_index: HashMap<Spur, ClassId>,
    mirror_to_class_index: HashMap<HeapRef, ClassId>,
    classes: Vec<JvmClass>,
    methods: Vec<Method>,

    type_descriptors: Vec<DescriptorType>,
    type_descriptors_index: HashMap<Symbol, TypeDescriptorId>,

    method_descriptors: Vec<MethodDescriptor>,
    method_descriptors_index: HashMap<Symbol, MethodDescriptorId>,

    interner: Arc<ThreadedRodeo>,
    bootstrap_registry: BootstrapRegistry,
}

#[cfg_attr(feature = "hotpath", hotpath::measure_all)]
impl MethodArea {
    pub fn new(
        vm_config: &VmConfig,
        string_interner: Arc<ThreadedRodeo>,
    ) -> Result<Self, JvmError> {
        debug_log!("Creating Method Area...");
        let bootstrap_class_loader = ClassLoader::new(vm_config)?;

        let mut method_area = Self {
            bootstrap_class_loader,
            class_name_to_index: HashMap::new(),
            mirror_to_class_index: HashMap::new(),
            classes: Vec::new(),
            methods: Vec::new(),
            type_descriptors: Vec::new(),
            type_descriptors_index: HashMap::new(),
            method_descriptors: Vec::new(),
            method_descriptors_index: HashMap::new(),
            bootstrap_registry: BootstrapRegistry::new(&string_interner),
            interner: string_interner,
        };

        let java_lang_class_id =
            method_area.get_class_id_or_load(method_area.br().java_lang_class_sym)?;
        let java_lang_object_id =
            method_area.get_class_id_or_load(method_area.br().java_lang_object_sym)?;
        let java_lang_throwable_id =
            method_area.get_class_id_or_load(method_area.br().java_lang_throwable_sym)?;

        for primitive_type in PrimitiveType::values() {
            let name_sym = method_area.br().get_primitive_sym(primitive_type);
            let primitive_class =
                JvmClass::Primitive(PrimitiveClass::new(name_sym, *primitive_type));
            let class_id = method_area.push_class(primitive_class);
            method_area.class_name_to_index.insert(name_sym, class_id);
        }

        method_area
            .bootstrap_registry
            .set_java_lang_class_id(java_lang_class_id)?;
        method_area
            .bootstrap_registry
            .set_java_lang_object_id(java_lang_object_id)?;
        method_area
            .bootstrap_registry
            .set_java_lang_throwable_id(java_lang_throwable_id)?;

        Ok(method_area)
    }

    pub fn br(&self) -> &BootstrapRegistry {
        &self.bootstrap_registry
    }

    pub fn interner(&self) -> &ThreadedRodeo {
        &self.interner
    }

    pub fn build_fully_qualified_native_method_key(
        &self,
        method_id: &MethodId,
    ) -> FullyQualifiedMethodKey {
        let method = self.get_method(method_id);
        let name = match self.get_class(&method.class_id()) {
            JvmClass::Instance(instance) => instance.name(),
            _ => panic!("Not an instance class"),
        };
        FullyQualifiedMethodKey::new(name, method.name, method.desc)
    }

    fn push_type_descriptor(&mut self, ty: DescriptorType) -> TypeDescriptorId {
        self.type_descriptors.push(ty);
        TypeDescriptorId::from_usize(self.type_descriptors.len())
    }

    pub fn get_type_descriptor(&self, id: &TypeDescriptorId) -> &DescriptorType {
        &self.type_descriptors[id.to_index()]
    }

    fn push_method_descriptor(&mut self, descriptor: MethodDescriptor) -> MethodDescriptorId {
        self.method_descriptors.push(descriptor);
        MethodDescriptorId::from_usize(self.method_descriptors.len())
    }

    pub fn get_method_descriptor(&self, id: &MethodDescriptorId) -> &MethodDescriptor {
        &self.method_descriptors[id.to_index()]
    }

    pub fn get_method_descriptor_by_method_id(&self, method_id: &MethodId) -> &MethodDescriptor {
        let method = self.get_method(method_id);
        self.get_method_descriptor(&method.descriptor_id())
    }

    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn get_or_new_method_descriptor_id(
        &mut self,
        descriptor: &Symbol,
    ) -> Result<MethodDescriptorId, MethodDescriptorErr> {
        if let Some(method_desc) = self.method_descriptors_index.get(descriptor) {
            return Ok(*method_desc);
        }
        let descriptor_str = self.interner.resolve(descriptor);
        let method_descriptor = MethodDescriptor::try_from(descriptor_str)?;
        Ok(self.push_method_descriptor(method_descriptor))
    }

    pub fn get_or_new_type_descriptor_id(
        &mut self,
        descriptor: Symbol,
    ) -> Result<TypeDescriptorId, JvmError> {
        if let Some(type_desc) = self.type_descriptors_index.get(&descriptor) {
            return Ok(*type_desc);
        }
        let descriptor_str = self.interner.resolve(&descriptor);
        let ty = DescriptorType::try_from(descriptor_str)?;
        Ok(self.push_type_descriptor(ty))
    }

    pub fn push_method(&mut self, method: Method) -> MethodId {
        self.methods.push(method);
        MethodId::from_usize(self.methods.len())
    }

    pub fn get_method(&self, method_id: &MethodId) -> &Method {
        &self.methods[method_id.to_index()]
    }

    pub fn push_class(&mut self, class: JvmClass) -> ClassId {
        self.classes.push(class);
        ClassId::from_usize(self.classes.len())
    }

    pub fn get_class(&self, class_id: &ClassId) -> &JvmClass {
        &self.classes[class_id.to_index()]
    }

    pub fn is_instance_class(&self, class_id: &ClassId) -> bool {
        matches!(self.get_class(class_id), JvmClass::Instance(_))
    }

    pub fn get_instance_class(&self, class_id: &ClassId) -> Result<&InstanceClass, JvmError> {
        match self.get_class(class_id) {
            JvmClass::Instance(ic) => Ok(ic),
            _ => Err(JvmError::NotAJavaInstanceTodo(
                "Not an instance class".to_string(),
            )),
        }
    }

    pub fn get_interface_class(&self, class_id: &ClassId) -> Result<&InterfaceClass, JvmError> {
        match self.get_class(class_id) {
            JvmClass::Interface(ic) => Ok(ic),
            _ => Err(JvmError::Todo("Not an interface class".to_string())),
        }
    }

    pub fn get_class_like(&self, class_id: &ClassId) -> Result<&dyn ClassLike, JvmError> {
        self.get_class(class_id).as_class_like()
    }

    pub fn get_cp(&self, class_id: &ClassId) -> Result<&RuntimeConstantPool, JvmError> {
        self.get_class(class_id).get_cp()
    }

    fn search_interfaces_for_field(
        &self,
        class_id: ClassId,
        field_key: &FieldKey,
    ) -> Result<ClassId, JvmError> {
        let class = self.get_instance_class(&class_id)?;

        for interface_id in class.get_interfaces()? {
            if let Ok(result) = self.resolve_static_field_actual_class_id(*interface_id, field_key)
            {
                return Ok(result);
            }
        }

        Err(JvmError::Todo(format!(
            "Static field {:?} not found in interfaces of {:?}",
            field_key, class_id
        )))
    }

    pub fn resolve_static_field_actual_class_id(
        &self,
        class_id: ClassId,
        field_key: &FieldKey,
    ) -> Result<ClassId, JvmError> {
        match self.get_class(&class_id) {
            JvmClass::Instance(inst) => {
                let mut cur_id = Some(class_id);

                while let Some(id) = cur_id {
                    let class = self.get_instance_class(&id)?;
                    if class.has_static_field(field_key)? {
                        return Ok(id);
                    }
                    cur_id = class.get_super()
                }

                self.search_interfaces_for_field(class_id, field_key)
            }
            JvmClass::Interface(interface) => {
                if interface.has_static_field(field_key)? {
                    return Ok(class_id);
                }
                // TODO: super interfaces?
                Err(JvmError::Todo(format!(
                    "Static field {:?} not found in interface {:?}",
                    field_key, class_id
                )))
            }
            _ => Err(JvmError::Todo(
                "Not an instance or interface class".to_string(),
            )),
        }
    }

    pub fn get_cp_by_method_id(
        &self,
        method_id: &MethodId,
    ) -> Result<&RuntimeConstantPool, JvmError> {
        let class_id = self.get_method(method_id).class_id();
        self.get_cp(&class_id)
    }

    pub(crate) fn load_array_class(&mut self, name_sym: Symbol) -> Result<ClassId, JvmError> {
        let type_descriptor_id = self.get_or_new_type_descriptor_id(name_sym)?;
        let type_descriptor = self.get_type_descriptor(&type_descriptor_id);
        let obj_class_id = self.br().get_java_lang_object_id()?;
        let vtable = self
            .get_instance_class(&obj_class_id)?
            .get_vtable()?
            .clone();
        let vtable_index = self
            .get_instance_class(&obj_class_id)?
            .get_vtable_index()?
            .clone();

        let class = if let Some(primitive_type) = type_descriptor.get_primitive_array_element_type()
        {
            JvmClass::PrimitiveArray(PrimitiveArrayClass {
                name: name_sym,
                super_id: self.br().get_java_lang_object_id()?,
                element_type: primitive_type,
                vtable,
                vtable_index,
                mirror_ref: OnceCell::new(),
            })
        } else if let Some(instance_type) = type_descriptor.get_instance_array_element_type() {
            JvmClass::InstanceArray(ObjectArrayClass {
                name: name_sym,
                super_id: self.br().get_java_lang_object_id()?,
                element_class_id: self
                    .get_class_id_or_load(self.interner.get_or_intern(instance_type))?,
                vtable,
                vtable_index,
                mirror_ref: OnceCell::new(),
            })
        } else {
            Err(JvmError::Todo(
                "Array class with non-array or non-primitive type descriptor".to_string(),
            ))?
        };
        let class_id = self.push_class(class);
        self.class_name_to_index.insert(name_sym, class_id);
        Ok(class_id)
    }

    fn is_subclass_of(&self, this_class: ClassId, target_class: ClassId) -> bool {
        if this_class == target_class {
            return true;
        }

        let this = self.get_class(&this_class);

        if let Some(super_id) = this.get_super_id() {
            if self.is_subclass_of(super_id, target_class) {
                return true;
            }
        }

        // TODO: check interfaces?
        /*
        for interface_id in this.get_interfaces() {
            if self.is_subclass_of(*interface_id, target_class) {
                return true;
            }
        }
         */

        false
    }

    //TODO: probably need try to load?
    pub fn instance_of(&self, this_class_id: ClassId, other_sym: Symbol) -> bool {
        if let Some(&other_class_id) = self.class_name_to_index.get(&other_sym) {
            self.is_subclass_of(this_class_id, other_class_id)
        } else {
            false
        }
    }

    fn load_class(&mut self, name_sym: Symbol) -> Result<ClassId, JvmError> {
        let data = {
            let name_str = self.interner.resolve(&name_sym);
            if name_str.starts_with("[") {
                return self.load_array_class(name_sym);
            }
            self.bootstrap_class_loader.load(name_str)?
        };
        let cf = ClassFile::try_from(data).map_err(LinkageError::from)?;
        let super_id = match cf.get_super_class_name() {
            Some(super_name) => {
                let super_name = super_name.unwrap();
                let super_name_sym = self.interner.get_or_intern(super_name);
                Some(self.get_class_id_or_load(super_name_sym)?)
            }
            None => None,
        };
        let class_id = if cf.access_flags.is_interface() {
            InterfaceClass::load_and_link(cf, self, super_id)?
        } else {
            InstanceClass::load_and_link(cf, self, super_id)?
        };
        self.class_name_to_index.insert(name_sym, class_id);
        Ok(class_id)
    }

    pub fn get_class_id_or_load(&mut self, name_sym: Symbol) -> Result<ClassId, JvmError> {
        if let Some(class_id) = self.class_name_to_index.get(&name_sym) {
            return Ok(*class_id);
        }
        let class_id = self.load_class(name_sym)?;
        Ok(class_id)
    }

    pub fn get_class_id_by_mirror(&self, mirror: &HeapRef) -> Result<ClassId, JvmError> {
        self.mirror_to_class_index
            .get(mirror)
            .copied()
            .ok_or(JvmError::Todo(
                "Class ID not found for given mirror reference".to_string(),
            ))
    }

    pub fn get_mirror_ref_or_create(
        &mut self,
        class_id: ClassId,
        heap: &mut Heap,
    ) -> Result<HeapRef, JvmError> {
        if let Some(mirror_ref) = self.get_class(&class_id).get_mirror_ref() {
            return Ok(mirror_ref);
        }
        let mirror_ref = heap.alloc_instance(self, self.br().get_java_lang_class_id()?)?;
        self.mirror_to_class_index.insert(mirror_ref, class_id);
        let target_class = self.get_class(&class_id);
        target_class.set_mirror_ref(mirror_ref)?;
        let name_sym = target_class.get_name();
        let name_ref = heap.alloc_string_with_char_mapping(name_sym, self, &|c| {
            if c == '/' { '.' } else { c }
        })?;
        let name_field_index = self
            .get_instance_class(&self.br().get_java_lang_class_id()?)?
            .get_instance_field_offset(&self.br().class_name_fk)?;
        heap.write_instance_field(mirror_ref, name_field_index as usize, Value::Ref(name_ref))?;
        Ok(mirror_ref)
    }
}
