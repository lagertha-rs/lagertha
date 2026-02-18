use crate::attribute::SharedAttribute;
use crate::attribute::method::MethodAttribute;
use crate::constant_pool::ConstantPool;
use crate::flags::ClassFlags;
use crate::javap_fmt::fmt_class_name;
use crate::member::MethodInfo;
use common::descriptor::MethodDescriptor;
use common::error::ClassFormatErr;
use common::jtype::JavaType;
use common::signature::MethodSignature;
use common::utils::indent_write::Indented;
use either::Either;
use std::fmt::Write as _;

impl MethodInfo {
    fn get_descriptor(
        &self,
        cp: &ConstantPool,
        raw_descriptor: &str,
    ) -> Result<Either<MethodSignature, MethodDescriptor>, ClassFormatErr> {
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

    fn javap_fmt_flags(&self, ind: &mut Indented) -> Result<(), ClassFormatErr> {
        write!(ind, "flags: (0x{:04x}) ", self.access_flags.get_raw())?;
        self.access_flags.fmt_class_javap_like_list(ind)?;
        writeln!(ind)?;
        Ok(())
    }

    fn javap_fmt_throws(
        &self,
        ind: &mut Indented,
        cp: &ConstantPool,
        descriptor: &Either<MethodSignature, MethodDescriptor>,
    ) -> Result<(), ClassFormatErr> {
        if let Either::Left(sig) = descriptor
            && !sig.throws.is_empty()
        {
            write!(ind, "{}", sig.fmt_throws())?;
            return Ok(());
        }

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
                let ex_name = fmt_class_name(cp.get_class_name(ex_index)?);
                write!(ind, "{ex_name}")?;
            }
        }
        Ok(())
    }

    fn javap_fmt_name_and_ret_type(
        &self,
        ind: &mut Indented,
        cp: &ConstantPool,
        this: &u16,
        descriptor: &Either<MethodSignature, MethodDescriptor>,
    ) -> Result<(), ClassFormatErr> {
        let method_name = fmt_class_name(cp.get_utf8(&self.name_index)?);
        if method_name == "<init>" {
            write!(ind, "{}", fmt_class_name(cp.get_class_name(this)?))?;
        } else if method_name == "<clinit>" {
            write!(ind, "{{}}")?;
        } else {
            match descriptor {
                Either::Left(signature) => {
                    write!(ind, "{signature} {method_name}")?;
                }
                Either::Right(descriptor) => {
                    write!(ind, "{} {}", &descriptor.ret, method_name)?;
                }
            }
        }
        Ok(())
    }

    fn javap_fmt_params(
        &self,
        ind: &mut Indented,
        descriptor: &Either<MethodSignature, MethodDescriptor>,
    ) -> Result<(), ClassFormatErr> {
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
        ind.write_char(')')?;
        Ok(())
    }

    pub(crate) fn javap_fmt(
        &self,
        ind: &mut Indented,
        cp: &ConstantPool,
        this: &u16,
        class_flags: &ClassFlags,
    ) -> Result<(), ClassFormatErr> {
        let raw_descriptor = cp.get_utf8(&self.descriptor_index)?;
        let descriptor = self.get_descriptor(cp, raw_descriptor)?;
        let is_default = class_flags.is_interface()
            && !self.access_flags.is_abstract()
            && !self.access_flags.is_static()
            && !self.access_flags.is_private()
            && self
                .attributes
                .iter()
                .any(|attr| matches!(attr, MethodAttribute::Code { .. }));
        self.access_flags.javap_fmt_java_like_prefix(ind)?;
        if is_default {
            write!(ind, "default ")?;
        }
        self.javap_fmt_name_and_ret_type(ind, cp, this, &descriptor)?;
        if cp.get_utf8(&self.name_index)? != "<clinit>" {
            self.javap_fmt_params(ind, &descriptor)?;
            self.javap_fmt_throws(ind, cp, &descriptor)?;
        }
        writeln!(ind, ";")?;

        ind.with_indent(|ind| {
            writeln!(ind, "descriptor: {}", raw_descriptor)?;
            self.javap_fmt_flags(ind)?;
            for attr in &self.attributes {
                attr.javap_fmt(
                    ind,
                    cp,
                    &MethodDescriptor::try_from(raw_descriptor)?,
                    this,
                    self.access_flags.is_static(),
                )?;
            }
            Ok(())
        })?;

        Ok(())
    }
}
