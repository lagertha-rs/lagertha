use crate::constant::pool::ConstantPool;
use crate::error::ClassFileErr;
use common::access::{ClassAccessFlag, MethodAccessFlag};
use common::instruction::Instruction;
use std::fmt::Write;

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

// TODO: looks like shit
pub fn get_pretty_instruction(
    instruction: &Instruction,
    cp: &ConstantPool,
    pc: i32,
) -> Result<String, ClassFileErr> {
    let comment_value = |index: u16| -> Result<Option<String>, ClassFileErr> {
        let constant = cp.get_raw(index)?;
        Ok(Some(constant.get_pretty_value(cp)?))
    };
    let (val, name, comment, is_position) = match instruction {
        Instruction::Aload(val) => (Some(*val as i32), "aload", None, true),
        Instruction::Aload0 => (None, "aload_0", None, false),
        Instruction::Aload1 => (None, "aload_1", None, false),
        Instruction::Aload2 => (None, "aload_2", None, false),
        Instruction::Aload3 => (None, "aload_3", None, false),
        Instruction::Astore(val) => (Some(*val as i32), "astore", None, true),
        Instruction::Astore0 => (None, "astore_0", None, false),
        Instruction::Astore1 => (None, "astore_1", None, false),
        Instruction::Astore2 => (None, "astore_2", None, false),
        Instruction::Astore3 => (None, "astore_3", None, false),
        Instruction::Athrow => (None, "athrow", None, false),
        Instruction::Checkcast(val) => {
            (Some(*val as i32), "checkcast", comment_value(*val)?, false)
        }
        Instruction::Dup => (None, "dup", None, false),
        Instruction::Getstatic(val) => {
            (Some(*val as i32), "getstatic", comment_value(*val)?, false)
        }
        Instruction::Goto(val) => (Some(*val as i32 + pc), "goto", None, true),
        Instruction::IconstM1 => (None, "iconst_m1", None, false),
        Instruction::Iconst0 => (None, "iconst_0", None, false),
        Instruction::Iconst1 => (None, "iconst_1", None, false),
        Instruction::Iconst2 => (None, "iconst_2", None, false),
        Instruction::Iconst3 => (None, "iconst_3", None, false),
        Instruction::Iconst4 => (None, "iconst_4", None, false),
        Instruction::Iconst5 => (None, "iconst_5", None, false),
        Instruction::Iload0 => (None, "iload_0", None, false),
        Instruction::Iload1 => (None, "iload_1", None, false),
        Instruction::Iload2 => (None, "iload_2", None, false),
        Instruction::Iload3 => (None, "iload_3", None, false),
        Instruction::Instanceof(val) => (Some(*val as i32), "instanceof", None, false),
        Instruction::IfAcmpNe(val) => (Some(*val as i32 + pc), "if_acmpne", None, true),
        Instruction::Ifeq(val) => (Some(*val as i32 + pc), "ifeq", None, true),
        Instruction::Ifne(val) => (Some(*val as i32 + pc), "ifne", None, true),
        Instruction::Iflt(val) => (Some(*val as i32 + pc), "iflt", None, true),
        Instruction::Ifge(val) => (Some(*val as i32 + pc), "ifge", None, true),
        Instruction::Ifgt(val) => (Some(*val as i32 + pc), "ifgt", None, true),
        Instruction::Ifle(val) => (Some(*val as i32 + pc), "ifle", None, true),
        Instruction::IfIcmpeq(val) => (Some(*val as i32 + pc), "if_icmpeq", None, true),
        Instruction::IfIcmpne(val) => (Some(*val as i32 + pc), "if_icmpne", None, true),
        Instruction::IfIcmplt(val) => (Some(*val as i32 + pc), "if_icmplt", None, true),
        Instruction::IfIcmpge(val) => (Some(*val as i32 + pc), "if_icmpge", None, true),
        Instruction::IfIcmpgt(val) => (Some(*val as i32 + pc), "if_icmpgt", None, true),
        Instruction::IfIcmple(val) => (Some(*val as i32 + pc), "if_icmple", None, true),
        Instruction::Ireturn => (None, "ireturn", None, false),
        Instruction::Invokespecial(val) => (
            Some(*val as i32),
            "invokespecial",
            comment_value(*val)?,
            false,
        ),
        Instruction::Invokestatic(val) => (
            Some(*val as i32),
            "invokestatic",
            comment_value(*val)?,
            false,
        ),
        Instruction::Invokevirtual(val) => (
            Some(*val as i32),
            "invokevirtual",
            comment_value(*val)?,
            false,
        ),
        Instruction::Lcmp => (None, "lcmp", None, false),
        Instruction::Lconst0 => (None, "lconst_0", None, false),
        Instruction::Lconst1 => (None, "lconst_1", None, false),
        Instruction::Lload0 => (None, "lload_0", None, false),
        Instruction::Lload1 => (None, "lload_1", None, false),
        Instruction::Lload2 => (None, "lload_2", None, false),
        Instruction::Lload3 => (None, "lload_3", None, false),
        Instruction::Lstore0 => (None, "lstore_0", None, false),
        Instruction::Lstore1 => (None, "lstore_1", None, false),
        Instruction::Lstore2 => (None, "lstore_2", None, false),
        Instruction::Lstore3 => (None, "lstore_3", None, false),
        Instruction::Ldc(val) => (Some(*val as i32), "ldc", comment_value(*val)?, false),
        Instruction::Ldc2w(val) => (Some(*val as i32), "ldc2_w", comment_value(*val)?, false),
        Instruction::New(val) => (Some(*val as i32), "new", comment_value(*val)?, false),
        Instruction::Pop => (None, "pop", None, false),
        Instruction::Return => (None, "return", None, false),
        Instruction::Areturn => (None, "areturn", None, false),
        Instruction::Ladd => (None, "ladd", None, false),
    };

    let mut out = String::with_capacity(32);
    write!(&mut out, "{:<13}", name).unwrap();
    if let Some(v) = val {
        write!(&mut out, " {}{v:<18}", if !is_position { "#" } else { "" }).unwrap();
        if let Some(c) = comment {
            out.push_str(" // ");
            out.push_str(&c);
        }
    }
    Ok(out)
}
