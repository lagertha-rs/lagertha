//! Constant pool types and structures.
//!
//! The constant pool is a table of structures representing various string constants,
//! class and interface names, field names, and other constants that are referred to
//! within the ClassFile structure and its substructures.

use common::error::ClassFormatErr;

mod entry;
mod types;

// Re-export commonly used types at the module level
pub use entry::{ConstantEntry, ConstantKind};
pub use types::{Dynamic, MethodHandle, MethodHandleKind, NameAndType, Reference};

/// The constant pool of a class file.
///
/// https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4
#[derive(Debug, Clone, PartialEq)]
pub struct ConstantPool {
    pub inner: Vec<ConstantEntry>,
}

impl ConstantPool {
    pub fn get_utf8(&self, idx: &u16) -> Result<&str, ClassFormatErr> {
        self.inner
            .get(*idx as usize)
            .ok_or(ClassFormatErr::ConstantNotFound(*idx))
            .and_then(|entry| match entry {
                ConstantEntry::Utf8(value) => Ok(value.as_str()),
                e => Err(ClassFormatErr::TypeError(
                    *idx,
                    ConstantKind::Utf8.to_string(),
                    e.get_kind().to_string(),
                )),
            })
    }

    pub fn get_class_name(&self, idx: &u16) -> Result<&str, ClassFormatErr> {
        let name_index = self.get_class(idx)?;
        self.get_utf8(&name_index)
    }

    pub fn get_class(&self, idx: &u16) -> Result<u16, ClassFormatErr> {
        self.inner
            .get(*idx as usize)
            .ok_or(ClassFormatErr::ConstantNotFound(*idx))
            .and_then(|entry| match entry {
                ConstantEntry::Class(name_index) => Ok(*name_index),
                e => Err(ClassFormatErr::TypeError(
                    *idx,
                    ConstantKind::Class.to_string(),
                    e.get_kind().to_string(),
                )),
            })
    }
}
