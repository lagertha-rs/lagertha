// TODO: very primitive implementation, ok for right now

use crate::JvmError;
use crate::rt::class::class::Class;
use crate::rt::constant_pool::reference::NameAndTypeReference;
use common::jtype::{HeapAddr, Value};
use std::sync::Arc;
use tracing_log::log::debug;

#[cfg_attr(test, derive(serde::Serialize))]
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

#[cfg(test)]
impl serde::Serialize for ClassInstance {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("ClassInstance", 2)?;
        state.serialize_field("class", &self.class.name().unwrap())?;
        state.serialize_field("fields", &self.fields)?;
        state.end()
    }
}

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-2.html#jvms-2.5.3
#[cfg_attr(test, derive(serde::Serialize))]
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

    pub fn alloc_instance(&mut self, class: Arc<Class>) -> HeapAddr {
        let mut fields = Vec::with_capacity(class.fields().len());

        let mut cur_class = Some(&class);

        while let Some(c) = cur_class {
            for field in c.fields() {
                fields.push(field.descriptor().resolved().get_default_value());
            }
            cur_class = c.super_class();
        }

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
