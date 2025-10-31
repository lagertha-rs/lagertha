use crate::class_loader::ClassLoader;
use crate::rt::class::Class;
use crate::rt::method::Method;
use crate::{ClassId, FieldDescriptorId, MethodId, Symbol, VmConfig};
use common::error::{JvmError, LinkageError};
use common::jtype::Type;
use jclass::ClassFile;
use lasso::{Spur, ThreadedRodeo};
use std::collections::HashMap;
use std::sync::Arc;

pub struct MethodArea {
    bootstrap_class_loader: ClassLoader,
    class_name_to_index: HashMap<Spur, ClassId>,
    classes: Vec<Class>,
    methods: Vec<Method>,
    field_descriptors: Vec<Type>,
    field_descriptors_index: HashMap<Symbol, FieldDescriptorId>,
    pub string_interner: Arc<ThreadedRodeo>,
    constructor_symbols: (Symbol, Symbol),
}

impl MethodArea {
    pub fn new(
        vm_config: &VmConfig,
        string_interner: Arc<ThreadedRodeo>,
    ) -> Result<Self, JvmError> {
        let bootstrap_class_loader = ClassLoader::new(vm_config)?;
        let constructor_symbols = (
            string_interner.get_or_intern("<init>"),
            string_interner.get_or_intern("<clinit>"),
        );

        Ok(Self {
            bootstrap_class_loader,
            class_name_to_index: HashMap::new(),
            classes: Vec::new(),
            methods: Vec::new(),
            field_descriptors: Vec::new(),
            field_descriptors_index: HashMap::new(),
            string_interner,
            constructor_symbols,
        })
    }

    // todo: probably there is better place to put this
    pub fn is_constructor_symbol(&self, name: Symbol) -> bool {
        self.constructor_symbols.0 == name || self.constructor_symbols.1 == name
    }

    fn push_field_descriptor(&mut self, ty: Type) -> FieldDescriptorId {
        self.field_descriptors.push(ty);
        FieldDescriptorId::from_usize(self.field_descriptors.len())
    }

    pub fn get_field_descriptor(&self, id: &FieldDescriptorId) -> &Type {
        &self.field_descriptors[id.to_index()]
    }

    pub fn get_or_new_field_descriptor_id(
        &mut self,
        descriptor: &Symbol,
    ) -> Result<FieldDescriptorId, JvmError> {
        if let Some(field_desc) = self.field_descriptors_index.get(descriptor) {
            return Ok(*field_desc);
        }
        let descriptor_str = self.string_interner.resolve(descriptor);
        let ty = Type::try_from(descriptor_str)?;
        Ok(self.push_field_descriptor(ty))
    }

    pub fn push_method(&mut self, method: Method) -> MethodId {
        self.methods.push(method);
        MethodId::from_usize(self.methods.len())
    }

    pub fn get_method(&self, method_id: &MethodId) -> &Method {
        &self.methods[method_id.to_index()]
    }

    pub fn push_class(&mut self, class: Class) -> ClassId {
        self.classes.push(class);
        ClassId::from_usize(self.classes.len())
    }

    pub fn get_class(&self, class_id: &ClassId) -> &Class {
        &self.classes[class_id.to_index()]
    }

    fn load_class(&mut self, name_sym: Symbol) -> Result<ClassId, JvmError> {
        let data = {
            let name_str = self.string_interner.resolve(&name_sym);
            self.bootstrap_class_loader.load(name_str)?
        };
        let cf = ClassFile::try_from(data).map_err(LinkageError::from)?;
        let super_id = match cf.get_super_class_name() {
            Some(super_name) => {
                let super_name = super_name.unwrap();
                let super_name_sym = self.string_interner.get_or_intern(super_name);
                Some(self.get_class_id_or_load(super_name_sym)?)
            }
            None => None,
        };
        let class_id = Class::load_and_link(cf, self, super_id)?;
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
