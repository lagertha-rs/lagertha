use crate::constant_pool::{ConstantEntry, ConstantPool};
use common::error::ClassFormatErr;
use common::utils::indent_write::Indented;
use std::fmt::Write as _;

impl ConstantPool {
    pub(super) fn get_raw_entry(&self, idx: u16) -> Result<&ConstantEntry, ClassFormatErr> {
        self.inner
            .get(idx as usize)
            .ok_or(ClassFormatErr::ConstantNotFound(idx))
    }
}

impl ConstantEntry {
    pub(super) fn fmt_jasm(
        &self,
        ind: &mut Indented,
        cp: &ConstantPool,
    ) -> Result<(), ClassFormatErr> {
        match self {
            ConstantEntry::Utf8(s) => write!(ind, "{}", s)?,
            ConstantEntry::Class(class_idx) => cp.get_raw_entry(*class_idx)?.fmt_jasm(ind, cp)?,
            ConstantEntry::NameAndType(name_and_type_idx) => {
                cp.get_raw_entry(name_and_type_idx.name_index)?
                    .fmt_jasm(ind, cp)?;
                write!(ind, " ")?;
                cp.get_raw_entry(name_and_type_idx.descriptor_index)?
                    .fmt_jasm(ind, cp)?;
            }
            ConstantEntry::MethodRef(method_ref) => {
                cp.get_raw_entry(method_ref.class_index)?
                    .fmt_jasm(ind, cp)?;
                write!(ind, " ")?;
                cp.get_raw_entry(method_ref.name_and_type_index)?
                    .fmt_jasm(ind, cp)?;
            }
            ConstantEntry::FieldRef(field_ref) => {
                cp.get_raw_entry(field_ref.class_index)?.fmt_jasm(ind, cp)?;
                write!(ind, " ")?;
                cp.get_raw_entry(field_ref.name_and_type_index)?
                    .fmt_jasm(ind, cp)?;
            }
            ConstantEntry::String(idx) => {
                write!(ind, "\"")?;
                cp.get_raw_entry(*idx)?.fmt_jasm(ind, cp)?;
                write!(ind, "\"")?;
            }
            un => unimplemented!("{:?} is not supported for writing right now", un),
        };
        Ok(())
    }
}
