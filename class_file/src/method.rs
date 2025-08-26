use crate::ClassFileErr;
use crate::attribute::method::MethodAttribute;
use crate::constant::pool::ConstantPool;
use common::pretty_try;
use common::utils::cursor::ByteCursor;
#[cfg(test)]
use serde::Serialize;

#[cfg_attr(test, derive(Serialize))]
#[derive(Debug)]
pub struct MethodInfo {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Vec<MethodAttribute>,
}

impl<'a> MethodInfo {
    pub(crate) fn read(
        constant_pool: &ConstantPool,
        cursor: &mut ByteCursor<'a>,
    ) -> Result<Self, ClassFileErr> {
        let access_flags = cursor.u16()?;
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

    #[cfg(feature = "pretty_print")]
    pub(crate) fn fmt_pretty(
        &self,
        ind: &mut common::utils::indent_write::Indented<'_>,
        cp: &ConstantPool,
    ) -> std::fmt::Result {
        use crate::print::{get_method_javap_like_list, get_method_pretty_java_like_prefix};
        use common::descriptor::MethodDescriptor;
        use common::{pretty_class_name_try, pretty_try};
        use itertools::Itertools;
        use std::fmt::Write as _;

        let raw_descriptor = pretty_try!(ind, cp.get_utf8(&self.descriptor_index));
        let descriptor = pretty_try!(ind, MethodDescriptor::try_from(raw_descriptor));
        let throws = {
            let exc_opt = self.attributes.iter().find_map(|attr| {
                if let MethodAttribute::Exceptions(exc) = attr {
                    (!exc.is_empty()).then_some(exc)
                } else {
                    None
                }
            });

            if let Some(exc) = exc_opt {
                format!(
                    "throws {}",
                    pretty_try!(
                        ind,
                        exc.iter()
                            .map(|index| cp.get_pretty_class_name(index))
                            .collect::<Result<Vec<_>, _>>()
                    )
                    .join(", ")
                )
            } else {
                String::new()
            }
        };

        writeln!(
            ind,
            "{} {} {}({}) {throws}",
            get_method_pretty_java_like_prefix(self.access_flags),
            descriptor.ret,
            pretty_class_name_try!(ind, cp.get_utf8(&self.name_index)),
            descriptor.params.iter().map(|v| v.to_string()).join(", ")
        )?;
        ind.with_indent(|ind| {
            writeln!(ind, "descriptor: {}", raw_descriptor)?;
            writeln!(
                ind,
                "flags: (0x{:04X}) {}",
                self.access_flags,
                get_method_javap_like_list(self.access_flags)
            )?;
            for attr in &self.attributes {
                attr.fmt_pretty(ind, cp, &descriptor)?;
            }
            Ok(())
        })?;

        Ok(())
    }
}
