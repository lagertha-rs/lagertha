// TODO: very primitive implementation, ok for right now

use crate::error::JvmError;
use crate::rt::class::class::Class;
use crate::rt::constant_pool::reference::NameAndTypeReference;
use common::jtype::{HeapAddr, Value};
use std::cell::OnceCell;
use std::collections::HashMap;
use std::sync::Arc;
use tracing_log::log::debug;

pub enum HeapObject {
    Instance(ClassInstance),
    Array(ArrayInstance),
}

#[derive(Clone)]
pub struct ArrayInstance {
    class: Arc<Class>,
    elements: Vec<Value>,
}

impl ArrayInstance {
    pub fn new(class: Arc<Class>, elements: Vec<Value>) -> Self {
        Self { class, elements }
    }

    pub fn class(&self) -> &Arc<Class> {
        &self.class
    }

    pub fn elements(&self) -> &Vec<Value> {
        &self.elements
    }

    pub fn elements_mut(&mut self) -> &mut Vec<Value> {
        &mut self.elements
    }

    pub fn length(&self) -> usize {
        self.elements.len()
    }

    pub fn get_element(&self, index: usize) -> &Value {
        self.elements.get(index).expect("invalid array index")
    }
}

#[derive(Clone)]
pub struct ClassInstance {
    class: Arc<Class>,
    fields: Vec<Value>,
}

impl ClassInstance {
    pub fn new(class: Arc<Class>, fields: Vec<Value>) -> Self {
        Self { class, fields }
    }

    pub fn get_field_mut(&mut self, index: usize) -> &mut Value {
        self.fields.get_mut(index).expect("invalid field index")
    }

    pub fn get_field(&self, index: usize) -> &Value {
        self.fields.get(index).expect("invalid field index")
    }

    pub fn class(&self) -> &Arc<Class> {
        &self.class
    }

    pub fn get_field_value(&self, name: &str, descriptor: &str) -> Option<&Value> {
        let index = self.class.get_field_index(name, descriptor).ok()?;
        self.fields.get(index)
    }
}

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-2.html#jvms-2.5.3
pub struct Heap {
    string_class: OnceCell<Arc<Class>>,
    objects: Vec<HeapObject>,
    string_pool: HashMap<String, HeapAddr>,
    char_class: OnceCell<Arc<Class>>,
}

impl Heap {
    pub fn new() -> Self {
        debug!("Creating Heap...");
        Self {
            string_class: OnceCell::new(),
            string_pool: HashMap::new(),
            objects: Vec::new(),
            char_class: OnceCell::new(),
        }
    }

    pub fn set_char_class(&self, char_class: Arc<Class>) {
        if self.char_class.set(char_class).is_err() {
            panic!("Heap: char_class is already set");
        }
        debug!("Heap char_class set");
    }

    pub fn initialize(&self, string_class: Arc<Class>) {
        if self.string_class.set(string_class).is_err() {
            panic!("Heap: string_class is already set");
        }
        debug!("Heap initialized");
    }

    fn push(&mut self, obj: HeapObject) -> HeapAddr {
        let idx = self.objects.len();
        self.objects.push(obj);
        idx
    }

    pub fn alloc_array(&mut self, class: Arc<Class>, length: usize) -> HeapAddr {
        let default_value = if let Some(primitive_type) = class.primitive() {
            Value::from(&primitive_type)
        } else {
            Value::Null
        };
        let elements = vec![default_value; length];
        self.push(HeapObject::Array(ArrayInstance::new(class, elements)))
    }

