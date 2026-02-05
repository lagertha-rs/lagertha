//! Attribute types for class files.
//!
//! https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7

use crate::ClassFormatErr;
use core::fmt;
use std::fmt::Formatter;

mod annotation;
mod class;
mod field;
pub mod method;
mod shared;
mod type_annotation;

pub use annotation::{Annotation, ElementKind, ElementValue, ElementValuePair};
pub use class::{BootstrapMethodEntry, ClassAttribute, InnerClassEntry};
pub use field::FieldAttribute;
pub use method::{
    CodeAttribute, ExceptionTableEntry, MethodAttribute, MethodParameterEntry, ParameterAnnotations,
};
pub use shared::SharedAttribute;
pub use type_annotation::{LocalVarEntry, TargetInfo, TypeAnnotation, TypePath, TypePathEntry};

/// Discriminant for attribute types defined in the JVM specification.
///
/// https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttributeKind {
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

impl AttributeKind {
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

impl TryFrom<&str> for AttributeKind {
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

impl fmt::Display for AttributeKind {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// Re-export the old name for backwards compatibility during transition
#[doc(hidden)]
#[deprecated(note = "Use AttributeKind instead")]
pub type AttributeType = AttributeKind;
