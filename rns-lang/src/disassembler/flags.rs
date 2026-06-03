use crate::disassembler::indent_write::Indented;
use crate::disassembler::DisasmResult;
use jclass::flags::{ClassFlags, MethodFlags};
use std::fmt::Write as _;

pub(crate) fn fmt_class_flags_rns(flags: &ClassFlags, ind: &mut Indented) -> DisasmResult<()> {
    write!(ind, ".class ")?;
    if flags.is_public() {
        write!(ind, "public ")?;
    }
    if flags.is_module() {
        write!(ind, "module ")?;
    }
    if flags.is_annotation() {
        write!(ind, "annotation ")?;
    }
    if flags.is_interface() {
        write!(ind, "interface ")?;
    }
    if flags.is_abstract() {
        write!(ind, "abstract ")?;
    }
    if flags.is_final() {
        write!(ind, "final ")?;
    }
    if flags.is_enum() {
        write!(ind, "enum ")?;
    }
    if flags.is_module() {
        write!(ind, "module ")?;
    }
    if flags.is_super() {
        write!(ind, "super ")?;
    }
    Ok(())
}

pub(crate) fn fmt_method_flags_rns(flags: &MethodFlags, ind: &mut Indented) -> DisasmResult<()> {
    if flags.is_public() {
        write!(ind, "public ")?;
    } else if flags.is_protected() {
        write!(ind, "protected ")?;
    } else if flags.is_private() {
        write!(ind, "private ")?;
    }

    if flags.is_static() {
        write!(ind, "static ")?;
    }
    if flags.is_final() {
        write!(ind, "final ")?;
    }
    if flags.is_synchronized() {
        write!(ind, "synchronized ")?;
    }
    if flags.is_native() {
        write!(ind, "native ")?;
    }
    if flags.is_abstract() {
        write!(ind, "abstract ")?;
    }
    if flags.is_strict() {
        write!(ind, "strictfp ")?;
    }
    Ok(())
}
