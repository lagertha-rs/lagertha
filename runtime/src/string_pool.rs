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
        debug!("Getting or creating string in pool: {}", text);
        if let Some(&h) = self.pool.get(text) {
            debug!("String found in pool: {}", text);
            return h;
        }
        debug!("String not found in pool. Creating new one: {}", text);
        let h = heap.alloc_string(text);
        self.pool.insert(text.to_string(), h);
        h
    }

    pub fn contains(&self, s: &str) -> bool {
        self.pool.contains_key(s)
    }
}

#[cfg(test)]
impl serde::Serialize for StringPool {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut values = self.pool.keys().collect::<Vec<_>>();
        values.sort_unstable();
        values.serialize(serializer)
    }
}
