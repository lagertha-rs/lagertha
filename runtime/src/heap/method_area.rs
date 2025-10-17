use crate::class_loader::ClassLoader;
use crate::error::JvmError;
use crate::rt::LinkageError;
use crate::rt::class::Class;
use crate::{ClassId, VmConfig};
use class_file::ClassFile;
use common::instruction::ArrayType;
use std::collections::HashMap;
use std::sync::Arc;
use tracing_log::log::debug;

//TODO: finally need to decide to return Arc<Class> or &Arc<Class>
//TODO: the class loading process is working stub, need to be improved and respect the spec
/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-2.html#jvms-2.5.4
pub struct MethodArea {
    bootstrap_class_loader: ClassLoader,
    // TODO: make class name Arc<str>?
    classes_idx: HashMap<String, ClassId>,
    classes: Vec<Arc<Class>>,
}

impl MethodArea {
    pub fn new(vm_config: &VmConfig) -> Result<Self, JvmError> {
        debug!("Initializing MethodArea...");
        let bootstrap_class_loader = ClassLoader::new(vm_config)?;
        let method_area = Self {
            classes_idx: HashMap::new(),
            classes: Vec::new(),
            bootstrap_class_loader,
        };

        debug!("MethodArea initialized");
        Ok(method_area)
    }

    pub fn get_class_by_id(&self, class_id: ClassId) -> Result<&Arc<Class>, JvmError> {
        self.classes
            .get(class_id)
            .ok_or(JvmError::ClassNotFound2(class_id))
    }

    pub fn add_raw_bytecode(&mut self, data: Vec<u8>) -> Result<Arc<Class>, JvmError> {
        let cf = ClassFile::try_from(data).map_err(LinkageError::from)?;
        let class = Class::new(cf, self)?;
        self.add_class(class.clone())?;
        Ok(class)
    }

    fn add_class(&mut self, class: Arc<Class>) -> Result<(), JvmError> {
        let id = self.classes.len();
        class.set_id(id)?;
        self.classes.push(class.clone());
        self.classes_idx.insert(class.name().to_string(), id);
        Ok(())
    }

    pub fn get_class(&mut self, name: &str) -> Result<Arc<Class>, JvmError> {
        if let Some(class_id) = self.classes_idx.get(name) {
            return self.get_class_by_id(*class_id);
        }
        if name.starts_with("[") {
            self.create_array_class(name)
        } else {
            let class_data = self.bootstrap_class_loader.load(name)?;
            self.add_raw_bytecode(class_data)
        }
    }

    fn create_array_class(&mut self, name: &str) -> Result<Arc<Class>, JvmError> {
        if let Some(class_id) = self.classes_idx.get(name) {
            return self.get_class_by_id(*class_id);
        }
        let class = if let Ok(primitive) = ArrayType::try_from(name) {
            Class::new_primitive_array(primitive)?
        } else {
            Class::new_array(name)?
        };
        self.add_class(class.clone())?;
        Ok(class)
    }

    pub fn get_class_id(&self, name: &str) -> Result<ClassId, JvmError> {
        if let Some(class_id) = self.classes_idx.get(name) {
            return Ok(*class_id);
        }
        Err(JvmError::ClassNotFound(name.to_string()))
    }
}
