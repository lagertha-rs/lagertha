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

pub const ATTR_CONSTANT_VALUE: &'static str = "ConstantValue";
pub const ATTR_CODE: &'static str = "Code";
pub const ATTR_EXCEPTIONS: &'static str = "Exceptions";
pub const ATTR_SOURCE_FILE: &'static str = "SourceFile";
pub const ATTR_LINE_NUMBER_TABLE: &'static str = "LineNumberTable";
pub const ATTR_LOCAL_VARIABLE_TABLE: &'static str = "LocalVariableTable";
pub const ATTR_INNER_CLASSES: &'static str = "InnerClasses";
pub const ATTR_SYNTHETIC: &'static str = "Synthetic";
pub const ATTR_DEPRECATED: &'static str = "Deprecated";
pub const ATTR_ENCLOSING_METHOD: &'static str = "EnclosingMethod";
pub const ATTR_SIGNATURE: &'static str = "Signature";
pub const ATTR_SOURCE_DEBUG_EXTENSION: &'static str = "SourceDebugExtension";
pub const ATTR_LOCAL_VARIABLE_TYPE_TABLE: &'static str = "LocalVariableTypeTable";
pub const ATTR_RUNTIME_VISIBLE_ANNOTATIONS: &'static str = "RuntimeVisibleAnnotations";
pub const ATTR_RUNTIME_INVISIBLE_ANNOTATIONS: &'static str = "RuntimeInvisibleAnnotations";
pub const ATTR_RUNTIME_VISIBLE_PARAMETER_ANNOTATIONS: &'static str =
    "RuntimeVisibleParameterAnnotations";
pub const ATTR_RUNTIME_INVISIBLE_PARAMETER_ANNOTATIONS: &'static str =
    "RuntimeInvisibleParameterAnnotations";
pub const ATTR_ANNOTATION_DEFAULT: &'static str = "AnnotationDefault";
pub const ATTR_STACK_MAP_TABLE: &'static str = "StackMapTable";
pub const ATTR_BOOTSTRAP_METHODS: &'static str = "BootstrapMethods";
pub const ATTR_RUNTIME_VISIBLE_TYPE_ANNOTATIONS: &'static str = "RuntimeVisibleTypeAnnotations";
pub const ATTR_RUNTIME_INVISIBLE_TYPE_ANNOTATIONS: &'static str = "RuntimeInvisibleTypeAnnotations";
pub const ATTR_METHOD_PARAMETERS: &'static str = "MethodParameters";
pub const ATTR_MODULE: &'static str = "Module";
pub const ATTR_MODULE_PACKAGES: &'static str = "ModulePackages";
pub const ATTR_MODULE_MAIN_CLASS: &'static str = "ModuleMainClass";
pub const ATTR_NEST_HOST: &'static str = "NestHost";
pub const ATTR_NEST_MEMBERS: &'static str = "NestMembers";
pub const ATTR_RECORD: &'static str = "Record";
pub const ATTR_PERMITTED_SUBCLASSES: &'static str = "PermittedSubclasses";

