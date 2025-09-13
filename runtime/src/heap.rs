// TODO: very primitive implementation, ok for right now

use crate::ClassId;
use common::jtype::{HeapAddr, ObjectRef};
use tracing_log::log::debug;

pub enum HeapObject {
    Instance {
        class_id: ClassId,
        fields: Vec<ObjectRef>,
    },
    JavaString {
        utf8: String,
    },
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

    pub fn alloc_instance(&mut self, class_id: ClassId, field_count: usize) -> HeapAddr {
        self.push(HeapObject::Instance {
            class_id,
            fields: vec![ObjectRef::Null; field_count],
        })
    }

    pub fn alloc_string<S: Into<String>>(&mut self, s: S) -> HeapAddr {
        self.push(HeapObject::JavaString { utf8: s.into() })
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
