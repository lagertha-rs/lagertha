// TODO: very primitive implementation, ok for right now

use crate::heap::method_area::MethodArea;
use crate::{ClassId, Symbol, throw_exception};
use common::error::JvmError;
use common::jtype::{HeapRef, Value};
use once_cell::sync::OnceCell;
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

    pub fn get_element(&self, index: i32) -> Result<&Value, JvmError> {
        let index = if index >= 0 && (index as usize) < self.data.len() {
            index as usize
        } else {
            throw_exception!(
                ArrayIndexOutOfBoundsException,
                "Index {} out of bounds for length {}",
                index,
                self.data.len()
            )?
        };
        Ok(&self.data[index])
    }

    pub fn get_element_mut(&mut self, index: i32) -> Result<&mut Value, JvmError> {
        let index = if index >= 0 && (index as usize) < self.data.len() {
            index as usize
        } else {
            throw_exception!(
                ArrayIndexOutOfBoundsException,
                "Index {} out of bounds for length {}",
                index,
                self.data.len()
            )?
        };
        Ok(&mut self.data[index])
    }

    pub fn data(&self) -> &Vec<Value> {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut Vec<Value> {
        &mut self.data
    }

    pub fn data_len(&self) -> usize {
        self.data.len()
    }
}

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-2.html#jvms-2.5.3
pub struct Heap {
    objects: Vec<HeapObject>,
    // TODO: need to think about the string pool since I have the interner
    string_pool: HashMap<Symbol, HeapRef>,
    // Cache class ids for commonly used classes
    string_class_id: OnceCell<ClassId>,
    char_array_class_id: OnceCell<ClassId>,
}

impl Heap {
    pub fn new() -> Result<Self, JvmError> {
        debug!("Creating Heap...");
        Ok(Self {
            string_class_id: OnceCell::new(),
            char_array_class_id: OnceCell::new(),
            string_pool: HashMap::new(),
            objects: Vec::new(),
        })
    }

    fn push(&mut self, obj: HeapObject) -> HeapRef {
        let idx = self.objects.len();
        self.objects.push(obj);
        idx
    }

    pub(crate) fn get(&self, h: &HeapRef) -> Result<&HeapObject, JvmError> {
        self.objects
            .get(*h)
            .ok_or(JvmError::Todo("invalid heap address".to_string()))
    }

    pub(crate) fn get_mut(&mut self, h: &HeapRef) -> Result<&mut HeapObject, JvmError> {
        self.objects
            .get_mut(*h)
            .ok_or(JvmError::Todo("invalid heap address".to_string()))
    }

    pub fn get_instance(&mut self, h: &HeapRef) -> Result<&mut Instance, JvmError> {
        match self.get_mut(h)? {
            HeapObject::Instance(ins) => Ok(ins),
            _ => Err(JvmError::Todo("Non instance".to_string())),
        }
    }

    pub fn get_array(&self, h: &HeapRef) -> Result<&Instance, JvmError> {
        let heap_obj = self.get(h)?;
        match heap_obj {
            HeapObject::Array(arr) => Ok(arr),
            _ => Err(JvmError::Todo("Non instance".to_string())),
        }
    }

    pub fn is_array(&self, h: &HeapRef) -> Result<bool, JvmError> {
        let heap_obj = self.get(h)?;
        match heap_obj {
            HeapObject::Array(_) => Ok(true),
            _ => Ok(false),
        }
    }

    pub fn get_array_mut(&mut self, h: &HeapRef) -> Result<&mut Instance, JvmError> {
        let heap_obj = self.get_mut(h)?;
        match heap_obj {
            HeapObject::Array(arr) => Ok(arr),
            _ => Err(JvmError::Todo("Non instance".to_string())),
        }
    }

    pub fn get_array_len(&mut self, h: &HeapRef) -> Result<usize, JvmError> {
        let arr = self.get_array(h)?;
        Ok(arr.data_len())
    }

    pub fn get_class_id(&self, h: &HeapRef) -> Result<ClassId, JvmError> {
        match self.objects.get(*h) {
            Some(HeapObject::Instance(ins)) => Ok(ins.class_id),
            Some(HeapObject::Array(arr)) => Ok(arr.class_id),
            None => Err(JvmError::Todo("invalid heap address".to_string())),
        }
    }

    pub fn alloc_array_with_default_value(
        &mut self,
        class_id: ClassId,
        default_value: Value,
        length: usize,
    ) -> Result<HeapRef, JvmError> {
        let elements = vec![default_value; length];
        Ok(self.push(HeapObject::Array(Instance {
            class_id,
            data: elements,
        })))
    }

    pub fn get_or_new_string_with_char_mapping(
        &mut self,
        val_sym: Symbol,
        method_area: &mut MethodArea,
        f: &dyn Fn(char) -> char,
    ) -> Result<HeapRef, JvmError> {
        if let Some(h) = self.string_pool.get(&val_sym) {
            Ok(*h)
        } else {
            let string_class_id = *self.string_class_id.get_or_try_init(|| {
                method_area.get_class_id_or_load(method_area.br().java_lang_string_sym)
            })?;
            let char_array_class_id = *self.char_array_class_id.get_or_try_init(|| {
                method_area.get_class_id_or_load(method_area.br().desc_char_array_sym)
            })?;
            let chars_val = {
                let s = method_area.interner().resolve(&val_sym);
                s.chars()
                    .map(|c| Value::Integer(f(c) as i32))
                    .collect::<Vec<_>>()
            };
            let char_arr_ref = self.push(HeapObject::Array(Instance {
                class_id: char_array_class_id,
                data: chars_val,
            }));
            let instance = self.alloc_instance(method_area, string_class_id)?;
            self.string_pool.insert(val_sym, instance);
            self.write_instance_field(instance, 0, Value::Ref(char_arr_ref))?;
            Ok(instance)
        }
    }

