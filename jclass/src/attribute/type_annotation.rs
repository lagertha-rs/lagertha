//! Type annotation types for class file attributes.
//!
//! https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.20

use super::annotation::ElementValuePair;
use crate::ClassFormatErr;
use common::utils::cursor::ByteCursor;

/// Target information for a type annotation, identifying where the type is used.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TargetInfo {
    TypeParameter {
        type_parameter_index: u8,
    },
    Supertype {
        supertype_index: u16,
    },
    TypeParameterBound {
        type_parameter_index: u8,
        bound_index: u8,
    },
    Empty,
    MethodFormalParameter {
        formal_parameter_index: u8,
    },
    Throws {
        throws_type_index: u16,
    },
    LocalVar {
        localvar_table: Vec<LocalVarEntry>,
    },
    Catch {
        exception_table_index: u16,
    },
    Offset {
        offset: u16,
    },
    TypeArgument {
        offset: u16,
        type_argument_index: u8,
    },
}

/// An entry in the local variable table for type annotations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalVarEntry {
    pub start_pc: u16,
    pub length: u16,
    pub index: u16,
}

/// A path to a specific nested type within a compound type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypePath {
    pub path: Vec<TypePathEntry>,
}

/// A single step in a type path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypePathEntry {
    pub type_path_kind: u8,
    pub type_argument_index: u8,
}

/// A type annotation with its target and path information.
///
/// https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.20
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeAnnotation {
    pub target_info: TargetInfo,
    pub target_path: TypePath,
    pub type_index: u16,
    pub element_value_pairs: Vec<ElementValuePair>,
}

impl<'a> TypeAnnotation {
    pub(crate) fn read(cursor: &mut ByteCursor<'a>) -> Result<Self, ClassFormatErr> {
        let target_type = cursor.u8()?;

        let target_info = match target_type {
            0x00 => TargetInfo::TypeParameter {
                type_parameter_index: cursor.u8()?,
            },
            0x01 => TargetInfo::Supertype {
                supertype_index: cursor.u16()?,
            },
            0x10 => TargetInfo::TypeParameterBound {
                type_parameter_index: cursor.u8()?,
                bound_index: cursor.u8()?,
            },
            0x11 => TargetInfo::Empty,
            0x12 => TargetInfo::MethodFormalParameter {
                formal_parameter_index: cursor.u8()?,
            },
            0x13 => TargetInfo::Throws {
                throws_type_index: cursor.u16()?,
            },
            0x14 => {
                let table_length = cursor.u16()?;
                let mut localvar_table = Vec::with_capacity(table_length as usize);
                for _ in 0..table_length {
                    localvar_table.push(LocalVarEntry {
                        start_pc: cursor.u16()?,
                        length: cursor.u16()?,
                        index: cursor.u16()?,
                    });
                }
                TargetInfo::LocalVar { localvar_table }
            }
            0x15 => TargetInfo::Catch {
                exception_table_index: cursor.u16()?,
            },
            0x16 => TargetInfo::Offset {
                offset: cursor.u16()?,
            },
            0x17 => TargetInfo::TypeArgument {
                offset: cursor.u16()?,
                type_argument_index: cursor.u8()?,
            },
            _ => unimplemented!(),
        };

        // Read type_path
        let path_length = cursor.u8()?;
        let mut path = Vec::with_capacity(path_length as usize);
        for _ in 0..path_length {
            path.push(TypePathEntry {
                type_path_kind: cursor.u8()?,
                type_argument_index: cursor.u8()?,
            });
        }
        let target_path = TypePath { path };

        // Read annotation data
        let type_index = cursor.u16()?;
        let num_element_value_pairs = cursor.u16()?;
        let mut element_value_pairs = Vec::with_capacity(num_element_value_pairs as usize);
        for _ in 0..num_element_value_pairs {
            element_value_pairs.push(ElementValuePair::read(cursor)?);
        }

        Ok(Self {
            target_info,
            target_path,
            type_index,
            element_value_pairs,
        })
    }
}
