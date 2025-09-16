use crate::class_loader::ClassLoader;
use crate::rt::class::LinkageError;
use crate::rt::class::class::Class;
use crate::{JvmError, VmConfig};
use class_file::ClassFile;
use dashmap::DashMap;
use std::sync::Arc;
use tracing_log::log::debug;

//TODO: finally need to decide to return Arc<Class> or &Arc<Class>

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-2.html#jvms-2.5.4
pub struct MethodArea {
    vm_config: Arc<VmConfig>,
    bootstrap_class_loader: ClassLoader,
    classes: DashMap<String, Arc<Class>>,
}

impl MethodArea {
    pub fn new(vm_config: Arc<VmConfig>) -> Result<Self, JvmError> {
        debug!("Initializing MethodArea...");
        let bootstrap_class_loader = ClassLoader::new(vm_config.clone())?;
        let classes = DashMap::new();
        debug!("MethodArea initialized");
        Ok(Self {
            vm_config,
            classes,
            bootstrap_class_loader,
        })
    }

    fn load_with_bytes(&self, data: Vec<u8>) -> Result<Class, JvmError> {
        let cf = ClassFile::try_from(data).map_err(LinkageError::from)?;
        let class = Class::new(cf, self)?;
        Ok(class)
    }

    pub fn add_class(&self, raw_class: Vec<u8>) -> Result<Arc<Class>, JvmError> {
        debug!("Adding class from bytes...");
        let class = Arc::new(self.load_with_bytes(raw_class)?);
        let name = class.name()?.to_string();
        debug!("Class \"{}\" added", name);
        self.classes.insert(name, class.clone());
        Ok(class)
    }

    pub fn get_class(&self, name: &str) -> Result<Arc<Class>, JvmError> {
        if let Some(class) = self.classes.get(name) {
            return Ok(class.clone());
        }
        let class_data = self.bootstrap_class_loader.load(name)?;
        let class = Arc::new(self.load_with_bytes(class_data)?);

        self.classes.insert(name.to_string(), class.clone());

        Ok(class)
    }
}
