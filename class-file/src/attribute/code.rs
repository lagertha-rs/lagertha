use crate::attribute::{get_utf8, ATTR_LINE_NUMBER_TABLE, ATTR_LOCAL_VARIABLE_TABLE};
use crate::constant_pool::ConstantInfo;
use crate::ClassFileErr;
use common::ByteCursor;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
struct LineNumberEntry {
    start_pc: u16,
    line_number: u16,
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct LocalVariableEntry {
    start_pc: u16,
    length: u16,
    name_index: u16,
    descriptor_index: u16,
    index: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CodeAttribute {
    LineNumberTable(Vec<LineNumberEntry>),
    LocalVariableTable(Vec<LocalVariableEntry>),
    Unknown { name_index: u16, info: Vec<u8> },
}

impl<'a> CodeAttribute {
    pub(crate) fn read(
        pool: &Vec<ConstantInfo>,
        cursor: &mut ByteCursor<'a>,
    ) -> Result<Self, ClassFileErr> {
        let attribute_name_index = cursor.u16()?;
        let attribute_length = cursor.u32()? as usize;

        let utf8 = get_utf8(attribute_name_index, pool)?.as_bytes();
        match utf8 {
            ATTR_LINE_NUMBER_TABLE => {
                let line_number_table_length = cursor.u16()? as usize;
                let mut line_number_table = Vec::with_capacity(line_number_table_length);
                for _ in 0..line_number_table_length {
                    line_number_table.push(LineNumberEntry {
                        start_pc: cursor.u16()?,
                        line_number: cursor.u16()?,
                    });
                }
                Ok(CodeAttribute::LineNumberTable(line_number_table))
            }
            ATTR_LOCAL_VARIABLE_TABLE => {
                let local_variable_table_length = cursor.u16()?;
                let mut local_variable_table =
                    Vec::with_capacity(local_variable_table_length as usize);
                for _ in 0..local_variable_table_length {
                    let start_pc = cursor.u16()?;
                    let length = cursor.u16()?;
                    let name_index = cursor.u16()?;
                    let descriptor_index = cursor.u16()?;
                    let index = cursor.u16()?;
                    local_variable_table.push(LocalVariableEntry {
                        start_pc,
                        length,
                        name_index,
                        descriptor_index,
                        index,
                    });
                }
                Ok(CodeAttribute::LocalVariableTable(local_variable_table))
            }
            _ => {
                let mut buf = vec![0u8; attribute_length];
                cursor.read_exact(&mut buf)?;
                Ok(CodeAttribute::Unknown {
                    name_index: attribute_name_index,
                    info: buf,
                })
            }
        }
    }
}

impl Display for CodeAttribute {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CodeAttribute::LineNumberTable(table) => {
                write!(f, "LineNumberTable{:?}", table)
            }
            CodeAttribute::LocalVariableTable(table) => {
                write!(f, "LocalVariableTable{:?}", table)
            }
            CodeAttribute::Unknown { name_index, info } => write!(
                f,
                "Unsupported(name_index: {}, data: {} bytes)",
                name_index,
                info.len()
            ),
        }
    }
}
