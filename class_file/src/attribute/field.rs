use crate::attribute::{AttributeType, SharedAttribute};
use crate::constant::pool::ConstantPool;
use crate::error::ClassFormatErr;
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

    #[cfg(feature = "pretty_print")]
    pub(crate) fn fmt_pretty(
        &self,
        ind: &mut common::utils::indent_write::Indented<'_>,
        cp: &ConstantPool,
    ) -> std::fmt::Result {
        use common::pretty_try;
        use std::fmt::Write as _;

        match self {
            FieldAttribute::Shared(shared) => shared.fmt_pretty(ind, cp)?,
            FieldAttribute::ConstantValue(val) => {
                let constant = pretty_try!(ind, cp.get_raw(val));
                writeln!(
                    ind,
                    "ConstantValue: {}",
                    pretty_try!(ind, constant.get_pretty_value(cp, &0))
                )?;
            }
        }

        Ok(())
    }
}
