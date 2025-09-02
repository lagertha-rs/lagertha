use crate::constant::pool::ConstantPool;
use crate::error::ClassFileErr;
use common::access::{ClassAccessFlag, FieldAccessFlag, MethodAccessFlag};
use common::instruction::Instruction;
use std::fmt::Write;

/// Java-like modifier prefix for a field header
pub fn get_field_pretty_java_like_prefix(raw_flags: u16) -> String {
    let flags = FieldAccessFlag::new(raw_flags);
    let mut parts = Vec::with_capacity(6);

    if flags.is_public() {
        parts.push("public");
    } else if flags.is_protected() {
        parts.push("protected");
    } else if flags.is_private() {
        parts.push("private");
    }

    if flags.is_static() {
        parts.push("static");
    }
    if flags.is_final() {
        parts.push("final");
    }
    if flags.is_volatile() {
        parts.push("volatile");
    }
    if flags.is_transient() {
        parts.push("transient");
    }
    if flags.is_enum() {
        parts.push("enum");
    }
    if flags.is_synthetic() {
        parts.push("synthetic");
    }

    parts.join(" ")
}

/// Java-like modifier prefix for a method header
pub fn get_method_pretty_java_like_prefix(raw_flags: u16) -> String {
    let flags = MethodAccessFlag::new(raw_flags);
    let mut parts = Vec::with_capacity(6);

    if flags.is_public() {
        parts.push("public");
    } else if flags.is_protected() {
        parts.push("protected");
    } else if flags.is_private() {
        parts.push("private");
    }

    if flags.is_static() {
        parts.push("static");
    }
    if flags.is_final() {
        parts.push("final");
    }
    if flags.is_synchronized() {
        parts.push("synchronized");
    }
    if flags.is_native() {
        parts.push("native");
    }
    if flags.is_abstract() {
        parts.push("abstract");
    }
    if flags.is_strict() {
        parts.push("strictfp");
    }

    parts.join(" ")
}

/// `javap`-like flag list, e.g. "ACC_PUBLIC, ACC_STATIC, ACC_FINAL, …".
pub fn get_method_javap_like_list(raw_flags: u16) -> String {
    let flags = MethodAccessFlag::new(raw_flags);
    let mut parts = Vec::with_capacity(12);

    if flags.is_public() {
        parts.push("ACC_PUBLIC");
    }
    if flags.is_private() {
        parts.push("ACC_PRIVATE");
    }
    if flags.is_protected() {
        parts.push("ACC_PROTECTED");
    }
    if flags.is_static() {
        parts.push("ACC_STATIC");
    }
    if flags.is_final() {
        parts.push("ACC_FINAL");
    }
    if flags.is_synchronized() {
        parts.push("ACC_SYNCHRONIZED");
    }
    if flags.is_bridge() {
        parts.push("ACC_BRIDGE");
    }
    if flags.is_varargs() {
        parts.push("ACC_VARARGS");
    }
    if flags.is_native() {
        parts.push("ACC_NATIVE");
    }
    if flags.is_abstract() {
        parts.push("ACC_ABSTRACT");
    }
    if flags.is_strict() {
        parts.push("ACC_STRICT");
    }
    if flags.is_synthetic() {
        parts.push("ACC_SYNTHETIC");
    }

    parts.join(", ")
}

/// Returns a pretty string like "public abstract class" or "public interface"
pub fn get_class_pretty_java_like_prefix(raw_flags: u16) -> String {
    let flags = ClassAccessFlag::new(raw_flags);
    let mut parts = Vec::with_capacity(3);

    if flags.is_public() {
        parts.push("public");
    }

    let is_iface_like = flags.is_interface() || flags.is_annotation() || flags.is_module();

    if flags.is_abstract() && !is_iface_like {
        parts.push("abstract");
    }
    if flags.is_final() && !is_iface_like {
        parts.push("final");
    }

    if flags.is_module() {
        parts.push("module");
    } else if flags.is_annotation() {
        parts.push("@interface");
    } else if flags.is_interface() {
        parts.push("interface");
    } else if flags.is_enum() {
        parts.push("enum");
    } else {
        parts.push("class");
    }

    parts.join(" ")
}

