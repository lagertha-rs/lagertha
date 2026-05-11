//! Shared attribute types that can appear at multiple locations in a class file.
//!
//! These attributes can be attached to classes, fields, methods, and record components.

use super::AttributeKind;
use super::annotation::Annotation;
use super::type_annotation::TypeAnnotation;
use crate::ClassFormatErr;
use common::utils::cursor::ByteCursor;

/// Attribute payloads that can appear at multiple locations (class, field, method, record).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SharedAttribute {
    Synthetic,
    Deprecated,
    Signature(u16),
    RuntimeVisibleAnnotations(Vec<Annotation>),
    RuntimeInvisibleAnnotations(Vec<Annotation>),
    RuntimeVisibleTypeAnnotations(Vec<TypeAnnotation>),
    RuntimeInvisibleTypeAnnotations(Vec<TypeAnnotation>),
}

impl<'a> SharedAttribute {
    pub(crate) fn read(
        attr_type: AttributeKind,
        cursor: &mut ByteCursor<'a>,
    ) -> Result<Self, ClassFormatErr> {
        match attr_type {
            AttributeKind::Synthetic => Ok(SharedAttribute::Synthetic),
            AttributeKind::Deprecated => Ok(SharedAttribute::Deprecated),
            AttributeKind::Signature => {
                let signature_index = cursor.u16()?;
                Ok(SharedAttribute::Signature(signature_index))
            }
            AttributeKind::RuntimeVisibleAnnotations => {
                let num_annotations = cursor.u16()?;
                let mut annotations = Vec::with_capacity(num_annotations as usize);
                for _ in 0..num_annotations {
                    annotations.push(Annotation::read(cursor)?);
                }
                Ok(SharedAttribute::RuntimeVisibleAnnotations(annotations))
            }
            AttributeKind::RuntimeInvisibleAnnotations => {
                let num_annotations = cursor.u16()?;
                let mut annotations = Vec::with_capacity(num_annotations as usize);
                for _ in 0..num_annotations {
                    annotations.push(Annotation::read(cursor)?);
                }
                Ok(SharedAttribute::RuntimeInvisibleAnnotations(annotations))
            }
            AttributeKind::RuntimeInvisibleTypeAnnotations => {
                let num_annotations = cursor.u16()?;
                let mut annotations = Vec::with_capacity(num_annotations as usize);
                for _ in 0..num_annotations {
                    annotations.push(TypeAnnotation::read(cursor)?);
                }
                Ok(SharedAttribute::RuntimeInvisibleTypeAnnotations(
                    annotations,
                ))
            }
            AttributeKind::RuntimeVisibleTypeAnnotations => {
                let num_annotations = cursor.u16()?;
                let mut annotations = Vec::with_capacity(num_annotations as usize);
                for _ in 0..num_annotations {
                    annotations.push(TypeAnnotation::read(cursor)?);
                }
                Ok(SharedAttribute::RuntimeVisibleTypeAnnotations(annotations))
            }
            _ => Err(ClassFormatErr::AttributeIsNotShared(attr_type.to_string())),
        }
    }
}
