use crate::ClassFileErr;
use crate::attribute::method::MethodAttribute;
use crate::constant::pool::ConstantPool;
use common::pretty_try;
use common::signature::MethodSignature;
use common::utils::cursor::ByteCursor;
use either::Either;
#[cfg(test)]
use serde::Serialize;

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.6
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
        this: &u16,
    ) -> std::fmt::Result {
        use crate::attribute::SharedAttribute;
        use crate::print::{get_method_javap_like_list, get_method_pretty_java_like_prefix};
        use common::descriptor::MethodDescriptor;
        use common::{pretty_class_name_try, pretty_try};
        use itertools::Itertools;
        use std::fmt::Write as _;

        let raw_descriptor = pretty_try!(ind, cp.get_utf8(&self.descriptor_index));
        let descriptor = {
            let generic_signature_opt = self.attributes.iter().find_map(|attr| {
                if let MethodAttribute::Shared(shared) = attr {
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
                Either::Left(pretty_try!(ind, MethodSignature::try_from(raw_sig)))
            } else {
                Either::Right(pretty_try!(ind, MethodDescriptor::try_from(raw_descriptor)))
            }
        };

        // TODO: can be replaced by method signature?
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
                    " throws {}",
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

        // constructor special handling
        let ret_and_method_name = {
            let method_name = pretty_class_name_try!(ind, cp.get_utf8(&self.name_index));
            if method_name != "<init>" {
                match &descriptor {
                    Either::Left(signature) => {
                        let generic_ret = if signature.type_params.len() == 1 {
                            format!("<{}> ", signature.type_params[0])
                        } else {
                            String::new()
                        };
                        format!("{}{} {}", generic_ret, &signature.ret, method_name)
                    }
                    Either::Right(descriptor) => format!("{} {}", &descriptor.ret, method_name),
                }
            } else {
                pretty_class_name_try!(ind, cp.get_class_name(this))
            }
        };

        let mut params = match &descriptor {
            Either::Left(sig) => &sig.params,
            Either::Right(desc) => &desc.params,
        }
        .iter()
        .map(|v| v.to_string())
        .join(", ");

        let is_varargs = (self.access_flags & 0x0080) != 0;
        if is_varargs {
            params = format!("{}...", params.trim_end_matches("[]"));
        }

        writeln!(
            ind,
            "{} {}({params}){throws};",
            get_method_pretty_java_like_prefix(self.access_flags),
            ret_and_method_name,
        )?;
        ind.with_indent(|ind| {
            let is_static = (self.access_flags & 0x0008) != 0;
            writeln!(ind, "descriptor: {}", raw_descriptor)?;
            writeln!(
                ind,
                "flags: (0x{:04x}) {}",
                self.access_flags,
                get_method_javap_like_list(self.access_flags)
            )?;
            for attr in &self.attributes {
                attr.fmt_pretty(
                    ind,
                    cp,
                    //TODO: avoid double conversion, not sure that method signature is needed here
                    &pretty_try!(ind, MethodDescriptor::try_from(raw_descriptor)),
                    this,
                    is_static,
                )?;
            }
            Ok(())
        })?;

        Ok(())
    }
}
