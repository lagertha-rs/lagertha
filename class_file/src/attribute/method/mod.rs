use crate::attribute::method::code::CodeAttributeInfo;
use crate::attribute::{AttributeType, SharedAttribute};
use crate::constant::pool::ConstantPool;
use crate::ClassFileErr;
use common::descriptor::MethodDescriptor;
use common::instruction::Instruction;
use common::utils::cursor::ByteCursor;

pub mod code;

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.7.3
#[derive(Debug, Clone, PartialEq, Eq)]
struct ExceptionTableEntry {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.7.3
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeAttribute {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<u8>,
    pub exception_table: Vec<ExceptionTableEntry>,
    pub attributes: Vec<CodeAttributeInfo>,
}

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.7
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MethodAttribute {
    Shared(SharedAttribute),
    Code(CodeAttribute),
    Exceptions(Vec<u16>),
    RuntimeVisibleParameterAnnotations,
    RuntimeInvisibleParameterAnnotations,
    AnnotationsDefault,
    MethodParameters,
}

impl<'a> MethodAttribute {
    pub(crate) fn read(
        pool: &ConstantPool,
        cursor: &mut ByteCursor<'a>,
    ) -> Result<Self, ClassFileErr> {
        let attribute_name_index = cursor.u16()?;
        let _attribute_length = cursor.u32()? as usize;

        let attribute_type = AttributeType::try_from(pool.get_utf8(attribute_name_index)?)?;
        match attribute_type {
            AttributeType::Code => Ok(MethodAttribute::Code(CodeAttribute::read(pool, cursor)?)),
            AttributeType::RuntimeVisibleAnnotations
            | AttributeType::Synthetic
            | AttributeType::Deprecated
            | AttributeType::Signature => Ok(MethodAttribute::Shared(SharedAttribute::read(
                attribute_type,
                cursor,
            )?)),
            AttributeType::Exceptions => {
                let number_of_exceptions = cursor.u16()?;
                let mut exception_index_table = Vec::with_capacity(number_of_exceptions as usize);
                for _ in 0..number_of_exceptions {
                    exception_index_table.push(cursor.u16()?);
                }
                Ok(MethodAttribute::Exceptions(exception_index_table))
            }
            _ => unimplemented!(),
        }
    }

    #[cfg(feature = "pretty_print")]
    pub(crate) fn fmt_pretty(
        &self,
        ind: &mut common::utils::indent_write::Indented<'_>,
        cp: &ConstantPool,
        descriptor: &MethodDescriptor,
    ) -> std::fmt::Result {
        match self {
            MethodAttribute::Shared(_) => {}
            MethodAttribute::Code(code) => code.fmt_pretty(ind, cp, descriptor)?,
            MethodAttribute::Exceptions(_) => {}
            MethodAttribute::RuntimeVisibleParameterAnnotations => {}
            MethodAttribute::RuntimeInvisibleParameterAnnotations => {}
            MethodAttribute::AnnotationsDefault => {}
            MethodAttribute::MethodParameters => {}
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
    ) -> std::fmt::Result {
        use crate::print::get_pretty_instruction;
        use common::pretty_try;
        use std::fmt::Write as _;

        writeln!(ind, "Code: ")?;
        ind.with_indent(|nested| {
            writeln!(
                nested,
                "stack={}, locals={}, args_size={}",
                self.max_stack,
                self.max_locals,
                method_descriptor.params.len() + 1 // +1 for 'this'
            )?;
            let instructions = pretty_try!(nested, Instruction::new_instruction_set(&self.code));
            let mut byte_pos = 0;
            for instruction in instructions {
                writeln!(
                    nested,
                    "{byte_pos:4}: {:<24}",
                    pretty_try!(
                        nested,
                        get_pretty_instruction(&instruction, cp, byte_pos as i32)
                    )
                )?;
                byte_pos += instruction.byte_size();
            }
            Ok(())
        })?;

        Ok(())
    }
}

/*
impl Display for MethodAttribute {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MethodAttribute::Code(code) => {
                let code_str = code
                    .code
                    .iter()
                    .map(|b| format!("{:02X}", b))
                    .collect::<Vec<_>>()
                    .join(" ");

                write!(
                    f,
                    "Code(max_stack: {}, max_locals: {}, code: \"{}\"",
                    code.max_stack, code.max_locals, code_str
                )?;

                if !code.exception_table.is_empty() {
                    write!(f, ", exception_table: {:?} ", code.exception_table)?;
                }
                if !code.attributes.is_empty() {
                    write!(f, ", attributes: [")?;
                    for (i, attr) in code.attributes.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", attr)?;
                    }
                    write!(f, "]")?;
                }
                write!(f, ")")
            }
            MethodAttribute::RuntimeVisibleAnnotations(annotations) => {
                write!(f, "RuntimeVisibleAnnotations {annotations:?}")
            }
            MethodAttribute::Exceptions(exceptions) => {
                write!(f, "Exceptions {exceptions:?}")
            }
            MethodAttribute::Signature(idx) => write!(f, "Signature: {idx:?}"),
            MethodAttribute::Deprecated => write!(f, "Deprecated"),
            MethodAttribute::Unknown { name_index, info } => write!(
                f,
                "Unsupported(name_index: {}, data: {} bytes)",
                name_index,
                info.len()
            ),
        }
    }
}

 */
