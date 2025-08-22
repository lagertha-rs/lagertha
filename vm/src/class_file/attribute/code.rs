use crate::byte_cursor::ByteCursor;
use crate::class_file::attribute::get_utf8;
use crate::class_file::constant_pool::ConstantInfo;
use crate::ClassFileErr;
use num_enum::TryFromPrimitive;
use std::fmt;
use std::fmt::{Display, Formatter};

pub const ATTR_LOCAL_VARIABLE_TABLE: &[u8] = b"LocalVariableTable";
pub const ATTR_LINE_NUMBER_TABLE: &[u8] = b"LineNumberTable";
pub const ATTR_STACK_MAP_TABLE: &[u8] = b"StackMapTable";

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.7.12
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineNumberEntry {
    pub start_pc: u16,
    pub line_number: u16,
}

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.7.13
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalVariableEntry {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub index: u16,
}

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.7.4
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StackMapFrame {
    Same {
        offset_delta: u16,
    },
    SameLocals1StackItem {
        offset_delta: u16,
        stack: VerificationTypeInfo,
    },
    SameLocals1StackItemExtended {
        offset_delta: u16,
        stack: VerificationTypeInfo,
    },
    Chop {
        k: u8,
        offset_delta: u16,
    },
    SameExtended {
        offset_delta: u16,
    },
    Append {
        k: u8,
        offset_delta: u16,
        locals: Vec<VerificationTypeInfo>,
    },
    Full {
        offset_delta: u16,
        locals: Vec<VerificationTypeInfo>,
        stack: Vec<VerificationTypeInfo>,
    },
}

impl<'a> StackMapFrame {
    pub(crate) fn read(cursor: &mut ByteCursor<'a>) -> Result<Self, ClassFileErr> {
        let frame_type = cursor.u8()?;
        match frame_type {
            0..64 => Ok(StackMapFrame::Same {
                offset_delta: u16::from(frame_type),
            }),
            64..128 => Ok(StackMapFrame::SameLocals1StackItem {
                offset_delta: u16::from(frame_type - 64),
                stack: VerificationTypeInfo::read(cursor)?,
            }),
            247 => Ok(StackMapFrame::SameLocals1StackItemExtended {
                offset_delta: cursor.u16()?,
                stack: VerificationTypeInfo::read(cursor)?,
            }),
            248..251 => Ok(StackMapFrame::Chop {
                k: (251 - frame_type),
                offset_delta: cursor.u16()?,
            }),
            251 => Ok(StackMapFrame::SameExtended {
                offset_delta: cursor.u16()?,
            }),
            252..255 => Ok(StackMapFrame::Append {
                k: (frame_type - 251),
                offset_delta: cursor.u16()?,
                locals: (0..usize::from(frame_type - 251))
                    .map(|_| VerificationTypeInfo::read(cursor)) // -> Result<_, E>
                    .collect::<Result<Vec<_>, _>>()?,
            }),
            255 => {
                let offset_delta = cursor.u16()?;
                let number_of_locals = cursor.u16()?;
                let mut locals = Vec::with_capacity(number_of_locals as usize);
                for _ in 0..number_of_locals {
                    locals.push(VerificationTypeInfo::read(cursor)?)
                }
                let number_of_stack_items = cursor.u16()?;
                let mut stack = Vec::with_capacity(number_of_stack_items as usize);
                for _ in 0..number_of_stack_items {
                    stack.push(VerificationTypeInfo::read(cursor)?)
                }
                Ok(StackMapFrame::Full {
                    offset_delta,
                    locals,
                    stack,
                })
            }
            _ => Err(ClassFileErr::TypeError), //TODO: Error
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum VerificationTypeTag {
    Top,
    Integer,
    Float,
    Double,
    Long,
    Null,
    UninitializedThis,
    Object,
    Uninitialized,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerificationTypeInfo {
    Top,
    Integer,
    Float,
    Double,
    Long,
    Null,
    UninitializedThis,
    Object(u16),
    Uninitialized(u16),
}

impl<'a> VerificationTypeInfo {
    pub(crate) fn read(cursor: &mut ByteCursor<'a>) -> Result<Self, ClassFileErr> {
        let raw_tag = cursor.u8()?;
        let frame_type: VerificationTypeTag = VerificationTypeTag::try_from_primitive(raw_tag)
            .map_err(|_| ClassFileErr::UnknownTag(raw_tag))?;
        Ok(match frame_type {
            VerificationTypeTag::Top => Self::Top,
            VerificationTypeTag::Integer => Self::Integer,
            VerificationTypeTag::Float => Self::Float,
            VerificationTypeTag::Double => Self::Double,
            VerificationTypeTag::Long => Self::Long,
            VerificationTypeTag::Null => Self::Null,
            VerificationTypeTag::UninitializedThis => Self::UninitializedThis,
            VerificationTypeTag::Object => Self::Object(cursor.u16()?),
            VerificationTypeTag::Uninitialized => Self::Uninitialized(cursor.u16()?),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CodeAttributeInfo {
    LineNumberTable(Vec<LineNumberEntry>),
    LocalVariableTable(Vec<LocalVariableEntry>),
    StackMapTable(Vec<StackMapFrame>),
    Unknown { name_index: u16, info: Vec<u8> },
}

impl<'a> CodeAttributeInfo {
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
                Ok(CodeAttributeInfo::LineNumberTable(line_number_table))
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
                Ok(CodeAttributeInfo::LocalVariableTable(local_variable_table))
            }
            ATTR_STACK_MAP_TABLE => {
                let frames_count = cursor.u16()?;
                let mut frames = Vec::with_capacity(frames_count as usize);
                for _ in 0..frames_count {
                    frames.push(StackMapFrame::read(cursor)?)
                }
                Ok(CodeAttributeInfo::StackMapTable(frames))
            }
            _ => {
                let mut buf = vec![0u8; attribute_length];
                cursor.read_exact(&mut buf)?;
                Ok(CodeAttributeInfo::Unknown {
                    name_index: attribute_name_index,
                    info: buf,
                })
            }
        }
    }
}

impl Display for CodeAttributeInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CodeAttributeInfo::LineNumberTable(table) => {
                write!(f, "LineNumberTable{table:?}")
            }
            CodeAttributeInfo::LocalVariableTable(table) => {
                write!(f, "LocalVariableTable {table:?}")
            }
            CodeAttributeInfo::StackMapTable(table) => {
                write!(f, "StackMapTable {table:?}")
            }
            CodeAttributeInfo::Unknown { name_index, info } => write!(
                f,
                "Unsupported(name_index: {}, data: {} bytes)",
                name_index,
                info.len()
            ),
        }
    }
}
