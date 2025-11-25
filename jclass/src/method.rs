use crate::ClassFormatErr;
use crate::attribute::method::MethodAttribute;
use crate::constant::pool::ConstantPool;
use crate::flags::MethodFlags;
use common::utils::cursor::ByteCursor;

/// https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.6
#[derive(Debug)]
pub struct MethodInfo {
    pub access_flags: MethodFlags,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Vec<MethodAttribute>,
}

impl<'a> MethodInfo {
    pub(crate) fn read(
        constant_pool: &ConstantPool,
        cursor: &mut ByteCursor<'a>,
    ) -> Result<Self, ClassFormatErr> {
        let access_flags = MethodFlags::new(cursor.u16()?);
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
}

#[cfg(feature = "pretty_print")]
impl MethodInfo {
    fn get_descriptor(
        &self,
        cp: &ConstantPool,
        raw_descriptor: &str,
    ) -> Result<
        either::Either<common::signature::MethodSignature, common::descriptor::MethodDescriptor>,
        ClassFormatErr,
    > {
        use crate::attribute::SharedAttribute;
        use common::descriptor::MethodDescriptor;
        use common::signature::MethodSignature;
        use either::Either;

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
        Ok(if let Some(sig_index) = generic_signature_opt {
            let raw_sig = cp.get_utf8(sig_index)?;
            Either::Left(MethodSignature::try_from(raw_sig)?)
        } else {
            Either::Right(MethodDescriptor::try_from(raw_descriptor)?)
        })
    }

    fn fmt_pretty_javap_flags(
        &self,
        ind: &mut common::utils::indent_write::Indented<'_>,
    ) -> std::fmt::Result {
        use std::fmt::Write as _;

        write!(ind, "flags: (0x{:04x}) ", self.access_flags.get_raw(),)?;
        self.access_flags.fmt_class_javap_like_list(ind)?;
        writeln!(ind)
    }

    fn fmt_pretty_throws(
        &self,
        ind: &mut common::utils::indent_write::Indented<'_>,
        cp: &ConstantPool,
    ) -> std::fmt::Result {
        use common::pretty_class_name_try;
        use std::fmt::Write as _;

        let exceptions_attr_opt = self.attributes.iter().find_map(|attr| {
            if let MethodAttribute::Exceptions(exception_index_table) = attr {
                Some(exception_index_table)
            } else {
                None
            }
        });
        if let Some(exception_index_table) = exceptions_attr_opt {
            write!(ind, " throws ")?;
            for (i, ex_index) in exception_index_table.iter().enumerate() {
                if i > 0 {
                    write!(ind, ", ")?;
                }
                let ex_name = pretty_class_name_try!(ind, cp.get_class_name(ex_index));
                write!(ind, "{ex_name}")?;
            }
        }
        Ok(())
    }

    fn fmt_pretty_name_and_ret_type(
        &self,
        ind: &mut common::utils::indent_write::Indented<'_>,
        cp: &ConstantPool,
        this: &u16,
        descriptor: &either::Either<
            common::signature::MethodSignature,
            common::descriptor::MethodDescriptor,
        >,
    ) -> std::fmt::Result {
        use common::pretty_class_name_try;
        use either::Either;
        use std::fmt::Write as _;

        let method_name = pretty_class_name_try!(ind, cp.get_utf8(&self.name_index));
        if method_name == "<init>" {
            write!(
                ind,
                "{}",
                pretty_class_name_try!(ind, cp.get_class_name(this))
            )
        } else if method_name == "<clinit>" {
            write!(ind, "{{}}")
        } else {
            match descriptor {
                Either::Left(signature) => {
                    write!(ind, "{signature} {method_name}")
                }
                Either::Right(descriptor) => write!(ind, "{} {}", &descriptor.ret, method_name),
            }
        }
    }

    fn fmt_pretty_params(
        &self,
        ind: &mut common::utils::indent_write::Indented<'_>,
        descriptor: &either::Either<
            common::signature::MethodSignature,
            common::descriptor::MethodDescriptor,
        >,
    ) -> std::fmt::Result {
        use common::jtype::JavaType;
        use either::Either;
        use std::fmt::Write as _;

        let params: &[JavaType] = match descriptor {
            Either::Left(sig) => &sig.params,
            Either::Right(desc) => &desc.params,
        };

        ind.write_char('(')?;
        for (i, ty) in params.iter().enumerate() {
            if i > 0 {
                ind.write_str(", ")?;
            }

            let is_last = i + 1 == params.len();
            if is_last
                && self.access_flags.is_varargs()
                && let JavaType::Array(elem) = ty
            {
                write!(ind, "{}", &**elem)?;
                ind.write_str("...")?;
                continue;
            }

            write!(ind, "{ty}")?;
        }
        ind.write_char(')')
    }

    pub(crate) fn fmt_pretty(
        &self,
        ind: &mut common::utils::indent_write::Indented<'_>,
        cp: &ConstantPool,
        this: &u16,
        class_flags: &crate::flags::ClassFlags,
    ) -> std::fmt::Result {
        use common::descriptor::MethodDescriptor;
        use common::pretty_try;
        use std::fmt::Write as _;

        let raw_descriptor = pretty_try!(ind, cp.get_utf8(&self.descriptor_index));
        let descriptor = pretty_try!(ind, self.get_descriptor(cp, raw_descriptor));
        let is_default = class_flags.is_interface()
            && !self.access_flags.is_abstract()
            && !self.access_flags.is_static()
            && !self.access_flags.is_private()
            && self
                .attributes
                .iter()
                .any(|attr| matches!(attr, MethodAttribute::Code { .. }));
        self.access_flags.fmt_pretty_java_like_prefix(ind)?;
        if is_default {
            write!(ind, "default ")?;
        }
        self.fmt_pretty_name_and_ret_type(ind, cp, this, &descriptor)?;
        if pretty_try!(ind, cp.get_utf8(&self.name_index)) != "<clinit>" {
            self.fmt_pretty_params(ind, &descriptor)?;
            self.fmt_pretty_throws(ind, cp)?;
        }
        writeln!(ind, ";")?;

        ind.with_indent(|ind| {
            writeln!(ind, "descriptor: {}", raw_descriptor)?;
            self.fmt_pretty_javap_flags(ind)?;
            for attr in &self.attributes {
                attr.fmt_pretty(
                    ind,
                    cp,
                    //TODO: avoid double conversion, not sure that method signature is needed here
                    &pretty_try!(ind, MethodDescriptor::try_from(raw_descriptor)),
                    this,
                    self.access_flags.is_static(),
                )?;
            }
            Ok(())
        })?;

        Ok(())
    }
}
