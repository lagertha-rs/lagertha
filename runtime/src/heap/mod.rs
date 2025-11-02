// TODO: very primitive implementation, ok for right now

use crate::ClassId;
use crate::heap::method_area::MethodArea;
use common::error::JvmError;
use common::jtype::{HeapAddr, Value};
use std::collections::HashMap;
use tracing_log::log::debug;

pub mod method_area;

pub enum HeapObject {
    Instance(Instance),
    Array(Instance),
}

#[derive(Clone)]
pub struct Instance {
    class_id: ClassId,
    data: Vec<Value>,
}

impl Instance {
    pub fn new(class_id: ClassId, elements: Vec<Value>) -> Self {
        Self {
            class_id,
            data: elements,
        }
    }
}

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-2.html#jvms-2.5.3
pub struct Heap {
    objects: Vec<HeapObject>,
    string_pool: HashMap<String, HeapAddr>,
}

impl Heap {
    pub fn new() -> Result<Self, JvmError> {
        debug!("Creating Heap...");
        Ok(Self {
            string_pool: HashMap::new(),
            objects: Vec::new(),
        })
    }

    fn push(&mut self, obj: HeapObject) -> HeapAddr {
        let idx = self.objects.len();
        self.objects.push(obj);
        idx
    }

    fn get_mut(&mut self, h: HeapAddr) -> Result<&mut HeapObject, JvmError> {
        self.objects
            .get_mut(h)
            .ok_or(JvmError::Todo("invalid heap address".to_string()))
    }

    pub fn alloc_instance(
        &mut self,
        method_area: &mut MethodArea,
        class_id: ClassId,
    ) -> Result<HeapAddr, JvmError> {
        let fields = method_area
            .get_class(&class_id)
            .get_instance_fields()
            .iter()
            .map(|f| method_area.get_field_descriptor(&f.descriptor_id))
            .map(|d| d.get_default_value())
            .collect::<Vec<Value>>();
        Ok(self.push(HeapObject::Instance(Instance {
            class_id,
            data: fields,
        })))
    }

    pub fn write_instance_field(
        &mut self,
        h: HeapAddr,
        offset: usize,
        val: Value,
    ) -> Result<(), JvmError> {
        match self.get_mut(h)? {
            HeapObject::Instance(instance) => {
                if offset >= instance.data.len() {
                    return Err(JvmError::Todo("invalid field index".to_string()));
                }
                instance.data[offset] = val;
            }
            _ => Err(JvmError::Todo(
                "heap: write_instance_field on non-instance".to_string(),
            ))?,
        }
        Ok(())
    }
}