    pub fn alloc_array_with_value(
        &mut self,
        class: Arc<Class>,
        length: usize,
        value: Value,
    ) -> HeapAddr {
        let elements = vec![value; length];
        self.push(HeapObject::Array(ArrayInstance::new(class, elements)))
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

    pub fn alloc_instance(&mut self, class: Arc<Class>) -> HeapAddr {
        let fields = Self::create_default_fields(&class);
        self.push(HeapObject::Instance(ClassInstance { class, fields }))
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
        let mut fields = Self::create_default_fields(&self.string_class.get().unwrap());

        let chars = s.chars().map(|c| Value::Integer(c as i32)).collect();
        let value = self.push(HeapObject::Array(ArrayInstance::new(
            self.char_class.get().unwrap().clone(),
            chars,
        )));

        // The "value" field is always the first field in java.lang.String, but probably should be looked up by name
        fields[0] = Value::Ref(value);

        self.push(HeapObject::Instance(ClassInstance {
            class: self.string_class.get().unwrap().clone(),
            fields,
        }))
    }

    // TODO: return Result and handle errors
    pub fn get(&self, h: HeapAddr) -> Option<&HeapObject> {
        self.objects.get(h)
    }

    // TODO: return Result and handle errors
    pub fn get_instance(&self, h: &HeapAddr) -> &ClassInstance {
        let heap_obj = self.get(*h).unwrap();
        match heap_obj {
            HeapObject::Instance(inst) => inst,
            _ => panic!("get_by_ref called with non-instance HeapObject",),
        }
    }

    pub fn get_instance_mut(&mut self, h: &HeapAddr) -> &mut ClassInstance {
        let heap_obj = self.get_mut(*h);
        match heap_obj {
            HeapObject::Instance(inst) => inst,
            _ => panic!("get_by_ref called with non-instance HeapObject",),
        }
    }

    pub fn get_array(&self, h: &HeapAddr) -> &ArrayInstance {
        let heap_obj = self.get(*h).unwrap();
        match heap_obj {
            HeapObject::Array(arr) => arr,
            _ => panic!("get_array called with non-array HeapObject",),
        }
    }

    pub fn get_instance_field_by_nat(
        &mut self,
        h: &HeapAddr,
        nat: &NameAndTypeReference,
    ) -> &Value {
        let instance = self.get_instance(h);
        let slot = instance.class.get_field_index_by_nat(nat).unwrap();
        instance.fields.get(slot).unwrap()
    }

    pub fn get_instance_field(&mut self, h: &HeapAddr, name: &str, descriptor: &str) -> &Value {
        let instance = self.get_instance(h);
        let slot = instance.class.get_field_index(name, descriptor).unwrap();
        instance.fields.get(slot).unwrap()
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
        let instance = self.get_instance(&h);
        let value_field = instance
            .get_field_value("value", "[B")
            .ok_or(JvmError::Uninitialized)?;
        let array_addr = match value_field {
            Value::Ref(addr) => *addr,
            _ => {
                return Err(JvmError::NullPointerException);
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
        index: usize,
        val: Value,
    ) -> Result<(), JvmError> {
        match self.get_mut(h) {
            HeapObject::Array(array) => {
                if index >= array.elements.len() {
                    return Err(JvmError::ArrayIndexOutOfBoundsException);
                }
                array.elements[index] = val;
            }
            _ => panic!("heap: write_array_element on non-array"),
        }
        Ok(())
    }

    pub fn write_instance_field(
        &mut self,
        h: HeapAddr,
        field_name: &str,
        field_descriptor: &str,
        val: Value,
    ) -> Result<(), JvmError> {
        match self.get_mut(h) {
            HeapObject::Instance(instance) => {
                let slot = instance
                    .class
                    .get_field_index(field_name, field_descriptor)?;
                instance.fields[slot] = val;
            }
            _ => panic!("heap: write_instance_field on non-instance"),
        }
        Ok(())
    }

    pub fn write_instance_field_by_nat(
        &mut self,
        h: HeapAddr,
        field_nat: &NameAndTypeReference,
        val: Value,
    ) -> Result<(), JvmError> {
        match self.get_mut(h) {
            HeapObject::Instance(instance) => {
                let slot = instance.class.get_field_index_by_nat(field_nat)?;
                instance.fields[slot] = val;
            }
            _ => panic!("heap: write_instance_field on non-instance"),
        }
        Ok(())
    }

    pub fn clone_object(&mut self, h: HeapAddr) -> HeapAddr {
        let obj = self.get(h).expect("heap: invalid handle (clone_object)");
        match obj {
            HeapObject::Instance(inst) => {
                let new_fields = inst.fields.clone();
                let new_instance = ClassInstance {
                    class: inst.class.clone(),
                    fields: new_fields,
                };
                self.push(HeapObject::Instance(new_instance))
            }
            HeapObject::Array(arr) => {
                let new_elements = arr.elements.clone();
                let new_array = ArrayInstance {
                    class: arr.class.clone(),
                    elements: new_elements,
                };
                self.push(HeapObject::Array(new_array))
            }
        }
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
        state.serialize_field("class", &self.class.name())?;
        state.serialize_field("fields", &self.fields)?;
        state.end()
    }
}

#[cfg(test)]
impl serde::Serialize for Heap {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        struct SerializableHeapObject<'a> {
            address: usize,
            object: &'a HeapObject,
        }

        impl<'a> serde::Serialize for SerializableHeapObject<'a> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                match self.object {
                    HeapObject::Instance(inst) => {
                        let mut state = serializer.serialize_struct("HeapObject", 3)?;
                        state.serialize_field("address", &self.address)?;
                        state.serialize_field("type", "Instance")?;
                        state.serialize_field("value", inst)?;
                        state.end()
                    }
                    HeapObject::Array(arr) => {
                        let mut state = serializer.serialize_struct("HeapObject", 4)?;
                        state.serialize_field("address", &self.address)?;
                        state.serialize_field("type", "Array")?;
                        state.serialize_field("class", arr.class().name())?;
                        state.serialize_field("elements", arr.elements())?;
                        state.end()
                    }
                }
            }
        }

        struct SerializableStringPoolEntry<'a> {
            string: &'a String,
            address: HeapAddr,
        }

        impl<'a> serde::Serialize for SerializableStringPoolEntry<'a> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let mut state = serializer.serialize_struct("StringPoolEntry", 2)?;
                state.serialize_field("string", self.string)?;
                state.serialize_field("address", &self.address)?;
                state.end()
            }
        }

        let mut state = serializer.serialize_struct("Heap", 2)?;

        let serializable_objects: Vec<_> = self
            .objects
            .iter()
            .enumerate()
            .map(|(address, object)| SerializableHeapObject { address, object })
            .collect();

        let mut string_pool: Vec<_> = self
            .string_pool
            .iter()
            .map(|(s, &addr)| SerializableStringPoolEntry {
                string: s,
                address: addr,
            })
            .collect();

        string_pool.sort_by_key(|entry| entry.string);

        state.serialize_field("objects", &serializable_objects)?;
        state.serialize_field("string_pool", &string_pool)?;

        state.end()
    }
}
