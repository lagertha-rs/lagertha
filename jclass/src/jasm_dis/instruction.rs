use crate::bytecode::Instruction;
use crate::constant_pool::ConstantPool;
use common::error::ClassFormatErr;
use common::utils::indent_write::Indented;
use std::fmt::Write as _;

impl Instruction {
    pub(super) fn fmt_jasm(
        &self,
        ind: &mut Indented,
        cp: &ConstantPool,
    ) -> Result<(), ClassFormatErr> {
        match self {
            Instruction::InvokeSpecial(idx)
            | Instruction::InvokeVirtual(idx)
            | Instruction::Getstatic(idx)
            | Instruction::Ldc(idx) => {
                write!(ind, "{self} ")?;
                cp.get_raw_entry(*idx)?.fmt_jasm(ind, cp)?;
                writeln!(ind)?;
            }
            _ => writeln!(ind, "{}", self)?,
        }
        Ok(())
    }
}
