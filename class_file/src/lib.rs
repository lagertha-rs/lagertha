use crate::access::ClassAccessFlag;
use crate::attribute::class::ClassAttribute;
use crate::constant_pool::{ConstantPool, ConstantTag};
use common::cursor::{ByteCursor, CursorError};
#[cfg(feature = "pretty_print")]
use common::indent_write::Indented;
use common::{pretty_class_name_try, pretty_try};
use constant_pool::ConstantInfo;
use field::FieldInfo;
use method::MethodInfo;
#[cfg(feature = "pretty_print")]
use std::fmt::{self, Write as _};
use thiserror::Error;

pub mod access;
pub mod attribute;
pub mod constant_pool;
pub mod field;
pub mod method;

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html
#[derive(Debug)]
pub struct ClassFile {
    pub minor_version: u16,
    pub major_version: u16,
    pub cp: ConstantPool,
    pub access_flags: ClassAccessFlag,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces: Vec<u16>,
    pub fields: Vec<FieldInfo>,
    pub methods: Vec<MethodInfo>,
    pub attributes: Vec<ClassAttribute>,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum DescriptorErr {
    #[error("")]
    ShouldStartWithParentheses,
    #[error("")]
    MissingClosingParenthesis,
    #[error("")]
    UnexpectedEnd,
    #[error("")]
    InvalidType,
    #[error("")]
    TrailingCharacters,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ClassFileErr {
    #[error(transparent)]
    Cursor(#[from] CursorError),
    #[error(transparent)]
    MethodDescriptor(#[from] DescriptorErr),
    #[error("")]
    WrongMagic,
    #[error("")]
    TrailingBytes,
    #[error("")]
    UnknownTag(u8),
    #[error("Expected type `{1}` with index `{0}` but found `{2}`")]
    TypeError(u16, ConstantTag, ConstantTag),
    #[error("Constant with index `{0}` isn't found in constant pool.")]
    ConstantNotFound(u16),
    #[error("Unknown stack frame type {0}.")]
    UnknownStackFrameType(u8),
}

impl ClassFile {
    const MAGIC: u32 = 0xCAFEBABE;
    fn validate_magic(val: u32) -> Result<(), ClassFileErr> {
        (val == ClassFile::MAGIC)
            .then_some(())
            .ok_or(ClassFileErr::WrongMagic)
    }
}

impl TryFrom<Vec<u8>> for ClassFile {
    type Error = ClassFileErr;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let mut cursor = ByteCursor::new(&value);
        let magic = cursor.u32()?;
        ClassFile::validate_magic(magic)?;
        let minor_version = cursor.u16()?;
        let major_version = cursor.u16()?;
        let constant_pool_count = cursor.u16()?;
        let mut constant_pool = Vec::with_capacity((constant_pool_count + 1) as usize);
        constant_pool.push(ConstantInfo::Dummy);
        let mut i = 1;
        while i < constant_pool_count {
            let constant = ConstantInfo::read(&mut cursor)?;
            constant_pool.push(constant.clone());
            match constant {
                ConstantInfo::Long(_) | ConstantInfo::Double(_) => {
                    constant_pool.push(ConstantInfo::Dummy);
                    i += 2;
                }
                _ => {
                    i += 1;
                }
            }
        }
        let constant_pool = ConstantPool { cp: constant_pool };

        let access_flags = ClassAccessFlag::new(cursor.u16()?);
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
            Err(ClassFileErr::TrailingBytes)
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
            })
        }
    }
}

#[cfg(feature = "pretty_print")]
impl fmt::Display for ClassFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let constant_kind_width = 16;
        let mut ind = Indented::new(f);
        writeln!(
            ind,
            "{} {}",
            self.access_flags.get_pretty_java_like_prefix(),
            pretty_class_name_try!(ind, self.cp.get_class_name(self.this_class))
        )?;
        ind.with_indent(|ind| {
            writeln!(ind, "minor version: {}", self.minor_version)?;
            writeln!(ind, "major version: {}", self.major_version)?;
            writeln!(
                ind,
                "flags: (0x{:04X}) {}",
                self.access_flags.get_raw(),
                self.access_flags.get_javap_like_list()
            )?;
            writeln!(
                ind,
                "this_class: {:<24}//{}",
                format!("#{}", self.this_class),
                pretty_try!(ind, self.cp.get_class_name(self.this_class))
            )?;
            writeln!(ind, "super_class: #{}", self.super_class)?;
            writeln!(
                ind,
                "interfaces: {}, fields: {}, methods: {}, attributes: {}",
                self.interfaces.len(),
                self.fields.len(),
                self.methods.len(),
                self.attributes.len()
            )?;
            Ok(())
        })?;
        writeln!(ind, "Constant pool:")?;
        ind.with_indent(|ind| {
            let counter_width = self.cp.cp.len().checked_ilog10().map_or(0, |d| d as usize) + 2;
            for (i, c) in self.cp.cp.iter().enumerate() {
                if matches!(c, ConstantInfo::Dummy) {
                    continue;
                }
                let tag = format_args!("{:<kw$}", c.get_tag(), kw = constant_kind_width);
                write!(ind, "{:>w$} = {} ", format!("#{i}"), tag, w = counter_width)?;
                c.fmt_pretty(ind, &self.cp)?;
            }
            Ok(())
        })?;
        /*
        pub minor_version: u16,
        pub major_version: u16,
        pub constant_pool: Vec<ConstantInfo>,
        pub access_flags: u16,
        pub this_class: u16,
        pub super_class: u16,
        pub interfaces: Vec<u16>,
        pub fields: Vec<FieldInfo>,
        pub methods: Vec<MethodInfo>,
        pub attributes: Vec<ClassAttribute>,
             */

        Ok(())
    }
}
