//! Assembly serialization module: converts `ClassFile` structures into `.class` bytecode.
//!
//! Gated behind the `rns_assemble` feature flag.

mod attribute;
pub mod builder;
pub mod class_file;
mod constant_pool;
mod flags;
mod method;

pub type AttributeNameMap = std::collections::HashMap<crate::attribute::AttributeKind, u16>;

use crate::ClassFile;

impl ClassFile {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(&ClassFile::MAGIC.to_be_bytes());
        bytes.extend(&self.minor_version.to_be_bytes());
        bytes.extend(&self.major_version.to_be_bytes());
        bytes.extend(&((self.cp.inner.len()) as u16).to_be_bytes());
        for entry in &self.cp.inner[1..] {
            entry.write_to(&mut bytes);
        }
        bytes.extend(&self.access_flags.get_raw().to_be_bytes());
        bytes.extend(&self.this_class.to_be_bytes());
        bytes.extend(&self.super_class.to_be_bytes());
        bytes.extend(&(self.interfaces.len() as u16).to_be_bytes());
        if !self.interfaces.is_empty() {
            todo!("assembling interfaces is not implemented yet");
        }
        bytes.extend(&(self.fields.len() as u16).to_be_bytes());
        if !self.fields.is_empty() {
            todo!("assembling fields is not implemented yet");
        }
        bytes.extend(&(self.methods.len() as u16).to_be_bytes());
        for method in &self.methods {
            method.write_to(&mut bytes, &self.attribute_names);
        }
        bytes.extend(&(self.attributes.len() as u16).to_be_bytes());
        if !self.attributes.is_empty() {
            todo!("assembling class attributes is not implemented yet");
        }
        bytes
    }
}
