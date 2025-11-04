use crate::class_loader::ClassLoader;
use crate::rt::class::InstanceClass;
use crate::rt::constant_pool::RuntimeConstantPool;
use crate::rt::method::Method;
use crate::{
    ClassId, TypeDescriptorId, FullyQualifiedMethodKey, MethodDescriptorId, MethodId, Symbol,
    VmConfig,
};
use common::descriptor::MethodDescriptor;
use common::error::{JvmError, LinkageError, MethodDescriptorErr};
use common::jtype::Type;
use jclass::ClassFile;
use lasso::{Spur, ThreadedRodeo};
use std::collections::HashMap;
use std::sync::Arc;
use common::instruction::ArrayType;
use crate::rt::JvmClass;

pub struct MethodArea {
    bootstrap_class_loader: ClassLoader,
    class_name_to_index: HashMap<Spur, ClassId>,
    classes: Vec<JvmClass>,
    methods: Vec<Method>,

    type_descriptors: Vec<Type>,
    type_descriptors_index: HashMap<Symbol, TypeDescriptorId>,

    method_descriptors: Vec<MethodDescriptor>,
    method_descriptors_index: HashMap<Symbol, MethodDescriptorId>,

    pub interner: Arc<ThreadedRodeo>,
    constructor_symbols: (Symbol, Symbol),
}

impl MethodArea {
    fn bootstrap(&mut self) -> Result<(), JvmError> {
        let java_lang_object_sym = self.interner.get_or_intern("java/lang/Object");
        let class_id = self.get_class_id_or_load(java_lang_object_sym)?;
        Ok(())
    }
    pub fn new(
        vm_config: &VmConfig,
        string_interner: Arc<ThreadedRodeo>,
    ) -> Result<Self, JvmError> {
        let bootstrap_class_loader = ClassLoader::new(vm_config)?;
        let constructor_symbols = (
            string_interner.get_or_intern("<init>"),
            string_interner.get_or_intern("<clinit>"),
        );

        let method_area = Self {
            bootstrap_class_loader,
            class_name_to_index: HashMap::new(),
            classes: Vec::new(),
            methods: Vec::new(),
            type_descriptors: Vec::new(),
            type_descriptors_index: HashMap::new(),
            method_descriptors: Vec::new(),
            method_descriptors_index: HashMap::new(),
            interner: string_interner,
            constructor_symbols,
        };

        Ok(method_area)
    }

    pub fn build_fully_qualified_method_key(
        &self,
        method_id: &MethodId,
    ) -> FullyQualifiedMethodKey {
        let method = self.get_method(method_id);
        let name= match self.get_class(&method.class_id()) {
            JvmClass::Instance(instance) => instance.name,
            _ => panic!("Not an instance class"),
        };
        FullyQualifiedMethodKey::new(name, method.name, method.desc)
    }

    // todo: probably there is better place to put this
    pub fn is_constructor_symbol(&self, name: Symbol) -> bool {
        self.constructor_symbols.0 == name || self.constructor_symbols.1 == name
    }

    fn push_type_descriptor(&mut self, ty: Type) -> TypeDescriptorId {
        self.type_descriptors.push(ty);
        TypeDescriptorId::from_usize(self.type_descriptors.len())
    }

    pub fn get_type_descriptor(&self, id: &TypeDescriptorId) -> &Type {
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
        let ty = Type::try_from(descriptor_str)?;
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
            _ => Err(JvmError::NotAJavaInstanceTodo("Not an instance class".to_string())),
        }
    }

    pub fn get_class_id_by_name(&self, name_sym: &Symbol) -> ClassId {
        *self.class_name_to_index.get(name_sym).unwrap()
    }

    pub fn get_cp(&self, class_id: &ClassId) -> Result<&RuntimeConstantPool, JvmError> {
        self.get_instance_class(class_id).map(|c| &c.cp)
    }

    pub fn get_cp_by_method_id(&self, method_id: &MethodId) -> Result<&RuntimeConstantPool, JvmError> {
        let class_id = self.get_method(method_id).class_id();
        self.get_cp(&class_id)
    }

    fn load_array_class(&mut self, name_sym: Symbol) -> Result<ClassId, JvmError> {
        let type_descriptor_id = self.get_or_new_type_descriptor_id(name_sym)?;
        let type_descriptor = self.get_type_descriptor(&type_descriptor_id);
        todo!()
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
        let class_id = InstanceClass::load_and_link(cf, self, super_id)?;
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
}
