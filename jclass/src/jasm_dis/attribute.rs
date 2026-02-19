use crate::attribute::{CodeAttribute, MethodAttribute};
use crate::bytecode::Instruction;
use crate::constant_pool::ConstantPool;
use common::error::ClassFormatErr;
use common::utils::indent_write::Indented;
use std::fmt::Write as _;

impl MethodAttribute {
    pub(crate) fn fmt_jasm(
        &self,
        ind: &mut Indented,
        cp: &ConstantPool,
    ) -> Result<(), ClassFormatErr> {
        ind.with_indent(|ind| match self {
            MethodAttribute::Code(code) => code.fmt_jasm(ind, cp),
            _ => unimplemented!("Only Code attribute is supported for writing right now"),
        })
    }
}

impl CodeAttribute {
    /* TODO: it adds PC as comment, can have also more info but should be under verbose flag
    fn fmt_instructions_vec(
        ind: &mut Indented,
        instructions: Vec<(usize, String)>,
    ) -> Result<(), ClassFormatErr> {
        let max_width = instructions.iter().map(|s| s.1.len()).max().unwrap_or(0) + 2;
        writeln!(ind, "{:>width$}  ; PC", "", width = max_width)?;
        for (pc, instr) in instructions {
            writeln!(ind, "{:<width$}  ; {}", instr, pc, width = max_width)?;
        }
            Ok(())
        }
         */

    pub(super) fn fmt_jasm(
        &self,
        ind: &mut Indented,
        cp: &ConstantPool,
    ) -> Result<(), ClassFormatErr> {
        writeln!(
            ind,
            ".code stack {} locals {}",
            self.max_stack, self.max_locals
        )?;
        ind.with_indent(|ind| {
            let mut pc = 0;
            let code_len = self.code.len();
            while pc < code_len {
                let inst = Instruction::new_at(&self.code, pc)?;
                inst.fmt_jasm(ind, cp)?;
                pc += inst.byte_size() as usize;
            }
            Ok(())
        })?;
        writeln!(ind, ".end code")?;
        Ok(())
    }
}
