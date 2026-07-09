use crate::ClassFormatErr;
use crate::attribute::{Annotation, AttributeKind, SharedAttribute};
use crate::constant_pool::ConstantPool;
use lvm_common::utils::cursor::ByteCursor;

pub mod code;

pub use code::{
    CodeAttributeInfo, LineNumberEntry, LocalVariableEntry, LocalVariableTypeEntry, StackMapFrame,
    VerificationTypeInfo, VerificationTypeTag,
};

/// https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.3
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExceptionTableEntry {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

/// https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.3
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeAttribute {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<u8>,
    pub exception_table: Vec<ExceptionTableEntry>,
    pub attributes: Vec<CodeAttributeInfo>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodParameterEntry {
    pub name_index: u16,
    pub access_flags: u16,
}

/// Parameter annotations for a single parameter
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterAnnotations {
    pub annotations: Vec<Annotation>,
}

/// https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MethodAttribute {
    Shared(SharedAttribute),
    Code {
        attr_name_idx: u16,
        code_attr: CodeAttribute,
    },
    Exceptions {
        attr_name_idx: u16,
        exception_idx_table: Vec<u16>,
    },
    RuntimeVisibleParameterAnnotations {
        attr_name_idx: u16,
        annotations: Vec<ParameterAnnotations>,
    },
    RuntimeInvisibleParameterAnnotations {
        attr_name_idx: u16,
        annotations: Vec<ParameterAnnotations>,
    },
    AnnotationsDefault {
        attr_name_idx: u16,
    },
    MethodParameters {
        attr_name_idx: u16,
        params: Vec<MethodParameterEntry>,
    },
}

impl MethodParameterEntry {
    pub(crate) fn new(name_index: u16, access_flags: u16) -> Self {
        Self {
            name_index,
            access_flags,
        }
    }
}

impl<'a> MethodAttribute {
    pub(crate) fn read(
        pool: &ConstantPool,
        cursor: &mut ByteCursor<'a>,
    ) -> Result<Self, ClassFormatErr> {
        let attr_name_idx = cursor.u16()?;
        let _attribute_length = cursor.u32()? as usize;

        let attribute_kind = AttributeKind::try_from(pool.get_utf8(&attr_name_idx)?)?;
        match attribute_kind {
            AttributeKind::Code => Ok(MethodAttribute::Code {
                attr_name_idx,
                code_attr: CodeAttribute::read(pool, cursor)?,
            }),
            AttributeKind::RuntimeVisibleAnnotations
            | AttributeKind::Synthetic
            | AttributeKind::Deprecated
            | AttributeKind::RuntimeInvisibleAnnotations
            | AttributeKind::Signature => Ok(MethodAttribute::Shared(SharedAttribute::read(
                attr_name_idx,
                attribute_kind,
                cursor,
            )?)),
            AttributeKind::MethodParameters => {
                let parameters_count = cursor.u8()? as usize;
                let mut params = Vec::with_capacity(parameters_count);
                for _ in 0..parameters_count {
                    params.push(MethodParameterEntry::new(cursor.u16()?, cursor.u16()?));
                }
                Ok(MethodAttribute::MethodParameters {
                    attr_name_idx,
                    params,
                })
            }
            AttributeKind::Exceptions => {
                let number_of_exceptions = cursor.u16()?;
                let mut exception_idx_table = Vec::with_capacity(number_of_exceptions as usize);
                for _ in 0..number_of_exceptions {
                    exception_idx_table.push(cursor.u16()?);
                }
                Ok(MethodAttribute::Exceptions {
                    attr_name_idx,
                    exception_idx_table,
                })
            }
            AttributeKind::RuntimeVisibleParameterAnnotations => {
                let number_of_parameters = cursor.u8()?;
                let mut parameter_annotations = Vec::with_capacity(number_of_parameters as usize);
                for _ in 0..number_of_parameters {
                    let num_annotations = cursor.u16()?;
                    let mut annotations = Vec::with_capacity(num_annotations as usize);
                    for _ in 0..num_annotations {
                        annotations.push(Annotation::read(cursor)?);
                    }
                    parameter_annotations.push(ParameterAnnotations { annotations });
                }
                Ok(MethodAttribute::RuntimeVisibleParameterAnnotations {
                    attr_name_idx,
                    annotations: parameter_annotations,
                })
            }
            AttributeKind::RuntimeInvisibleParameterAnnotations => {
                let number_of_parameters = cursor.u8()?;
                let mut parameter_annotations = Vec::with_capacity(number_of_parameters as usize);
                for _ in 0..number_of_parameters {
                    let num_annotations = cursor.u16()?;
                    let mut annotations = Vec::with_capacity(num_annotations as usize);
                    for _ in 0..num_annotations {
                        annotations.push(Annotation::read(cursor)?);
                    }
                    parameter_annotations.push(ParameterAnnotations { annotations });
                }
                Ok(MethodAttribute::RuntimeInvisibleParameterAnnotations {
                    attr_name_idx,
                    annotations: parameter_annotations,
                })
            }
            other => unimplemented!("Method attribute {:?} not implemented", other),
        }
    }
}

impl<'a> CodeAttribute {
    pub(crate) fn read(
        pool: &ConstantPool,
        cursor: &mut ByteCursor<'a>,
    ) -> Result<Self, ClassFormatErr> {
        let max_stack = cursor.u16()?;
        let max_locals = cursor.u16()?;
        let code_length = cursor.u32()? as usize;

        let mut code = vec![0u8; code_length];
        cursor.read_exact(&mut code)?;

        let exception_table_length = cursor.u16()? as usize;
        let mut exception_table = Vec::with_capacity(exception_table_length);
        for _ in 0..exception_table_length {
            exception_table.push(ExceptionTableEntry {
                start_pc: cursor.u16()?,
                end_pc: cursor.u16()?,
                handler_pc: cursor.u16()?,
                catch_type: cursor.u16()?,
            });
        }

        let attributes_count = cursor.u16()? as usize;
        let mut attributes = Vec::with_capacity(attributes_count);
        for _ in 0..attributes_count {
            attributes.push(CodeAttributeInfo::read(pool, cursor)?);
        }

        Ok(Self {
            max_stack,
            max_locals,
            code,
            exception_table,
            attributes,
        })
    }
}
