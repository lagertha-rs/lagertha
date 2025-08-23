use crate::constant_pool::ConstantPool;
use crate::ClassFileErr;
use common::cursor::ByteCursor;
use core::fmt;

#[derive(Debug)]
pub struct FieldInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    //attributes: Vec<AttributeInfo>,
}

impl<'a> FieldInfo {
    pub(crate) fn read(
        _constant_pool: &ConstantPool,
        cursor: &mut ByteCursor<'a>,
    ) -> Result<Self, ClassFileErr> {
        let access_flags = cursor.u16()?;
        let name_index = cursor.u16()?;
        let descriptor_index = cursor.u16()?;
        let attributes_count = cursor.u16()?;
        //let mut attributes = Vec::with_capacity(attributes_count as usize);
        for _ in 0..attributes_count {
            //attributes.push(AttributeInfo::read(constant_pool, cursor)?);
        }

        Ok(FieldInfo {
            access_flags,
            name_index,
            descriptor_index,
            //attributes,
        })
    }
}

impl fmt::Display for FieldInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FieldInfo {} name_index={} descriptor_index={} attributes=[",
            self.access_flags, self.name_index, self.descriptor_index
        )?;

        /*
        for (i, attr) in self.attributes.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", attr)?;
        }
         */

        write!(f, "]")
    }
}
