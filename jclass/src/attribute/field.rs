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

    #[cfg(feature = "javap_print")]
    pub(crate) fn javap_fmt(
        &self,
        ind: &mut common::utils::indent_write::Indented<'_>,
        cp: &ConstantPool,
    ) -> std::fmt::Result {
        use common::try_javap_print;
        use std::fmt::Write as _;

        match self {
            FieldAttribute::Shared(shared) => shared.javap_fmt(ind, cp)?,
            FieldAttribute::ConstantValue(val) => {
                let constant = try_javap_print!(ind, cp.get_raw(val));
                writeln!(
                    ind,
                    "ConstantValue: {}",
                    try_javap_print!(ind, constant.get_javap_type_and_value(cp, &0))
                )?;
            }
        }

        Ok(())
    }
}
