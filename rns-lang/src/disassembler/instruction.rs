use crate::disassembler::DisasmResult;
use crate::disassembler::constant_pool::fmt_cp_entry_rns;
use crate::disassembler::indent_write::Indented;
use jclass::bytecode::Instruction;
use jclass::constant_pool::ConstantPool;
use std::fmt::Write as _;

pub(crate) fn fmt_instruction_rns(
    instruction: &Instruction,
    ind: &mut Indented,
    cp: &ConstantPool,
) -> DisasmResult<()> {
    match instruction {
        Instruction::InvokeSpecial(idx)
        | Instruction::InvokeVirtual(idx)
        | Instruction::Getstatic(idx)
        | Instruction::Ldc(idx) => {
            write!(ind, "{instruction} ")?;
            fmt_cp_entry_rns(ind, cp, *idx)?;
            writeln!(ind)?;
        }
        _ => writeln!(ind, "{instruction}")?,
    }
    Ok(())
}
