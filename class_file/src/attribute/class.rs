use crate::ClassFormatErr;
use crate::attribute::{AttributeType, SharedAttribute};
use crate::constant::pool::ConstantPool;
use common::utils::cursor::ByteCursor;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClassAttribute {
    Shared(SharedAttribute),
    SourceFile(u16),
    InnerClasses(Vec<InnerClassEntry>),
    EnclosingMethod(u16, u16),
    SourceDebugExtension,
    BootstrapMethods(Vec<BootstrapMethodEntry>),
    Module,
    ModulePackages,
    ModuleMainClass,
    NestHost(u16),
    NestMembers(Vec<u16>),
    Record,
    PermittedSubclasses(Vec<u16>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapMethodEntry {
    pub bootstrap_method_ref: u16,
    pub bootstrap_arguments: Vec<u16>,
}

impl BootstrapMethodEntry {
    pub fn new(bootstrap_method_ref: u16, bootstrap_arguments: Vec<u16>) -> Self {
        Self {
            bootstrap_method_ref,
            bootstrap_arguments,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InnerClassEntry {
    pub inner_class_info_index: u16,
    pub outer_class_info_index: u16,
    pub inner_name_index: u16,
    pub inner_class_access_flags: u16,
}

impl InnerClassEntry {
    pub fn new(
        inner_class_info_index: u16,
        outer_class_info_index: u16,
        inner_name_index: u16,
        inner_class_access_flags: u16,
    ) -> Self {
        Self {
            inner_class_info_index,
            outer_class_info_index,
            inner_name_index,
            inner_class_access_flags,
        }
    }
}

impl<'a> ClassAttribute {
    pub(crate) fn read(
        pool: &ConstantPool,
        cursor: &mut ByteCursor<'a>,
    ) -> Result<Self, ClassFormatErr> {
        let attribute_name_index = cursor.u16()?;
        let _attribute_length = cursor.u32()? as usize;

        let attribute_type = AttributeType::try_from(pool.get_utf8(&attribute_name_index)?)?;
        match attribute_type {
            AttributeType::SourceFile => Ok(ClassAttribute::SourceFile(cursor.u16()?)),
            AttributeType::BootstrapMethods => {
                let number_of_bootstrap_methods = cursor.u16()? as usize;
                let mut methods = Vec::with_capacity(number_of_bootstrap_methods);
                for _ in 0..number_of_bootstrap_methods {
                    let bootstrap_method_ref = cursor.u16()?;
                    let number_of_bootstrap_arguments = cursor.u16()? as usize;
                    let mut bootstrap_arguments = Vec::with_capacity(number_of_bootstrap_arguments);
                    for _ in 0..number_of_bootstrap_arguments {
                        bootstrap_arguments.push(cursor.u16()?);
                    }
                    methods.push(BootstrapMethodEntry::new(
                        bootstrap_method_ref,
                        bootstrap_arguments,
                    ));
                }
                Ok(ClassAttribute::BootstrapMethods(methods))
            }
            AttributeType::InnerClasses => {
                let number_of_classes = cursor.u16()? as usize;
                let mut classes = Vec::with_capacity(number_of_classes);
                for _ in 0..number_of_classes {
                    classes.push(InnerClassEntry::new(
                        cursor.u16()?,
                        cursor.u16()?,
                        cursor.u16()?,
                        cursor.u16()?,
                    ));
                }
                Ok(ClassAttribute::InnerClasses(classes))
            }
            AttributeType::NestMembers => {
                let number_of_classes = cursor.u16()? as usize;
                let mut classes = Vec::with_capacity(number_of_classes);
                for _ in 0..number_of_classes {
                    classes.push(cursor.u16()?);
                }
                Ok(ClassAttribute::NestMembers(classes))
            }
            AttributeType::NestHost => {
                let host_class_index = cursor.u16()?;
                Ok(ClassAttribute::NestHost(host_class_index))
            }
            AttributeType::EnclosingMethod => {
                let class_index = cursor.u16()?;
                let method_index = cursor.u16()?;
                Ok(ClassAttribute::EnclosingMethod(class_index, method_index))
            }
            AttributeType::PermittedSubclasses => {
                let number_of_classes = cursor.u16()? as usize;
                let mut classes = Vec::with_capacity(number_of_classes);
                for _ in 0..number_of_classes {
                    classes.push(cursor.u16()?);
                }
                Ok(ClassAttribute::PermittedSubclasses(classes))
            }
            AttributeType::RuntimeVisibleAnnotations
            | AttributeType::Synthetic
            | AttributeType::Deprecated
            | AttributeType::RuntimeInvisibleAnnotations
            | AttributeType::Signature => Ok(ClassAttribute::Shared(SharedAttribute::read(
                attribute_type,
                cursor,
            )?)),
            other => unimplemented!("Class attribute {:?} not implemented", other),
        }
    }

    #[cfg(feature = "pretty_print")]
    pub(crate) fn fmt_pretty(
        &self,
        ind: &mut common::utils::indent_write::Indented<'_>,
        cp: &ConstantPool,
    ) -> std::fmt::Result {
        use crate::flags::InnerClassFlags;
        use common::pretty_try;
        use std::fmt::Write as _;

        match self {
            ClassAttribute::Shared(shared) => shared.fmt_pretty(ind, cp)?,
            ClassAttribute::SourceFile(idx) => {
                writeln!(
                    ind,
                    "SourceFile: \"{}\"",
                    pretty_try!(ind, cp.get_utf8(idx))
                )?;
            }
            ClassAttribute::InnerClasses(inner) => {
                writeln!(ind, "InnerClasses:")?;
                ind.with_indent(|ind| {
                    for entry in inner {
                        let inner_class =
                            pretty_try!(ind, cp.get_raw(&entry.inner_class_info_index));
                        // Anonymous class
                        if entry.outer_class_info_index == 0 && entry.inner_name_index == 0 {
                            writeln!(
                                ind,
                                "#{:<42} // {}",
                                format!("{};", entry.inner_class_info_index),
                                pretty_try!(ind, inner_class.get_pretty_type_and_value(cp, &0)),
                            )?;
                        } else {
                            let inner_access_flags =
                                InnerClassFlags::new(entry.inner_class_access_flags);
                            let outer_class =
                                pretty_try!(ind, cp.get_raw(&entry.outer_class_info_index));
                            writeln!(
                                ind,
                                "{:<43} // {}={} of {}",
                                format!(
                                    "{} #{}= #{} of #{};",
                                    inner_access_flags.pretty_java_like_prefix(),
                                    entry.inner_name_index,
                                    entry.inner_class_info_index,
                                    entry.outer_class_info_index
                                ),
                                pretty_try!(ind, cp.get_utf8(&entry.inner_name_index)),
                                pretty_try!(ind, inner_class.get_pretty_type_and_value(cp, &0)),
                                pretty_try!(ind, outer_class.get_pretty_type_and_value(cp, &0))
                            )?;
                        }
                    }
                    Ok(())
                })?;
            }
            ClassAttribute::EnclosingMethod(class_idx, method_idx) => {
                let method = if *method_idx == 0 {
                    ""
                } else {
                    pretty_try!(ind, cp.get_method_or_field_name_by_nat_idx(method_idx))
                };
                writeln!(
                    ind,
                    "{:<24} // {}{}{}",
                    format!("EnclosingMethod: #{}.#{}", class_idx, method_idx),
                    pretty_try!(ind, cp.get_pretty_class_name(class_idx)),
                    if method.is_empty() { "" } else { "." },
                    method
                )?;
            }
            ClassAttribute::SourceDebugExtension => unimplemented!(),
            ClassAttribute::BootstrapMethods(bootstrap_methods) => {
                writeln!(ind, "BootstrapMethods:")?;
                ind.with_indent(|ind| {
                    for (i, method) in bootstrap_methods.iter().enumerate() {
                        let method_handle =
                            pretty_try!(ind, cp.get_raw(&method.bootstrap_method_ref));
                        writeln!(
                            ind,
                            "{}: #{} {}",
                            i,
                            method.bootstrap_method_ref,
                            pretty_try!(ind, method_handle.get_pretty_type_and_value(cp, &0))
                        )?;
                        ind.with_indent(|ind| {
                            writeln!(ind, "Method arguments:")?;
                            ind.with_indent(|ind| {
                                for arg in &method.bootstrap_arguments {
                                    let argument = pretty_try!(ind, cp.get_raw(arg));
                                    writeln!(
                                        ind,
                                        "#{} {}",
                                        arg,
                                        pretty_try!(ind, argument.get_pretty_value(cp, &0))
                                    )?;
                                }
                                Ok(())
                            })?;
                            Ok(())
                        })?;
                    }
                    Ok(())
                })?;
            }
            ClassAttribute::Module => unimplemented!(),
            ClassAttribute::ModulePackages => unimplemented!(),
            ClassAttribute::ModuleMainClass => unimplemented!(),
            ClassAttribute::NestHost(idx) => {
                let constant = pretty_try!(ind, cp.get_raw(idx));
                writeln!(
                    ind,
                    "NestHost: {}",
                    pretty_try!(ind, constant.get_pretty_type_and_value(cp, &0))
                )?;
            }
            ClassAttribute::NestMembers(members) => {
                writeln!(ind, "NestMembers:")?;
                ind.with_indent(|ind| {
                    for member in members {
                        writeln!(ind, "{}", pretty_try!(ind, cp.get_class_name(member)))?;
                    }
                    Ok(())
                })?;
            }
            ClassAttribute::Record => unimplemented!(),
            ClassAttribute::PermittedSubclasses(classes) => {
                writeln!(ind, "PermittedSubclasses:")?;
                ind.with_indent(|ind| {
                    for class in classes {
                        writeln!(ind, "{}", pretty_try!(ind, cp.get_class_name(class)))?;
                    }
                    Ok(())
                })?;
            }
        }

        Ok(())
    }
}
