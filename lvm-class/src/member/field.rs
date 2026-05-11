use crate::attribute::FieldAttribute;
use crate::constant_pool::ConstantPool;
use crate::flags::FieldFlags;
use common::error::ClassFormatErr;
use common::utils::cursor::ByteCursor;

/// A field in a class file.
///
/// https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.5
#[derive(Debug)]
pub struct FieldInfo {
    pub access_flags: FieldFlags,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Vec<FieldAttribute>,
}

impl<'a> FieldInfo {
    pub(crate) fn read(
        pool: &ConstantPool,
        cursor: &mut ByteCursor<'a>,
    ) -> Result<Self, ClassFormatErr> {
        let access_flags = FieldFlags::new(cursor.u16()?);
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
