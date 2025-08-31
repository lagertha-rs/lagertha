use crate::ClassFileErr;
use crate::attribute::method::code::CodeAttributeInfo;
use crate::attribute::{AttributeType, SharedAttribute};
use crate::constant::pool::ConstantPool;
use common::descriptor::MethodDescriptor;
use common::instruction::Instruction;
use common::utils::cursor::ByteCursor;
#[cfg(test)]
use serde::Serialize;

pub mod code;

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.7.3
#[cfg_attr(test, derive(Serialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExceptionTableEntry {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.7.3
#[cfg_attr(test, derive(Serialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeAttribute {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<u8>,
    pub exception_table: Vec<ExceptionTableEntry>,
    pub attributes: Vec<CodeAttributeInfo>,
}

#[cfg_attr(test, derive(Serialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodParameterEntry {
    pub name_index: u16,
    pub access_flags: u16,
}

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.7
#[cfg_attr(test, derive(Serialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MethodAttribute {
    Shared(SharedAttribute),
    Code(CodeAttribute),
    Exceptions(Vec<u16>),
    RuntimeVisibleParameterAnnotations,
    RuntimeInvisibleParameterAnnotations,
    AnnotationsDefault,
    MethodParameters(Vec<MethodParameterEntry>),
}

impl MethodParameterEntry {
    pub fn new(name_index: u16, access_flags: u16) -> Self {
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
    ) -> Result<Self, ClassFileErr> {
        let attribute_name_index = cursor.u16()?;
        let _attribute_length = cursor.u32()? as usize;

        let attribute_type = AttributeType::try_from(pool.get_utf8(&attribute_name_index)?)?;
        match attribute_type {
            AttributeType::Code => Ok(MethodAttribute::Code(CodeAttribute::read(pool, cursor)?)),
            AttributeType::RuntimeVisibleAnnotations
            | AttributeType::Synthetic
            | AttributeType::Deprecated
            | AttributeType::Signature => Ok(MethodAttribute::Shared(SharedAttribute::read(
                attribute_type,
                cursor,
            )?)),
            AttributeType::MethodParameters => {
                let parameters_count = cursor.u8()? as usize;
                let mut parameters = Vec::with_capacity(parameters_count);
                for _ in 0..parameters_count {
                    parameters.push(MethodParameterEntry::new(cursor.u16()?, cursor.u16()?));
                }
                Ok(MethodAttribute::MethodParameters(parameters))
            }
            AttributeType::Exceptions => {
                let number_of_exceptions = cursor.u16()?;
                let mut exception_index_table = Vec::with_capacity(number_of_exceptions as usize);
                for _ in 0..number_of_exceptions {
                    exception_index_table.push(cursor.u16()?);
                }
                Ok(MethodAttribute::Exceptions(exception_index_table))
            }
            other => unimplemented!("Method attribute {:?} not implemented", other),
        }
    }

    #[cfg(feature = "pretty_print")]
    pub(crate) fn fmt_pretty(
        &self,
        ind: &mut common::utils::indent_write::Indented<'_>,
        cp: &ConstantPool,
        descriptor: &MethodDescriptor,
        this: &u16,
    ) -> std::fmt::Result {
        use common::pretty_try;
        use std::fmt::Write as _;
        match self {
            MethodAttribute::Shared(shared) => shared.fmt_pretty(ind, cp)?,
            MethodAttribute::Code(code) => code.fmt_pretty(ind, cp, descriptor, this)?,
            MethodAttribute::Exceptions(exc) => {
                writeln!(ind, "Exceptions:")?;
                ind.with_indent(|ind| {
                    writeln!(
                        ind,
                        "throws {}",
                        pretty_try!(
                            ind,
                            exc.iter()
                                .map(|index| cp.get_pretty_class_name(index))
                                .collect::<Result<Vec<_>, _>>()
                        )
                        .join(", ")
                    )?;
                    Ok(())
                })?
            }
            MethodAttribute::RuntimeVisibleParameterAnnotations => unimplemented!(),
            MethodAttribute::RuntimeInvisibleParameterAnnotations => unimplemented!(),
            MethodAttribute::AnnotationsDefault => unimplemented!(),
            MethodAttribute::MethodParameters(params) => {
                const W_NAME: usize = 32;
                writeln!(ind, "MethodParameters:")?;
                ind.with_indent(|ind| {
                    writeln!(ind, "{:>W_NAME$} Flags", "Name")?;
                    for param in params {
                        let name = if param.name_index == 0 {
                            "<no name>".to_string()
                        } else {
                            pretty_try!(ind, cp.get_utf8(&param.name_index)).to_string()
                        };
                        writeln!(ind, "{:>W_NAME$} 0x{:04x}", name, param.access_flags,)?;
                    }
                    Ok(())
                })?;
            }
        }

        Ok(())
    }
}

impl<'a> CodeAttribute {
    pub(crate) fn read(
        pool: &ConstantPool,
        cursor: &mut ByteCursor<'a>,
    ) -> Result<Self, ClassFileErr> {
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

    #[cfg(feature = "pretty_print")]
    pub(crate) fn fmt_pretty(
        &self,
        ind: &mut common::utils::indent_write::Indented<'_>,
        cp: &ConstantPool,
        method_descriptor: &MethodDescriptor,
        this: &u16,
    ) -> std::fmt::Result {
        use crate::print::get_pretty_instruction;
        use common::pretty_try;
        use std::fmt::Write as _;

        writeln!(ind, "Code: ")?;
        ind.with_indent(|ind| {
            writeln!(
                ind,
                "stack={}, locals={}, args_size={}",
                self.max_stack,
                self.max_locals,
                method_descriptor.params.len() + 1 // +1 for 'this'
            )?;
            let instructions = pretty_try!(ind, Instruction::new_instruction_set(&self.code));
            let mut byte_pos = 0;
            for instruction in instructions {
                writeln!(
                    ind,
                    "{byte_pos:4}: {:<24}",
                    pretty_try!(
                        ind,
                        get_pretty_instruction(&instruction, cp, byte_pos as i32, this)
                    )
                )?;
                byte_pos += instruction.byte_size();
            }
            if !self.exception_table.is_empty() {
                const W_START: usize = 6;
                const W_LENGTH: usize = 8;
                const W_SLOT: usize = 5;
                writeln!(ind, "Exception table:")?;
                ind.with_indent(|ind| {
                    writeln!(
                        ind,
                        "{:>W_START$} {:>W_LENGTH$} {:>W_SLOT$} type",
                        "from", "to", "target"
                    )?;
                    for entry in &self.exception_table {
                        let catch_type = if entry.catch_type == 0 {
                            "any"
                        } else {
                            pretty_try!(ind, cp.get_class_name(&entry.catch_type))
                        };
                        writeln!(
                            ind,
                            "{:>W_START$} {:>W_LENGTH$} {:>W_SLOT$}  {}{}",
                            entry.start_pc,
                            entry.end_pc,
                            entry.handler_pc,
                            if catch_type != "any" { "Class " } else { "" },
                            catch_type
                        )?;
                    }
                    Ok(())
                })?;
            }
            for attr in &self.attributes {
                attr.fmt_pretty(ind, cp, this)?;
            }
            Ok(())
        })?;

        Ok(())
    }
}
