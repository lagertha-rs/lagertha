use crate::ClassFileErr;
use crate::attribute::field::FieldAttribute;
use crate::constant::pool::ConstantPool;
use common::utils::cursor::ByteCursor;
#[cfg(test)]
use serde::Serialize;

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.5
#[cfg_attr(test, derive(Serialize))]
#[derive(Debug)]
pub struct FieldInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes: Vec<FieldAttribute>,
}

impl<'a> FieldInfo {
    pub(crate) fn read(
        pool: &ConstantPool,
        cursor: &mut ByteCursor<'a>,
    ) -> Result<Self, ClassFileErr> {
        let access_flags = cursor.u16()?;
        let name_index = cursor.u16()?;
        let descriptor_index = cursor.u16()?;
        let attributes_count = cursor.u16()?;
        let mut attributes = Vec::with_capacity(attributes_count as usize);
        for _ in 0..attributes_count {
            attributes.push(FieldAttribute::read(pool, cursor)?);
        }

        Ok(FieldInfo {
            access_flags,
            name_index,
            descriptor_index,
            attributes,
        })
    }
}
