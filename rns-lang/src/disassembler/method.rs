use crate::disassembler::attribute::fmt_method_attribute_rns;
use crate::disassembler::constant_pool::fmt_cp_entry_rns;
use crate::disassembler::flags::fmt_method_flags_rns;
use crate::disassembler::indent_write::Indented;
use crate::disassembler::DisasmResult;
use jclass::constant_pool::ConstantPool;
use jclass::member::MethodInfo;
use std::fmt::Write as _;

fn fmt_signature(method: &MethodInfo, ind: &mut Indented, cp: &ConstantPool) -> DisasmResult<()> {
    write!(ind, ".method ")?;
    fmt_method_flags_rns(&method.access_flags, ind)?;
    fmt_cp_entry_rns(ind, cp, method.name_index)?;
    write!(ind, " ")?;
    fmt_cp_entry_rns(ind, cp, method.descriptor_index)?;
    writeln!(ind)?;
    Ok(())
}

pub(crate) fn fmt_method_rns(
    method: &MethodInfo,
    ind: &mut Indented,
    cp: &ConstantPool,
) -> DisasmResult<()> {
    fmt_signature(method, ind, cp)?;
    ind.with_indent(|ind| {
        for attr in &method.attributes {
            fmt_method_attribute_rns(attr, ind, cp)?;
        }
        Ok(())
    })?;
    writeln!(ind, ".end method")?;
    Ok(())
}
