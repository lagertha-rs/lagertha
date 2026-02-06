use crate::attribute::FieldAttribute;
use crate::constant_pool::ConstantPool;
use crate::flags::FieldFlags;
use common::error::ClassFormatErr;
use common::utils::cursor::ByteCursor;

/// A field in a class file.
///
/// https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.5
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
    ) -> Result<Self, ClassFormatErr> {
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

#[cfg(feature = "javap_print")]
impl FieldInfo {
    fn javap_fmt_type(
        &self,
        ind: &mut common::utils::indent_write::Indented,
        cp: &ConstantPool,
        raw_descriptor: &str,
    ) -> std::fmt::Result {
        use crate::attribute::SharedAttribute;
        use common::jtype::JavaType;
        use common::try_javap_print;
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
                let raw_sig = try_javap_print!(ind, cp.get_utf8(sig_index));
                try_javap_print!(ind, JavaType::try_from(raw_sig))
            } else {
                try_javap_print!(ind, JavaType::try_from(raw_descriptor))
            }
        };
        write!(ind, "{field_type} ")
    }

    pub(crate) fn javap_fmt(
        &self,
        ind: &mut common::utils::indent_write::Indented,
        cp: &ConstantPool,
    ) -> std::fmt::Result {
        use common::try_javap_print;
        use std::fmt::Write as _;

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
