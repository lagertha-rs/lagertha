use crate::heap::method_area::MethodArea;
use crate::heap::{HeapObject, Instance};
use crate::{ClassId, Symbol};
use common::error::JvmError;
use common::instruction::ArrayType;
use common::jtype::{JavaType, PrimitiveType};
use common::{HeapRef, Value};
use std::collections::HashMap;
use std::num::NonZeroU32;

#[repr(C)]
pub struct ObjectHeader {
    size: u32, // total bytes (header + data)
    class_id: NonZeroU32,
    marked: bool, // for GC in future
    _padding: [u8; 7],
}

impl ObjectHeader {
    const SIZE: usize = size_of::<ObjectHeader>();
}

pub struct Heap {
    memory: *mut u8,
    capacity: usize,
    allocated: usize,
    string_pool: HashMap<Symbol, HeapRef>,
}

impl Heap {
    pub fn new(size_mb: usize) -> Result<Self, JvmError> {
        let capacity = size_mb * 1024 * 1024;

        let memory = unsafe {
            libc::mmap(
                std::ptr::null_mut(),
                capacity,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_PRIVATE | libc::MAP_ANON,
                -1,
                0,
            )
        };

        if memory == libc::MAP_FAILED {
            return Err(JvmError::Todo("mmap failed".to_string()));
        }

        Ok(Heap {
            memory: memory as *mut u8,
            capacity,
            allocated: ObjectHeader::SIZE,
            string_pool: HashMap::new(),
        })
    }

    pub fn allocate_instance(
        &mut self,
        instance_size: usize,
        class_id: ClassId,
    ) -> Result<HeapRef, JvmError> {
        let heap_ref = self.allocate_raw(instance_size)?;

        let header = unsafe { self.get_header_mut(heap_ref) };
        header.class_id = class_id.0;
        header.size = (ObjectHeader::SIZE + instance_size) as u32;
        header.marked = false;

        Ok(heap_ref)
    }

    fn allocate_array_internal(
        &mut self,
        class_id: ClassId,
        length: i32,
        element_size: usize,
    ) -> Result<HeapRef, JvmError> {
        if length < 0 {
            return Err(JvmError::Todo("Negative array length".to_string()));
        }

        let array_data_size = 4 + (length as usize * element_size);
        let heap_ref = self.allocate_raw(array_data_size)?;

        let header = unsafe { self.get_header_mut(heap_ref) };
        header.class_id = class_id.0;
        header.size = (ObjectHeader::SIZE + array_data_size) as u32;
        header.marked = false;

        let data_ptr = unsafe { self.get_data_ptr(heap_ref) };
        unsafe {
            *(data_ptr as *mut i32) = length;
        }

        Ok(heap_ref)
    }

    pub fn allocate_primitive_array(
        &mut self,
        class_id: ClassId,
        array_type: ArrayType,
        length: i32,
    ) -> Result<HeapRef, JvmError> {
        self.allocate_array_internal(class_id, length, array_type.get_byte_size() as usize)
    }

    pub fn allocate_object_array(
        &mut self,
        class_id: ClassId,
        length: i32,
    ) -> Result<HeapRef, JvmError> {
        self.allocate_array_internal(class_id, length, size_of::<HeapRef>())
    }

    pub fn read_array_length(&self, heap_ref: HeapRef) -> Result<i32, JvmError> {
        let data_ptr = unsafe { self.get_data_ptr(heap_ref) };
        let length = unsafe { *(data_ptr as *const i32) };
        Ok(length)
    }

    pub fn write_field(
        &mut self,
        heap_ref: HeapRef,
        field_offset: usize,
        value: Value,
        field_type: &JavaType,
    ) -> Result<(), JvmError> {
        let data_ptr = unsafe { self.get_data_ptr(heap_ref) };
        let target_ptr = unsafe { data_ptr.add(field_offset) };

        match (value, field_type) {
            (Value::Integer(i), JavaType::Primitive(PrimitiveType::Boolean)) => {
                unsafe {
                    *(target_ptr) = if i != 0 { 1 } else { 0 };
                }
                Ok(())
            }
            (Value::Integer(i), JavaType::Primitive(PrimitiveType::Byte)) => {
                unsafe {
                    *(target_ptr as *mut i8) = i as i8;
                }
                Ok(())
            }
            (Value::Integer(i), JavaType::Primitive(PrimitiveType::Short)) => {
                unsafe {
                    *(target_ptr as *mut i16) = i as i16;
                }
                Ok(())
            }
            (Value::Integer(i), JavaType::Primitive(PrimitiveType::Char)) => {
                unsafe {
                    *(target_ptr as *mut u16) = i as u16;
                }
                Ok(())
            }
            (Value::Integer(i), JavaType::Primitive(PrimitiveType::Int)) => {
                unsafe {
                    *(target_ptr as *mut i32) = i;
                }
                Ok(())
            }
            (Value::Long(l), JavaType::Primitive(PrimitiveType::Long)) => {
                unsafe {
                    *(target_ptr as *mut i64) = l;
                }
                Ok(())
            }
            (Value::Float(f), JavaType::Primitive(PrimitiveType::Float)) => {
                unsafe {
                    *(target_ptr as *mut f32) = f;
                }
                Ok(())
            }
            (Value::Double(d), JavaType::Primitive(PrimitiveType::Double)) => {
                unsafe {
                    *(target_ptr as *mut f64) = d;
                }
                Ok(())
            }
            (Value::Ref(r), JavaType::Instance(_) | JavaType::Array(_)) => {
                unsafe {
                    *(target_ptr as *mut HeapRef) = r;
                }
                Ok(())
            }
            (Value::Null, JavaType::Instance(_) | JavaType::Array(_)) => {
                unsafe {
                    *(target_ptr as *mut HeapRef) = 0usize;
                }
                Ok(())
            }
            _ => Err(JvmError::Todo("Type mismatch in write_field".to_string())),
        }
    }

