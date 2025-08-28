use crate::attribute::method::{CodeAttribute, MethodAttribute};
use crate::attribute::{AttributeType, SharedAttribute};
use crate::constant::pool::ConstantPool;
use crate::error::ClassFileErr;
use common::utils::cursor::ByteCursor;
#[cfg(test)]
use serde::Serialize;

#[cfg_attr(test, derive(Serialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldAttribute {
    Shared(SharedAttribute),
    ConstantValue(u16),
}

impl<'a> FieldAttribute {
    pub(crate) fn read(
        pool: &ConstantPool,
        cursor: &mut ByteCursor<'a>,
    ) -> Result<Self, ClassFileErr> {
        let attribute_name_index = cursor.u16()?;
        let _attribute_length = cursor.u32()? as usize;

        let attribute_type = AttributeType::try_from(pool.get_utf8(&attribute_name_index)?)?;
        match attribute_type {
            AttributeType::ConstantValue => Ok(FieldAttribute::ConstantValue(cursor.u16()?)),
            AttributeType::RuntimeVisibleAnnotations
            | AttributeType::Synthetic
            | AttributeType::Deprecated
            | AttributeType::Signature => Ok(FieldAttribute::Shared(SharedAttribute::read(
                attribute_type,
                cursor,
            )?)),
            _ => unimplemented!(),
        }
    }
}
