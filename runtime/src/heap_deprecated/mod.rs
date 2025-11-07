// TODO: very primitive implementation, ok for right now

use crate::heap_deprecated::method_area_deprecated::MethodAreaDeprecated;
use crate::rt::class_deprecated::ClassDeprecated;
use crate::{ClassIdDeprecated, throw_exception};
use common::error::JavaExceptionFromJvm;
use common::error::JvmError;
use common::jtype::{HeapRef, Value};
use std::collections::HashMap;
use std::sync::Arc;
use tracing_log::log::debug;

pub mod method_area_deprecated;

pub enum HeapObjectDeprecated {
    Instance(InstanceDeprecated),
    Array(InstanceDeprecated),
}

#[derive(Clone)]
pub struct InstanceDeprecated {
    class_id: ClassIdDeprecated,
    data: Vec<Value>,
}

impl InstanceDeprecated {
    pub fn new(class_id: ClassIdDeprecated, elements: Vec<Value>) -> Self {
        Self {
            class_id,
            data: elements,
        }
    }

    pub fn class_id(&self) -> &ClassIdDeprecated {
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
        // TODO: this block is repeated at least 3 times, refactor
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
}

// TODO: probably should be different
// right now to avoid self.alloc_instance(&self.class_class)?; don't want to clone to avoid using self self
enum AllocClass<'a> {
    String,
    Char,
    Class,
    Other(&'a Arc<ClassDeprecated>),
}

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-2.html#jvms-2.5.3
pub struct HeapDeprecated {
    objects: Vec<HeapObjectDeprecated>,
    string_pool: HashMap<String, HeapRef>,
    // TODO: find a better way to expose MethodArea functionality
    string_class: Arc<ClassDeprecated>,
    char_class: Arc<ClassDeprecated>,
    class_class: Arc<ClassDeprecated>,
    mirrors: HashMap<HeapRef, Arc<ClassDeprecated>>,
    primitives: HashMap<HeapRef, HeapRef>,
}

impl HeapDeprecated {
    pub fn new(method_area: &mut MethodAreaDeprecated) -> Result<Self, JvmError> {
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

    pub fn get_class_id(&self, h: &HeapRef) -> ClassIdDeprecated {
        match self.get(*h).unwrap() {
            HeapObjectDeprecated::Instance(inst) => inst.class_id,
            HeapObjectDeprecated::Array(arr) => arr.class_id,
        }
    }

    fn push(&mut self, obj: HeapObjectDeprecated) -> HeapRef {
        let idx = self.objects.len();
        self.objects.push(obj);
        idx
    }

    pub fn alloc_array(
        &mut self,
        class: &Arc<ClassDeprecated>,
        length: usize,
    ) -> Result<HeapRef, JvmError> {
        let default_value = if let Some(primitive_type) = class.primitive() {
            Value::from(&primitive_type)
        } else {
            Value::Null
        };
        let elements = vec![default_value; length];
        Ok(
            self.push(HeapObjectDeprecated::Array(InstanceDeprecated::new(
                *class.id(),
                elements,
            ))),
        )
    }

    pub fn alloc_array_with_value(
        &mut self,
        class: &Arc<ClassDeprecated>,
        length: usize,
        value: Value,
    ) -> Result<HeapRef, JvmError> {
        let elements = vec![value; length];
        Ok(
            self.push(HeapObjectDeprecated::Array(InstanceDeprecated::new(
                *class.id(),
                elements,
            ))),
        )
    }

    fn create_default_fields(class: &Arc<ClassDeprecated>) -> Vec<Value> {
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

    fn alloc_instance_internal(&mut self, class: AllocClass) -> Result<HeapRef, JvmError> {
        let class = match class {
            AllocClass::String => &self.string_class,
            AllocClass::Char => &self.char_class,
            AllocClass::Class => &self.class_class,
            AllocClass::Other(c) => c,
        };
        let fields = Self::create_default_fields(class);
        Ok(
            self.push(HeapObjectDeprecated::Instance(InstanceDeprecated {
                class_id: *class.id(),
                data: fields,
            })),
        )
    }

    pub fn alloc_instance(&mut self, class: &Arc<ClassDeprecated>) -> Result<HeapRef, JvmError> {
        self.alloc_instance_internal(AllocClass::Other(class))
    }

    pub fn get_or_new_string(&mut self, value: &str) -> HeapRef {
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

    fn alloc_string(&mut self, s: &str) -> HeapRef {
        let mut fields = Self::create_default_fields(&self.string_class);

        let chars = s.chars().map(|c| Value::Integer(c as i32)).collect();
        let value = self.push(HeapObjectDeprecated::Array(InstanceDeprecated::new(
            *self.char_class.id(),
            chars,
        )));

        // The "value" field is always the first field in java.lang.String
        fields[0] = Value::Ref(value);

        self.push(HeapObjectDeprecated::Instance(InstanceDeprecated {
            class_id: *self.string_class.id(),
            data: fields,
        }))
    }

    // TODO: return Result and handle errors
    pub fn get(&self, h: HeapRef) -> Result<&HeapObjectDeprecated, JavaExceptionFromJvm> {
        self.objects.get(h).ok_or_else(|| {
            JavaExceptionFromJvm::InternalError(Some(format!("Invalid heap address: {}", h)))
        })
    }

    pub fn get_instance(&self, h: &HeapRef) -> Result<&InstanceDeprecated, JvmError> {
        let heap_obj = self.get(*h)?;
        match heap_obj {
            HeapObjectDeprecated::Instance(inst) => Ok(inst),
            _ => Err(JvmError::Todo(
                "get_by_ref called with non-instance HeapObject".to_string(),
            )),
        }
    }

    pub fn get_instance_mut(&mut self, h: &HeapRef) -> &mut InstanceDeprecated {
        let heap_obj = self.get_mut(*h);
        match heap_obj {
            HeapObjectDeprecated::Instance(inst) => inst,
            _ => panic!("get_by_ref called with non-instance HeapObject",),
        }
    }

    pub fn get_array(&self, h: &HeapRef) -> Result<&InstanceDeprecated, JavaExceptionFromJvm> {
        let heap_obj = self.get(*h)?;
        match heap_obj {
            HeapObjectDeprecated::Array(arr) => Ok(arr),
            _ => Err(JavaExceptionFromJvm::ArrayStoreException(None)),
        }
    }

    pub fn get_array_mut(&mut self, h: &HeapRef) -> &mut InstanceDeprecated {
        let heap_obj = self.get_mut(*h);
        match heap_obj {
            HeapObjectDeprecated::Array(arr) => arr,
            _ => panic!("get_array called with non-array HeapObject",),
        }
    }

    pub fn get_instance_field(
        &mut self,
        addr: &HeapRef,
        offset: usize,
    ) -> Result<&Value, JvmError> {
        let instance = self.get_instance(addr)?;
        instance
            .data
            .get(offset)
            .ok_or(JvmError::Todo("invalid field index".to_string()))
    }

    pub fn get_mut(&mut self, h: HeapRef) -> &mut HeapObjectDeprecated {
        self.objects
            .get_mut(h)
            .expect("heap: invalid handle (get_mut)")
    }

    pub fn addr_is_instance(&self, h: &HeapRef) -> Result<bool, JvmError> {
        let obj = self.get(*h)?;
        Ok(matches!(obj, HeapObjectDeprecated::Instance(_)))
    }

    pub fn addr_is_array(&self, h: &HeapRef) -> Result<bool, JvmError> {
        Ok(!self.addr_is_instance(h)?)
    }

    //TODO: design it lightweight
    pub fn get_string(&self, h: HeapRef) -> Result<String, JvmError> {
        let instance = self.get_instance(&h)?;
        let value_field = instance.get_element(0)?; // "value" field is always the first field in java.lang.String
        let array_addr = match value_field {
            Value::Ref(addr) => *addr,
            _ => {
                return Err(JvmError::JavaException(
                    JavaExceptionFromJvm::NullPointerException(None),
                ));
            }
        };
        let char_array = self.get_array(&array_addr)?;
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
        h: HeapRef,
        index: i32,
        val: Value,
    ) -> Result<(), JvmError> {
        match self.get_mut(h) {
            HeapObjectDeprecated::Array(array) => {
                let index = if index >= 0 && (index as usize) < array.length() {
                    index as usize
                } else {
                    throw_exception!(
                        ArrayIndexOutOfBoundsException,
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
        h: HeapRef,
        offset: usize,
        val: Value,
    ) -> Result<(), JvmError> {
        match self.get_mut(h) {
            HeapObjectDeprecated::Instance(instance) => {
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

    pub fn clone_object(&mut self, h: HeapRef) -> HeapRef {
        let obj = self.get(h).expect("heap: invalid handle (clone_object)");
        match obj {
            HeapObjectDeprecated::Instance(inst) => {
                let new_fields = inst.data.clone();
                let new_instance = InstanceDeprecated {
                    class_id: inst.class_id,
                    data: new_fields,
                };
                self.push(HeapObjectDeprecated::Instance(new_instance))
            }
            HeapObjectDeprecated::Array(arr) => {
                let new_elements = arr.data.clone();
                let new_array = InstanceDeprecated {
                    class_id: arr.class_id,
                    data: new_elements,
                };
                self.push(HeapObjectDeprecated::Array(new_array))
            }
        }
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
            let src_array = self.get_array(&src)?;
            let dest_array = self.get_array(&dest)?;

            if src_pos < 0
                || dest_pos < 0
                || length < 0
                || (src_pos as usize + length as usize) > src_array.length()
                || (dest_pos as usize + length as usize) > dest_array.length()
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
        let dest_ptr = self.get_array_mut(&dest).data.as_mut_ptr();

        unsafe {
            std::ptr::copy(
                src_ptr.add(src_pos),
                dest_ptr.add(dest_pos),
                length as usize,
            );
        }

        Ok(())
    }

    pub fn get_class_by_mirror(&self, mirror: &HeapRef) -> Option<&Arc<ClassDeprecated>> {
        self.mirrors.get(mirror)
    }

    pub(crate) fn get_mirror_addr(
        &mut self,
        target_class: &Arc<ClassDeprecated>,
    ) -> Result<HeapRef, JvmError> {
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
        name: &HeapRef,
    ) -> Result<HeapRef, JvmError> {
        if let Some(addr) = self.primitives.get(name) {
            Ok(*addr)
        } else {
            let mirror_addr = self.alloc_instance_internal(AllocClass::Class)?;
            self.primitives.insert(*name, mirror_addr);
            Ok(mirror_addr)
        }
    }

    pub fn addr_is_primitive(&self, addr: &HeapRef) -> bool {
        self.primitives.values().any(|v| v == addr)
    }
}
