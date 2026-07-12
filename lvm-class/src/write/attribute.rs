use crate::ClassAttribute;
use crate::attribute::method::{CodeAttribute, MethodAttribute};

impl ClassAttribute {
    pub fn write_to(&self, buf: &mut Vec<u8>) {
        match self {
            ClassAttribute::InnerClasses {
                attr_name_idx,
                classes,
            } => {
                buf.extend_from_slice(&attr_name_idx.to_be_bytes());
                // attribute_length: 2 (number_of_classes) + 8 * classes.len()
                let attr_len = 2 + (classes.len() * 8);
                buf.extend_from_slice(&(attr_len as u32).to_be_bytes());
                buf.extend_from_slice(&(classes.len() as u16).to_be_bytes());
                for entry in classes {
                    buf.extend_from_slice(&entry.inner_class_info_index.to_be_bytes());
                    buf.extend_from_slice(&entry.outer_class_info_index.to_be_bytes());
                    buf.extend_from_slice(&entry.inner_name_index.to_be_bytes());
                    buf.extend_from_slice(&entry.inner_class_access_flags.to_be_bytes());
                }
            }
            ClassAttribute::SourceFile {
                attr_name_idx,
                sourcefile_idx,
            } => {
                buf.extend_from_slice(&attr_name_idx.to_be_bytes());
                buf.extend_from_slice(&2u32.to_be_bytes()); // attribute_length
                buf.extend_from_slice(&sourcefile_idx.to_be_bytes());
            }
            ClassAttribute::NestMembers {
                attr_name_idx,
                classes,
            } => {
                buf.extend_from_slice(&attr_name_idx.to_be_bytes());
                // attribute_length: 2 (number_of_classes) + 2 * classes.len()
                let attr_len = 2 + (classes.len() * 2);
                buf.extend_from_slice(&(attr_len as u32).to_be_bytes());
                buf.extend_from_slice(&(classes.len() as u16).to_be_bytes());
                for class_idx in classes {
                    buf.extend_from_slice(&(*class_idx).to_be_bytes());
                }
            }
            ClassAttribute::NestHost {
                attr_name_idx,
                host_class_idx,
            } => {
                buf.extend_from_slice(&attr_name_idx.to_be_bytes());
                buf.extend_from_slice(&2u32.to_be_bytes()); // attribute_length
                buf.extend_from_slice(&host_class_idx.to_be_bytes());
            }
            _ => unimplemented!("{:?} attribute writing not implemented yet", self.kind()),
        }
    }
}

impl MethodAttribute {
    pub fn write_to(&self, buf: &mut Vec<u8>) {
        match self {
            MethodAttribute::Code {
                attr_name_idx,
                code_attr,
            } => {
                buf.extend_from_slice(&attr_name_idx.to_be_bytes());
                // TODO: avoid having a buffer, I can know the size before without it.
                let mut body = Vec::new();
                code_attr.write_to(&mut body);
                buf.extend_from_slice(&(body.len() as u32).to_be_bytes());
                buf.extend_from_slice(&body);
            }
            e => unimplemented!("{e:?} attribute writing not implemented yet"),
        }
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
