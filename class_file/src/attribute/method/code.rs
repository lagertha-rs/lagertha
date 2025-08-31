use crate::ClassFileErr;
use crate::attribute::AttributeType;
use crate::constant::pool::ConstantPool;
use common::utils::cursor::ByteCursor;
use num_enum::TryFromPrimitive;
#[cfg(test)]
use serde::Serialize;

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.7.3
#[cfg_attr(test, derive(Serialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CodeAttributeInfo {
    LineNumberTable(Vec<LineNumberEntry>),
    LocalVariableTable(Vec<LocalVariableEntry>),
    StackMapTable(Vec<StackMapFrame>),
    LocalVariableTypeTable(Vec<LocalVariableTypeEntry>),
    RuntimeVisibleTypeAnnotations,
    RuntimeInvisibleTypeAnnotations,
}

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.7.12
#[cfg_attr(test, derive(Serialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineNumberEntry {
    pub start_pc: u16,
    pub line_number: u16,
}

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.7.13
#[cfg_attr(test, derive(Serialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalVariableEntry {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub index: u16,
}

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.7.14
#[cfg_attr(test, derive(Serialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalVariableTypeEntry {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: u16,
    pub signature_index: u16,
    pub index: u16,
}

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.7.4
#[cfg_attr(test, derive(Serialize))]
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

#[cfg_attr(test, derive(Serialize))]
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

#[cfg_attr(test, derive(Serialize))]
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

