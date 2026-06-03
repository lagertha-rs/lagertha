use crate::disassembler::indent_write::Indented;
use crate::disassembler::instruction::fmt_instruction_rns;
use crate::disassembler::{DisasmError, DisasmResult};
use jclass::attribute::{CodeAttribute, MethodAttribute};
use jclass::bytecode::Instruction;
use jclass::constant_pool::ConstantPool;
use std::fmt::Write as _;

pub(crate) fn fmt_method_attribute_rns(
    attribute: &MethodAttribute,
    ind: &mut Indented,
    cp: &ConstantPool,
) -> DisasmResult<()> {
    ind.with_indent(|ind| match attribute {
        MethodAttribute::Code(code) => fmt_code_attribute_rns(code, ind, cp),
        other => Err(DisasmError::UnsupportedMethodAttribute(format!("{other:?}"))),
    })
}

fn fmt_code_attribute_rns(
    code: &CodeAttribute,
    ind: &mut Indented,
    cp: &ConstantPool,
) -> DisasmResult<()> {
    writeln!(ind, ".code stack {} locals {}", code.max_stack, code.max_locals)?;
    ind.with_indent(|ind| {
        let mut pc = 0;
        while pc < code.code.len() {
            let inst = Instruction::new_at(&code.code, pc)?;
            fmt_instruction_rns(&inst, ind, cp)?;
            pc += inst.byte_size() as usize;
        }
        Ok(())
    })?;

    if !code.exception_table.is_empty() {
        return Err(DisasmError::UnsupportedCodeAttribute(
            "exception table".to_string(),
        ));
    }

    if !code.attributes.is_empty() {
        return Err(DisasmError::UnsupportedCodeAttribute(
            "nested code attributes".to_string(),
        ));
    }

    writeln!(ind, ".end code")?;
    Ok(())
}
