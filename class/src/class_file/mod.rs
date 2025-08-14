use crate::class_file::attribute::class::ClassAttribute;
use common::{ByteCursor, CursorError};
use constant_pool::ConstantInfo;
use field::FieldInfo;
use method::MethodInfo;
use std::fmt;
use thiserror::Error;

pub mod attribute;
pub mod constant_pool;
pub mod field;
pub mod method;

/// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html
#[derive(Debug)]
pub struct ClassFile {
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
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum MethodDescriptorErr {
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
    MethodDescriptor(#[from] MethodDescriptorErr),
    #[error("")]
    WrongMagic,
    #[error("")]
    TrailingBytes,
    #[error("")]
    UnknownTag(u8),
    #[error("")]
    TypeError,
    #[error("")]
    ConstantNotFound,
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
        for _ in 1..constant_pool_count {
            constant_pool.push(ConstantInfo::read(&mut cursor)?);
        }
        let access_flags = cursor.u16()?;
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
                constant_pool,
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
