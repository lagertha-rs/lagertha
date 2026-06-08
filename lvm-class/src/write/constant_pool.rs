use crate::constant_pool::{ConstantEntry, ConstantKind, NameAndType, Reference};

impl ConstantEntry {
    pub fn write_to(&self, buf: &mut Vec<u8>) {
        match self {
            ConstantEntry::Unused => {}
            ConstantEntry::Utf8(s) => {
                buf.push(ConstantKind::Utf8 as u8);
                buf.extend_from_slice(&(s.len() as u16).to_be_bytes());
                buf.extend_from_slice(s.as_bytes());
            }
            ConstantEntry::Integer(i) => {
                buf.push(ConstantKind::Integer as u8);
                buf.extend_from_slice(&i.to_be_bytes());
            }
            ConstantEntry::Class(index) => {
                buf.push(ConstantKind::Class as u8);
                buf.extend_from_slice(&index.to_be_bytes());
            }
            ConstantEntry::String(index) => {
                buf.push(ConstantKind::String as u8);
                buf.extend_from_slice(&index.to_be_bytes());
            }
            ConstantEntry::MethodRef(ref_info) => {
                buf.push(ConstantKind::MethodRef as u8);
                ref_info.write_to(buf);
            }
            ConstantEntry::FieldRef(ref_info) => {
                buf.push(ConstantKind::FieldRef as u8);
                ref_info.write_to(buf);
            }
            ConstantEntry::NameAndType(nat) => {
                buf.push(ConstantKind::NameAndType as u8);
                nat.write_to(buf);
            }
            _ => todo!("Write not implemented for {self:?}"),
        }
    }
}

impl Reference {
    pub fn write_to(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.class_index.to_be_bytes());
        buf.extend_from_slice(&self.name_and_type_index.to_be_bytes());
    }
}

impl NameAndType {
    pub fn write_to(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.name_index.to_be_bytes());
        buf.extend_from_slice(&self.descriptor_index.to_be_bytes());
    }
}
