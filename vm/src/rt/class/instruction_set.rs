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
    Invokespecial = 0xB7,
    Invokevirtual = 0xB6,
    Return = 0xb1,
    Getstatic = 0xb2,
    Ldc = 0x12,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    Aload0,
    Ldc { index: u16 },
    Invokespecial { method_index: u16 },
    Invokevirtual { method_index: u16 },
    Getstatic { field_index: u16 },
    Return,
}

impl Instruction {
    pub fn byte_size(&self) -> u16 {
        match self {
            Instruction::Aload0 => 1,
            Instruction::Ldc { .. } => 2,
            Instruction::Invokespecial { .. } => 3,
            Instruction::Invokevirtual { .. } => 3,
            Instruction::Getstatic { .. } => 3,
            Instruction::Return => 1,
        }
    }
}

impl Instruction {
    pub fn new_instruction_set(code: Vec<u8>) -> Result<Vec<Instruction>, LoadingError> {
        let mut cursor = ByteCursor::new(code.as_slice());
        let mut res = Vec::new();

        while let Some(opcode_byte) = cursor.try_u8() {
            let opcode = Opcode::try_from(opcode_byte).map_err(|_| LoadingError::UnknownOpCode)?; //TODO: Err

            let instruction = match opcode {
                Opcode::Invokespecial => {
                    let method_index = ((cursor.u8()? as u16) << 8) | cursor.u8()? as u16;
                    Instruction::Invokespecial { method_index }
                }
                Opcode::Invokevirtual => {
                    let method_index = ((cursor.u8()? as u16) << 8) | cursor.u8()? as u16;
                    Instruction::Invokevirtual { method_index }
                }

                Opcode::Getstatic => {
                    let field_index = ((cursor.u8()? as u16) << 8) | cursor.u8()? as u16;
                    Instruction::Getstatic { field_index }
                }
                Opcode::Ldc => Instruction::Ldc {
                    index: cursor.u8()? as u16,
                },
                Opcode::Aload0 => Instruction::Aload0,
                Opcode::Return => Instruction::Return,
            };
            res.push(instruction)
        }
        Ok(res)
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            Instruction::Aload0 => "aload_0".to_string(),
            Instruction::Ldc { index } => format!("{:<13} #{index}", "ldc"),
            Instruction::Invokespecial { method_index } => {
                format!("{:<13} #{method_index}", "invokespecial")
            }
            Instruction::Invokevirtual { method_index } => {
                format!("{:<13} #{method_index}", "invokevirtual")
            }
            Instruction::Getstatic { field_index } => format!("{:<13} #{field_index}", "getstatic"),
            Instruction::Return => "return".to_string(),
        };
        f.pad(&s) // I use instruction display inside other display, and need to apply padding explicitly
    }
}
