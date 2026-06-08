//! Convenient re-exports of commonly used types.
//!
//! This module provides a single import point for the most frequently used types
//! in the jclass crate.
//!
//! # Usage
//!
//! ```
//! use lvm_class::prelude::*;
//! ```

// Core class file structure
pub use crate::ClassFile;

// Constant pool
pub use crate::constant_pool::{ConstantEntry, ConstantKind, ConstantPool};
pub use crate::constant_pool::{Dynamic, MethodHandle, MethodHandleKind, NameAndType, Reference};
#[cfg(feature = "write")]
pub use crate::write::builder::ConstantPoolBuilder;
#[cfg(feature = "write")]
pub use crate::write::class_file::ClassFileBuilder;

// Members
pub use crate::member::{FieldInfo, MethodInfo};

// Bytecode
pub use crate::bytecode::{ArrayType, Instruction, LookupSwitchData, Opcode, TableSwitchData};

// Attributes
pub use crate::attribute::{
    Annotation, AttributeKind, ClassAttribute, CodeAttribute, ElementKind, ElementValue,
    ElementValuePair, FieldAttribute, LocalVarEntry, MethodAttribute, SharedAttribute, TargetInfo,
    TypeAnnotation, TypePath, TypePathEntry,
};
// Flags
pub use crate::flags::{ClassFlags, FieldFlags, InnerClassFlags, MethodFlags, MethodParamFlags};
