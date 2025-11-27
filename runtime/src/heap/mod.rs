use crate::keys::ClassId;
use crate::{Symbol, debug_error_log, log_traces, throw_exception};
use common::error::JvmError;
use common::instruction::ArrayType;
use common::jtype::AllocationType;
use common::{HeapRef, Value};
use lasso::ThreadedRodeo;
use std::collections::HashMap;
use std::num::NonZeroU32;
use std::sync::Arc;

pub mod method_area;

#[repr(C)]
pub struct ObjectHeader {
    size: u32, // total bytes (header + data)
    class_id: NonZeroU32,
    marked: bool, // for GC in future
    is_array: bool,
    _padding: [u8; 3],
}

impl ObjectHeader {
    const SIZE: usize = size_of::<ObjectHeader>();

    pub fn is_array(&self) -> bool {
        self.is_array
    }
}

pub struct Heap {
    memory: *mut u8,
    capacity: usize,
    allocated: usize,
    interner: Arc<ThreadedRodeo>,
    string_pool: HashMap<Symbol, HeapRef>,
    byte_array_class_id: ClassId,
    string_class_id: ClassId,
    string_instance_size: usize,
}

impl Heap {
    pub const OBJECT_HEADER_SIZE: usize = ObjectHeader::SIZE;
    pub const ARRAY_LENGTH_OFFSET: usize = 0;
    pub const ARRAY_TYPE_OFFSET: usize = 4;
    pub const ARRAY_ELEMENTS_OFFSET: usize = 8;
    const LATIN1: i32 = 0;
    const UTF16: i32 = 1;

