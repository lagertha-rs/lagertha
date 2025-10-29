use crate::class_loader::ClassLoader;
use crate::rt::class::Class;
use crate::rt::method::Method;
use crate::{ClassId, MethodId, Symbol, VmConfig};
use common::error::{ClassFormatErr, JvmError, LinkageError};
use jclass::ClassFile;
use lasso::{Spur, ThreadedRodeo};
use std::collections::HashMap;
use std::sync::Arc;

pub struct MethodArea {
    bootstrap_class_loader: ClassLoader,
    class_name_to_index: HashMap<Spur, ClassId>,
    classes: Vec<Class>,
    methods: Vec<Method>,
    pub string_interner: Arc<ThreadedRodeo>,
}

impl MethodArea {
    pub fn new(
        vm_config: &VmConfig,
        string_interner: Arc<ThreadedRodeo>,
    ) -> Result<Self, JvmError> {
        let bootstrap_class_loader = ClassLoader::new(vm_config)?;

        Ok(Self {
            bootstrap_class_loader,
            class_name_to_index: HashMap::new(),
            classes: Vec::new(),
            methods: Vec::new(),
            string_interner,
        })
    }

    pub fn push_method(&mut self, method: Method) -> MethodId {
        self.methods.push(method);
        MethodId::from_usize(self.methods.len())
    }

    pub fn get_method(&self, method_id: MethodId) -> &Method {
        &self.methods[method_id.to_index()]
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
        let class = Class::new(cf, self, super_id)?;
        self.classes.push(class);
        let class_id = ClassId::from_usize(self.classes.len());
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
