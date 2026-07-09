//! Shared attribute types that can appear at multiple locations in a class file.
//!
//! These attributes can be attached to classes, fields, methods, and record components.

use super::AttributeKind;
use super::annotation::Annotation;
use super::type_annotation::TypeAnnotation;
use crate::ClassFormatErr;
use lvm_common::utils::cursor::ByteCursor;

/// Attribute payloads that can appear at multiple locations (class, field, method, record).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SharedAttribute {
    Synthetic {
        attr_name_idx: u16,
    },
    Deprecated {
        attr_name_idx: u16,
    },
    Signature {
        attr_name_idx: u16,
        signature_idx: u16,
    },
    RuntimeVisibleAnnotations {
        attr_name_idx: u16,
        annotations: Vec<Annotation>,
    },
    RuntimeInvisibleAnnotations {
        attr_name_idx: u16,
        annotations: Vec<Annotation>,
    },
    RuntimeVisibleTypeAnnotations {
        attr_name_idx: u16,
        annotations: Vec<TypeAnnotation>,
    },
    RuntimeInvisibleTypeAnnotations {
        attr_name_idx: u16,
        annotations: Vec<TypeAnnotation>,
    },
}

impl SharedAttribute {
    pub fn kind(&self) -> AttributeKind {
        match self {
            Self::Synthetic { .. } => AttributeKind::Synthetic,
            Self::Deprecated { .. } => AttributeKind::Deprecated,
            Self::Signature { .. } => AttributeKind::Signature,
            Self::RuntimeVisibleAnnotations { .. } => AttributeKind::RuntimeVisibleAnnotations,
            Self::RuntimeInvisibleAnnotations { .. } => AttributeKind::RuntimeInvisibleAnnotations,
            Self::RuntimeVisibleTypeAnnotations { .. } => {
                AttributeKind::RuntimeVisibleTypeAnnotations
            }
            Self::RuntimeInvisibleTypeAnnotations { .. } => {
                AttributeKind::RuntimeInvisibleTypeAnnotations
            }
        }
    }
}

impl<'a> SharedAttribute {
    pub(crate) fn read(
        attr_name_idx: u16,
        attr_type: AttributeKind,
        cursor: &mut ByteCursor<'a>,
    ) -> Result<Self, ClassFormatErr> {
        match attr_type {
            AttributeKind::Synthetic => Ok(SharedAttribute::Synthetic { attr_name_idx }),
            AttributeKind::Deprecated => Ok(SharedAttribute::Deprecated { attr_name_idx }),
            AttributeKind::Signature => {
                let signature_idx = cursor.u16()?;
                Ok(SharedAttribute::Signature {
                    attr_name_idx,
                    signature_idx,
                })
            }
            AttributeKind::RuntimeVisibleAnnotations => {
                let num_annotations = cursor.u16()?;
                let mut annotations = Vec::with_capacity(num_annotations as usize);
                for _ in 0..num_annotations {
                    annotations.push(Annotation::read(cursor)?);
                }
                Ok(SharedAttribute::RuntimeVisibleAnnotations {
                    attr_name_idx,
                    annotations,
                })
            }
            AttributeKind::RuntimeInvisibleAnnotations => {
                let num_annotations = cursor.u16()?;
                let mut annotations = Vec::with_capacity(num_annotations as usize);
                for _ in 0..num_annotations {
                    annotations.push(Annotation::read(cursor)?);
                }
                Ok(SharedAttribute::RuntimeInvisibleAnnotations {
                    attr_name_idx,
                    annotations,
                })
            }
            AttributeKind::RuntimeInvisibleTypeAnnotations => {
                let num_annotations = cursor.u16()?;
                let mut annotations = Vec::with_capacity(num_annotations as usize);
                for _ in 0..num_annotations {
                    annotations.push(TypeAnnotation::read(cursor)?);
                }
                Ok(SharedAttribute::RuntimeInvisibleTypeAnnotations {
                    attr_name_idx,
                    annotations,
                })
            }
            AttributeKind::RuntimeVisibleTypeAnnotations => {
                let num_annotations = cursor.u16()?;
                let mut annotations = Vec::with_capacity(num_annotations as usize);
                for _ in 0..num_annotations {
                    annotations.push(TypeAnnotation::read(cursor)?);
                }
                Ok(SharedAttribute::RuntimeVisibleTypeAnnotations {
                    attr_name_idx,
                    annotations,
                })
            }
            _ => Err(ClassFormatErr::AttributeIsNotShared(attr_type.to_string())),
        }
    }
}
