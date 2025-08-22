use crate::byte_cursor::ByteCursor;
use crate::class_file::attribute::annotation::Annotation;
use crate::class_file::attribute::code::CodeAttributeInfo;
use crate::class_file::attribute::get_utf8;
use crate::class_file::constant_pool::ConstantInfo;
use crate::ClassFileErr;
use std::fmt;
use std::fmt::{Display, Formatter};

const ATTR_CODE: &[u8] = b"Code";
const ATTR_RT_VISIBLE_ANNOTATIONS: &[u8] = b"RuntimeVisibleAnnotations";
const ATTR_SIGNATURE: &[u8] = b"Signature";
const ATTR_EXCEPTIONS: &[u8] = b"Exceptions";
const ATTR_DEPRECATED: &[u8] = b"Deprecated";

#[derive(Debug, Clone, PartialEq, Eq)]
struct ExceptionTableEntry {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeAttribute {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<u8>,
    pub exception_table: Vec<ExceptionTableEntry>,
    pub attributes: Vec<CodeAttributeInfo>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MethodAttribute {
    Code(CodeAttribute),
    RuntimeVisibleAnnotations(Vec<Annotation>),
    Signature(u16),
    Exceptions(Vec<u16>),
    Deprecated,
    Unknown { name_index: u16, info: Vec<u8> },
}

impl<'a> MethodAttribute {
    pub(crate) fn read(
        pool: &Vec<ConstantInfo>,
        cursor: &mut ByteCursor<'a>,
    ) -> Result<Self, ClassFileErr> {
        let attribute_name_index = cursor.u16()?;
        let attribute_length = cursor.u32()? as usize;

        let utf8 = get_utf8(attribute_name_index, pool)?.as_bytes();
        match utf8 {
            ATTR_CODE => {
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

                Ok(MethodAttribute::Code(CodeAttribute {
                    max_stack,
                    max_locals,
                    code,
                    exception_table,
                    attributes,
                }))
            }
            ATTR_RT_VISIBLE_ANNOTATIONS => {
                let num_annotations = cursor.u16()?;
                let mut annotations = Vec::with_capacity(num_annotations as usize);

                for _ in 0..num_annotations {
                    annotations.push(Annotation::read(cursor)?)
                }
                Ok(MethodAttribute::RuntimeVisibleAnnotations(annotations))
            }
            ATTR_SIGNATURE => Ok(MethodAttribute::Signature(cursor.u16()?)),
            ATTR_DEPRECATED => Ok(MethodAttribute::Deprecated),
            ATTR_EXCEPTIONS => {
                let number_of_exceptions = cursor.u16()?;
                let mut exception_index_table = Vec::with_capacity(number_of_exceptions as usize);
                for _ in 0..number_of_exceptions {
                    exception_index_table.push(cursor.u16()?);
                }
                Ok(MethodAttribute::Exceptions(exception_index_table))
            }
            _ => {
                let mut buf = vec![0u8; attribute_length];
                cursor.read_exact(&mut buf)?;
                Ok(MethodAttribute::Unknown {
                    name_index: attribute_name_index,
                    info: buf,
                })
            }
        }
    }
}

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
