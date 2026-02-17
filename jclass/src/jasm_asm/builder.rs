use crate::constant_pool::{ConstantEntry, ConstantPool, NameAndType, Reference};
use std::collections::HashMap;

// a little bit of extra complexity to match javac's order of constant pool entries
// e.g. all fn with "reserve" suffix are added to copy the javac
#[derive(Debug, Clone)]
pub struct ConstantPoolBuilder {
    entries: Vec<ConstantEntry>,

    utf8_map: HashMap<String, u16>,
    class_map: HashMap<u16, u16>,
    string_map: HashMap<u16, u16>,
    nat_map: HashMap<(u16, u16), u16>,
    methodref_map: HashMap<(u16, u16), u16>,
    fieldref_map: HashMap<(u16, u16), u16>,
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
            fieldref_map: HashMap::new(),
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

    // In javac when adding a Ref entry, the Class and NameAndType entries are reserved first
    // before any Utf8 entries are added.
    fn reserve_entry(&mut self) -> u16 {
        let idx = self.entries.len() as u16;
        self.entries.push(ConstantEntry::Unused);
        idx
    }

    // TODO: can't use jasm error, find how to bubble up errors from cp builder and handle them in jasm
    fn fill_entry(&mut self, idx: u16, entry: ConstantEntry) {
        debug_assert!(
            matches!(self.entries[idx as usize], ConstantEntry::Unused),
            "bug in cp builder"
        );
        self.entries[idx as usize] = entry;
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
        if let Some(&utf8_idx) = self.utf8_map.get(name)
            && let Some(&idx) = self.class_map.get(&utf8_idx)
        {
            return idx;
        }
        let class_idx = self.reserve_entry();
        let utf8_idx = self.add_utf8(name);
        self.fill_entry(class_idx, ConstantEntry::Class(utf8_idx));
        self.class_map.insert(utf8_idx, class_idx);
        class_idx
    }

    pub fn add_string(&mut self, s: &str) -> u16 {
        if let Some(&utf8_idx) = self.utf8_map.get(s)
            && let Some(&idx) = self.string_map.get(&utf8_idx)
        {
            return idx;
        }
        let string_idx = self.reserve_entry();
        let utf8_idx = self.add_utf8(s);
        self.fill_entry(string_idx, ConstantEntry::String(utf8_idx));
        self.string_map.insert(utf8_idx, string_idx);
        string_idx
    }

    pub fn add_methodref(&mut self, class: &str, name: &str, descriptor: &str) -> u16 {
        if let Some(idx) = self.find_ref(&self.methodref_map, class, name, descriptor) {
            return idx;
        }
        let (ref_idx, class_idx, nat_idx) = self.add_ref_entries(class, name, descriptor);
        self.fill_entry(
            ref_idx,
            ConstantEntry::MethodRef(Reference {
                class_index: class_idx,
                name_and_type_index: nat_idx,
            }),
        );
        self.methodref_map.insert((class_idx, nat_idx), ref_idx);
        ref_idx
    }

    pub fn add_fieldref(&mut self, class: &str, name: &str, descriptor: &str) -> u16 {
        if let Some(idx) = self.find_ref(&self.fieldref_map, class, name, descriptor) {
            return idx;
        }
        let (ref_idx, class_idx, nat_idx) = self.add_ref_entries(class, name, descriptor);
        self.fill_entry(
            ref_idx,
            ConstantEntry::FieldRef(Reference {
                class_index: class_idx,
                name_and_type_index: nat_idx,
            }),
        );
        self.fieldref_map.insert((class_idx, nat_idx), ref_idx);
        ref_idx
    }

    fn add_ref_entries(&mut self, class: &str, name: &str, descriptor: &str) -> (u16, u16, u16) {
        let ref_idx = self.reserve_entry();

        let class_idx = self.find_or_reserve_class(class);
        let nat_idx = self.find_or_reserve_nat(name, descriptor);

        let class_name_idx = self.add_utf8(class);
        let name_idx = self.add_utf8(name);
        let desc_idx = self.add_utf8(descriptor);

        self.fill_class_if_reserved(class_idx, class_name_idx);
        self.fill_nat_if_reserved(nat_idx, name_idx, desc_idx);

        (ref_idx, class_idx, nat_idx)
    }

    fn find_or_reserve_class(&mut self, name: &str) -> u16 {
        if let Some(&name_idx) = self.utf8_map.get(name)
            && let Some(&idx) = self.class_map.get(&name_idx)
        {
            return idx;
        }
        self.reserve_entry()
    }

    fn find_or_reserve_nat(&mut self, name: &str, descriptor: &str) -> u16 {
        if let Some(&name_idx) = self.utf8_map.get(name)
            && let Some(&desc_idx) = self.utf8_map.get(descriptor)
            && let Some(&idx) = self.nat_map.get(&(name_idx, desc_idx))
        {
            return idx;
        }
        self.reserve_entry()
    }

    fn fill_class_if_reserved(&mut self, class_idx: u16, name_idx: u16) {
        if matches!(self.entries[class_idx as usize], ConstantEntry::Unused) {
            self.entries[class_idx as usize] = ConstantEntry::Class(name_idx);
            self.class_map.insert(name_idx, class_idx);
        }
    }

    fn fill_nat_if_reserved(&mut self, nat_idx: u16, name_idx: u16, desc_idx: u16) {
        if matches!(self.entries[nat_idx as usize], ConstantEntry::Unused) {
            self.entries[nat_idx as usize] = ConstantEntry::NameAndType(NameAndType {
                name_index: name_idx,
                descriptor_index: desc_idx,
            });
            self.nat_map.insert((name_idx, desc_idx), nat_idx);
        }
    }

    fn find_ref(
        &self,
        ref_map: &HashMap<(u16, u16), u16>,
        class: &str,
        name: &str,
        descriptor: &str,
    ) -> Option<u16> {
        let name_idx = *self.utf8_map.get(class)?;
        let class_idx = *self.class_map.get(&name_idx)?;
        let method_name_idx = *self.utf8_map.get(name)?;
        let desc_idx = *self.utf8_map.get(descriptor)?;
        let nat_idx = *self.nat_map.get(&(method_name_idx, desc_idx))?;
        ref_map.get(&(class_idx, nat_idx)).copied()
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
