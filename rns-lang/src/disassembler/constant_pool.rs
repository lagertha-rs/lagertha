use crate::disassembler::indent_write::Indented;
use crate::disassembler::{DisasmError, DisasmResult};
use jclass::constant_pool::{ConstantEntry, ConstantPool};
use std::fmt::Write as _;

fn get_raw_entry(cp: &ConstantPool, idx: u16) -> DisasmResult<&ConstantEntry> {
    cp.inner
        .get(idx as usize)
        .ok_or(DisasmError::ConstantNotFound(idx))
}

pub(crate) fn fmt_cp_entry_rns(
    ind: &mut Indented,
    cp: &ConstantPool,
    idx: u16,
) -> DisasmResult<()> {
    let entry = get_raw_entry(cp, idx)?;
    fmt_entry_rns(entry, ind, cp)
}

pub(crate) fn fmt_entry_rns(
    entry: &ConstantEntry,
    ind: &mut Indented,
    cp: &ConstantPool,
) -> DisasmResult<()> {
    match entry {
        ConstantEntry::Utf8(s) => write!(ind, "{s}")?,
        ConstantEntry::Class(class_idx) => fmt_cp_entry_rns(ind, cp, *class_idx)?,
        ConstantEntry::NameAndType(name_and_type) => {
            fmt_cp_entry_rns(ind, cp, name_and_type.name_index)?;
            write!(ind, " ")?;
            fmt_cp_entry_rns(ind, cp, name_and_type.descriptor_index)?;
        }
        ConstantEntry::MethodRef(method_ref) => {
            fmt_cp_entry_rns(ind, cp, method_ref.class_index)?;
            write!(ind, " ")?;
            fmt_cp_entry_rns(ind, cp, method_ref.name_and_type_index)?;
        }
        ConstantEntry::FieldRef(field_ref) => {
            fmt_cp_entry_rns(ind, cp, field_ref.class_index)?;
            write!(ind, " ")?;
            fmt_cp_entry_rns(ind, cp, field_ref.name_and_type_index)?;
        }
        ConstantEntry::String(idx) => {
            write!(ind, "\"")?;
            fmt_cp_entry_rns(ind, cp, *idx)?;
            write!(ind, "\"")?;
        }
        other => return Err(DisasmError::UnsupportedConstant(format!("{other:?}"))),
    }

    Ok(())
}
