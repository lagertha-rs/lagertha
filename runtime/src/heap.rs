// TODO: very primitive implementation, ok for right now

use crate::JvmError;
use crate::rt::class::class::Class;
use crate::rt::class::field::Field;
use crate::rt::constant_pool::reference::NameAndTypeReference;
use common::jtype::{HeapAddr, Value};
use std::sync::Arc;
use tracing_log::log::debug;

pub enum HeapObject {
    Instance(ClassInstance),
    String(String),
}

#[derive(Clone)]
pub struct ClassInstance {
    class: Arc<Class>,
    fields: Vec<Value>,
}

impl ClassInstance {
    pub fn get_field(&mut self, index: usize) -> &mut Value {
        self.fields.get_mut(index).expect("invalid field index")
    }

    pub fn class(&self) -> &Arc<Class> {
        &self.class
    }
}

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-2.html#jvms-2.5.3
pub struct Heap {
    objects: Vec<HeapObject>,
}

impl Heap {
    pub fn new() -> Self {
        debug!("Initializing Heap...");
        Self {
            objects: Vec::new(),
        }
    }

    fn push(&mut self, obj: HeapObject) -> HeapAddr {
        let idx = self.objects.len();
        self.objects.push(obj);
        idx
    }

    //TODO: how to handle super classes?
    pub fn alloc_instance(&mut self, class: Arc<Class>) -> HeapAddr {
        let fields = class
            .fields()
            .iter()
            .map(|field| field.descriptor().resolved().get_default_value())
            .collect();
        self.push(HeapObject::Instance(ClassInstance { class, fields }))
    }

    pub fn alloc_string<S: Into<String>>(&mut self, s: S) -> HeapAddr {
        self.push(HeapObject::String(s.into()))
    }

    pub fn get(&self, h: HeapAddr) -> &HeapObject {
        self.objects.get(h).expect("heap: invalid handle (get)")
    }

    pub fn get_instance(&mut self, h: &HeapAddr) -> &ClassInstance {
        let heap_obj = self.get(*h);
        match heap_obj {
            HeapObject::Instance(inst) => inst,
            _ => panic!("get_by_ref called with non-instance HeapObject",),
        }
    }

    pub fn get_instance_field(&mut self, h: &HeapAddr, nat: &NameAndTypeReference) -> &Value {
        let instance = self.get_instance(h);
        let slot = instance.class.get_field_index(nat).unwrap();
        instance.fields.get(slot).unwrap()
    }

    pub fn get_mut(&mut self, h: HeapAddr) -> &mut HeapObject {
        self.objects
            .get_mut(h)
            .expect("heap: invalid handle (get_mut)")
    }

    pub fn write_instance_field(
        &mut self,
        h: HeapAddr,
        field_nat: &NameAndTypeReference,
        val: Value,
    ) -> Result<(), JvmError> {
        match self.get_mut(h) {
            HeapObject::Instance(instance) => {
                let slot = instance.class.get_field_index(field_nat)?;
                instance.fields[slot] = val;
            }
            _ => panic!("heap: write_instance_field on non-instance"),
        }
        Ok(())
    }
}
