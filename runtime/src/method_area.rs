use crate::VmConfig;
use crate::class_loader::ClassLoader;
use crate::error::JvmError;
use crate::heap::Heap;
use crate::rt::class::LinkageError;
use crate::rt::class::class::Class;
use class_file::ClassFile;
use common::jtype::HeapAddr;
use dashmap::DashMap;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use tracing_log::log::debug;

//TODO: finally need to decide to return Arc<Class> or &Arc<Class>
//TODO: the class loading process is working stub, need to be improved and respect the spec
/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-2.html#jvms-2.5.4
pub struct MethodArea {
    bootstrap_class_loader: ClassLoader,
    classes: DashMap<String, Arc<Class>>,
    primitives: DashMap<String, HeapAddr>,
}

impl MethodArea {
    pub fn new(vm_config: &VmConfig) -> Result<Self, JvmError> {
        debug!("Initializing MethodArea...");
        let bootstrap_class_loader = ClassLoader::new(vm_config)?;
        let classes = DashMap::new();
        let primitives = DashMap::new();
        let method_area = Self {
            classes,
            primitives,
            bootstrap_class_loader,
        };

        debug!("MethodArea initialized");
        Ok(method_area)
    }

    pub fn add_primitive(&self, name: &str, addr: HeapAddr) {
        debug!("Adding primitive mirror \"{}\" with addr {}", name, addr);
        self.primitives.insert(name.to_string(), addr);
    }

    fn load_with_bytes(&self, data: Vec<u8>) -> Result<Arc<Class>, JvmError> {
        let cf = ClassFile::try_from(data).map_err(LinkageError::from)?;
        let class = Class::new(cf, self)?;
        Ok(class)
    }

    pub fn add_class(&self, raw_class: Vec<u8>) -> Result<Arc<Class>, JvmError> {
        debug!("Adding class from bytes...");
        let class = self.load_with_bytes(raw_class)?;
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
        let class = self.load_with_bytes(class_data)?;

        self.classes.insert(name.to_string(), class.clone());

        Ok(class)
    }

    pub fn get_primitive_mirror(&self, name: &str) -> Option<HeapAddr> {
        self.primitives.get(name).map(|v| *v)
    }
}
