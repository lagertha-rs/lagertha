use crate::ClassFormatErr;
use common::utils::cursor::ByteCursor;
use core::fmt;
use num_enum::TryFromPrimitive;
use std::fmt::Formatter;

pub mod class;
pub mod field;
pub mod method;

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.7
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttributeType {
    ConstantValue,
    Code,
    Exceptions,
    SourceFile,
    LineNumberTable,
    LocalVariableTable,
    InnerClasses,
    Synthetic,
    Deprecated,
    EnclosingMethod,
    Signature,
    SourceDebugExtension,
    LocalVariableTypeTable,
    RuntimeVisibleAnnotations,
    RuntimeInvisibleAnnotations,
    RuntimeVisibleParameterAnnotations,
    RuntimeInvisibleParameterAnnotations,
    AnnotationDefault,
    StackMapTable,
    BootstrapMethods,
    RuntimeVisibleTypeAnnotations,
    RuntimeInvisibleTypeAnnotations,
    MethodParameters,
    Module,
    ModulePackages,
    ModuleMainClass,
    NestHost,
    NestMembers,
    Record,
    PermittedSubclasses,
}

/// Common attribute payloads that appear at multiple locations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SharedAttribute {
    Synthetic,
    Deprecated,
    Signature(u16),
    RuntimeVisibleAnnotations(Vec<Annotation>),
    RuntimeInvisibleAnnotations(Vec<Annotation>),
    RuntimeVisibleTypeAnnotations,
    RuntimeInvisibleTypeAnnotations,
}

