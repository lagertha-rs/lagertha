use crate::attribute::{AttributeKind, SharedAttribute};
use crate::constant_pool::ConstantPool;
use common::error::ClassFormatErr;
use common::utils::cursor::ByteCursor;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldAttribute {
    Shared(SharedAttribute),
    ConstantValue(u16),
}

impl<'a> FieldAttribute {
    pub(crate) fn read(
        pool: &ConstantPool,
        cursor: &mut ByteCursor<'a>,
    ) -> Result<Self, ClassFormatErr> {
        let attribute_name_index = cursor.u16()?;
        let _attribute_length = cursor.u32()? as usize;

        let attribute_kind = AttributeKind::try_from(pool.get_utf8(&attribute_name_index)?)?;
        match attribute_kind {
            AttributeKind::ConstantValue => Ok(FieldAttribute::ConstantValue(cursor.u16()?)),
            AttributeKind::RuntimeVisibleAnnotations
            | AttributeKind::Synthetic
            | AttributeKind::Deprecated
            | AttributeKind::Signature => Ok(FieldAttribute::Shared(SharedAttribute::read(
                attribute_kind,
                cursor,
            )?)),
            _ => unimplemented!(),
        }
    }
}
