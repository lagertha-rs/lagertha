use crate::heap::Heap;
use common::jtype::HeapAddr;
use std::collections::HashMap;
use tracing_log::log::debug;

pub struct StringPool {
    pool: HashMap<String, HeapAddr>,
}

impl StringPool {
    pub fn new() -> Self {
        debug!("Initializing StringPool...");
        Self {
            pool: HashMap::new(),
        }
    }
    pub fn get_or_new(&mut self, heap: &mut Heap, text: &str) -> HeapAddr {
        if let Some(&h) = self.pool.get(text) {
            return h;
        }
        let h = heap.alloc_string(text);
        self.pool.insert(text.to_string(), h);
        h
    }

    pub fn contains(&self, s: &str) -> bool {
        self.pool.contains_key(s)
    }
}
