use crate::attribute::FieldAttribute;
use crate::attribute::SharedAttribute;
use crate::constant_pool::ConstantPool;
use crate::member::FieldInfo;
use common::jtype::JavaType;
use common::try_javap_print;
use common::utils::indent_write::Indented;
use std::fmt;
use std::fmt::Write as _;

impl FieldInfo {
    fn javap_fmt_type(
        &self,
        ind: &mut Indented,
        cp: &ConstantPool,
        raw_descriptor: &str,
    ) -> fmt::Result {
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
                let raw_sig = try_javap_print!(ind, cp.get_utf8(sig_index));
                try_javap_print!(ind, JavaType::try_from(raw_sig))
            } else {
                try_javap_print!(ind, JavaType::try_from(raw_descriptor))
            }
        };
        write!(ind, "{field_type} ")
    }

    pub(crate) fn javap_fmt(&self, ind: &mut Indented, cp: &ConstantPool) -> fmt::Result {
        let raw_descriptor = try_javap_print!(ind, cp.get_utf8(&self.descriptor_index));

        self.access_flags.fmt_field_javap_java_like_prefix(ind)?;
        self.javap_fmt_type(ind, cp, raw_descriptor)?;
        writeln!(
            ind,
            "{};",
            try_javap_print!(ind, cp.get_utf8(&self.name_index))
        )?;
        ind.with_indent(|ind| {
            writeln!(ind, "descriptor: {raw_descriptor}")?;

            write!(ind, "flags: (0x{:04x}) ", self.access_flags.get_raw(),)?;
            self.access_flags.fmt_class_javap_like_list(ind)?;
            writeln!(ind)?;

            for attr in &self.attributes {
                attr.javap_fmt(ind, cp)?;
            }
            Ok(())
        })
    }
}