/// Returns javap-like list of flags like "ACC_PUBLIC, ACC_FINAL, ACC_SUPER"
pub fn get_class_javap_like_list(raw_flags: u16) -> String {
    let flags = ClassAccessFlag::new(raw_flags);
    let mut parts = Vec::with_capacity(9);
    if flags.is_public() {
        parts.push("ACC_PUBLIC");
    }
    if flags.is_final() {
        parts.push("ACC_FINAL");
    }
    if flags.is_super() {
        parts.push("ACC_SUPER");
    }
    if flags.is_interface() {
        parts.push("ACC_INTERFACE");
    }
    if flags.is_abstract() {
        parts.push("ACC_ABSTRACT");
    }
    if flags.is_synthetic() {
        parts.push("ACC_SYNTHETIC");
    }
    if flags.is_annotation() {
        parts.push("ACC_ANNOTATION");
    }
    if flags.is_enum() {
        parts.push("ACC_ENUM");
    }
    if flags.is_module() {
        parts.push("ACC_MODULE");
    }
    parts.join(", ")
}

/// Returns true if the instruction's operand is a position (like for `goto` or `if` instructions)
/// needs to decide whether to print `#` before the operand
fn instruction_is_position(instruction: &Instruction) -> bool {
    matches!(
        instruction,
        Instruction::Aload(_)
            | Instruction::Astore(_)
            | Instruction::Bipush(_)
            | Instruction::Goto(_)
            | Instruction::IfAcmpEq(_)
            | Instruction::IfAcmpNe(_)
            | Instruction::IfEq(_)
            | Instruction::IfNe(_)
            | Instruction::Ifnull(_)
            | Instruction::IfLt(_)
            | Instruction::IfGe(_)
            | Instruction::IfGt(_)
            | Instruction::IfLe(_)
            | Instruction::IfIcmpeq(_)
            | Instruction::IfIcmpne(_)
            | Instruction::IfIcmplt(_)
            | Instruction::IfIcmpge(_)
            | Instruction::IfIcmpgt(_)
            | Instruction::IfIcmple(_)
            | Instruction::Iinc(_, _)
            | Instruction::Iload(_)
            | Instruction::Ifnonnull(_)
            | Instruction::Istore(_)
            | Instruction::Lload(_)
            | Instruction::Lstore(_)
            | Instruction::Newarray(_)
            | Instruction::Sipush(_)
    )
}