    //TODO: review
    pub fn get_or_new_string(
        &mut self,
        val_sym: Symbol,
        method_area: &mut MethodArea,
    ) -> Result<HeapRef, JvmError> {
        self.get_or_new_string_with_char_mapping(val_sym, method_area, &|c| c)
    }

    pub fn clone_object(&mut self, h: &HeapRef) -> Result<HeapRef, JvmError> {
        match self.get(h)? {
            HeapObject::Instance(instance) => {
                let new_instance = Instance {
                    class_id: instance.class_id,
                    data: instance.data.clone(),
                };
                Ok(self.push(HeapObject::Instance(new_instance)))
            }
            HeapObject::Array(array) => {
                let new_array = Instance {
                    class_id: array.class_id,
                    data: array.data.clone(),
                };
                Ok(self.push(HeapObject::Array(new_array)))
            }
        }
    }

    pub fn alloc_instance(
        &mut self,
        method_area: &mut MethodArea,
        class_id: ClassId,
    ) -> Result<HeapRef, JvmError> {
        let fields = method_area
            .get_class(&class_id)
            .get_instance_fields()
            .iter()
            .map(|f| method_area.get_type_descriptor(&f.descriptor_id))
            .map(|d| d.get_default_value())
            .collect::<Vec<Value>>();
        Ok(self.push(HeapObject::Instance(Instance {
            class_id,
            data: fields,
        })))
    }

    pub fn read_instance_field(&self, h: &HeapRef, offset: usize) -> Result<Value, JvmError> {
        match self.objects.get(*h) {
            Some(HeapObject::Instance(instance)) => {
                if offset >= instance.data.len() {
                    return Err(JvmError::Todo("invalid field index".to_string()));
                }
                Ok(instance.data[offset].clone())
            }
            _ => Err(JvmError::Todo(
                "heap: read_instance_field on non-instance".to_string(),
            )),
        }
    }

    pub fn write_instance_field(
        &mut self,
        h: HeapRef,
        offset: usize,
        val: Value,
    ) -> Result<(), JvmError> {
        match self.get_mut(&h)? {
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

    //TODO: check type
    pub fn write_array_element(
        &mut self,
        h: HeapRef,
        index: usize,
        val: Value,
    ) -> Result<(), JvmError> {
        match self.get_mut(&h)? {
            HeapObject::Array(array) => {
                if index >= array.data.len() {
                    return Err(JvmError::Todo("invalid array index".to_string()));
                }
                array.data[index] = val;
            }
            _ => Err(JvmError::Todo(
                "heap: write_array_element on non-array".to_string(),
            ))?,
        }
        Ok(())
    }

    // TODO: just a stub right now
    pub fn get_rust_string_from_java_string(&mut self, h: &HeapRef) -> Result<String, JvmError> {
        let char_array_ref = self
            .read_instance_field(h, 0)
            .map(|reference| reference.as_obj_ref().unwrap())
            .unwrap_or(*h);
        let char_array_instance = self.get_array(&char_array_ref)?;
        let mut result = String::new();
        for val in &char_array_instance.data {
            match val {
                Value::Integer(i) => {
                    let c = (*i as u8) as char;
                    result.push(c);
                }
                _ => {
                    return Err(JvmError::Todo(
                        "Expected integer value in char array".to_string(),
                    ));
                }
            }
        }
        Ok(result)
    }

    //TODO: should be only primitive arrays?

    pub fn copy_primitive_slice(
        &mut self,
        src: HeapRef,
        src_pos: i32,
        dest: HeapRef,
        dest_pos: i32,
        length: i32,
    ) -> Result<(), JvmError> {
        {
            let src_rust_before = self.get_rust_string_from_java_string(&src)?;
            let dest_rust_before = self.get_rust_string_from_java_string(&dest)?;
            let src_array = self.get_array(&src)?;
            let dest_array = self.get_array(&dest)?;

            if src_pos < 0
                || dest_pos < 0
                || length < 0
                || (src_pos as usize + length as usize) > src_array.data_len()
                || (dest_pos as usize + length as usize) > dest_array.data_len()
            {
                throw_exception!(
                    ArrayIndexOutOfBoundsException,
                    "Start or destination index out of bounds"
                )?;
            }
        }
        let src_pos = src_pos as usize;
        let dest_pos = dest_pos as usize;

        let src_ptr = self.get_array(&src)?.data.as_ptr();
        let dest_ptr = self.get_array_mut(&dest)?.data.as_mut_ptr();

        unsafe {
            std::ptr::copy(
                src_ptr.add(src_pos),
                dest_ptr.add(dest_pos),
                length as usize,
            );
        }

        let src_rust_after = self.get_rust_string_from_java_string(&src)?;
        let dest_rust_after = self.get_rust_string_from_java_string(&dest)?;

        Ok(())
    }
}