impl<'a> SharedAttribute {
    pub(crate) fn read(
        attr_type: AttributeType,
        cursor: &mut ByteCursor<'a>,
    ) -> Result<Self, ClassFormatErr> {
        match attr_type {
            AttributeType::Synthetic => Ok(SharedAttribute::Synthetic),
            AttributeType::Deprecated => Ok(SharedAttribute::Deprecated),
            AttributeType::Signature => {
                let signature_index = cursor.u16()?;
                Ok(SharedAttribute::Signature(signature_index))
            }
            AttributeType::RuntimeVisibleAnnotations => {
                let num_annotations = cursor.u16()?;
                let mut annotations = Vec::with_capacity(num_annotations as usize);
                for _ in 0..num_annotations {
                    annotations.push(Annotation::read(cursor)?);
                }
                Ok(SharedAttribute::RuntimeVisibleAnnotations(annotations))
            }
            AttributeType::RuntimeInvisibleAnnotations => {
                let num_annotations = cursor.u16()?;
                let mut annotations = Vec::with_capacity(num_annotations as usize);
                for _ in 0..num_annotations {
                    annotations.push(Annotation::read(cursor)?);
                }
                Ok(SharedAttribute::RuntimeInvisibleAnnotations(annotations))
            }
            AttributeType::RuntimeInvisibleTypeAnnotations
            | AttributeType::RuntimeVisibleTypeAnnotations => unimplemented!(),
            _ => Err(ClassFormatErr::AttributeIsNotShared(attr_type)),
        }
    }

    #[cfg(feature = "pretty_print")]
    pub(crate) fn fmt_pretty(
        &self,
        ind: &mut common::utils::indent_write::Indented<'_>,
        cp: &crate::constant::pool::ConstantPool,
    ) -> fmt::Result {
        use common::pretty_class_name_try;
        use common::pretty_try;
        use itertools::Itertools;
        use std::fmt::Write as _;

        match self {
            SharedAttribute::Synthetic => unimplemented!(),
            SharedAttribute::Deprecated => writeln!(ind, "Deprecated: true")?,
            SharedAttribute::Signature(index) => writeln!(
                ind,
                "Signature: #{:<26} // {}",
                index,
                pretty_try!(ind, cp.get_utf8(index)),
            )?,
            SharedAttribute::RuntimeVisibleAnnotations(annotations) => {
                writeln!(ind, "RuntimeVisibleAnnotations:")?;
                ind.with_indent(|ind| {
                    for (i, annotation) in annotations.iter().enumerate() {
                        writeln!(
                            ind,
                            "{i}: #{}({})",
                            annotation.type_index,
                            annotation
                                .element_value_pairs
                                .iter()
                                .map(|pair| format!(
                                    "#{}={}",
                                    pair.element_name_index,
                                    pair.value.get_pretty_descriptor()
                                ))
                                .join(",")
                        )?;
                        ind.with_indent(|ind| {
                            write!(
                                ind,
                                "{}",
                                pretty_class_name_try!(ind, cp.get_utf8(&annotation.type_index))
                            )?;
                            if !annotation.element_value_pairs.is_empty() {
                                writeln!(ind, "(")?;
                                for param in &annotation.element_value_pairs {
                                    ind.with_indent(|ind| {
                                        writeln!(
                                            ind,
                                            "{}={}",
                                            pretty_try!(
                                                ind,
                                                cp.get_utf8(&param.element_name_index)
                                            ),
                                            pretty_try!(ind, param.value.get_pretty_value(cp))
                                        )?;
                                        Ok(())
                                    })?;
                                }
                                writeln!(ind, ")")?;
                            } else {
                                writeln!(ind)?;
                            }
                            Ok(())
                        })?;
                    }
                    Ok(())
                })?
            }
            SharedAttribute::RuntimeInvisibleAnnotations => unimplemented!(),
            SharedAttribute::RuntimeVisibleTypeAnnotations => unimplemented!(),
            SharedAttribute::RuntimeInvisibleTypeAnnotations => unimplemented!(),
        }

        Ok(())
    }
}

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.7.16
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

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.7.16
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
    pub(crate) fn read(cursor: &mut ByteCursor<'a>) -> Result<Self, ClassFormatErr> {
        let raw_tag = cursor.u8()?;
        let tag = ElementTag::try_from_primitive(raw_tag)
            .map_err(|_| ClassFormatErr::UnknownTag(raw_tag))?;

        let ev = match tag {
            ElementTag::Byte => Self::Byte(cursor.u16()?),
            ElementTag::Char => Self::Char(cursor.u16()?),
            ElementTag::Double => Self::Double(cursor.u16()?),
            ElementTag::Float => Self::Float(cursor.u16()?),
            ElementTag::Int => Self::Int(cursor.u16()?),
            ElementTag::Long => Self::Long(cursor.u16()?),
            ElementTag::Short => Self::Short(cursor.u16()?),
            ElementTag::Boolean => Self::Boolean(cursor.u16()?),
            ElementTag::String => Self::String(cursor.u16()?),
            ElementTag::EnumClass => Self::EnumConstValue {
                type_name_index: cursor.u16()?,
                const_name_index: cursor.u16()?,
            },
            ElementTag::Class => Self::Class(cursor.u16()?),
            ElementTag::AnnotationInterface => Self::AnnotationValue(Annotation::read(cursor)?),
            ElementTag::ArrayType => {
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

    #[cfg(feature = "pretty_print")]
    pub fn get_pretty_descriptor(&self) -> String {
        match self {
            ElementValue::Boolean(v) => format!("Z#{}", v),
            ElementValue::String(v) => format!("s#{}", v),
            _ => unimplemented!(),
        }
    }

    #[cfg(feature = "pretty_print")]
    pub(crate) fn get_pretty_value(
        &self,
        cp: &crate::constant::pool::ConstantPool,
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

impl AttributeType {
    const ATTR_CONSTANT_VALUE: &'static str = "ConstantValue";
    const ATTR_CODE: &'static str = "Code";
    const ATTR_EXCEPTIONS: &'static str = "Exceptions";
    const ATTR_SOURCE_FILE: &'static str = "SourceFile";
    const ATTR_LINE_NUMBER_TABLE: &'static str = "LineNumberTable";
    const ATTR_LOCAL_VARIABLE_TABLE: &'static str = "LocalVariableTable";
    const ATTR_INNER_CLASSES: &'static str = "InnerClasses";
    const ATTR_SYNTHETIC: &'static str = "Synthetic";
    const ATTR_DEPRECATED: &'static str = "Deprecated";
    const ATTR_ENCLOSING_METHOD: &'static str = "EnclosingMethod";
    const ATTR_SIGNATURE: &'static str = "Signature";
    const ATTR_SOURCE_DEBUG_EXTENSION: &'static str = "SourceDebugExtension";
    const ATTR_LOCAL_VARIABLE_TYPE_TABLE: &'static str = "LocalVariableTypeTable";
    const ATTR_RUNTIME_VISIBLE_ANNOTATIONS: &'static str = "RuntimeVisibleAnnotations";
    const ATTR_RUNTIME_INVISIBLE_ANNOTATIONS: &'static str = "RuntimeInvisibleAnnotations";
    const ATTR_RUNTIME_VISIBLE_PARAMETER_ANNOTATIONS: &'static str =
        "RuntimeVisibleParameterAnnotations";
    const ATTR_RUNTIME_INVISIBLE_PARAMETER_ANNOTATIONS: &'static str =
        "RuntimeInvisibleParameterAnnotations";
    const ATTR_ANNOTATION_DEFAULT: &'static str = "AnnotationDefault";
    const ATTR_STACK_MAP_TABLE: &'static str = "StackMapTable";
    const ATTR_BOOTSTRAP_METHODS: &'static str = "BootstrapMethods";
    const ATTR_RUNTIME_VISIBLE_TYPE_ANNOTATIONS: &'static str = "RuntimeVisibleTypeAnnotations";
    const ATTR_RUNTIME_INVISIBLE_TYPE_ANNOTATIONS: &'static str = "RuntimeInvisibleTypeAnnotations";
    const ATTR_METHOD_PARAMETERS: &'static str = "MethodParameters";
    const ATTR_MODULE: &'static str = "Module";
    const ATTR_MODULE_PACKAGES: &'static str = "ModulePackages";
    const ATTR_MODULE_MAIN_CLASS: &'static str = "ModuleMainClass";
    const ATTR_NEST_HOST: &'static str = "NestHost";
    const ATTR_NEST_MEMBERS: &'static str = "NestMembers";
    const ATTR_RECORD: &'static str = "Record";
    const ATTR_PERMITTED_SUBCLASSES: &'static str = "PermittedSubclasses";

    #[inline]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::ConstantValue => Self::ATTR_CONSTANT_VALUE,
            Self::Code => Self::ATTR_CODE,
            Self::Exceptions => Self::ATTR_EXCEPTIONS,
            Self::SourceFile => Self::ATTR_SOURCE_FILE,
            Self::LineNumberTable => Self::ATTR_LINE_NUMBER_TABLE,
            Self::LocalVariableTable => Self::ATTR_LOCAL_VARIABLE_TABLE,
            Self::InnerClasses => Self::ATTR_INNER_CLASSES,
            Self::Synthetic => Self::ATTR_SYNTHETIC,
            Self::Deprecated => Self::ATTR_DEPRECATED,
            Self::EnclosingMethod => Self::ATTR_ENCLOSING_METHOD,
            Self::Signature => Self::ATTR_SIGNATURE,
            Self::SourceDebugExtension => Self::ATTR_SOURCE_DEBUG_EXTENSION,
            Self::LocalVariableTypeTable => Self::ATTR_LOCAL_VARIABLE_TYPE_TABLE,
            Self::RuntimeVisibleAnnotations => Self::ATTR_RUNTIME_VISIBLE_ANNOTATIONS,
            Self::RuntimeInvisibleAnnotations => Self::ATTR_RUNTIME_INVISIBLE_ANNOTATIONS,
            Self::RuntimeVisibleParameterAnnotations => {
                Self::ATTR_RUNTIME_VISIBLE_PARAMETER_ANNOTATIONS
            }
            Self::RuntimeInvisibleParameterAnnotations => {
                Self::ATTR_RUNTIME_INVISIBLE_PARAMETER_ANNOTATIONS
            }
            Self::AnnotationDefault => Self::ATTR_ANNOTATION_DEFAULT,
            Self::StackMapTable => Self::ATTR_STACK_MAP_TABLE,
            Self::BootstrapMethods => Self::ATTR_BOOTSTRAP_METHODS,
            Self::RuntimeVisibleTypeAnnotations => Self::ATTR_RUNTIME_VISIBLE_TYPE_ANNOTATIONS,
            Self::RuntimeInvisibleTypeAnnotations => Self::ATTR_RUNTIME_INVISIBLE_TYPE_ANNOTATIONS,
            Self::MethodParameters => Self::ATTR_METHOD_PARAMETERS,
            Self::Module => Self::ATTR_MODULE,
            Self::ModulePackages => Self::ATTR_MODULE_PACKAGES,
            Self::ModuleMainClass => Self::ATTR_MODULE_MAIN_CLASS,
            Self::NestHost => Self::ATTR_NEST_HOST,
            Self::NestMembers => Self::ATTR_NEST_MEMBERS,
            Self::Record => Self::ATTR_RECORD,
            Self::PermittedSubclasses => Self::ATTR_PERMITTED_SUBCLASSES,
        }
    }
}

impl TryFrom<&str> for AttributeType {
    type Error = ClassFormatErr;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(match s {
            Self::ATTR_CONSTANT_VALUE => Self::ConstantValue,
            Self::ATTR_CODE => Self::Code,
            Self::ATTR_EXCEPTIONS => Self::Exceptions,
            Self::ATTR_SOURCE_FILE => Self::SourceFile,
            Self::ATTR_LINE_NUMBER_TABLE => Self::LineNumberTable,
            Self::ATTR_LOCAL_VARIABLE_TABLE => Self::LocalVariableTable,
            Self::ATTR_INNER_CLASSES => Self::InnerClasses,
            Self::ATTR_SYNTHETIC => Self::Synthetic,
            Self::ATTR_DEPRECATED => Self::Deprecated,
            Self::ATTR_ENCLOSING_METHOD => Self::EnclosingMethod,
            Self::ATTR_SIGNATURE => Self::Signature,
            Self::ATTR_SOURCE_DEBUG_EXTENSION => Self::SourceDebugExtension,
            Self::ATTR_LOCAL_VARIABLE_TYPE_TABLE => Self::LocalVariableTypeTable,
            Self::ATTR_RUNTIME_VISIBLE_ANNOTATIONS => Self::RuntimeVisibleAnnotations,
            Self::ATTR_RUNTIME_INVISIBLE_ANNOTATIONS => Self::RuntimeInvisibleAnnotations,
            Self::ATTR_RUNTIME_VISIBLE_PARAMETER_ANNOTATIONS => {
                Self::RuntimeVisibleParameterAnnotations
            }
            Self::ATTR_RUNTIME_INVISIBLE_PARAMETER_ANNOTATIONS => {
                Self::RuntimeInvisibleParameterAnnotations
            }
            Self::ATTR_ANNOTATION_DEFAULT => Self::AnnotationDefault,
            Self::ATTR_STACK_MAP_TABLE => Self::StackMapTable,
            Self::ATTR_BOOTSTRAP_METHODS => Self::BootstrapMethods,
            Self::ATTR_RUNTIME_VISIBLE_TYPE_ANNOTATIONS => Self::RuntimeVisibleTypeAnnotations,
            Self::ATTR_RUNTIME_INVISIBLE_TYPE_ANNOTATIONS => Self::RuntimeInvisibleTypeAnnotations,
            Self::ATTR_METHOD_PARAMETERS => Self::MethodParameters,
            Self::ATTR_MODULE => Self::Module,
            Self::ATTR_MODULE_PACKAGES => Self::ModulePackages,
            Self::ATTR_MODULE_MAIN_CLASS => Self::ModuleMainClass,
            Self::ATTR_NEST_HOST => Self::NestHost,
            Self::ATTR_NEST_MEMBERS => Self::NestMembers,
            Self::ATTR_RECORD => Self::Record,
            Self::ATTR_PERMITTED_SUBCLASSES => Self::PermittedSubclasses,
            _ => return Err(ClassFormatErr::UnknownAttribute(s.to_string())),
        })
    }
}

impl fmt::Display for AttributeType {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
