use crate::ClassFormatErr;
use crate::attribute::{AttributeKind, SharedAttribute};
use crate::constant_pool::ConstantPool;
use lvm_common::utils::cursor::ByteCursor;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClassAttribute {
    Shared(SharedAttribute),
    SourceFile {
        attr_name_idx: u16,
        sourcefile_idx: u16,
    },
    InnerClasses {
        attr_name_idx: u16,
        classes: Vec<InnerClassEntry>,
    },
    EnclosingMethod {
        attr_name_idx: u16,
        class_idx: u16,
        method_idx: u16,
    },
    SourceDebugExtension {
        attr_name_idx: u16,
    },
    BootstrapMethods {
        attr_name_idx: u16,
        bootstrap_methods: Vec<BootstrapMethodEntry>,
    },
    Module {
        attr_name_idx: u16,
    },
    ModulePackages {
        attr_name_idx: u16,
    },
    ModuleMainClass {
        attr_name_idx: u16,
    },
    NestHost {
        attr_name_idx: u16,
        host_class_idx: u16,
    },
    NestMembers {
        attr_name_idx: u16,
        classes: Vec<u16>,
    },
    Record {
        attr_name_idx: u16,
    },
    PermittedSubclasses {
        attr_name_idx: u16,
        classes: Vec<u16>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapMethodEntry {
    pub bootstrap_method_idx: u16,
    pub bootstrap_arguments: Vec<u16>,
}

impl BootstrapMethodEntry {
    pub(crate) fn new(bootstrap_method_ref: u16, bootstrap_arguments: Vec<u16>) -> Self {
        Self {
            bootstrap_method_idx: bootstrap_method_ref,
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
    pub(crate) fn new(
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
        let attr_name_idx = cursor.u16()?;
        let _attribute_length = cursor.u32()? as usize;

        let attribute_kind = AttributeKind::try_from(pool.get_utf8(&attr_name_idx)?)?;
        match attribute_kind {
            AttributeKind::SourceFile => Ok(ClassAttribute::SourceFile {
                attr_name_idx,
                sourcefile_idx: cursor.u16()?,
            }),
            AttributeKind::BootstrapMethods => {
                let number_of_bootstrap_methods = cursor.u16()? as usize;
                let mut bootstrap_methods = Vec::with_capacity(number_of_bootstrap_methods);
                for _ in 0..number_of_bootstrap_methods {
                    let bootstrap_method_ref = cursor.u16()?;
                    let number_of_bootstrap_arguments = cursor.u16()? as usize;
                    let mut bootstrap_arguments = Vec::with_capacity(number_of_bootstrap_arguments);
                    for _ in 0..number_of_bootstrap_arguments {
                        bootstrap_arguments.push(cursor.u16()?);
                    }
                    bootstrap_methods.push(BootstrapMethodEntry::new(
                        bootstrap_method_ref,
                        bootstrap_arguments,
                    ));
                }
                Ok(ClassAttribute::BootstrapMethods {
                    attr_name_idx,
                    bootstrap_methods,
                })
            }
            AttributeKind::InnerClasses => {
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
                Ok(ClassAttribute::InnerClasses {
                    attr_name_idx,
                    classes,
                })
            }
            AttributeKind::NestMembers => {
                let number_of_classes = cursor.u16()? as usize;
                let mut classes = Vec::with_capacity(number_of_classes);
                for _ in 0..number_of_classes {
                    classes.push(cursor.u16()?);
                }
                Ok(ClassAttribute::NestMembers {
                    attr_name_idx,
                    classes,
                })
            }
            AttributeKind::NestHost => {
                let host_class_idx = cursor.u16()?;
                Ok(ClassAttribute::NestHost {
                    attr_name_idx,
                    host_class_idx,
                })
            }
            AttributeKind::EnclosingMethod => {
                let class_idx = cursor.u16()?;
                let method_idx = cursor.u16()?;
                Ok(ClassAttribute::EnclosingMethod {
                    attr_name_idx,
                    class_idx,
                    method_idx,
                })
            }
            AttributeKind::PermittedSubclasses => {
                let number_of_classes = cursor.u16()? as usize;
                let mut classes = Vec::with_capacity(number_of_classes);
                for _ in 0..number_of_classes {
                    classes.push(cursor.u16()?);
                }
                Ok(ClassAttribute::PermittedSubclasses {
                    attr_name_idx,
                    classes,
                })
            }
            AttributeKind::RuntimeVisibleAnnotations
            | AttributeKind::RuntimeVisibleTypeAnnotations
            | AttributeKind::RuntimeInvisibleTypeAnnotations
            | AttributeKind::Synthetic
            | AttributeKind::Deprecated
            | AttributeKind::RuntimeInvisibleAnnotations
            | AttributeKind::Signature => Ok(ClassAttribute::Shared(SharedAttribute::read(
                attr_name_idx,
                attribute_kind,
                cursor,
            )?)),
            other => unimplemented!("Class attribute {:?} not implemented", other),
        }
    }
}
