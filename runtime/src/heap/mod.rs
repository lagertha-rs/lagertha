// TODO: very primitive implementation, ok for right now

use crate::heap::method_area::MethodArea;
use crate::{ClassId, Symbol};
use common::error::JvmError;
use common::jtype::{HeapAddr, Value};
use lasso::ThreadedRodeo;
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::sync::Arc;
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
    // TODO: need to think about the string pool since I have the interner
    string_pool: HashMap<Symbol, HeapAddr>,
    interner: Arc<ThreadedRodeo>,
    string_class_id: OnceCell<ClassId>,
}

impl Heap {
    pub fn new(interner: Arc<ThreadedRodeo>) -> Result<Self, JvmError> {
        debug!("Creating Heap...");
        Ok(Self {
            string_class_id: OnceCell::new(),
            string_pool: HashMap::new(),
            objects: Vec::new(),
            interner,
        })
    }

    fn push(&mut self, obj: HeapObject) -> HeapAddr {
        let idx = self.objects.len();
        self.objects.push(obj);
        idx
    }

    fn get_mut(&mut self, h: &HeapAddr) -> Result<&mut HeapObject, JvmError> {
        self.objects
            .get_mut(*h)
            .ok_or(JvmError::Todo("invalid heap address".to_string()))
    }

    fn get_instance_mut(&mut self, h: &HeapAddr) -> Result<&mut Instance, JvmError> {
        match self.get_mut(h)? {
            HeapObject::Instance(ins) => Ok(ins),
            _ => Err(JvmError::Todo("Non instance".to_string())),
        }
    }

    pub fn get_or_new_string(
        &mut self,
        val_sym: Symbol,
        method_area: &mut MethodArea,
    ) -> Result<HeapAddr, JvmError> {
        if let Some(h) = self.string_pool.get(&val_sym) {
            Ok(*h)
        } else {
            let string_class_id = *self.string_class_id.get_or_try_init(|| {
                method_area
                    .get_class_id_or_load(self.interner.get_or_intern("java/lang/String"))})?;
            let instance = self.alloc_instance(
                method_area,
                string_class_id
                )?;
            self.string_pool.insert(val_sym, instance);
            let instance = self.get_instance_mut(&instance)?;
            //instance.data[0] = need to alloc char array (needs a mirror class for char[])

            todo!()
        }
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
