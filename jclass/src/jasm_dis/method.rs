use crate::constant_pool::ConstantPool;
use crate::member::MethodInfo;
use common::error::ClassFormatErr;
use common::utils::indent_write::Indented;
use std::fmt::Write as _;

impl MethodInfo {
    fn fmt_signature(&self, ind: &mut Indented, cp: &ConstantPool) -> Result<(), ClassFormatErr> {
        write!(ind, ".method ")?;
        self.access_flags.fmt_jasm(ind)?;
        cp.get_raw_entry(self.name_index)?.fmt_jasm(ind, cp)?;
        write!(ind, " ")?;
        cp.get_raw_entry(self.descriptor_index)?.fmt_jasm(ind, cp)?;
        writeln!(ind)?;
        Ok(())
    }

    pub(super) fn fmt_jasm(
        &self,
        ind: &mut Indented,
        cp: &ConstantPool,
    ) -> Result<(), ClassFormatErr> {
        self.fmt_signature(ind, cp)?;
        ind.with_indent(|ind| {
            for attr in &self.attributes {
                attr.fmt_jasm(ind, cp)?;
            }
            Ok(())
        })?;
        writeln!(ind, ".end method")?;
        Ok(())
    }
}