    pub fn new(
        size_mb: usize,
        interner: Arc<ThreadedRodeo>,
        string_class_id: ClassId,
        string_instance_size: usize,
        char_array_class_id: ClassId,
    ) -> Result<Self, JvmError> {
        // TODO: delete in the future
        assert_eq!(size_of::<ObjectHeader>(), 16);
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
            interner,
            string_class_id,
            string_instance_size,
            byte_array_class_id: char_array_class_id,
        })
    }

    fn alloc_raw(&mut self, size: usize) -> Result<HeapRef, JvmError> {
        let total_needed = ObjectHeader::SIZE + size;

        // align to 8 bytes
        let aligned_total = (total_needed + 7) & !7;

        if self.allocated + aligned_total > self.capacity {
            // TODO: OOM
            return Err(JvmError::Todo("Heap full".to_string()));
        }

        let offset = self.allocated;
        self.allocated += aligned_total;

        // zero initialize
        let data_ptr = unsafe { self.get_data_ptr(offset) };
        unsafe {
            std::ptr::write_bytes(data_ptr, 0, size);
        }

        Ok(offset)
    }

    pub fn is_array(&self, heap_ref: HeapRef) -> Result<bool, JvmError> {
        let header = self.get_header(heap_ref);
        Ok(header.is_array())
    }

    fn get_header_mut(&mut self, heap_ref: HeapRef) -> &mut ObjectHeader {
        unsafe { &mut *(self.memory.add(heap_ref) as *mut ObjectHeader) }
    }

    pub fn get_header(&self, heap_ref: HeapRef) -> &ObjectHeader {
        unsafe { &*(self.memory.add(heap_ref) as *const ObjectHeader) }
    }

    unsafe fn get_data_ptr(&self, heap_ref: HeapRef) -> *mut u8 {
        self.memory.add(heap_ref + ObjectHeader::SIZE)
    }

    fn get_allocation_type(&self, heap_ref: HeapRef) -> Result<AllocationType, JvmError> {
        self.is_array(heap_ref)?;
        let data_ptr = unsafe { self.get_data_ptr(heap_ref) };
        let type_byte = unsafe { *(data_ptr.add(Self::ARRAY_TYPE_OFFSET) as *const u8) };
        AllocationType::try_from(type_byte)
            .map_err(|_| JvmError::Todo("Invalid allocation type".to_string()))
    }

    pub fn alloc_instance(
        &mut self,
        instance_size: usize,
        class_id: ClassId,
    ) -> Result<HeapRef, JvmError> {
        let heap_ref = self.alloc_raw(instance_size)?;

        let header = self.get_header_mut(heap_ref);
        header.class_id = class_id.into_inner();
        header.size = (ObjectHeader::SIZE + instance_size) as u32;
        header.marked = false;
        header.is_array = false;

        Ok(heap_ref)
    }

    fn alloc_array_internal(
        &mut self,
        class_id: ClassId,
        length: i32,
        allocation_type: AllocationType,
    ) -> Result<HeapRef, JvmError> {
        if length < 0 {
            return Err(JvmError::Todo("Negative array length".to_string()));
        }

        let element_size = allocation_type.byte_size();
        let array_data_size = Self::ARRAY_ELEMENTS_OFFSET + (length as usize * element_size);
        let heap_ref = self.alloc_raw(array_data_size)?;

        let header = self.get_header_mut(heap_ref);
        header.class_id = class_id.into_inner();
        header.size = (ObjectHeader::SIZE + array_data_size) as u32;
        header.marked = false;
        header.is_array = true;

        let data_ptr = unsafe { self.get_data_ptr(heap_ref) };
        unsafe {
            *(data_ptr as *mut i32) = length;
            *(data_ptr.add(Self::ARRAY_TYPE_OFFSET)) = allocation_type as u8;
        }

        Ok(heap_ref)
    }

    pub fn alloc_primitive_array(
        &mut self,
        class_id: ClassId,
        array_type: ArrayType,
        length: i32,
    ) -> Result<HeapRef, JvmError> {
        let allocation_type = match array_type {
            ArrayType::Boolean => AllocationType::Boolean,
            ArrayType::Byte => AllocationType::Byte,
            ArrayType::Short => AllocationType::Short,
            ArrayType::Char => AllocationType::Char,
            ArrayType::Int => AllocationType::Int,
            ArrayType::Long => AllocationType::Long,
            ArrayType::Float => AllocationType::Float,
            ArrayType::Double => AllocationType::Double,
        };
        let heap_ref = self.alloc_array_internal(class_id, length, allocation_type)?;
        Ok(heap_ref)
    }

    pub fn alloc_object_array(
        &mut self,
        class_id: ClassId,
        length: i32,
    ) -> Result<HeapRef, JvmError> {
        self.alloc_array_internal(class_id, length, AllocationType::Reference)
    }

    pub fn get_class_id(&self, heap_ref: HeapRef) -> Result<ClassId, JvmError> {
        let header = self.get_header(heap_ref);
        Ok(ClassId::new(header.class_id))
    }

    pub fn get_array_length(&self, heap_ref: HeapRef) -> Result<i32, JvmError> {
        self.is_array(heap_ref)?;
        let data_ptr = unsafe { self.get_data_ptr(heap_ref) };
        let length = unsafe { *(data_ptr as *const i32) };
        Ok(length)
    }

    pub fn write_array_element(
        &mut self,
        heap_ref: HeapRef,
        index: i32,
        value: Value,
    ) -> Result<(), JvmError> {
        let length = self.get_array_length(heap_ref)?;
        if index < 0 || index >= length {
            throw_exception!(
                ArrayIndexOutOfBoundsException,
                "Index {} out of bounds for length {}",
                index,
                length
            )?
        }

        let element_type = self.get_allocation_type(heap_ref)?;
        let element_size = element_type.byte_size();
        let field_offset = Self::ARRAY_ELEMENTS_OFFSET + (index as usize * element_size);

        self.write_field(heap_ref, field_offset, value, element_type)
    }

    pub fn read_array_element(&self, heap_ref: HeapRef, index: i32) -> Result<Value, JvmError> {
        let length = self.get_array_length(heap_ref)?;
        if index < 0 || index >= length {
            throw_exception!(
                ArrayIndexOutOfBoundsException,
                "Index {} out of bounds for length {}",
                index,
                length
            )?
        }

        let element_type = self.get_allocation_type(heap_ref)?;
        let field_offset =
            Self::ARRAY_ELEMENTS_OFFSET + (index as usize * element_type.byte_size());

        self.read_field(heap_ref, field_offset, element_type)
    }

    pub fn write_field(
        &mut self,
        heap_ref: HeapRef,
        field_offset: usize,
        value: Value,
        field_type: AllocationType,
    ) -> Result<(), JvmError> {
        let data_ptr = unsafe { self.get_data_ptr(heap_ref) };
        let target_ptr = unsafe { data_ptr.add(field_offset) };

        match (value, field_type) {
            (Value::Integer(i), AllocationType::Boolean) => {
                unsafe {
                    *(target_ptr) = if i != 0 { 1 } else { 0 };
                }
                Ok(())
            }
            (Value::Integer(i), AllocationType::Byte) => {
                unsafe {
                    *(target_ptr as *mut i8) = i as i8;
                }
                Ok(())
            }
            (Value::Integer(i), AllocationType::Short) => {
                unsafe {
                    *(target_ptr as *mut i16) = i as i16;
                }
                Ok(())
            }
            (Value::Integer(i), AllocationType::Char) => {
                unsafe {
                    *(target_ptr as *mut u16) = i as u16;
                }
                Ok(())
            }
            (Value::Integer(i), AllocationType::Int) => {
                unsafe {
                    *(target_ptr as *mut i32) = i;
                }
                Ok(())
            }
            (Value::Long(l), AllocationType::Long) => {
                unsafe {
                    *(target_ptr as *mut i64) = l;
                }
                Ok(())
            }
            (Value::Float(f), AllocationType::Float) => {
                unsafe {
                    *(target_ptr as *mut f32) = f;
                }
                Ok(())
            }
            (Value::Double(d), AllocationType::Double) => {
                unsafe {
                    *(target_ptr as *mut f64) = d;
                }
                Ok(())
            }
            (Value::Ref(r), AllocationType::Reference) => {
                unsafe {
                    *(target_ptr as *mut HeapRef) = r;
                }
                Ok(())
            }
            (Value::Null, AllocationType::Reference) => {
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
        field_type: AllocationType,
    ) -> Result<Value, JvmError> {
        let data_ptr = unsafe { self.get_data_ptr(heap_ref) };
        let source_ptr = unsafe { data_ptr.add(field_offset) };

        match field_type {
            AllocationType::Boolean => {
                let byte_val = unsafe { *(source_ptr as *const u8) };
                Ok(Value::Integer(if byte_val != 0 { 1 } else { 0 }))
            }
            AllocationType::Byte => {
                let byte_val = unsafe { *(source_ptr as *const i8) };
                Ok(Value::Integer(byte_val as i32))
            }
            AllocationType::Short => {
                let short_val = unsafe { *(source_ptr as *const i16) };
                Ok(Value::Integer(short_val as i32))
            }
            AllocationType::Char => {
                let char_val = unsafe { *(source_ptr as *const u16) };
                Ok(Value::Integer(char_val as i32))
            }
            AllocationType::Int => {
                let int_val = unsafe { *(source_ptr as *const i32) };
                Ok(Value::Integer(int_val))
            }
            AllocationType::Long => {
                let long_val = unsafe { *(source_ptr as *const i64) };
                Ok(Value::Long(long_val))
            }
            AllocationType::Float => {
                let float_val = unsafe { *(source_ptr as *const f32) };
                Ok(Value::Float(float_val))
            }
            AllocationType::Double => {
                let double_val = unsafe { *(source_ptr as *const f64) };
                Ok(Value::Double(double_val))
            }
            AllocationType::Reference => {
                let ref_val = unsafe { *(source_ptr as *const HeapRef) };
                if ref_val == 0 {
                    Ok(Value::Null)
                } else {
                    Ok(Value::Ref(ref_val))
                }
            }
        }
    }

    pub fn alloc_string(&mut self, s: &str) -> Result<HeapRef, JvmError> {
        self.alloc_string_from_str_with_char_mapping(s, None)
    }

    fn can_encode_latin1(s: &str) -> bool {
        s.chars().all(|c| (c as u32) <= 0xFF)
    }
    fn can_encode_latin1_with_map(s: &str, f: Option<&dyn Fn(char) -> char>) -> bool {
        if let Some(mapper) = f {
            s.chars().map(mapper).all(|c| (c as u32) <= 0xFF)
        } else {
            Self::can_encode_latin1(s)
        }
    }

    fn alloc_ascii_byte_array_internal(&mut self, s: &str) -> Result<(HeapRef, i32), JvmError> {
        let byte_array =
            self.alloc_primitive_array(self.byte_array_class_id, ArrayType::Byte, s.len() as i32)?;

        let byte_slice = self.get_byte_array_slice_mut(byte_array)?;

        unsafe {
            std::ptr::copy_nonoverlapping(
                s.as_ptr() as *const i8,
                byte_slice.as_mut_ptr(),
                s.len(),
            );
        }

        Ok((byte_array, Self::LATIN1))
    }

    fn alloc_latin1_byte_array_internal(
        &mut self,
        s: &str,
        f: Option<&dyn Fn(char) -> char>,
    ) -> Result<(HeapRef, i32), JvmError> {
        let char_count = s.chars().count();
        let byte_array = self.alloc_primitive_array(
            self.byte_array_class_id,
            ArrayType::Byte,
            char_count as i32,
        )?;

        let byte_slice = self.get_byte_array_slice_mut(byte_array)?;

        if let Some(mapper) = f {
            for (i, c) in s.chars().map(mapper).enumerate() {
                byte_slice[i] = c as i8;
            }
        } else {
            for (i, c) in s.chars().enumerate() {
                byte_slice[i] = c as i8;
            }
        }

        Ok((byte_array, Self::LATIN1))
    }

    fn alloc_utf16_byte_array_internal(
        &mut self,
        s: &str,
        f: Option<&dyn Fn(char) -> char>,
    ) -> Result<(HeapRef, i32), JvmError> {
        let utf16_units: Vec<u16> = if let Some(mapper) = f {
            let mut units = Vec::with_capacity(s.len() + (s.len() / 5)); // rough estimate
            for c in s.chars() {
                let mapped = mapper(c);
                let mut buf = [0u16; 2];
                let encoded = mapped.encode_utf16(&mut buf);
                units.extend_from_slice(encoded);
            }
            units
        } else {
            s.encode_utf16().collect()
        };

        let byte_count = utf16_units.len() * 2;
        let byte_array = self.alloc_primitive_array(
            self.byte_array_class_id,
            ArrayType::Byte,
            byte_count as i32,
        )?;

        let byte_slice = self.get_byte_array_slice_mut(byte_array)?;

        unsafe {
            std::ptr::copy_nonoverlapping(
                utf16_units.as_ptr() as *const i8,
                byte_slice.as_mut_ptr(),
                byte_count,
            );
        }

        Ok((byte_array, Self::UTF16))
    }

    pub fn alloc_string_from_str_with_char_mapping(
        &mut self,
        s: &str,
        f: Option<&dyn Fn(char) -> char>,
    ) -> Result<HeapRef, JvmError> {
        let (byte_array_ref, coder) = if f.is_none() && s.is_ascii() {
            // latin1 but optimized for ASCII
            self.alloc_ascii_byte_array_internal(s)?
        } else if Self::can_encode_latin1_with_map(s, f) {
            // latin1
            self.alloc_latin1_byte_array_internal(s, f)?
        } else {
            // UTF-16
            // TODO: it is added but not tested at all, and my jclass can't take MUTF-8 strings yet
            self.alloc_utf16_byte_array_internal(s, f)?
        };

        let string_instance =
            self.alloc_instance(self.string_instance_size, self.string_class_id)?;

        // Write byte[] reference to field 0
        self.write_field(
            string_instance,
            0,
            Value::Ref(byte_array_ref),
            AllocationType::Reference,
        )?;

        // Write coder to field 1
        self.write_field(
            string_instance,
            AllocationType::Reference.byte_size(), // offset 8, after byte[] reference
            Value::Integer(coder),
            AllocationType::Byte,
        )?;

        Ok(string_instance)
    }

    pub fn alloc_string_from_interned_with_char_mapping(
        &mut self,
        val_sym: Symbol,
        f: Option<&dyn Fn(char) -> char>,
    ) -> Result<HeapRef, JvmError> {
        let interner = self.interner.clone();
        let s = interner.resolve(&val_sym);
        self.alloc_string_from_str_with_char_mapping(s, f)
    }

    pub fn alloc_string_from_interned(&mut self, val_sym: Symbol) -> Result<HeapRef, JvmError> {
        self.alloc_string_from_interned_with_char_mapping(val_sym, None)
    }

    pub fn get_str_from_pool_or_new(&mut self, val_sym: Symbol) -> Result<HeapRef, JvmError> {
        if let Some(h) = self.string_pool.get(&val_sym) {
            Ok(*h)
        } else {
            let res = self.alloc_string_from_interned(val_sym)?;
            self.string_pool.insert(val_sym, res);
            Ok(res)
        }
    }

    // TODO: just a stub right now
    pub fn get_rust_string_from_java_string(&mut self, h: HeapRef) -> Result<String, JvmError> {
        // Read byte[] value field (offset 0)
        let byte_array_ref = match self.read_field(h, 0, AllocationType::Reference)? {
            Value::Ref(r) => r,
            Value::Null => return Err(JvmError::Todo("String.value is null".to_string())),
            _ => {
                return Err(JvmError::Todo(
                    "String.value is not a reference".to_string(),
                ));
            }
        };

        // Read coder field (offset 8)
        let coder = match self.read_field(h, 8, AllocationType::Byte)? {
            Value::Integer(c) => c,
            _ => return Err(JvmError::Todo("String.coder is not a byte".to_string())),
        };

        let byte_slice = self.get_byte_array_slice(byte_array_ref)?;

        match coder {
            Self::LATIN1 => {
                let chars: String = byte_slice.iter().map(|&b| (b as u8) as char).collect();
                Ok(chars)
            }
            Self::UTF16 => {
                if byte_slice.len() % 2 != 0 {
                    return Err(JvmError::Todo(
                        "Invalid UTF-16 byte array (odd length)".to_string(),
                    ));
                }

                let mut utf16_units = Vec::with_capacity(byte_slice.len() / 2);
                for chunk in byte_slice.chunks_exact(2) {
                    let code_unit = u16::from_le_bytes([chunk[0] as u8, chunk[1] as u8]);
                    utf16_units.push(code_unit);
                }

                Ok(String::from_utf16_lossy(&utf16_units))
            }
            _ => Err(JvmError::Todo(format!("Unknown String coder: {}", coder))),
        }
    }

    pub fn copy_primitive_slice(
        &mut self,
        src: HeapRef,
        src_pos: i32,
        dest: HeapRef,
        dest_pos: i32,
        length: i32,
    ) -> Result<(), JvmError> {
        {
            let src_type = self.get_allocation_type(src)?;
            let dest_type = self.get_allocation_type(dest)?;

            /* TODO
            if src_type != dest_type {
                return Err(JvmError::Todo(
                    "Array types must match for copy".to_string(),
                ));
            }
             */

            let src_array_len = self.get_array_length(src)?;
            let dest_array_len = self.get_array_length(dest)?;

            if src_pos < 0
                || dest_pos < 0
                || length < 0
                || (src_pos + length) > src_array_len
                || (dest_pos + length) > dest_array_len
            {
                throw_exception!(
                    ArrayIndexOutOfBoundsException,
                    "Start or destination index out of bounds"
                )?;
            }
        }

        let src_pos = src_pos as usize;
        let dest_pos = dest_pos as usize;
        let allocation_type = self.get_allocation_type(src)?;
        let element_size = allocation_type.byte_size();

        let src_data_ptr = unsafe { self.get_data_ptr(src) };
        let dest_data_ptr = unsafe { self.get_data_ptr(dest) };

        let src_ptr =
            unsafe { src_data_ptr.add(Self::ARRAY_ELEMENTS_OFFSET + src_pos * element_size) };
        let dest_ptr =
            unsafe { dest_data_ptr.add(Self::ARRAY_ELEMENTS_OFFSET + dest_pos * element_size) };

        unsafe {
            std::ptr::copy(src_ptr, dest_ptr, length as usize * element_size);
        }

        Ok(())
    }

    pub fn clone_object(&mut self, src: HeapRef) -> Result<HeapRef, JvmError> {
        let (class_id, data_size, is_array) = {
            let src_header = self.get_header(src);
            (
                src_header.class_id,
                src_header.size as usize - ObjectHeader::SIZE,
                src_header.is_array,
            )
        };

        let dest = self.alloc_raw(data_size)?;

        let src_data_ptr = unsafe { self.get_data_ptr(src) };
        let dest_data_ptr = unsafe { self.get_data_ptr(dest) };

        unsafe {
            std::ptr::copy_nonoverlapping(src_data_ptr, dest_data_ptr, data_size);
        }

        let dest_header = self.get_header_mut(dest);
        dest_header.class_id = class_id;
        dest_header.marked = false;
        dest_header.is_array = is_array;

        Ok(dest)
    }

    pub fn get_array_bytes(&self, heap_ref: HeapRef) -> Result<&[u8], JvmError> {
        let header = self.get_header(heap_ref);
        if !header.is_array() {
            return Err(JvmError::Todo("Not an array".to_string()));
        }

        let length = self.get_array_length(heap_ref)?;
        let allocation_type = self.get_allocation_type(heap_ref)?;
        let element_size = allocation_type.byte_size();
        let total_bytes = length as usize * element_size;

        let data_ptr = unsafe { self.get_data_ptr(heap_ref) };
        let elements_ptr = unsafe { data_ptr.add(Self::ARRAY_ELEMENTS_OFFSET) };

        Ok(unsafe { std::slice::from_raw_parts(elements_ptr, total_bytes) })
    }

    pub fn get_char_array_slice(&self, heap_ref: HeapRef) -> Result<&[u16], JvmError> {
        let allocation_type = self.get_allocation_type(heap_ref)?;
        if allocation_type != AllocationType::Char {
            return Err(JvmError::Todo("Not a char array".to_string()));
        }

        let length = self.get_array_length(heap_ref)?;
        let data_ptr = unsafe { self.get_data_ptr(heap_ref) };
        let elements_ptr = unsafe { data_ptr.add(Self::ARRAY_ELEMENTS_OFFSET) };

        Ok(unsafe { std::slice::from_raw_parts(elements_ptr as *const u16, length as usize) })
    }

    pub fn get_byte_array_slice(&self, heap_ref: HeapRef) -> Result<&[i8], JvmError> {
        let allocation_type = self.get_allocation_type(heap_ref)?;
        if allocation_type != AllocationType::Byte {
            return Err(JvmError::Todo("Not a byte array".to_string()));
        }

        let length = self.get_array_length(heap_ref)?;
        let data_ptr = unsafe { self.get_data_ptr(heap_ref) };
        let elements_ptr = unsafe { data_ptr.add(Self::ARRAY_ELEMENTS_OFFSET) };

        Ok(unsafe { std::slice::from_raw_parts(elements_ptr as *const i8, length as usize) })
    }

    pub fn get_byte_array_slice_mut(&self, heap_ref: HeapRef) -> Result<&mut [i8], JvmError> {
        let allocation_type = self.get_allocation_type(heap_ref)?;
        if allocation_type != AllocationType::Byte {
            return Err(JvmError::Todo("Not a byte array".to_string()));
        }

        let length = self.get_array_length(heap_ref)?;
        let data_ptr = unsafe { self.get_data_ptr(heap_ref) };
        let elements_ptr = unsafe { data_ptr.add(Self::ARRAY_ELEMENTS_OFFSET) };

        Ok(unsafe { std::slice::from_raw_parts_mut(elements_ptr as *mut i8, length as usize) })
    }

    pub fn get_int_array_slice(&self, heap_ref: HeapRef) -> Result<&[i32], JvmError> {
        let allocation_type = self.get_allocation_type(heap_ref)?;
        if allocation_type != AllocationType::Int {
            return Err(JvmError::Todo("Not an int array".to_string()));
        }

        let length = self.get_array_length(heap_ref)?;
        let data_ptr = unsafe { self.get_data_ptr(heap_ref) };
        let elements_ptr = unsafe { data_ptr.add(Self::ARRAY_ELEMENTS_OFFSET) };

        Ok(unsafe { std::slice::from_raw_parts(elements_ptr as *const i32, length as usize) })
    }
}

impl Drop for Heap {
    fn drop(&mut self) {
        unsafe {
            libc::munmap(self.memory as *mut libc::c_void, self.capacity);
            let result = libc::munmap(self.memory as *mut libc::c_void, self.capacity);
            if result != 0 {
                debug_error_log!("munmap failed during Heap drop");
            }
        }
    }
}
