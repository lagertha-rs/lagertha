//! Java annotation types for class file attributes.
//!
//! https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.16

use crate::ClassFormatErr;
use common::utils::cursor::ByteCursor;
use num_enum::TryFromPrimitive;

/// A runtime annotation on a class, field, method, or record component.
///
/// https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.16
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Annotation {
    pub type_index: u16,
    pub element_value_pairs: Vec<ElementValuePair>,
}

impl<'a> Annotation {
    pub(crate) fn read(cursor: &mut ByteCursor<'a>) -> Result<Self, ClassFormatErr> {
        let type_index = cursor.u16()?;
        let num_element_value_pairs = cursor.u16()?;
        let mut element_value_pairs = Vec::with_capacity(num_element_value_pairs as usize);

        for _ in 0..num_element_value_pairs {
            element_value_pairs.push(ElementValuePair::read(cursor)?)
        }

        Ok(Self {
            type_index,
            element_value_pairs,
        })
    }
}

/// A name-value pair in an annotation.
///
/// https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.16
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElementValuePair {
    pub element_name_index: u16,
    pub value: ElementValue,
}

impl<'a> ElementValuePair {
    pub(crate) fn read(cursor: &mut ByteCursor<'a>) -> Result<Self, ClassFormatErr> {
        Ok(Self {
            element_name_index: cursor.u16()?,
            value: ElementValue::read(cursor)?,
        })
    }
}

/// Tag identifying the type of an element value in an annotation.
///
/// https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.16.1
#[derive(Debug, Clone, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum ElementKind {
    Byte = b'B',
    Char = b'C',
    Double = b'D',
    Float = b'F',
    Int = b'I',
    Long = b'J',
    Short = b'S',
    Boolean = b'Z',
    String = b's',
    EnumClass = b'e',
    Class = b'c',
    AnnotationInterface = b'@',
    ArrayType = b'[',
}

/// The value of an element in an annotation.
///
/// https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.16.1
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElementValue {
    Byte(u16),
    Char(u16),
    Double(u16),
    Float(u16),
    Int(u16),
    Long(u16),
    Short(u16),
    Boolean(u16),
    String(u16),
    EnumConstValue {
        type_name_index: u16,
        const_name_index: u16,
    },
    Class(u16),
    AnnotationValue(Annotation),
    Array(Vec<ElementValue>),
}

impl<'a> ElementValue {
    pub(crate) fn read(cursor: &mut ByteCursor<'a>) -> Result<Self, ClassFormatErr> {
        let raw_tag = cursor.u8()?;
        let tag = ElementKind::try_from_primitive(raw_tag)
            .map_err(|_| ClassFormatErr::UnknownTag(raw_tag))?;

        let ev = match tag {
            ElementKind::Byte => Self::Byte(cursor.u16()?),
            ElementKind::Char => Self::Char(cursor.u16()?),
            ElementKind::Double => Self::Double(cursor.u16()?),
            ElementKind::Float => Self::Float(cursor.u16()?),
            ElementKind::Int => Self::Int(cursor.u16()?),
            ElementKind::Long => Self::Long(cursor.u16()?),
            ElementKind::Short => Self::Short(cursor.u16()?),
            ElementKind::Boolean => Self::Boolean(cursor.u16()?),
            ElementKind::String => Self::String(cursor.u16()?),
            ElementKind::EnumClass => Self::EnumConstValue {
                type_name_index: cursor.u16()?,
                const_name_index: cursor.u16()?,
            },
            ElementKind::Class => Self::Class(cursor.u16()?),
            ElementKind::AnnotationInterface => Self::AnnotationValue(Annotation::read(cursor)?),
            ElementKind::ArrayType => {
                let element_types = cursor.u16()?;
                let mut elements = Vec::with_capacity(element_types as usize);
                for _ in 0..element_types {
                    elements.push(Self::read(cursor)?)
                }
                ElementValue::Array(elements)
            }
        };

        Ok(ev)
    }

    #[cfg(feature = "javap_print")]
    pub fn get_javap_descriptor(&self) -> String {
        match self {
            ElementValue::Boolean(v) => format!("Z#{}", v),
            ElementValue::String(v) => format!("s#{}", v),
            _ => unimplemented!(),
        }
    }

    #[cfg(feature = "javap_print")]
    pub(crate) fn get_javap_value(
        &self,
        cp: &crate::constant_pool::ConstantPool,
    ) -> Result<String, ClassFormatErr> {
        Ok(match self {
            ElementValue::Boolean(idx) => match cp.get_integer(idx)? {
                0 => "false".to_string(),
                1 => "true".to_string(),
                _ => unimplemented!(),
            },
            ElementValue::String(idx) => format!("\"{}\"", cp.get_utf8(idx)?),
            _ => unimplemented!(),
        })
    }
}
