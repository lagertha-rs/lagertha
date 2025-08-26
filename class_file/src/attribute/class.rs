use crate::ClassFileErr;
use crate::attribute::{AttributeType, SharedAttribute};
use crate::constant::pool::ConstantPool;
use common::utils::cursor::ByteCursor;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClassAttribute {
    Shared(SharedAttribute),
    SourceFile(u16),
    InnerClasses,
    EnclosingMethod,
    SourceDebugExtension,
    BootstrapMethods,
    Module,
    ModulePackages,
    ModuleMainClass,
    NestHost,
    NestMembers,
    Record,
    PermittedSubclasses,
}

impl<'a> ClassAttribute {
    pub(crate) fn read(
        pool: &ConstantPool,
        cursor: &mut ByteCursor<'a>,
    ) -> Result<Self, ClassFileErr> {
        let attribute_name_index = cursor.u16()?;
        let _attribute_length = cursor.u32()? as usize;

        let attribute_type = AttributeType::try_from(pool.get_utf8(&attribute_name_index)?)?;
        match attribute_type {
            AttributeType::SourceFile => Ok(ClassAttribute::SourceFile(cursor.u16()?)),
            _ => unimplemented!(),
        }
    }
}
