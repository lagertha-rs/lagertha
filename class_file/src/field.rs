use crate::ClassFileErr;
use crate::attribute::field::FieldAttribute;
use crate::constant::pool::ConstantPool;
use crate::flags::FieldFlags;
use common::utils::cursor::ByteCursor;

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.5
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
    ) -> Result<Self, ClassFileErr> {
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

#[cfg(feature = "pretty_print")]
impl FieldInfo {
    fn fmt_pretty_type(
        &self,
        ind: &mut common::utils::indent_write::Indented,
        cp: &ConstantPool,
        raw_descriptor: &str,
    ) -> std::fmt::Result {
        use crate::attribute::SharedAttribute;
        use common::jtype::Type;
        use common::pretty_try;
        use std::fmt::Write as _;

        let field_type = {
            let generic_signature_opt = self.attributes.iter().find_map(|attr| {
                if let FieldAttribute::Shared(shared) = attr {
                    match shared {
                        SharedAttribute::Signature(sig_index) => Some(sig_index),
                        _ => None,
                    }
                } else {
                    None
                }
            });
            if let Some(sig_index) = generic_signature_opt {
                let raw_sig = pretty_try!(ind, cp.get_utf8(sig_index));
                pretty_try!(ind, Type::try_from(raw_sig))
            } else {
                pretty_try!(ind, Type::try_from(raw_descriptor))
            }
        };
        write!(ind, "{field_type} ")
    }

    pub(crate) fn fmt_pretty(
        &self,
        ind: &mut common::utils::indent_write::Indented,
        cp: &ConstantPool,
    ) -> std::fmt::Result {
        use common::pretty_try;
        use std::fmt::Write as _;

        let raw_descriptor = pretty_try!(ind, cp.get_utf8(&self.descriptor_index));

        self.access_flags.fmt_field_pretty_java_like_prefix(ind)?;
        self.fmt_pretty_type(ind, cp, raw_descriptor)?;
        writeln!(ind, "{};", pretty_try!(ind, cp.get_utf8(&self.name_index)))?;
        ind.with_indent(|ind| {
            writeln!(ind, "descriptor: {raw_descriptor}")?;

            write!(ind, "flags: (0x{:04x}) ", self.access_flags.get_raw(),)?;
            self.access_flags.fmt_class_javap_like_list(ind)?;
            writeln!(ind)?;

            for attr in &self.attributes {
                attr.fmt_pretty(ind, cp)?;
            }
            Ok(())
        })
    }
}
