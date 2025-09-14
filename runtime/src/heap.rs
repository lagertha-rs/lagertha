// TODO: very primitive implementation, ok for right now

use crate::rt::class::field::Field;
use common::jtype::{HeapAddr, ObjectRef, PrimitiveValue, Type, Value};
use tracing_log::log::debug;

pub enum HeapObject {
    Instance {
        class_idx: u16,
        fields: Vec<ObjectRef>,
    },
    String(String),
    Primitive(PrimitiveValue),
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
    pub fn alloc_instance(&mut self, class_idx: u16, fields: &Vec<Field>) -> HeapAddr {
        self.push(HeapObject::Instance {
            class_idx,
            fields: fields
                .iter()
                .map(|field| match field.descriptor().resolved() {
                    Type::Instance(_) => ObjectRef::Null,
                    Type::Array(_) => ObjectRef::Null,
                    other => {
                        let default_value = other.get_default_value();
                        if let Value::Primitive(p) = default_value {
                            ObjectRef::Primitive(p)
                        } else {
                            unimplemented!()
                        }
                    }
                })
                .collect(),
        })
    }

    pub fn alloc_string<S: Into<String>>(&mut self, s: S) -> HeapAddr {
        self.push(HeapObject::String(s.into()))
    }

    pub fn get(&self, h: HeapAddr) -> &HeapObject {
        self.objects.get(h).expect("heap: invalid handle (get)")
    }

    pub fn get_mut(&mut self, h: HeapAddr) -> &mut HeapObject {
        self.objects
            .get_mut(h)
            .expect("heap: invalid handle (get_mut)")
    }

    pub fn write_instance_field(&mut self, h: HeapAddr, slot: usize, val: ObjectRef) {
        match self.get_mut(h) {
            HeapObject::Instance { fields, .. } => {
                let cell = fields.get_mut(slot).expect("heap: instance field slot OOB");
                *cell = val;
            }
            _ => panic!("heap: write_instance_field on non-instance"),
        }
    }

    pub fn read_instance_field(&self, h: HeapAddr, slot: usize) -> ObjectRef {
        match self.get(h) {
            HeapObject::Instance { fields, .. } => fields
                .get(slot)
                .expect("heap: instance field slot OOB")
                .clone(),
            _ => panic!("heap: read_instance_field on non-instance"),
        }
    }
}
