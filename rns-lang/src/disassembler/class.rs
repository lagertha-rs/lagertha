use crate::disassembler::DisasmResult;
use crate::disassembler::constant_pool::fmt_cp_entry_rns;
use crate::disassembler::flags::fmt_class_flags_rns;
use crate::disassembler::indent_write::Indented;
use crate::disassembler::method::fmt_method_rns;
use jclass::ClassFile;
use std::fmt::Write as _;

fn fmt_signature(class: &ClassFile, ind: &mut Indented) -> DisasmResult<()> {
    fmt_class_flags_rns(&class.access_flags, ind)?;
    fmt_cp_entry_rns(ind, &class.cp, class.this_class)?;
    writeln!(ind)?;
    Ok(())
}

fn fmt_super_class(class: &ClassFile, ind: &mut Indented) -> DisasmResult<()> {
    if class.super_class == 0 {
        return Ok(());
    }

    write!(ind, ".super ")?;
    fmt_cp_entry_rns(ind, &class.cp, class.super_class)?;
    writeln!(ind)?;
    Ok(())
}

pub(crate) fn fmt_rns(class: &ClassFile) -> DisasmResult<String> {
    let mut out = String::new();
    let mut ind = Indented::new(&mut out);
    fmt_signature(class, &mut ind)?;
    fmt_super_class(class, &mut ind)?;
    writeln!(ind)?;

    for (i, method) in class.methods.iter().enumerate() {
        fmt_method_rns(method, &mut ind, &class.cp)?;
        if i + 1 < class.methods.len() {
            writeln!(ind)?;
        }
    }

    writeln!(ind, ".end class")?;
    Ok(out)
}
