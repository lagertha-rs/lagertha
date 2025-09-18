use crate::ClassFormatErr;
use crate::attribute::{AttributeType, SharedAttribute};
use crate::constant::pool::ConstantPool;
use common::utils::cursor::ByteCursor;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClassAttribute {
    Shared(SharedAttribute),
    SourceFile(u16),
    InnerClasses,
    EnclosingMethod,
    SourceDebugExtension,
    BootstrapMethods(Vec<BootstrapMethodEntry>),
    Module,
    ModulePackages,
    ModuleMainClass,
    NestHost(u16),
    NestMembers(Vec<u16>),
    Record,
    PermittedSubclasses,
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
                Ok(ClassAttribute::InnerClasses)
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
            AttributeType::Signature => Ok(ClassAttribute::Shared(SharedAttribute::read(
                attribute_type,
                cursor,
            )?)),
            other => unimplemented!("Class attribute {:?} not implemented", other),
        }
    }
}
