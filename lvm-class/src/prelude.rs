//! Convenient re-exports of commonly used types.
//!
//! # Usage
//!
//! ```
//! use lvm_class::prelude::*;
//! ```

// Constant pool
pub use crate::constant_pool::ConstantEntry;

// Members
pub use crate::member::MethodInfo;

// Bytecode
pub use crate::bytecode::{ArrayType, Instruction, LookupSwitchData, TableSwitchData};

// Attributes
pub use crate::attribute::{CodeAttribute, MethodAttribute};

// Write (feature-gated)
#[cfg(feature = "write")]
pub use crate::write::builder::ConstantPoolBuilder;
#[cfg(feature = "write")]
pub use crate::write::class_file::ClassFileBuilder;

// Flags
pub use crate::flags::MethodFlags;
