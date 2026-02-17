use crate::attribute::method::MethodAttribute;
use crate::constant_pool::ConstantPool;
use crate::flags::MethodFlags;
use common::error::ClassFormatErr;
use common::utils::cursor::ByteCursor;

/// A method in a class file.
///
/// https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.6
#[derive(Debug, Clone)]
pub struct MethodInfo {
    pub access_flags: MethodFlags,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Vec<MethodAttribute>,
}

impl<'a> MethodInfo {
    pub(crate) fn read(
        constant_pool: &ConstantPool,
        cursor: &mut ByteCursor<'a>,
    ) -> Result<Self, ClassFormatErr> {
        let access_flags = MethodFlags::new(cursor.u16()?);
        let name_index = cursor.u16()?;
        let descriptor_index = cursor.u16()?;
        let attribute_count = cursor.u16()?;
        let mut attributes = Vec::with_capacity(attribute_count as usize);
        for _ in 0..attribute_count {
            attributes.push(MethodAttribute::read(constant_pool, cursor)?);
        }
        Ok(Self {
            access_flags,
            name_index,
            descriptor_index,
            attributes,
        })
    }
}
