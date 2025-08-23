use crate::constant_pool::ConstantInfo;
use crate::ClassFileErr;

pub mod annotation;
pub mod class;
pub mod code;
pub mod method;

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.7
pub(super) fn get_utf8(idx: u16, pool: &Vec<ConstantInfo>) -> Result<&String, ClassFileErr> {
    pool.get(idx as usize)
        .ok_or(ClassFileErr::ConstantNotFound)
        .and_then(|entry| match entry {
            ConstantInfo::Utf8(value) => Ok(value),
            _ => Err(ClassFileErr::TypeError),
        })
}
