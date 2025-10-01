use crate::class_loader::ClassLoader;
use crate::error::JvmError;
use crate::heap::Heap;
use crate::rt::class::LinkageError;
use crate::rt::class::class::Class;
use crate::{ClassId, VmConfig};
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
    // TODO: make class name Arc<str>?
    classes_idx: HashMap<String, ClassId>,
    classes: Vec<Arc<Class>>,

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
            classes_idx: HashMap::new(),
            classes: Vec::new(),
            bootstrap_class_loader,
            mirrors: HashMap::new(),
            primitives: HashMap::new(),
            array_primitives: HashMap::new(),
        };

        debug!("MethodArea initialized");
        Ok(method_area)
    }

    pub fn get_class_by_id(&self, class_id: ClassId) -> Result<Arc<Class>, JvmError> {
        self.classes
            .get(class_id)
            .cloned()
            .ok_or(JvmError::ClassNotFound2(class_id))
    }

    pub fn add_class(&mut self, data: Vec<u8>) -> Result<Arc<Class>, JvmError> {
        let cf = ClassFile::try_from(data).map_err(LinkageError::from)?;
        let class = Class::new(cf, self)?;
        let id = self.classes.len();
        class.set_id(id)?;
        self.classes.push(class.clone());
        self.classes_idx.insert(class.name()?.to_string(), id);
        Ok(class)
    }

    pub fn get_class(&mut self, name: &str) -> Result<Arc<Class>, JvmError> {
        if let Some(class_id) = self.classes_idx.get(name) {
            self.get_class_by_id(*class_id)
        } else {
            let class_data = self.bootstrap_class_loader.load(name)?;
            self.add_class(class_data)
        }
    }

    pub fn get_class_id(&self, name: &str) -> Result<ClassId, JvmError> {
        if let Some(class_id) = self.classes_idx.get(name) {
            return Ok(*class_id);
        }
        Err(JvmError::ClassNotFound(name.to_string()))
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
