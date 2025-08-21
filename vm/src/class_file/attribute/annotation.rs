use crate::byte_cursor::ByteCursor;
use crate::class_file::ClassFileErr;
use num_enum::TryFromPrimitive;

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.7.16
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Annotation {
    pub type_index: u16,
    pub element_value_pairs: Vec<ElementValuePair>,
}

impl<'a> Annotation {
    pub(crate) fn read(cursor: &mut ByteCursor<'a>) -> Result<Self, ClassFileErr> {
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

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.7.16
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElementValuePair {
    pub element_name_index: u16,
    pub value: ElementValue,
}

impl<'a> ElementValuePair {
    pub(crate) fn read(cursor: &mut ByteCursor<'a>) -> Result<Self, ClassFileErr> {
        Ok(Self {
            element_name_index: cursor.u16()?,
            value: ElementValue::read(cursor)?,
        })
    }
}

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.7.16.1
#[derive(Debug, Clone, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum ElementTag {
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

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.7.16.1
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
    pub(crate) fn read(cursor: &mut ByteCursor<'a>) -> Result<Self, ClassFileErr> {
        let raw_tag = cursor.u8()?;
        let tag = ElementTag::try_from_primitive(raw_tag)
            .map_err(|_| ClassFileErr::UnknownTag(raw_tag))?;

        let ev = match tag {
            ElementTag::Byte => ElementValue::Byte(cursor.u16()?),
            ElementTag::Char => ElementValue::Char(cursor.u16()?),
            ElementTag::Double => ElementValue::Double(cursor.u16()?),
            ElementTag::Float => ElementValue::Float(cursor.u16()?),
            ElementTag::Int => ElementValue::Int(cursor.u16()?),
            ElementTag::Long => ElementValue::Long(cursor.u16()?),
            ElementTag::Short => ElementValue::Short(cursor.u16()?),
            ElementTag::Boolean => ElementValue::Boolean(cursor.u16()?),
            ElementTag::String => ElementValue::String(cursor.u16()?),
            ElementTag::EnumClass => ElementValue::EnumConstValue {
                type_name_index: cursor.u16()?,
                const_name_index: cursor.u16()?,
            },
            ElementTag::Class => ElementValue::Class(cursor.u16()?),
            ElementTag::AnnotationInterface => {
                ElementValue::AnnotationValue(Annotation::read(cursor)?)
            }
            ElementTag::ArrayType => {
                let element_types = cursor.u16()?;
                let mut elements = Vec::with_capacity(element_types as usize);
                for _ in 0..element_types {
                    elements.push(ElementValue::read(cursor)?)
                }
                ElementValue::Array(elements)
            }
        };

        Ok(ev)
    }
}
