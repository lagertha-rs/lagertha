use crate::constant_pool::{ConstantEntry, ConstantPool, NameAndType, Reference};
use std::collections::HashMap;

pub struct ConstantPoolBuilder {
    entries: Vec<ConstantEntry>,

    utf8_map: HashMap<String, u16>,
    class_map: HashMap<u16, u16>,
    string_map: HashMap<u16, u16>,
    nat_map: HashMap<(u16, u16), u16>,
    methodref_map: HashMap<(u16, u16), u16>,
}

impl ConstantPoolBuilder {
    pub fn new() -> Self {
        Self {
            entries: vec![ConstantEntry::Unused],
            utf8_map: HashMap::new(),
            class_map: HashMap::new(),
            string_map: HashMap::new(),
            nat_map: HashMap::new(),
            methodref_map: HashMap::new(),
        }
    }

    // TODO: constant pool limits?
    fn add_entry(&mut self, entry: ConstantEntry) -> u16 {
        let idx = self.entries.len() as u16;
        let is_wide = matches!(entry, ConstantEntry::Long(_) | ConstantEntry::Double(_));
        self.entries.push(entry);
        if is_wide {
            // long and double take two slots
            self.entries.push(ConstantEntry::Unused);
        }
        idx
    }

    pub fn add_utf8(&mut self, s: &str) -> u16 {
        if let Some(&idx) = self.utf8_map.get(s) {
            return idx;
        }
        let idx = self.add_entry(ConstantEntry::Utf8(s.to_string()));
        self.utf8_map.insert(s.to_string(), idx);
        idx
    }

    pub fn add_class(&mut self, name: &str) -> u16 {
        let name_idx = self.add_utf8(name);
        if let Some(&idx) = self.class_map.get(&name_idx) {
            return idx;
        }
        let idx = self.add_entry(ConstantEntry::Class(name_idx));
        self.class_map.insert(name_idx, idx);
        idx
    }

    pub fn add_string(&mut self, s: &str) -> u16 {
        let utf8_idx = self.add_utf8(s);
        if let Some(&idx) = self.string_map.get(&utf8_idx) {
            return idx;
        }
        let idx = self.add_entry(ConstantEntry::String(utf8_idx));
        self.string_map.insert(utf8_idx, idx);
        idx
    }

    pub fn add_name_and_type(&mut self, name: &str, descriptor: &str) -> u16 {
        let name_idx = self.add_utf8(name);
        let desc_idx = self.add_utf8(descriptor);
        let key = (name_idx, desc_idx);
        if let Some(&idx) = self.nat_map.get(&key) {
            return idx;
        }
        let idx = self.add_entry(ConstantEntry::NameAndType(NameAndType {
            name_index: name_idx,
            descriptor_index: desc_idx,
        }));
        self.nat_map.insert(key, idx);
        idx
    }

    pub fn add_methodref(&mut self, class: &str, name: &str, descriptor: &str) -> u16 {
        let class_idx = self.add_class(class);
        let nat_idx = self.add_name_and_type(name, descriptor);
        let key = (class_idx, nat_idx);
        if let Some(&idx) = self.methodref_map.get(&key) {
            return idx;
        }
        let idx = self.add_entry(ConstantEntry::MethodRef(Reference {
            class_index: class_idx,
            name_and_type_index: nat_idx,
        }));
        self.methodref_map.insert(key, idx);
        idx
    }

    pub fn build(self) -> ConstantPool {
        ConstantPool {
            inner: self.entries,
        }
    }

    pub fn len(&self) -> u16 {
        self.entries.len() as u16
    }

    pub fn is_empty(&self) -> bool {
        self.entries.len() <= 1
    }
}

impl Default for ConstantPoolBuilder {
    fn default() -> Self {
        Self::new()
    }
}
