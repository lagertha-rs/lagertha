// TODO: very primitive implementation, ok for right now

use crate::heap::method_area::MethodArea;
use crate::rt::class::Class;
use crate::{ClassId, throw_index_out_of_bounds_exception};
use common::error::JavaExceptionFromJvm;
use common::error::JavaLangError;
use common::error::JvmError;
use common::jtype::{HeapAddr, Value};
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

    pub fn class_id(&self) -> &ClassId {
        &self.class_id
    }

    pub fn elements(&self) -> &Vec<Value> {
        &self.data
    }

    pub fn elements_mut(&mut self) -> &mut Vec<Value> {
        &mut self.data
    }

    pub fn length(&self) -> usize {
        self.data.len()
    }

    pub fn get_element(&self, index: i32) -> Result<&Value, JvmError> {
        // TODO: this block is repeated at least 3 times, refactor
        let index = if index >= 0 && (index as usize) < self.data.len() {
            index as usize
        } else {
            throw_index_out_of_bounds_exception!(
                "Index {} out of bounds for length {}",
                index,
                self.data.len()
            )?
        };
        Ok(&self.data[index])
    }

    pub fn get_element_mut(&mut self, index: i32) -> Result<&mut Value, JvmError> {
        // TODO: this block is repeated at least 3 times, refactor
        let index = if index >= 0 && (index as usize) < self.data.len() {
            index as usize
        } else {
            throw_index_out_of_bounds_exception!(
                "Index {} out of bounds for length {}",
                index,
                self.data.len()
            )?
        };
        Ok(&mut self.data[index])
    }
}

// TODO: probably should be different
// right now to avoid self.alloc_instance(&self.class_class)?; don't want to clone to avoid using self self
enum AllocClass<'a> {
    String,
    Char,
    Class,
    Other(&'a Arc<Class>),
}

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-2.html#jvms-2.5.3
pub struct Heap {
    objects: Vec<HeapObject>,
    string_pool: HashMap<String, HeapAddr>,
    // TODO: find a better way to expose MethodArea functionality
    string_class: Arc<Class>,
    char_class: Arc<Class>,
    class_class: Arc<Class>,
    mirrors: HashMap<HeapAddr, Arc<Class>>,
    primitives: HashMap<HeapAddr, HeapAddr>,
}

impl Heap {
    pub fn new(method_area: &mut MethodArea) -> Result<Self, JvmError> {
        debug!("Creating Heap...");
        let char_class = method_area.get_class_or_load_by_name("[C")?.clone();
        let string_class = method_area
            .get_class_or_load_by_name("java/lang/String")?
            .clone();
        let class_class = method_area
            .get_class_or_load_by_name("java/lang/Class")?
            .clone();
        Ok(Self {
            string_pool: HashMap::new(),
            objects: Vec::new(),
            mirrors: HashMap::new(),
            primitives: HashMap::new(),
            char_class,
            string_class,
            class_class,
        })
    }

    pub fn get_class_id(&self, h: &HeapAddr) -> ClassId {
        match self.get(*h).unwrap() {
            HeapObject::Instance(inst) => inst.class_id,
            HeapObject::Array(arr) => arr.class_id,
        }
    }

    fn push(&mut self, obj: HeapObject) -> HeapAddr {
        let idx = self.objects.len();
        self.objects.push(obj);
        idx
    }

    pub fn alloc_array(&mut self, class: &Arc<Class>, length: usize) -> Result<HeapAddr, JvmError> {
        let default_value = if let Some(primitive_type) = class.primitive() {
            Value::from(&primitive_type)
        } else {
            Value::Null
        };
        let elements = vec![default_value; length];
        Ok(self.push(HeapObject::Array(Instance::new(*class.id(), elements))))
    }

    pub fn alloc_array_with_value(
        &mut self,
        class: &Arc<Class>,
        length: usize,
        value: Value,
    ) -> Result<HeapAddr, JvmError> {
        let elements = vec![value; length];
        Ok(self.push(HeapObject::Array(Instance::new(*class.id(), elements))))
    }

    fn create_default_fields(class: &Arc<Class>) -> Vec<Value> {
        let mut fields = Vec::with_capacity(class.fields().len());

        let mut cur_class = Some(class);

        while let Some(c) = cur_class {
            for field in c.fields() {
                fields.push(field.descriptor().resolved().get_default_value());
            }
            cur_class = c.super_class();
        }

        fields
    }

    fn alloc_instance_internal(&mut self, class: AllocClass) -> Result<HeapAddr, JvmError> {
        let class = match class {
            AllocClass::String => &self.string_class,
            AllocClass::Char => &self.char_class,
            AllocClass::Class => &self.class_class,
            AllocClass::Other(c) => c,
        };
        let fields = Self::create_default_fields(class);
        Ok(self.push(HeapObject::Instance(Instance {
            class_id: *class.id(),
            data: fields,
        })))
    }

    pub fn alloc_instance(&mut self, class: &Arc<Class>) -> Result<HeapAddr, JvmError> {
        self.alloc_instance_internal(AllocClass::Other(class))
    }

    pub fn get_or_new_string(&mut self, value: &str) -> HeapAddr {
        debug!("Getting or creating string in pool: {}", value);
        if let Some(&h) = self.string_pool.get(value) {
            debug!("String found in pool: {}", value);
            return h;
        }
        debug!("String not found in pool. Creating new one: {}", value);
        let h = self.alloc_string(value);
        self.string_pool.insert(value.to_string(), h);
        h
    }

