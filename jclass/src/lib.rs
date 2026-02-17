//! Complete Java 25 `.class` file parser.
//!
//! This crate provides structured representation of Java class files with parsing,
//! validation, and javap-style printing capabilities.

use crate::attribute::ClassAttribute;
use crate::constant_pool::{ConstantEntry, ConstantPool};
use crate::flags::ClassFlags;
use crate::member::{FieldInfo, MethodInfo};
use common::error::ClassFormatErr;
use common::utils::cursor::ByteCursor;

pub mod attribute;
pub mod bytecode;
pub mod constant_pool;
pub mod flags;
#[cfg(feature = "jasm_assemble")]
pub mod jasm_asm;
#[cfg(feature = "jasm_disassemble")]
pub mod jasm_disas;
#[cfg(feature = "javap_print")]
pub mod javap_fmt;
pub mod member;
pub mod prelude;

// TODO: review all access levels in the crate (methods, fields, modules, structs, etc.)

/// A Rust representation of a Java `.class` file.
///
/// https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html
///
/// All structures in the crate have public fields for easier access,
/// because they will be remapped to runtime structures.
///
/// All print related code is behind the `javap_print` feature flag.
#[derive(Debug)]
pub struct ClassFile {
    pub minor_version: u16,
    pub major_version: u16,
    pub cp: ConstantPool,
    pub access_flags: ClassFlags,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces: Vec<u16>,
    pub fields: Vec<FieldInfo>,
    pub methods: Vec<MethodInfo>,
    pub attributes: Vec<ClassAttribute>,
    #[cfg(feature = "jasm_assemble")]
    pub attribute_names: jasm_asm::AttributeNameMap,
}

impl ClassFile {
    const MAGIC: u32 = 0xCAFEBABE;
    fn validate_magic(val: u32) -> Result<(), ClassFormatErr> {
        (val == ClassFile::MAGIC)
            .then_some(())
            .ok_or(ClassFormatErr::WrongMagic(val))
    }

    pub fn get_super_class_name(&self) -> Option<Result<&str, ClassFormatErr>> {
        if self.super_class == 0 {
            None
        } else {
            Some(self.cp.get_class_name(&self.super_class))
        }
    }
}

impl TryFrom<Vec<u8>> for ClassFile {
    type Error = ClassFormatErr;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let mut cursor = ByteCursor::new(&value);
        let magic = cursor.u32()?;
        ClassFile::validate_magic(magic)?;
        let minor_version = cursor.u16()?;
        let major_version = cursor.u16()?;
        let constant_pool_count = cursor.u16()?;
        let mut constant_pool = Vec::with_capacity((constant_pool_count + 1) as usize);
        constant_pool.push(ConstantEntry::Unused);
        let mut i = 1;
        while i < constant_pool_count {
            let constant = ConstantEntry::read(&mut cursor)?;
            constant_pool.push(constant.clone());
            match constant {
                // described in JVM spec that Long and Double take two entries in the constant pool
                // https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.5
                ConstantEntry::Long(_) | ConstantEntry::Double(_) => {
                    constant_pool.push(ConstantEntry::Unused);
                    i += 2;
                }
                _ => {
                    i += 1;
                }
            }
        }
        let constant_pool = ConstantPool {
            inner: constant_pool,
        };

        let access_flags = ClassFlags::new(cursor.u16()?);
        let this_class = cursor.u16()?;
        let super_class = cursor.u16()?;
        let interfaces_count = cursor.u16()?;
        let mut interfaces = Vec::with_capacity(interfaces_count as usize);
        for _ in 0..interfaces_count {
            interfaces.push(cursor.u16()?);
        }
        let fields_count = cursor.u16()?;
        let mut fields = Vec::with_capacity(fields_count as usize);
        for _ in 0..fields_count {
            fields.push(FieldInfo::read(&constant_pool, &mut cursor)?);
        }
        let methods_count = cursor.u16()?;
        let mut methods = Vec::with_capacity(methods_count as usize);
        for _ in 0..methods_count {
            methods.push(MethodInfo::read(&constant_pool, &mut cursor)?);
        }
        let attributes_count = cursor.u16()?;
        let mut attributes = Vec::with_capacity(attributes_count as usize);
        for _ in 0..attributes_count {
            attributes.push(ClassAttribute::read(&constant_pool, &mut cursor)?);
        }

        if cursor.u8().is_ok() {
            Err(ClassFormatErr::TrailingBytes)
        } else {
            Ok(Self {
                minor_version,
                major_version,
                cp: constant_pool,
                access_flags,
                this_class,
                super_class,
                interfaces,
                fields,
                methods,
                attributes,
                #[cfg(feature = "jasm_assemble")]
                attribute_names: jasm_asm::AttributeNameMap::new(),
            })
        }
    }
}