    pub fn read_field(
        &self,
        heap_ref: HeapRef,
        field_offset: usize,
        field_type: &JavaType,
    ) -> Result<Value, JvmError> {
        let data_ptr = unsafe { self.get_data_ptr(heap_ref) };
        let source_ptr = unsafe { data_ptr.add(field_offset) };

        match field_type {
            JavaType::Primitive(prim) => match prim {
                PrimitiveType::Boolean => {
                    let byte_val = unsafe { *(source_ptr as *const u8) };
                    Ok(Value::Integer(if byte_val != 0 { 1 } else { 0 }))
                }
                PrimitiveType::Byte => {
                    let byte_val = unsafe { *(source_ptr as *const i8) };
                    Ok(Value::Integer(byte_val as i32))
                }
                PrimitiveType::Short => {
                    let short_val = unsafe { *(source_ptr as *const i16) };
                    Ok(Value::Integer(short_val as i32))
                }
                PrimitiveType::Char => {
                    let char_val = unsafe { *(source_ptr as *const u16) };
                    Ok(Value::Integer(char_val as i32))
                }
                PrimitiveType::Int => {
                    let int_val = unsafe { *(source_ptr as *const i32) };
                    Ok(Value::Integer(int_val))
                }
                PrimitiveType::Long => {
                    let long_val = unsafe { *(source_ptr as *const i64) };
                    Ok(Value::Long(long_val))
                }
                PrimitiveType::Float => {
                    let float_val = unsafe { *(source_ptr as *const f32) };
                    Ok(Value::Float(float_val))
                }
                PrimitiveType::Double => {
                    let double_val = unsafe { *(source_ptr as *const f64) };
                    Ok(Value::Double(double_val))
                }
            },
            JavaType::Instance(_) | JavaType::Array(_) => {
                let ref_val = unsafe { *(source_ptr as *const HeapRef) };
                if ref_val == 0 {
                    Ok(Value::Null)
                } else {
                    Ok(Value::Ref(ref_val))
                }
            }
            _ => Err(JvmError::Todo(
                "Unsupported field type in read_field".to_string(),
            )),
        }
    }

    fn allocate_raw(&mut self, size: usize) -> Result<HeapRef, JvmError> {
        let total_needed = ObjectHeader::SIZE + size;

        let aligned_total = (total_needed + 7) & !7;

        if self.allocated + aligned_total > self.capacity {
            return Err(JvmError::Todo("Heap full".to_string()));
        }

        let offset = self.allocated;
        self.allocated += aligned_total;

        Ok(offset)
    }

    unsafe fn get_header_mut(&mut self, heap_ref: HeapRef) -> &mut ObjectHeader {
        &mut *(self.memory.add(heap_ref) as *mut ObjectHeader)
    }

    unsafe fn get_header(&self, heap_ref: HeapRef) -> &ObjectHeader {
        &*(self.memory.add(heap_ref) as *const ObjectHeader)
    }

    unsafe fn get_data_ptr(&self, heap_ref: HeapRef) -> *mut u8 {
        self.memory.add(heap_ref + ObjectHeader::SIZE)
    }

    pub fn alloc_string_from_interned_with_char_mapping(
        &mut self,
        val_sym: Symbol,
        method_area: &mut MethodArea,
        f: &dyn Fn(char) -> char,
    ) -> Result<HeapRef, JvmError> {
        let string_class_id = *self.string_class_id.get_or_try_init(|| {
            method_area.get_class_id_or_load(method_area.br().java_lang_string_sym)
        })?;
        let char_array_class_id = *self.char_array_class_id.get_or_try_init(|| {
            method_area.get_class_id_or_load(method_area.br().char_array_desc)
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
        self.write_instance_field(instance, 0, Value::Ref(char_arr_ref))?;
        Ok(instance)
    }

    pub fn alloc_string_from_interned(
        &mut self,
        val_sym: Symbol,
        method_area: &mut MethodArea,
    ) -> Result<HeapRef, JvmError> {
        self.alloc_string_from_interned_with_char_mapping(val_sym, method_area, &|c| c)
    }

    pub fn get_str_from_pool_or_new(
        &mut self,
        val_sym: Symbol,
        method_area: &mut MethodArea,
    ) -> Result<HeapRef, JvmError> {
        if let Some(h) = self.string_pool.get(&val_sym) {
            Ok(*h)
        } else {
            let res = self.alloc_string_from_interned(val_sym, method_area)?;
            self.string_pool.insert(val_sym, res);
            Ok(res)
        }
    }
}
