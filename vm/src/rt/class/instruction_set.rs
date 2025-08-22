use crate::byte_cursor::ByteCursor;
use crate::rt::class::LoadingError;
use num_enum::TryFromPrimitive;
use std::fmt;
use std::fmt::Formatter;

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html
#[derive(Debug, Clone, Copy, TryFromPrimitive)]
#[repr(u8)]
pub enum Opcode {
    Aload = 0x19,
    Aload0 = 0x2A,
    Aload1 = 0x2B,
    Aload2 = 0x2C,
    Aload3 = 0x2D,
    Astore = 0x3A,
    Astore0 = 0x4b,
    Astore1 = 0x4c,
    Astore2 = 0x4d,
    Astore3 = 0x4e,
    Athrow = 0xBF,
    Checkcast = 0xC0,
    Dup = 0x59,
    Getstatic = 0xB2,
    Goto = 0xA7,
    IconstM1 = 0x02,
    Iconst0 = 0x03,
    Iconst1 = 0x04,
    Iconst2 = 0x05,
    Iconst3 = 0x06,
    Iconst4 = 0x07,
    Iconst5 = 0x08,
    IfAcmpne = 0xA6,
    Ifeq = 0x99,
    Ifne = 0x9a,
    Iflt = 0x9b,
    Ifge = 0x9c,
    Ifgt = 0x9d,
    Ifle = 0x9e,
    Instanceof = 0xC1,
    Invokespecial = 0xB7,
    Invokestatic = 0xB8,
    Invokevirtual = 0xB6,
    Ireturn = 0xAC,
    Lcmp = 0x94,
    Lconst0 = 0x09,
    Lconst1 = 0x0A,
    Lload0 = 0x1E,
    Lload1 = 0x1F,
    Lload2 = 0x20,
    Lload3 = 0x21,
    Ldc = 0x12,
    New = 0xBB,
    Areturn = 0xB0,
    Return = 0xB1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    Aload(u8),
    Aload0,
    Aload1,
    Aload2,
    Aload3,
    Astore(u8),
    Astore0,
    Astore1,
    Astore2,
    Astore3,
    Athrow,
    Checkcast(u16),
    Dup,
    Getstatic(u16),
    Goto(i16),
    IconstM1,
    Iconst0,
    Iconst1,
    Iconst2,
    Iconst3,
    Iconst4,
    Iconst5,
    Instanceof(u16),
    IfAcmpNe(i16),
    Ifeq(i16),
    Ifne(i16),
    Iflt(i16),
    Ifge(i16),
    Ifgt(i16),
    Ifle(i16),
    Ireturn,
    Invokespecial(u16),
    Invokestatic(u16),
    Invokevirtual(u16),
    Lcmp,
    Lconst0,
    Lconst1,
    Lload0,
    Lload1,
    Lload2,
    Lload3,
    Ldc(u16),
    New(u16),
    Return,
    Areturn,
}

impl Instruction {
    pub fn byte_size(&self) -> u16 {
        match self {
            Self::Ldc(_) | Self::Astore(_) | Self::Aload(_) => 2,
            Self::New(_)
            | Self::Checkcast(_)
            | Self::Ifeq(_)
            | Self::Ifne(_)
            | Self::Iflt(_)
            | Self::Ifge(_)
            | Self::Ifgt(_)
            | Self::Ifle(_)
            | Self::Invokespecial(_)
            | Self::Invokestatic(_)
            | Self::Invokevirtual(_)
            | Self::Instanceof(_)
            | Self::Getstatic(_)
            | Self::Goto(_)
            | Self::IfAcmpNe(_) => 3,
            _ => 1,
        }
    }
}

