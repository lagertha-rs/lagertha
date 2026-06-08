use super::AttributeNameMap;
use crate::attribute::AttributeKind;
use crate::attribute::method::{CodeAttribute, MethodAttribute};

impl MethodAttribute {
    pub fn write_to(&self, buf: &mut Vec<u8>, attr_names: &AttributeNameMap) {
        let kind = match self {
            MethodAttribute::Code(_) => AttributeKind::Code,
            other => unimplemented!("Method attribute {:?} not implemented for writing", other),
        };

        let name_index = attr_names[&kind];
        buf.extend_from_slice(&name_index.to_be_bytes());

        // TODO: avoid having a buffer, I can know the size before without it.
        let mut body = Vec::new();
        match self {
            MethodAttribute::Code(code) => code.write_to(&mut body),
            _ => unreachable!(),
        }

        buf.extend_from_slice(&(body.len() as u32).to_be_bytes());
        buf.extend_from_slice(&body);
    }
}

impl CodeAttribute {
    pub fn write_to(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.max_stack.to_be_bytes());
        buf.extend_from_slice(&self.max_locals.to_be_bytes());
        buf.extend_from_slice(&(self.code.len() as u32).to_be_bytes());
        buf.extend_from_slice(&self.code);
        buf.extend_from_slice(&(self.exception_table.len() as u16).to_be_bytes());
        for entry in &self.exception_table {
            buf.extend_from_slice(&entry.start_pc.to_be_bytes());
            buf.extend_from_slice(&entry.end_pc.to_be_bytes());
            buf.extend_from_slice(&entry.handler_pc.to_be_bytes());
            buf.extend_from_slice(&entry.catch_type.to_be_bytes());
        }
        buf.extend_from_slice(&(self.attributes.len() as u16).to_be_bytes());
        if !self.attributes.is_empty() {
            todo!("Code attribute with attributes not supported yet");
        }
    }
}
