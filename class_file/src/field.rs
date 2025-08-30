use crate::ClassFileErr;
use crate::attribute::field::FieldAttribute;
use crate::constant::pool::ConstantPool;
use crate::print::get_method_javap_like_list;
use common::utils::cursor::ByteCursor;
use common::utils::indent_write::Indented;
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
    #[cfg(feature = "pretty_print")]
    pub(crate) fn fmt_pretty(&self, ind: &mut Indented, cp: &ConstantPool) -> std::fmt::Result {
        use crate::print::get_field_pretty_java_like_prefix;
        use common::jtype::Type;
        use common::pretty_try;
        use std::fmt::Write as _;

        let raw_descriptor = pretty_try!(ind, cp.get_utf8(&self.descriptor_index));
        let descriptor = pretty_try!(ind, Type::try_from(raw_descriptor));
        writeln!(
            ind,
            "{} {} {};",
            get_field_pretty_java_like_prefix(self.access_flags),
            descriptor,
            pretty_try!(ind, cp.get_utf8(&self.name_index))
        )?;
        ind.with_indent(|ind| {
            writeln!(ind, "descriptor: {raw_descriptor}")?;
            writeln!(
                ind,
                "flags: (0x{:04x}) {}",
                self.access_flags,
                get_method_javap_like_list(self.access_flags)
            )?;
            for attr in &self.attributes {
                attr.fmt_pretty(ind, cp)?;
            }
            Ok(())
        })
    }
}
