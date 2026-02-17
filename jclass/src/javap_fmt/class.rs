use crate::ClassFile;
use crate::attribute::SharedAttribute;
use crate::constant_pool::ConstantEntry;
use common::signature::ClassSignature;
use common::utils::indent_write::Indented;
use common::{try_javap_print, try_javap_print_class_name};
use std::fmt;
use std::fmt::Write as _;

impl ClassFile {
    const COMMENT_WIDTH: usize = 24;
    const CONSTANT_KIND_WIDTH: usize = 18;

    fn fmt_generic_signature(&self, ind: &mut Indented) -> fmt::Result {
        // for java.lang.Object
        if self.super_class == 0 {
            return Ok(());
        }
        let super_class_name =
            try_javap_print!(ind, self.cp.get_javap_class_name(&self.super_class));
        let super_is_object = super_class_name == "java.lang.Object";
        if let Some(sig_index) = self.attributes.iter().find_map(|attr| {
            if let crate::attribute::ClassAttribute::Shared(shared) = attr {
                match shared {
                    SharedAttribute::Signature(sig_index) => Some(sig_index),
                    _ => None,
                }
            } else {
                None
            }
        }) {
            let raw_sig = try_javap_print!(ind, self.cp.get_utf8(sig_index));
            let sig = try_javap_print!(
                ind,
                ClassSignature::new(raw_sig, self.access_flags.is_interface())
            );
            write!(ind, "{}", sig)
        } else if !super_is_object || !self.interfaces.is_empty() {
            let interfaces_names = try_javap_print!(
                ind,
                self.interfaces
                    .iter()
                    .map(|i| self.cp.get_javap_class_name(i))
                    .collect::<Result<Vec<_>, _>>()
            );

            if !super_is_object && !self.access_flags.is_interface() {
                write!(ind, "extends {}", super_class_name)?;
                if !interfaces_names.is_empty() {
                    write!(ind, " ")?;
                }
            }
            if !interfaces_names.is_empty() {
                if self.access_flags.is_interface() {
                    write!(ind, "extends ")?;
                } else {
                    write!(ind, "implements ")?;
                }
                write!(ind, "{}", interfaces_names.join(", "))?;
            }
            Ok(())
        } else {
            Ok(())
        }
    }

    fn fmt_java_like_signature(&self, ind: &mut Indented) -> fmt::Result {
        self.access_flags.javap_fmt_java_like_prefix(ind)?;
        write!(
            ind,
            "{} ",
            try_javap_print_class_name!(ind, self.cp.get_class_name(&self.this_class))
        )?;
        self.fmt_generic_signature(ind)?;
        writeln!(ind)
    }
}

impl fmt::Display for ClassFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ind = Indented::new(f);

        self.fmt_java_like_signature(&mut ind)?;

        ind.with_indent(|ind| {
            writeln!(ind, "minor version: {}", self.minor_version)?;
            writeln!(ind, "major version: {}", self.major_version)?;

            write!(ind, "flags: (0x{:04X}) ", self.access_flags.get_raw(),)?;
            self.access_flags.fmt_class_javap_like_list(ind)?;
            writeln!(ind)?;

            writeln!(
                ind,
                "this_class: {:<w$} //{}",
                format!("#{}", self.this_class),
                try_javap_print!(ind, self.cp.get_class_name(&self.this_class)),
                w = Self::COMMENT_WIDTH
            )?;
            write!(
                ind,
                "super_class: {:<w$}",
                format!("#{}", self.super_class),
                w = Self::COMMENT_WIDTH
            )?;
            if self.super_class != 0 {
                write!(
                    ind,
                    "//{}",
                    try_javap_print!(ind, self.cp.get_class_name(&self.super_class))
                )?;
            }
            writeln!(ind)?;
            writeln!(
                ind,
                "interfaces: {}, fields: {}, methods: {}, attributes: {}",
                self.interfaces.len(),
                self.fields.len(),
                self.methods.len(),
                self.attributes.len()
            )?;
            Ok(())
        })?;
        writeln!(ind, "Constant pool:")?;
        ind.with_indent(|ind| {
            let counter_width = self
                .cp
                .inner
                .len()
                .checked_ilog10()
                .map_or(0, |d| d as usize)
                + 2;
            for (i, c) in self.cp.inner.iter().enumerate() {
                if matches!(c, ConstantEntry::Unused) {
                    continue;
                }
                let tag = format_args!("{:<kw$}", c.get_kind(), kw = Self::CONSTANT_KIND_WIDTH);
                write!(ind, "{:>w$} = {} ", format!("#{i}"), tag, w = counter_width)?;
                c.javap_fmt(ind, &self.cp)?;
            }
            Ok(())
        })?;
        writeln!(ind, "{{")?;
        ind.with_indent(|ind| {
            for (i, field) in self.fields.iter().enumerate() {
                field.javap_fmt(ind, &self.cp)?;
                if i + 1 < self.fields.len() {
                    writeln!(ind)?;
                }
            }
            if !self.fields.is_empty() {
                writeln!(ind)?;
            }

            for (i, method) in self.methods.iter().enumerate() {
                method.javap_fmt(ind, &self.cp, &self.this_class, &self.access_flags)?;
                if i + 1 < self.methods.len() {
                    writeln!(ind)?;
                }
            }
            Ok(())
        })?;
        writeln!(ind, "}}")?;
        for attribute in &self.attributes {
            attribute.javap_fmt(&mut ind, &self.cp)?;
        }
        Ok(())
    }
}
