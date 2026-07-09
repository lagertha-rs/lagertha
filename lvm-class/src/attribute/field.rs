use crate::attribute::{AttributeKind, SharedAttribute};
use crate::constant_pool::ConstantPool;
use crate::error::ClassFormatErr;
use lvm_common::utils::cursor::ByteCursor;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldAttribute {
    Shared(SharedAttribute),
    ConstantValue { attr_name_idx: u16, value_idx: u16 },
}

impl FieldAttribute {
    pub fn kind(&self) -> AttributeKind {
        match self {
            Self::Shared(attr) => attr.kind(),
            Self::ConstantValue { .. } => AttributeKind::ConstantValue,
        }
    }
}

impl<'a> FieldAttribute {
    pub(crate) fn read(
        pool: &ConstantPool,
        cursor: &mut ByteCursor<'a>,
    ) -> Result<Self, ClassFormatErr> {
        let attr_name_idx = cursor.u16()?;
        let _attribute_length = cursor.u32()? as usize;

        let attribute_kind = AttributeKind::try_from(pool.get_utf8(&attr_name_idx)?)?;
        match attribute_kind {
            AttributeKind::ConstantValue => Ok(FieldAttribute::ConstantValue {
                attr_name_idx,
                value_idx: cursor.u16()?,
            }),
            AttributeKind::RuntimeVisibleAnnotations
            | AttributeKind::Synthetic
            | AttributeKind::Deprecated
            | AttributeKind::Signature => Ok(FieldAttribute::Shared(SharedAttribute::read(
                attr_name_idx,
                attribute_kind,
                cursor,
            )?)),
            _ => unimplemented!(),
        }
    }
}
