use crate::constant::pool::ConstantPool;
use crate::error::ClassFormatErr;
use common::instruction::Instruction;
use std::fmt::Write;

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
            | Instruction::Lookupswitch(_)
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
        Instruction::InvokeDynamic(val) => Some(format!("{val}, 0")),
        Instruction::InvokeInterface(val1, val2) => Some(format!("{val1}, {val2}")),
        Instruction::InvokeSpecial(val) => Some(val.to_string()),
        Instruction::InvokeStatic(val) => Some(val.to_string()),
        Instruction::InvokeVirtual(val) => Some(val.to_string()),
        Instruction::Istore(val) => Some(val.to_string()),
        Instruction::Ldc(val) => Some(val.to_string()),
        Instruction::LdcW(val) => Some(val.to_string()),
        Instruction::Ldc2W(val) => Some(val.to_string()),
        Instruction::Lload(val) => Some(val.to_string()),
        Instruction::Lookupswitch(data) => {
            let mut s = String::new();
            s.push_str(&format!("{{ // {}\n", data.pairs.len()));
            for (k, v) in &data.pairs {
                s.push_str(&format!("{:>20}: {}\n", k, v + pc));
            }
            s.push_str(&format!(
                "{:>20}: {}\n",
                "default",
                data.default_offset + pc
            ));
            s.push_str(&format!("{:>7}", "}"));
            Some(s)
        }
        Instruction::Lstore(val) => Some(val.to_string()),
        Instruction::New(val) => Some(val.to_string()),
        Instruction::Newarray(val) => Some(val.to_string()),
        Instruction::Putfield(val) => Some(val.to_string()),
        Instruction::Putstatic(val) => Some(val.to_string()),
        Instruction::Sipush(val) => Some(val.to_string()),
        _ => None,
    }
}

fn get_instruction_comment(
    instruction: &Instruction,
    cp: &ConstantPool,
    this: &u16,
) -> Result<Option<String>, ClassFormatErr> {
    let comment_value = |index: &u16| -> Result<Option<String>, ClassFormatErr> {
        let constant = cp.get_raw(index)?;
        Ok(Some(constant.get_pretty_value(cp, this)?))
    };
    match instruction {
        Instruction::Anewarray(val) => comment_value(val),
        Instruction::Checkcast(val) => comment_value(val),
        Instruction::Getfield(val) => comment_value(val),
        Instruction::Getstatic(val) => comment_value(val),
        Instruction::Instanceof(val) => comment_value(val),
        Instruction::InvokeDynamic(val) => comment_value(val),
        Instruction::InvokeInterface(val, _) => comment_value(val),
        Instruction::InvokeSpecial(val) => comment_value(val),
        Instruction::InvokeStatic(val) => comment_value(val),
        Instruction::InvokeVirtual(val) => comment_value(val),
        Instruction::Ldc(val) => comment_value(val),
        Instruction::LdcW(val) => comment_value(val),
        Instruction::Ldc2W(val) => comment_value(val),
        Instruction::New(val) => comment_value(val),
        Instruction::Putfield(val) => comment_value(val),
        Instruction::Putstatic(val) => comment_value(val),
        _ => Ok(None),
    }
}

pub fn get_pretty_instruction(
    instruction: &Instruction,
    cp: &ConstantPool,
    pc: i32,
    this: &u16,
) -> Result<String, ClassFormatErr> {
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