fn get_instruction_value(instruction: &Instruction, pc: i32) -> Option<String> {
    match instruction {
        Instruction::Aload(val) => Some(val.to_string()),
        Instruction::Anewarray(val) => Some(val.to_string()),
        Instruction::Astore(val) => Some(val.to_string()),
        Instruction::Bipush(val) => Some(val.to_string()),
        Instruction::Checkcast(val) => Some(val.to_string()),
        Instruction::Getfield(val) => Some(val.to_string()),
        Instruction::Getstatic(val) => Some(val.to_string()),
        Instruction::Goto(val) => Some(((*val as i32) + pc).to_string()),
        Instruction::Instanceof(val) => Some(val.to_string()),
        Instruction::IfAcmpEq(val) => Some(((*val as i32) + pc).to_string()),
        Instruction::IfAcmpNe(val) => Some(((*val as i32) + pc).to_string()),
        Instruction::IfEq(val) => Some(((*val as i32) + pc).to_string()),
        Instruction::IfNe(val) => Some(((*val as i32) + pc).to_string()),
        Instruction::Ifnull(val) => Some(((*val as i32) + pc).to_string()),
        Instruction::IfLt(val) => Some(((*val as i32) + pc).to_string()),
        Instruction::IfGe(val) => Some(((*val as i32) + pc).to_string()),
        Instruction::IfGt(val) => Some(((*val as i32) + pc).to_string()),
        Instruction::IfLe(val) => Some(((*val as i32) + pc).to_string()),
        Instruction::IfIcmpeq(val) => Some(((*val as i32) + pc).to_string()),
        Instruction::IfIcmpne(val) => Some(((*val as i32) + pc).to_string()),
        Instruction::IfIcmplt(val) => Some(((*val as i32) + pc).to_string()),
        Instruction::IfIcmpge(val) => Some(((*val as i32) + pc).to_string()),
        Instruction::IfIcmpgt(val) => Some(((*val as i32) + pc).to_string()),
        Instruction::IfIcmple(val) => Some(((*val as i32) + pc).to_string()),
        Instruction::Iinc(val1, val2) => Some(format!("{}, {}", val1, val2)),
        Instruction::Iload(val) => Some(val.to_string()),
        Instruction::Ifnonnull(val) => Some(((*val as i32) + pc).to_string()),
        Instruction::InvokeInterface(val1, val2) => Some(format!("{}, {}", val1, val2)),
        Instruction::InvokeSpecial(val) => Some(val.to_string()),
        Instruction::InvokeStatic(val) => Some(val.to_string()),
        Instruction::InvokeVirtual(val) => Some(val.to_string()),
        Instruction::Istore(val) => Some(val.to_string()),
        Instruction::Ldc(val) => Some(val.to_string()),
        Instruction::LdcW(val) => Some(val.to_string()),
        Instruction::Ldc2W(val) => Some(val.to_string()),
        Instruction::Lload(val) => Some(val.to_string()),
        Instruction::Lstore(val) => Some(val.to_string()),
        Instruction::New(val) => Some(val.to_string()),
        Instruction::Newarray(val) => Some(val.to_string()),
        Instruction::Putfield(val) => Some(val.to_string()),
        Instruction::Sipush(val) => Some(val.to_string()),
        _ => None,
    }
}

fn get_instruction_comment(
    instruction: &Instruction,
    cp: &ConstantPool,
    this: &u16,
) -> Result<Option<String>, ClassFileErr> {
    let comment_value = |index: &u16| -> Result<Option<String>, ClassFileErr> {
        let constant = cp.get_raw(index)?;
        Ok(Some(constant.get_pretty_value(cp, this)?))
    };
    match instruction {
        Instruction::Anewarray(val) => comment_value(val),
        Instruction::Checkcast(val) => comment_value(val),
        Instruction::Getfield(val) => comment_value(val),
        Instruction::Getstatic(val) => comment_value(val),
        Instruction::Instanceof(val) => comment_value(val),
        Instruction::InvokeInterface(val, _) => comment_value(val),
        Instruction::InvokeSpecial(val) => comment_value(val),
        Instruction::InvokeStatic(val) => comment_value(val),
        Instruction::InvokeVirtual(val) => comment_value(val),
        Instruction::Ldc(val) => comment_value(val),
        Instruction::LdcW(val) => comment_value(val),
        Instruction::Ldc2W(val) => comment_value(val),
        Instruction::New(val) => comment_value(val),
        Instruction::Putfield(val) => comment_value(val),
        _ => Ok(None),
    }
}

pub fn get_pretty_instruction(
    instruction: &Instruction,
    cp: &ConstantPool,
    pc: i32,
    this: &u16,
) -> Result<String, ClassFileErr> {
    let val = get_instruction_value(instruction, pc);
    let comment = get_instruction_comment(instruction, cp, this)?;
    let is_position = instruction_is_position(instruction);

    let mut out = String::with_capacity(32);
    write!(&mut out, "{:<13}", instruction.get_name()).unwrap();
    if let Some(v) = val {
        write!(&mut out, " {}{v:<18}", if !is_position { "#" } else { "" }).unwrap();
        if let Some(c) = comment {
            out.push_str(" // ");
            out.push_str(&c);
        }
    }
    Ok(out)
}