impl<'a> StackMapFrame {
    pub(crate) fn read(cursor: &mut ByteCursor<'a>) -> Result<Self, ClassFileErr> {
        let frame_type = cursor.u8()?;
        match frame_type {
            0..=63 => Ok(StackMapFrame::Same {
                offset_delta: u16::from(frame_type),
            }),
            64..=127 => Ok(StackMapFrame::SameLocals1StackItem {
                offset_delta: u16::from(frame_type - 64),
                stack: VerificationTypeInfo::read(cursor)?,
            }),
            247 => Ok(StackMapFrame::SameLocals1StackItemExtended {
                offset_delta: cursor.u16()?,
                stack: VerificationTypeInfo::read(cursor)?,
            }),
            248..=250 => {
                let a = 2;
                if a == 2 {
                    // just to have a place for a breakpoint
                }
                Ok(StackMapFrame::Chop {
                    k: (251 - frame_type),
                    offset_delta: cursor.u16()?,
                })
            }
            251 => Ok(StackMapFrame::SameExtended {
                offset_delta: cursor.u16()?,
            }),
            252..=254 => Ok(StackMapFrame::Append {
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
            other => Err(ClassFileErr::UnknownStackFrameType(other)),
        }
    }

    #[cfg(feature = "pretty_print")]
    pub(crate) fn fmt_pretty(
        &self,
        ind: &mut common::utils::indent_write::Indented<'_>,
        cp: &ConstantPool,
        this: &u16,
    ) -> std::fmt::Result {
        use common::pretty_try;
        use std::fmt::Write as _;

        let get_pretty_verif_type =
            |locals: &Vec<VerificationTypeInfo>| -> Result<String, ClassFileErr> {
                locals
                    .iter()
                    .map(|v| v.get_pretty_value(cp, this))
                    .collect::<Result<Vec<_>, _>>()
                    .map(|v| v.join(", "))
            };

        write!(ind, "frame_type = ")?;
        match self {
            StackMapFrame::Same { offset_delta } => writeln!(ind, "{offset_delta} /* same */")?,
            StackMapFrame::SameLocals1StackItem {
                offset_delta,
                stack,
            } => {
                writeln!(ind, "{} /* same_locals_1_stack_item */", offset_delta + 64)?;
                ind.with_indent(|ind| {
                    writeln!(
                        ind,
                        "stack = [ {} ]",
                        pretty_try!(ind, stack.get_pretty_value(cp, this))
                    )?;
                    Ok(())
                })?;
            }
            StackMapFrame::SameLocals1StackItemExtended { .. } => unimplemented!(),
            StackMapFrame::Chop { k, offset_delta } => {
                writeln!(ind, "{} /* chop */", 251 - k)?;
                ind.with_indent(|ind| {
                    writeln!(ind, "offset_delta = {offset_delta}")?;
                    Ok(())
                })?;
            }
            StackMapFrame::SameExtended { offset_delta } => {
                writeln!(ind, "251 /* same_frame_extended */")?;
                ind.with_indent(|ind| {
                    writeln!(ind, "offset_delta = {offset_delta}")?;
                    Ok(())
                })?;
            }
            StackMapFrame::Append {
                k,
                offset_delta,
                locals,
            } => {
                writeln!(ind, "{} /* append */", 251 + k)?;
                ind.with_indent(|ind| {
                    writeln!(ind, "offset_delta = {offset_delta}")?;
                    writeln!(
                        ind,
                        "locals = [{}]",
                        pretty_try!(ind, get_pretty_verif_type(locals))
                    )?;
                    Ok(())
                })?;
            }
            StackMapFrame::Full {
                offset_delta,
                locals,
                stack,
            } => {
                writeln!(ind, "255 /* full_frame */")?;
                ind.with_indent(|ind| {
                    writeln!(ind, "offset_delta = {offset_delta}")?;
                    writeln!(
                        ind,
                        "locals = [{}]",
                        pretty_try!(ind, get_pretty_verif_type(locals))
                    )?;
                    writeln!(
                        ind,
                        "stack = [{}]",
                        pretty_try!(ind, get_pretty_verif_type(stack))
                    )?;
                    Ok(())
                })?;
            }
        }
        Ok(())
    }
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

    #[cfg(feature = "pretty_print")]
    pub(crate) fn get_pretty_value(
        &self,
        cp: &ConstantPool,
        this: &u16,
    ) -> Result<String, ClassFileErr> {
        Ok(match self {
            VerificationTypeInfo::Top => "top".to_string(),
            VerificationTypeInfo::Integer => "int".to_string(),
            VerificationTypeInfo::Float => "float".to_string(),
            VerificationTypeInfo::Double => "double".to_string(),
            VerificationTypeInfo::Long => "long".to_string(),
            VerificationTypeInfo::Null => "null".to_string(),
            VerificationTypeInfo::UninitializedThis => unimplemented!(),
            VerificationTypeInfo::Object(cp_index) => {
                cp.get_raw(cp_index)?.get_pretty_value(cp, this)?
            }
            VerificationTypeInfo::Uninitialized(_) => unimplemented!(),
        })
    }
}

impl<'a> CodeAttributeInfo {
    pub(crate) fn read(
        pool: &ConstantPool,
        cursor: &mut ByteCursor<'a>,
    ) -> Result<Self, ClassFileErr> {
        let attribute_name_index = cursor.u16()?;
        let _attribute_length = cursor.u32()? as usize;

        let attribute_type = AttributeType::try_from(pool.get_utf8(&attribute_name_index)?)?;
        match attribute_type {
            AttributeType::LineNumberTable => {
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
            AttributeType::LocalVariableTable => {
                let local_variable_table_length = cursor.u16()?;
                let mut local_variable_table =
                    Vec::with_capacity(local_variable_table_length as usize);
                for _ in 0..local_variable_table_length {
                    local_variable_table.push(LocalVariableEntry {
                        start_pc: cursor.u16()?,
                        length: cursor.u16()?,
                        name_index: cursor.u16()?,
                        descriptor_index: cursor.u16()?,
                        index: cursor.u16()?,
                    });
                }
                Ok(CodeAttributeInfo::LocalVariableTable(local_variable_table))
            }
            AttributeType::LocalVariableTypeTable => {
                let local_variable_table_type_length = cursor.u16()?;
                let mut local_variable_type_table =
                    Vec::with_capacity(local_variable_table_type_length as usize);
                for _ in 0..local_variable_table_type_length {
                    local_variable_type_table.push(LocalVariableTypeEntry {
                        start_pc: cursor.u16()?,
                        length: cursor.u16()?,
                        name_index: cursor.u16()?,
                        signature_index: cursor.u16()?,
                        index: cursor.u16()?,
                    });
                }
                Ok(CodeAttributeInfo::LocalVariableTypeTable(
                    local_variable_type_table,
                ))
            }
            AttributeType::StackMapTable => {
                let frames_count = cursor.u16()?;
                let mut frames = Vec::with_capacity(frames_count as usize);
                for _ in 0..frames_count {
                    frames.push(StackMapFrame::read(cursor)?)
                }
                Ok(CodeAttributeInfo::StackMapTable(frames))
            }
            other => unimplemented!("{other:?}"),
        }
    }

    #[cfg(feature = "pretty_print")]
    pub(crate) fn fmt_pretty(
        &self,
        ind: &mut common::utils::indent_write::Indented<'_>,
        cp: &ConstantPool,
        this: &u16,
    ) -> std::fmt::Result {
        use common::pretty_try;
        use std::fmt::Write as _;

        match self {
            CodeAttributeInfo::LineNumberTable(table) => {
                writeln!(ind, "LineNumberTable:")?;
                ind.with_indent(|ind| {
                    for entry in table {
                        writeln!(ind, "line {}: {}", entry.line_number, entry.start_pc)?;
                    }
                    Ok(())
                })?;
            }
            CodeAttributeInfo::LocalVariableTable(table) => {
                const W_START: usize = 6;
                const W_LENGTH: usize = 8;
                const W_SLOT: usize = 5;
                const W_NAME: usize = 6;
                writeln!(ind, "LocalVariableTable:")?;
                writeln!(
                    ind,
                    "{:>W_START$} {:>W_LENGTH$} {:>W_SLOT$}  {:<W_NAME$} {}",
                    "Start", "Length", "Slot", "Name", "Signature",
                )?;
                for lv in table {
                    let name = pretty_try!(ind, cp.get_utf8(&lv.name_index));
                    let descriptor = pretty_try!(ind, cp.get_utf8(&lv.descriptor_index));
                    writeln!(
                        ind,
                        "{:>W_START$} {:>W_LENGTH$} {:>W_SLOT$}  {:<W_NAME$} {}",
                        lv.start_pc, lv.length, lv.index, name, descriptor,
                    )?;
                }
            }
            CodeAttributeInfo::StackMapTable(table) => {
                writeln!(ind, "StackMapTable: number_of_entries = {}", table.len())?;
                ind.with_indent(|ind| {
                    for frame in table {
                        frame.fmt_pretty(ind, cp, this)?;
                    }
                    Ok(())
                })?;
            }
            CodeAttributeInfo::LocalVariableTypeTable(table) => {
                writeln!(ind, "LocalVariableTypeTable:")?;
                const W_START: usize = 6;
                const W_LENGTH: usize = 8;
                const W_SLOT: usize = 5;
                const W_NAME: usize = 4;
                ind.with_indent(|ind| {
                    writeln!(
                        ind,
                        "{:>W_START$} {:>W_LENGTH$} {:>W_SLOT$}  {:<W_NAME$}   Signature",
                        "Start", "Length", "Slot", "Name"
                    )?;
                    for lv in table {
                        let name = pretty_try!(ind, cp.get_utf8(&lv.name_index));
                        let signature = pretty_try!(ind, cp.get_utf8(&lv.signature_index));
                        writeln!(
                            ind,
                            "{:>W_START$} {:>W_LENGTH$} {:>W_SLOT$}  {:<W_NAME$}   {}",
                            lv.start_pc, lv.length, lv.index, name, signature,
                        )?;
                    }
                    Ok(())
                })?;
            }
            CodeAttributeInfo::RuntimeVisibleTypeAnnotations => unimplemented!(),
            CodeAttributeInfo::RuntimeInvisibleTypeAnnotations => unimplemented!(),
        }
        Ok(())
    }
}