    fn alloc_string(&mut self, s: &str) -> HeapAddr {
        let mut fields = Self::create_default_fields(&self.string_class);

        let chars = s.chars().map(|c| Value::Integer(c as i32)).collect();
        let value = self.push(HeapObject::Array(Instance::new(
            *self.char_class.id(),
            chars,
        )));

        // The "value" field is always the first field in java.lang.String
        fields[0] = Value::Ref(value);

        self.push(HeapObject::Instance(Instance {
            class_id: *self.string_class.id(),
            data: fields,
        }))
    }

    // TODO: return Result and handle errors
    pub fn get(&self, h: HeapAddr) -> Option<&HeapObject> {
        self.objects.get(h)
    }

    pub fn get_instance(&self, h: &HeapAddr) -> Result<&Instance, JvmError> {
        let heap_obj = self.get(*h).unwrap();
        match heap_obj {
            HeapObject::Instance(inst) => Ok(inst),
            _ => Err(JvmError::Todo(
                "get_by_ref called with non-instance HeapObject".to_string(),
            )),
        }
    }

    pub fn get_instance_mut(&mut self, h: &HeapAddr) -> &mut Instance {
        let heap_obj = self.get_mut(*h);
        match heap_obj {
            HeapObject::Instance(inst) => inst,
            _ => panic!("get_by_ref called with non-instance HeapObject",),
        }
    }

    pub fn get_array(&self, h: &HeapAddr) -> &Instance {
        let heap_obj = self.get(*h).unwrap();
        match heap_obj {
            HeapObject::Array(arr) => arr,
            _ => panic!("get_array called with non-array HeapObject",),
        }
    }

    pub fn get_instance_field(
        &mut self,
        addr: &HeapAddr,
        offset: usize,
    ) -> Result<&Value, JvmError> {
        let instance = self.get_instance(addr)?;
        instance
            .data
            .get(offset)
            .ok_or(JvmError::Todo("invalid field index".to_string()))
    }

    pub fn get_mut(&mut self, h: HeapAddr) -> &mut HeapObject {
        self.objects
            .get_mut(h)
            .expect("heap: invalid handle (get_mut)")
    }

    pub fn addr_is_instance(&self, h: &HeapAddr) -> bool {
        matches!(self.get(*h), Some(HeapObject::Instance(_)))
    }

    //TODO: design it lightweight
    pub fn get_string(&self, h: HeapAddr) -> Result<String, JvmError> {
        let instance = self.get_instance(&h)?;
        let value_field = instance.get_element(0)?; // "value" field is always the first field in java.lang.String
        let array_addr = match value_field {
            Value::Ref(addr) => *addr,
            _ => {
                return Err(JvmError::JavaException(JavaExceptionFromJvm::JavaLang(
                    JavaLangError::NullPointerException,
                )));
            }
        };
        let char_array = self.get_array(&array_addr);
        let chars: String = char_array
            .elements()
            .iter()
            .map(|v| match v {
                Value::Integer(i) => std::char::from_u32(*i as u32).unwrap_or('*'),
                _ => '*',
            })
            .collect();
        Ok(chars)
    }

    pub fn write_array_element(
        &mut self,
        h: HeapAddr,
        index: i32,
        val: Value,
    ) -> Result<(), JvmError> {
        match self.get_mut(h) {
            HeapObject::Array(array) => {
                let index = if index >= 0 && (index as usize) < array.length() {
                    index as usize
                } else {
                    throw_index_out_of_bounds_exception!(
                        "Index {} out of bounds for length {}",
                        index,
                        array.length()
                    )?
                };
                array.data[index] = val;
            }
            _ => panic!("heap: write_array_element on non-array"),
        }
        Ok(())
    }

    pub fn write_instance_field(
        &mut self,
        h: HeapAddr,
        offset: usize,
        val: Value,
    ) -> Result<(), JvmError> {
        match self.get_mut(h) {
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

    pub fn clone_object(&mut self, h: HeapAddr) -> HeapAddr {
        let obj = self.get(h).expect("heap: invalid handle (clone_object)");
        match obj {
            HeapObject::Instance(inst) => {
                let new_fields = inst.data.clone();
                let new_instance = Instance {
                    class_id: inst.class_id,
                    data: new_fields,
                };
                self.push(HeapObject::Instance(new_instance))
            }
            HeapObject::Array(arr) => {
                let new_elements = arr.data.clone();
                let new_array = Instance {
                    class_id: arr.class_id,
                    data: new_elements,
                };
                self.push(HeapObject::Array(new_array))
            }
        }
    }

    pub fn get_class_by_mirror(&self, mirror: &HeapAddr) -> Option<&Arc<Class>> {
        self.mirrors.get(mirror)
    }

    pub(crate) fn get_mirror_addr(
        &mut self,
        target_class: &Arc<Class>,
    ) -> Result<HeapAddr, JvmError> {
        if let Some(mirror) = target_class.mirror() {
            return Ok(mirror);
        }
        let mirror = self.alloc_instance_internal(AllocClass::Class)?;
        target_class.set_mirror(mirror)?;
        self.mirrors.insert(mirror, target_class.clone());
        Ok(mirror)
    }

    pub(crate) fn get_primitive_mirror_addr(
        &mut self,
        name: &HeapAddr,
    ) -> Result<HeapAddr, JvmError> {
        if let Some(addr) = self.primitives.get(name) {
            Ok(*addr)
        } else {
            let mirror_addr = self.alloc_instance_internal(AllocClass::Class)?;
            self.primitives.insert(*name, mirror_addr);
            Ok(mirror_addr)
        }
    }

    pub fn addr_is_primitive(&self, addr: &HeapAddr) -> bool {
        self.primitives.values().any(|v| v == addr)
    }
}
