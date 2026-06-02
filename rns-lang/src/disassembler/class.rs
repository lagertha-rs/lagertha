use crate::disassembler::indent_write::Indented;
use jclass::ClassFile;
use std::fmt::Write as _;

fn fmt_signature(class: &ClassFile, ind: &mut Indented) -> Result<(), std::fmt::Error> {
    class.access_flags.fmt_rns(ind)?;
    class
        .cp
        .get_raw_entry(class.this_class)?
        .fmt_rns(ind, &class.cp)?;
    writeln!(ind)?;
    Ok(())
}

fn fmt_super_class(class: &ClassFile, ind: &mut Indented) -> Result<(), std::fmt::Error> {
    if class.super_class == 0 {
        return Ok(());
    }
    write!(ind, ".super ",)?;
    class
        .cp
        .get_raw_entry(class.super_class)?
        .fmt_rns(ind, &class.cp)?;
    writeln!(ind)?;
    Ok(())
}

pub fn fmt_rns(class: &ClassFile) -> Result<String, std::fmt::Error> {
    let mut out = String::new();
    let mut ind = Indented::new(&mut out);
    class.fmt_signature(&mut ind)?;

    class.fmt_super_class(&mut ind)?;
    writeln!(ind)?;
    for (i, method) in class.methods.iter().enumerate() {
        method.fmt_rns(&mut ind, &class.cp)?;
        if i < class.methods.len() - 1 {
            writeln!(ind)?;
        }
    }

    writeln!(ind, ".end class")?;
    Ok(out)
}