/// Discriminant for attribute types defined in the JVM specification.
///
/// https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    #[inline]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::ConstantValue => ATTR_CONSTANT_VALUE,
            Self::Code => ATTR_CODE,
            Self::Exceptions => ATTR_EXCEPTIONS,
            Self::SourceFile => ATTR_SOURCE_FILE,
            Self::LineNumberTable => ATTR_LINE_NUMBER_TABLE,
            Self::LocalVariableTable => ATTR_LOCAL_VARIABLE_TABLE,
            Self::InnerClasses => ATTR_INNER_CLASSES,
            Self::Synthetic => ATTR_SYNTHETIC,
            Self::Deprecated => ATTR_DEPRECATED,
            Self::EnclosingMethod => ATTR_ENCLOSING_METHOD,
            Self::Signature => ATTR_SIGNATURE,
            Self::SourceDebugExtension => ATTR_SOURCE_DEBUG_EXTENSION,
            Self::LocalVariableTypeTable => ATTR_LOCAL_VARIABLE_TYPE_TABLE,
            Self::RuntimeVisibleAnnotations => ATTR_RUNTIME_VISIBLE_ANNOTATIONS,
            Self::RuntimeInvisibleAnnotations => ATTR_RUNTIME_INVISIBLE_ANNOTATIONS,
            Self::RuntimeVisibleParameterAnnotations => ATTR_RUNTIME_VISIBLE_PARAMETER_ANNOTATIONS,
            Self::RuntimeInvisibleParameterAnnotations => {
                ATTR_RUNTIME_INVISIBLE_PARAMETER_ANNOTATIONS
            }
            Self::AnnotationDefault => ATTR_ANNOTATION_DEFAULT,
            Self::StackMapTable => ATTR_STACK_MAP_TABLE,
            Self::BootstrapMethods => ATTR_BOOTSTRAP_METHODS,
            Self::RuntimeVisibleTypeAnnotations => ATTR_RUNTIME_VISIBLE_TYPE_ANNOTATIONS,
            Self::RuntimeInvisibleTypeAnnotations => ATTR_RUNTIME_INVISIBLE_TYPE_ANNOTATIONS,
            Self::MethodParameters => ATTR_METHOD_PARAMETERS,
            Self::Module => ATTR_MODULE,
            Self::ModulePackages => ATTR_MODULE_PACKAGES,
            Self::ModuleMainClass => ATTR_MODULE_MAIN_CLASS,
            Self::NestHost => ATTR_NEST_HOST,
            Self::NestMembers => ATTR_NEST_MEMBERS,
            Self::Record => ATTR_RECORD,
            Self::PermittedSubclasses => ATTR_PERMITTED_SUBCLASSES,
        }
    }
}

impl TryFrom<&str> for AttributeKind {
    type Error = ClassFormatErr;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(match s {
            ATTR_CONSTANT_VALUE => Self::ConstantValue,
            ATTR_CODE => Self::Code,
            ATTR_EXCEPTIONS => Self::Exceptions,
            ATTR_SOURCE_FILE => Self::SourceFile,
            ATTR_LINE_NUMBER_TABLE => Self::LineNumberTable,
            ATTR_LOCAL_VARIABLE_TABLE => Self::LocalVariableTable,
            ATTR_INNER_CLASSES => Self::InnerClasses,
            ATTR_SYNTHETIC => Self::Synthetic,
            ATTR_DEPRECATED => Self::Deprecated,
            ATTR_ENCLOSING_METHOD => Self::EnclosingMethod,
            ATTR_SIGNATURE => Self::Signature,
            ATTR_SOURCE_DEBUG_EXTENSION => Self::SourceDebugExtension,
            ATTR_LOCAL_VARIABLE_TYPE_TABLE => Self::LocalVariableTypeTable,
            ATTR_RUNTIME_VISIBLE_ANNOTATIONS => Self::RuntimeVisibleAnnotations,
            ATTR_RUNTIME_INVISIBLE_ANNOTATIONS => Self::RuntimeInvisibleAnnotations,
            ATTR_RUNTIME_VISIBLE_PARAMETER_ANNOTATIONS => Self::RuntimeVisibleParameterAnnotations,
            ATTR_RUNTIME_INVISIBLE_PARAMETER_ANNOTATIONS => {
                Self::RuntimeInvisibleParameterAnnotations
            }
            ATTR_ANNOTATION_DEFAULT => Self::AnnotationDefault,
            ATTR_STACK_MAP_TABLE => Self::StackMapTable,
            ATTR_BOOTSTRAP_METHODS => Self::BootstrapMethods,
            ATTR_RUNTIME_VISIBLE_TYPE_ANNOTATIONS => Self::RuntimeVisibleTypeAnnotations,
            ATTR_RUNTIME_INVISIBLE_TYPE_ANNOTATIONS => Self::RuntimeInvisibleTypeAnnotations,
            ATTR_METHOD_PARAMETERS => Self::MethodParameters,
            ATTR_MODULE => Self::Module,
            ATTR_MODULE_PACKAGES => Self::ModulePackages,
            ATTR_MODULE_MAIN_CLASS => Self::ModuleMainClass,
            ATTR_NEST_HOST => Self::NestHost,
            ATTR_NEST_MEMBERS => Self::NestMembers,
            ATTR_RECORD => Self::Record,
            ATTR_PERMITTED_SUBCLASSES => Self::PermittedSubclasses,
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
