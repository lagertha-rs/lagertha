//! Shared attribute types that can appear at multiple locations in a class file.
//!
//! These attributes can be attached to classes, fields, methods, and record components.

use super::annotation::Annotation;
use super::type_annotation::TypeAnnotation;
use super::AttributeKind;
use crate::ClassFormatErr;
use common::utils::cursor::ByteCursor;

#[cfg(feature = "javap_print")]
use super::type_annotation::TargetInfo;
#[cfg(feature = "javap_print")]
use std::fmt;

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

    #[cfg(feature = "javap_print")]
    fn fmt_annotations(
        ind: &mut common::utils::indent_write::Indented<'_>,
        cp: &crate::constant_pool::ConstantPool,
        annotations: &[Annotation],
    ) -> fmt::Result {
        use common::try_javap_print;
        use common::try_javap_print_class_name;
        use itertools::Itertools;
        use std::fmt::Write as _;

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
                        pair.value.get_javap_descriptor()
                    ))
                    .join(",")
            )?;
            ind.with_indent(|ind| {
                write!(
                    ind,
                    "{}",
                    try_javap_print_class_name!(ind, cp.get_utf8(&annotation.type_index))
                )?;
                if !annotation.element_value_pairs.is_empty() {
                    writeln!(ind, "(")?;
                    for param in &annotation.element_value_pairs {
                        ind.with_indent(|ind| {
                            writeln!(
                                ind,
                                "{}={}",
                                try_javap_print!(ind, cp.get_utf8(&param.element_name_index)),
                                try_javap_print!(ind, param.value.get_javap_value(cp))
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
    }

    #[cfg(feature = "javap_print")]
    pub(crate) fn javap_fmt(
        &self,
        ind: &mut common::utils::indent_write::Indented<'_>,
        cp: &crate::constant_pool::ConstantPool,
    ) -> fmt::Result {
        use common::try_javap_print;
        use std::fmt::Write as _;

        match self {
            SharedAttribute::Synthetic => unimplemented!(),
            SharedAttribute::Deprecated => writeln!(ind, "Deprecated: true")?,
            SharedAttribute::Signature(index) => writeln!(
                ind,
                "Signature: #{:<26} // {}",
                index,
                try_javap_print!(ind, cp.get_utf8(index)),
            )?,
            SharedAttribute::RuntimeVisibleAnnotations(annotations) => {
                writeln!(ind, "RuntimeVisibleAnnotations:")?;
                ind.with_indent(|ind| Self::fmt_annotations(ind, cp, annotations))?
            }
            SharedAttribute::RuntimeInvisibleAnnotations(annotations) => {
                writeln!(ind, "RuntimeInvisibleAnnotations:")?;
                ind.with_indent(|ind| Self::fmt_annotations(ind, cp, annotations))?
            }
            SharedAttribute::RuntimeVisibleTypeAnnotations(annotations) => {
                writeln!(ind, "RuntimeVisibleTypeAnnotations:")?;
                ind.with_indent(|ind| Self::fmt_type_annotations(ind, cp, annotations))?
            }
            SharedAttribute::RuntimeInvisibleTypeAnnotations(annotations) => {
                writeln!(ind, "RuntimeInvisibleTypeAnnotations:")?;
                ind.with_indent(|ind| Self::fmt_type_annotations(ind, cp, annotations))?
            }
        }

        Ok(())
    }

    #[cfg(feature = "javap_print")]
    fn fmt_type_annotations(
        ind: &mut common::utils::indent_write::Indented<'_>,
        cp: &crate::constant_pool::ConstantPool,
        annotations: &[TypeAnnotation],
    ) -> fmt::Result {
        use common::try_javap_print_class_name;
        use itertools::Itertools;
        use std::fmt::Write as _;

        for (i, annotation) in annotations.iter().enumerate() {
            // Format target info
            let target_str = match &annotation.target_info {
                TargetInfo::TypeParameter {
                    type_parameter_index,
                } => format!("CLASS_TYPE_PARAMETER, param_index={type_parameter_index}"),
                TargetInfo::Supertype { supertype_index } => {
                    format!("CLASS_EXTENDS, type_index={supertype_index}")
                }
                TargetInfo::TypeParameterBound {
                    type_parameter_index,
                    bound_index,
                } => format!(
                    "CLASS_TYPE_PARAMETER_BOUND, param_index={type_parameter_index}, bound_index={bound_index}"
                ),
                TargetInfo::Empty => "EMPTY".to_string(),
                TargetInfo::MethodFormalParameter {
                    formal_parameter_index,
                } => format!("METHOD_FORMAL_PARAMETER, param_index={formal_parameter_index}"),
                TargetInfo::Throws { throws_type_index } => {
                    format!("THROWS, type_index={throws_type_index}")
                }
                TargetInfo::LocalVar { localvar_table } => {
                    let entries = localvar_table
                        .iter()
                        .map(|e| {
                            format!(
                                "start_pc={}, length={}, index={}",
                                e.start_pc, e.length, e.index
                            )
                        })
                        .join("; ");
                    format!("LOCAL_VARIABLE, {{{entries}}}")
                }
                TargetInfo::Catch {
                    exception_table_index,
                } => format!("EXCEPTION_PARAMETER, exception_index={exception_table_index}"),
                TargetInfo::Offset { offset } => format!("OFFSET, offset={offset}"),
                TargetInfo::TypeArgument {
                    offset,
                    type_argument_index,
                } => format!(
                    "TYPE_ARGUMENT, offset={offset}, type_argument_index={type_argument_index}"
                ),
            };

            // Format element value pairs
            let pairs_str = if annotation.element_value_pairs.is_empty() {
                String::new()
            } else {
                annotation
                    .element_value_pairs
                    .iter()
                    .map(|pair| {
                        format!(
                            "#{}={}",
                            pair.element_name_index,
                            pair.value.get_javap_descriptor()
                        )
                    })
                    .join(",")
            };

            writeln!(
                ind,
                "{i}: #{}({pairs_str}): {target_str}",
                annotation.type_index
            )?;
            ind.with_indent(|ind| {
                let type_name =
                    try_javap_print_class_name!(ind, cp.get_utf8(&annotation.type_index));
                writeln!(ind, "{type_name}")?;
                Ok(())
            })?;
        }
        Ok(())
    }
}
