use crate::byte_cursor::ByteCursor;
use crate::rt::class::LoadingError;
use num_enum::TryFromPrimitive;
use std::fmt;
use std::fmt::Formatter;

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html
#[derive(Debug, Clone, Copy, TryFromPrimitive)]
#[repr(u8)]
pub enum Opcode {
    Aload0 = 0x2A,
    Aload1 = 0x2B,
    Aload2 = 0x2C,
    Aload3 = 0x2D,
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
    Invokespecial = 0xB7,
    Invokestatic = 0xB8,
    Invokevirtual = 0xB6,
    Ireturn = 0xAC,
    Ldc = 0x12,
    New = 0xBB,
    Return = 0xB1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    Aload0,
    Aload1,
    Aload2,
    Aload3,
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
    IfAcmpNe(i16),
    Ireturn,
    Invokespecial(u16),
    Invokestatic(u16),
    Invokevirtual(u16),
    Ldc(u16),
    New(u16),
    Return,
}

impl Instruction {
    pub fn byte_size(&self) -> u16 {
        match self {
            Self::Ldc(_) => 2,
            Self::New(_)
            | Self::Invokespecial(_)
            | Self::Invokestatic(_)
            | Self::Invokevirtual(_)
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
                Opcode::Invokespecial => Self::Invokespecial(cursor.u16()?),
                Opcode::Invokestatic => Self::Invokestatic(cursor.u16()?),
                Opcode::Invokevirtual => Self::Invokevirtual(cursor.u16()?),
                Opcode::Getstatic => Self::Getstatic(cursor.u16()?),
                Opcode::Goto => Self::Goto(cursor.i16()?),
                Opcode::Ldc => Self::Ldc(cursor.u8()? as u16),
                Opcode::IfAcmpne => Self::IfAcmpNe(cursor.i16()?),
                Opcode::New => Self::New(cursor.u16()?),
                Opcode::Aload0 => Self::Aload0,
                Opcode::Aload1 => Self::Aload1,
                Opcode::Aload2 => Self::Aload2,
                Opcode::Aload3 => Self::Aload3,
                Opcode::Return => Self::Return,
                Opcode::IconstM1 => Self::IconstM1,
                Opcode::Iconst0 => Self::Iconst0,
                Opcode::Iconst1 => Self::Iconst1,
                Opcode::Iconst2 => Self::Iconst2,
                Opcode::Iconst3 => Self::Iconst3,
                Opcode::Iconst4 => Self::Iconst4,
                Opcode::Iconst5 => Self::Iconst5,
                Opcode::Ireturn => Self::Ireturn,
                Opcode::Dup => Self::Ireturn,
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
