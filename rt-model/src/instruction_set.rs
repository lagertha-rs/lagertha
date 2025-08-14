use crate::JvmError;
use common::ByteCursor;
use num_enum::TryFromPrimitive;

/// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html
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
    pub fn new_instruction_set(code: &Vec<u8>) -> Result<Vec<Instruction>, JvmError> {
        let mut cursor = ByteCursor::new(code.as_slice());
        let mut res = Vec::new();

        while let Some(opcode_byte) = cursor.try_u8() {
            let opcode = Opcode::try_from(opcode_byte).map_err(|_| JvmError::TrailingBytes)?; //TODO: Err

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
