use common::error::JvmError;
use common::jtype::HeapRef;

#[repr(C)]
pub struct ObjectHeader {
    size: u32, // total bytes (header + data)
    class_id: u32,
    marked: bool,      // for GC in future
    _padding: [u8; 3], // align to 8 bytes
}

impl ObjectHeader {
    const SIZE: usize = size_of::<ObjectHeader>();
}

pub struct Heap {
    memory: *mut u8,
    capacity: usize,
    allocated: usize,
}

impl Heap {
    pub fn new(size_mb: usize) -> Result<Self, String> {
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
            return Err("mmap failed".to_string());
        }

        Ok(Heap {
            memory: memory as *mut u8,
            capacity,
            allocated: 0,
        })
    }

    pub fn allocate_instance(
        &mut self,
        class_id: u32,
        instance_size: usize,
    ) -> Result<HeapRef, JvmError> {
        let heap_ref = self.allocate_raw(instance_size)?;

        let header = unsafe { self.get_header_mut(heap_ref) };
        header.class_id = class_id;
        header.size = (ObjectHeader::SIZE + instance_size) as u32;
        header.marked = false;

        Ok(heap_ref)
    }

    pub fn write_field(
        &mut self,
        heap_ref: HeapRef,
        field_offset: usize,
        data: &[u8],
    ) -> Result<(), String> {
        let data_ptr = unsafe { self.get_data_ptr(heap_ref) };
        unsafe {
            std::ptr::copy_nonoverlapping(data.as_ptr(), data_ptr.add(field_offset), data.len());
        }
        Ok(())
    }

    pub fn read_field(
        &self,
        heap_ref: HeapRef,
        field_offset: usize,
        size: usize,
    ) -> Result<Vec<u8>, String> {
        let mut buffer = vec![0u8; size];
        let data_ptr = unsafe { self.get_data_ptr(heap_ref) };
        unsafe {
            std::ptr::copy_nonoverlapping(data_ptr.add(field_offset), buffer.as_mut_ptr(), size);
        }
        Ok(buffer)
    }

    fn allocate_raw(&mut self, size: usize) -> Result<HeapRef, JvmError> {
        let total_needed = ObjectHeader::SIZE + size;

        if self.allocated + total_needed > self.capacity {
            return Err(JvmError::Todo("Heap full".to_string()));
        }

        let offset = self.allocated;
        self.allocated += total_needed;

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
}