impl Instruction {
    //TODO: Idk, don't really like such constructor
    pub fn new_instruction_set(code: Vec<u8>) -> Result<Vec<Instruction>, LoadingError> {
        let mut cursor = ByteCursor::new(code.as_slice());
        let mut res = Vec::new();

        while let Some(opcode_byte) = cursor.try_u8() {
            let opcode = Opcode::try_from(opcode_byte)
                .map_err(|_| LoadingError::UnsupportedOpCode(opcode_byte))?;

            let instruction = match opcode {
                Opcode::Astore => Self::Astore(cursor.u8()?),
                Opcode::Aload => Self::Aload(cursor.u8()?),
                Opcode::Checkcast => Self::Checkcast(cursor.u16()?),
                Opcode::Invokespecial => Self::Invokespecial(cursor.u16()?),
                Opcode::Invokestatic => Self::Invokestatic(cursor.u16()?),
                Opcode::Invokevirtual => Self::Invokevirtual(cursor.u16()?),
                Opcode::Instanceof => Self::Instanceof(cursor.u16()?),
                Opcode::Getstatic => Self::Getstatic(cursor.u16()?),
                Opcode::Goto => Self::Goto(cursor.i16()?),
                Opcode::Ldc => Self::Ldc(cursor.u8()? as u16),
                Opcode::IfAcmpne => Self::IfAcmpNe(cursor.i16()?),
                Opcode::Ifeq => Self::Ifeq(cursor.i16()?),
                Opcode::Ifne => Self::Ifne(cursor.i16()?),
                Opcode::Iflt => Self::Iflt(cursor.i16()?),
                Opcode::Ifge => Self::Ifge(cursor.i16()?),
                Opcode::Ifgt => Self::Ifgt(cursor.i16()?),
                Opcode::Ifle => Self::Ifle(cursor.i16()?),
                Opcode::New => Self::New(cursor.u16()?),
                Opcode::Astore0 => Self::Astore0,
                Opcode::Astore1 => Self::Astore1,
                Opcode::Astore2 => Self::Astore2,
                Opcode::Astore3 => Self::Astore3,
                Opcode::Aload0 => Self::Aload0,
                Opcode::Aload1 => Self::Aload1,
                Opcode::Aload2 => Self::Aload2,
                Opcode::Aload3 => Self::Aload3,
                Opcode::Athrow => Self::Athrow,
                Opcode::Return => Self::Return,
                Opcode::IconstM1 => Self::IconstM1,
                Opcode::Iconst0 => Self::Iconst0,
                Opcode::Iconst1 => Self::Iconst1,
                Opcode::Iconst2 => Self::Iconst2,
                Opcode::Iconst3 => Self::Iconst3,
                Opcode::Iconst4 => Self::Iconst4,
                Opcode::Iconst5 => Self::Iconst5,
                Opcode::Areturn => Self::Areturn,
                Opcode::Ireturn => Self::Ireturn,
                Opcode::Lconst0 => Self::Lconst0,
                Opcode::Lconst1 => Self::Lconst1,
                Opcode::Lload0 => Self::Lload0,
                Opcode::Lload1 => Self::Lload1,
                Opcode::Lload2 => Self::Lload2,
                Opcode::Lload3 => Self::Lload3,
                Opcode::Dup => Self::Ireturn,
                Opcode::Lcmp => Self::Lcmp,
            };
            res.push(instruction)
        }
        Ok(res)
    }
}

impl fmt::Display for Instruction {
    //TODO: avoid allocation in display
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            Instruction::Ldc(index) => format!("{:<13} #{index}", "ldc"),
            Instruction::Invokespecial(method_index) => {
                format!("{:<13} #{method_index}", "invokespecial")
            }
            Instruction::Invokevirtual(method_index) => {
                format!("{:<13} #{method_index}", "invokevirtual")
            }
            Instruction::IfAcmpNe(offset) => {
                format!("{:<13} #{offset}", "if_acmpne")
            }
            Instruction::Goto(offset) => {
                format!("{:<13} #{offset}", "goto")
            }
            Instruction::Getstatic(field_index) => format!("{:<13} #{field_index}", "getstatic"),
            no_arg => format!("{no_arg:?}"),
        };
        f.pad(&s) // I use instruction display inside other display, and need to apply padding explicitly
    }
}
