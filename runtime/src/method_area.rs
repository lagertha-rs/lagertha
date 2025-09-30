use crate::VmConfig;
use crate::class_loader::ClassLoader;
use crate::error::JvmError;
use crate::heap::Heap;
use crate::rt::class::LinkageError;
use crate::rt::class::class::Class;
use class_file::ClassFile;
use common::jtype::{HeapAddr, Value};
use dashmap::DashMap;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use tracing_log::log::debug;

//TODO: finally need to decide to return Arc<Class> or &Arc<Class>
//TODO: the class loading process is working stub, need to be improved and respect the spec
/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-2.html#jvms-2.5.4
pub struct MethodArea {
    heap: Rc<RefCell<Heap>>,
    bootstrap_class_loader: ClassLoader,
    classes: HashMap<String, Arc<Class>>,
    mirrors: HashMap<HeapAddr, Arc<Class>>,
    primitives: HashMap<HeapAddr, HeapAddr>,
    array_primitives: HashMap<String, HeapAddr>,
}

impl MethodArea {
    pub fn new(vm_config: &VmConfig, heap: Rc<RefCell<Heap>>) -> Result<Self, JvmError> {
        debug!("Initializing MethodArea...");
        let bootstrap_class_loader = ClassLoader::new(vm_config)?;
        let method_area = Self {
            heap,
            classes: HashMap::new(),
            bootstrap_class_loader,
            mirrors: HashMap::new(),
            primitives: HashMap::new(),
            array_primitives: HashMap::new(),
        };

        debug!("MethodArea initialized");
        Ok(method_area)
    }

    fn load_with_bytes(&mut self, data: Vec<u8>) -> Result<Arc<Class>, JvmError> {
        let cf = ClassFile::try_from(data).map_err(LinkageError::from)?;
        let class = Class::new(cf, self)?;
        Ok(class)
    }

    pub fn add_class(&mut self, raw_class: Vec<u8>) -> Result<Arc<Class>, JvmError> {
        debug!("Adding class from bytes...");
        let class = self.load_with_bytes(raw_class)?;
        let name = class.name()?.to_string();
        debug!("Class \"{}\" added", name);
        self.classes.insert(name, class.clone());
        Ok(class)
    }

    pub fn get_class(&mut self, name: &str) -> Result<Arc<Class>, JvmError> {
        if let Some(class) = self.classes.get(name) {
            return Ok(class.clone());
        }
        let class_data = self.bootstrap_class_loader.load(name)?;
        let class = self.load_with_bytes(class_data)?;

        self.classes.insert(name.to_string(), class.clone());

        Ok(class)
    }

    pub fn get_class_by_mirror(&self, mirror: &HeapAddr) -> Option<&Arc<Class>> {
        self.mirrors.get(mirror)
    }

    pub fn get_mirror_addr_by_name(&mut self, name: &str) -> Result<HeapAddr, JvmError> {
        if name.starts_with('[') {
            if let Some(addr) = self.array_primitives.get(name) {
                return Ok(*addr);
            }
            let class_class = self.get_class("java/lang/Class")?;
            let mirror_addr = {
                let mut heap = self.heap.borrow_mut();
                let mirror_addr = heap.alloc_instance(class_class.clone());
                let val = Value::Object(Some(heap.get_or_new_string(name)));
                heap.write_instance_field(mirror_addr, "name", "Ljava/lang/String;", val)?;
                self.mirrors.insert(mirror_addr, class_class);
                mirror_addr
            };
            self.array_primitives.insert(name.to_string(), mirror_addr);
            return Ok(mirror_addr);
        }
        let target_class = self.get_class(name)?;
        self.get_mirror_addr_by_class(&target_class)
    }

    pub fn get_mirror_addr_by_class(
        &mut self,
        target_class: &Arc<Class>,
    ) -> Result<HeapAddr, JvmError> {
        if let Some(mirror) = target_class.mirror() {
            return Ok(mirror);
        }
        let class_class = self.get_class("java/lang/Class")?;
        let mirror = self.heap.borrow_mut().alloc_instance(class_class);
        target_class.set_mirror(mirror)?;
        self.mirrors.insert(mirror, target_class.clone());
        Ok(mirror)
    }

    pub fn get_primitive_mirror_addr(&mut self, name: &HeapAddr) -> HeapAddr {
        if let Some(addr) = self.primitives.get(name) {
            *addr
        } else {
            let class_class = self.get_class("java/lang/Class").unwrap();
            let mirror_addr = self.heap.borrow_mut().alloc_instance(class_class);
            self.primitives.insert(*name, mirror_addr);
            mirror_addr
        }
    }
}
