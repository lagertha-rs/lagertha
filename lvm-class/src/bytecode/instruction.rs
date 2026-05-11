//! JVM bytecode instruction representation.
//!
//! The `Instruction` enum represents fully decoded bytecode instructions with their operands.

use super::opcode::Opcode;
use super::operand::{ArrayType, LookupSwitchData, TableSwitchData};
use common::error::InstructionErr;
use common::utils::cursor::ByteCursor;
use num_enum::TryFromPrimitive;
use std::fmt::Formatter;

/// A fully decoded JVM bytecode instruction with operands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    Aaload,
    Aastore,
    AconstNull,
    Aload(u8),
    Aload0,
    Aload1,
    Aload2,
    Aload3,
    Anewarray(u16),
    Areturn,
    ArrayLength,
    Astore(u8),
    Astore0,
    Astore1,
    Astore2,
    Astore3,
    Athrow,
    Baload,
    Bastore,
    Bipush(i8),
    Breakpoint,
    Caload,
    Castore,
    Checkcast(u16),
    D2f,
    D2i,
    D2l,
    Dadd,
    Daload,
    Dastore,
    Dcmpg,
    Dcmpl,
    Dconst0,
    Dconst1,
    Ddiv,
    Dload(u8),
    Dload0,
    Dload1,
    Dload2,
    Dload3,
    Dmul,
    Dneg,
    Drem,
    Dreturn,
    Dstore(u8),
    Dstore0,
    Dstore1,
    Dstore2,
    Dstore3,
    Dsub,
    Dup,
    Dup2,
    Dup2X1,
    Dup2X2,
    DupX1,
    DupX2,
    F2d,
    F2i,
    F2l,
    Fadd,
    Faload,
    Fastore,
    Fcmpg,
    Fcmpl,
    Fconst0,
    Fconst1,
    Fconst2,
    Fdiv,
    Fload(u8),
    Fload0,
    Fload1,
    Fload2,
    Fload3,
    Fmul,
    Fneg,
    Frem,
    Freturn,
    Fstore(u8),
    Fstore0,
    Fstore1,
    Fstore2,
    Fstore3,
    Fsub,
    Getfield(u16),
    Getstatic(u16),
    Goto(i16),
    GotoW(i32),
    I2b,
    I2c,
    I2d,
    I2f,
    I2l,
    I2s,
    Iadd,
    Iaload,
    Iand,
    Iastore,
    IconstM1,
    Iconst0,
    Iconst1,
    Iconst2,
    Iconst3,
    Iconst4,
    Iconst5,
    Idiv,
    IfAcmpEq(i16),
    IfAcmpNe(i16),
    IfEq(i16),
    IfGe(i16),
    IfGt(i16),
    IfIcmpeq(i16),
    IfIcmpge(i16),
    IfIcmpgt(i16),
    IfIcmple(i16),
    IfIcmplt(i16),
    IfIcmpne(i16),
    IfLe(i16),
    IfLt(i16),
    IfNe(i16),
    Ifnonnull(i16),
    Ifnull(i16),
    Iinc(u8, i8),
    Iload(u8),
    Iload0,
    Iload1,
    Iload2,
    Iload3,
    Imul,
    Ineg,
    Lookupswitch(LookupSwitchData),
    Instanceof(u16),
    InvokeDynamic(u16),
    InvokeInterface(u16, u8),
    InvokeSpecial(u16),
    InvokeStatic(u16),
    InvokeVirtual(u16),
    Ior,
    Irem,
    Ireturn,
    Ishl,
    Ishr,
    Istore(u8),
    Istore0,
    Istore1,
    Istore2,
    Istore3,
    Isub,
    Iushr,
    Ixor,
    Jsr(i16),
    JsrW(i32),
    L2d,
    L2f,
    L2i,
    Ladd,
    Laload,
    Land,
    Lastore,
    Lcmp,
    Lconst0,
    Lconst1,
    Ldc(u16),
    Ldc2W(u16),
    LdcW(u16),
    Ldiv,
    Lload(u8),
    Lload0,
    Lload1,
    Lload2,
    Lload3,
    Lmul,
    Lneg,
    Lor,
    Lrem,
    Lreturn,
    Lshl,
    Lshr,
    Lstore(u8),
    Lstore0,
    Lstore1,
    Lstore2,
    Lstore3,
    Lsub,
    Lushr,
    Lxor,
    Monitorenter,
    Monitorexit,
    Multianewarray(u16, u8),
    New(u16),
    Newarray(ArrayType),
    Nop,
    Pop,
    Pop2,
    Putfield(u16),
    Putstatic(u16),
    Ret(u8),
    Return,
    Saload,
    Sastore,
    Sipush(i16),
    Swap,
    Impdep1,
    Impdep2,
    TableSwitch(TableSwitchData),
}

impl Instruction {
    pub fn is_branch(&self) -> bool {
        matches!(
            self,
            Self::Goto(_)
                | Self::GotoW(_)
                | Self::Jsr(_)
                | Self::JsrW(_)
                | Self::IfAcmpEq(_)
                | Self::IfAcmpNe(_)
                | Self::IfEq(_)
                | Self::IfGe(_)
                | Self::IfGt(_)
                | Self::IfLe(_)
                | Self::IfLt(_)
                | Self::IfNe(_)
                | Self::Ifnonnull(_)
                | Self::Ifnull(_)
                | Self::IfIcmpeq(_)
                | Self::IfIcmpge(_)
                | Self::IfIcmpgt(_)
                | Self::IfIcmple(_)
                | Self::IfIcmplt(_)
                | Self::IfIcmpne(_)
                | Self::Lookupswitch(_)
                | Self::TableSwitch(_)
        )
    }

    pub fn byte_size(&self) -> u16 {
        match self {
            // Variable-size instructions
            Self::Lookupswitch(data) => {
                // opcode (1) + padding (0-3) + default (4) + npairs (4) + pairs (8 * n)
                1 + data.padding as u16 + 4 + 4 + (8 * data.pairs.len() as u16)
            }
            Self::TableSwitch(data) => {
                // opcode (1) + padding (0-3) + default (4) + low (4) + high (4) + offsets (4 * n)
                1 + data.padding as u16 + 4 + 4 + 4 + (4 * data.offsets.len() as u16)
            }
            // 5-byte instructions
            Self::GotoW(_)
            | Self::JsrW(_)
            | Self::InvokeDynamic(_)
            | Self::InvokeInterface(_, _) => 5,

            // 4-byte instructions
            Self::Multianewarray(_, _) => 4,

            // 3-byte instructions
            Self::Anewarray(_)
            | Self::Checkcast(_)
            | Self::Getfield(_)
            | Self::Getstatic(_)
            | Self::Goto(_)
            | Self::IfAcmpEq(_)
            | Self::IfAcmpNe(_)
            | Self::IfEq(_)
            | Self::IfGe(_)
            | Self::IfGt(_)
            | Self::IfLe(_)
            | Self::IfLt(_)
            | Self::IfNe(_)
            | Self::Ifnonnull(_)
            | Self::Ifnull(_)
            | Self::IfIcmpeq(_)
            | Self::IfIcmpge(_)
            | Self::IfIcmpgt(_)
            | Self::IfIcmple(_)
            | Self::IfIcmplt(_)
            | Self::IfIcmpne(_)
            | Self::Iinc(_, _)
            | Self::Instanceof(_)
            | Self::InvokeSpecial(_)
            | Self::InvokeStatic(_)
            | Self::InvokeVirtual(_)
            | Self::Ldc2W(_)
            | Self::LdcW(_)
            | Self::New(_)
            | Self::Putfield(_)
            | Self::Putstatic(_)
            | Self::Sipush(_)
            | Self::Jsr(_) => 3,

            // 2-byte instructions
            Self::Aload(_)
            | Self::Astore(_)
            | Self::Bipush(_)
            | Self::Dload(_)
            | Self::Dstore(_)
            | Self::Fload(_)
            | Self::Fstore(_)
            | Self::Iload(_)
            | Self::Istore(_)
            | Self::Ldc(_)
            | Self::Lload(_)
            | Self::Lstore(_)
            | Self::Newarray(_)
            | Self::Ret(_) => 2,

            // 1-byte instructions (everything else)
            _ => 1,
        }
    }
}

impl Instruction {
    fn switch_padding(pc: usize) -> u8 {
        ((4 - ((pc + 1) & 3)) & 3) as u8
    }

    pub fn new_at(code: &[u8], pc: usize) -> Result<Instruction, InstructionErr> {
        let mut cursor = ByteCursor::new(&code[pc..]);
        let opcode_byte = cursor.u8()?;
        let opcode = Opcode::try_from(opcode_byte)
            .map_err(|_| InstructionErr::UnsupportedOpCode(opcode_byte))?;

        let instruction = match opcode {
            Opcode::Aaload => Self::Aaload,
            Opcode::Aastore => Self::Aastore,
            Opcode::AconstNull => Self::AconstNull,
            Opcode::Aload => Self::Aload(cursor.u8()?),
            Opcode::Aload0 => Self::Aload0,
            Opcode::Aload1 => Self::Aload1,
            Opcode::Aload2 => Self::Aload2,
            Opcode::Aload3 => Self::Aload3,
            Opcode::Anewarray => Self::Anewarray(cursor.u16()?),
            Opcode::Areturn => Self::Areturn,
            Opcode::ArrayLength => Self::ArrayLength,
            Opcode::Astore => Self::Astore(cursor.u8()?),
            Opcode::Astore0 => Self::Astore0,
            Opcode::Astore1 => Self::Astore1,
            Opcode::Astore2 => Self::Astore2,
            Opcode::Astore3 => Self::Astore3,
            Opcode::Athrow => Self::Athrow,
            Opcode::Baload => Self::Baload,
            Opcode::Bastore => Self::Bastore,
            Opcode::Bipush => Self::Bipush(cursor.i8()?),
            Opcode::Breakpoint => Self::Breakpoint,
            Opcode::Caload => Self::Caload,
            Opcode::Castore => Self::Castore,
            Opcode::Checkcast => Self::Checkcast(cursor.u16()?),
            Opcode::D2f => Self::D2f,
            Opcode::D2i => Self::D2i,
            Opcode::D2l => Self::D2l,
            Opcode::Dadd => Self::Dadd,
            Opcode::Daload => Self::Daload,
            Opcode::Dastore => Self::Dastore,
            Opcode::Dcmpg => Self::Dcmpg,
            Opcode::Dcmpl => Self::Dcmpl,
            Opcode::Dconst0 => Self::Dconst0,
            Opcode::Dconst1 => Self::Dconst1,
            Opcode::Ddiv => Self::Ddiv,
            Opcode::Dload => Self::Dload(cursor.u8()?),
            Opcode::Dload0 => Self::Dload0,
            Opcode::Dload1 => Self::Dload1,
            Opcode::Dload2 => Self::Dload2,
            Opcode::Dload3 => Self::Dload3,
            Opcode::Dmul => Self::Dmul,
            Opcode::Dneg => Self::Dneg,
            Opcode::Drem => Self::Drem,
            Opcode::Dreturn => Self::Dreturn,
            Opcode::Dstore => Self::Dstore(cursor.u8()?),
            Opcode::Dstore0 => Self::Dstore0,
            Opcode::Dstore1 => Self::Dstore1,
            Opcode::Dstore2 => Self::Dstore2,
            Opcode::Dstore3 => Self::Dstore3,
            Opcode::Dsub => Self::Dsub,
            Opcode::Dup => Self::Dup,
            Opcode::Dup2 => Self::Dup2,
            Opcode::Dup2X1 => Self::Dup2X1,
            Opcode::Dup2X2 => Self::Dup2X2,
            Opcode::DupX1 => Self::DupX1,
            Opcode::DupX2 => Self::DupX2,
            Opcode::F2d => Self::F2d,
            Opcode::F2i => Self::F2i,
            Opcode::F2l => Self::F2l,
            Opcode::Fadd => Self::Fadd,
            Opcode::Faload => Self::Faload,
            Opcode::Fastore => Self::Fastore,
            Opcode::Fcmpg => Self::Fcmpg,
            Opcode::Fcmpl => Self::Fcmpl,
            Opcode::Fconst0 => Self::Fconst0,
            Opcode::Fconst1 => Self::Fconst1,
            Opcode::Fconst2 => Self::Fconst2,
            Opcode::Fdiv => Self::Fdiv,
            Opcode::Fload => Self::Fload(cursor.u8()?),
            Opcode::Fload0 => Self::Fload0,
            Opcode::Fload1 => Self::Fload1,
            Opcode::Fload2 => Self::Fload2,
            Opcode::Fload3 => Self::Fload3,
            Opcode::Fmul => Self::Fmul,
            Opcode::Fneg => Self::Fneg,
            Opcode::Frem => Self::Frem,
            Opcode::Freturn => Self::Freturn,
            Opcode::Fstore => Self::Fstore(cursor.u8()?),
            Opcode::Fstore0 => Self::Fstore0,
            Opcode::Fstore1 => Self::Fstore1,
            Opcode::Fstore2 => Self::Fstore2,
            Opcode::Fstore3 => Self::Fstore3,
            Opcode::Fsub => Self::Fsub,
            Opcode::Getfield => Self::Getfield(cursor.u16()?),
            Opcode::Getstatic => Self::Getstatic(cursor.u16()?),
            Opcode::Goto => Self::Goto(cursor.i16()?),
            Opcode::GotoW => Self::GotoW(cursor.i32()?),
            Opcode::I2b => Self::I2b,
            Opcode::I2c => Self::I2c,
            Opcode::I2d => Self::I2d,
            Opcode::I2f => Self::I2f,
            Opcode::I2l => Self::I2l,
            Opcode::I2s => Self::I2s,
            Opcode::Iadd => Self::Iadd,
            Opcode::Iaload => Self::Iaload,
            Opcode::Iand => Self::Iand,
            Opcode::Iastore => Self::Iastore,
            Opcode::IconstM1 => Self::IconstM1,
            Opcode::Iconst0 => Self::Iconst0,
            Opcode::Iconst1 => Self::Iconst1,
            Opcode::Iconst2 => Self::Iconst2,
            Opcode::Iconst3 => Self::Iconst3,
            Opcode::Iconst4 => Self::Iconst4,
            Opcode::Iconst5 => Self::Iconst5,
            Opcode::Idiv => Self::Idiv,
            Opcode::IfAcmpEq => Self::IfAcmpEq(cursor.i16()?),
            Opcode::IfAcmpNe => Self::IfAcmpNe(cursor.i16()?),
            Opcode::IfEq => Self::IfEq(cursor.i16()?),
            Opcode::IfGe => Self::IfGe(cursor.i16()?),
            Opcode::IfGt => Self::IfGt(cursor.i16()?),
            Opcode::IfLe => Self::IfLe(cursor.i16()?),
            Opcode::IfLt => Self::IfLt(cursor.i16()?),
            Opcode::IfNe => Self::IfNe(cursor.i16()?),
            Opcode::Ifnonnull => Self::Ifnonnull(cursor.i16()?),
            Opcode::Ifnull => Self::Ifnull(cursor.i16()?),
            Opcode::IfIcmpeq => Self::IfIcmpeq(cursor.i16()?),
            Opcode::IfIcmpge => Self::IfIcmpge(cursor.i16()?),
            Opcode::IfIcmpgt => Self::IfIcmpgt(cursor.i16()?),
            Opcode::IfIcmple => Self::IfIcmple(cursor.i16()?),
            Opcode::IfIcmplt => Self::IfIcmplt(cursor.i16()?),
            Opcode::IfIcmpne => Self::IfIcmpne(cursor.i16()?),
            Opcode::Iinc => Self::Iinc(cursor.u8()?, cursor.i8()?),
            Opcode::Iload => Self::Iload(cursor.u8()?),
            Opcode::Iload0 => Self::Iload0,
            Opcode::Iload1 => Self::Iload1,
            Opcode::Iload2 => Self::Iload2,
            Opcode::Iload3 => Self::Iload3,
            Opcode::Imul => Self::Imul,
            Opcode::Ineg => Self::Ineg,
            Opcode::Instanceof => Self::Instanceof(cursor.u16()?),
            Opcode::InvokeDynamic => {
                let index = cursor.u16()?;
                let _zero = cursor.u16()?; //TODO assert?
                Self::InvokeDynamic(index)
            }
            Opcode::InvokeInterface => {
                let index = cursor.u16()?;
                let count = cursor.u8()?;
                let _zero = cursor.u8()?; //TODO assert?
                Self::InvokeInterface(index, count)
            }
            Opcode::InvokeSpecial => Self::InvokeSpecial(cursor.u16()?),
            Opcode::InvokeStatic => Self::InvokeStatic(cursor.u16()?),
            Opcode::InvokeVirtual => Self::InvokeVirtual(cursor.u16()?),
            Opcode::Ior => Self::Ior,
            Opcode::Irem => Self::Irem,
            Opcode::Ireturn => Self::Ireturn,
            Opcode::Ishl => Self::Ishl,
            Opcode::Ishr => Self::Ishr,
            Opcode::Istore => Self::Istore(cursor.u8()?),
            Opcode::Istore0 => Self::Istore0,
            Opcode::Istore1 => Self::Istore1,
            Opcode::Istore2 => Self::Istore2,
            Opcode::Istore3 => Self::Istore3,
            Opcode::Isub => Self::Isub,
            Opcode::Iushr => Self::Iushr,
            Opcode::Ixor => Self::Ixor,
            Opcode::Jsr => Self::Jsr(cursor.i16()?),
            Opcode::JsrW => Self::JsrW(cursor.i32()?),
            Opcode::L2d => Self::L2d,
            Opcode::L2f => Self::L2f,
            Opcode::L2i => Self::L2i,
            Opcode::Ladd => Self::Ladd,
            Opcode::Laload => Self::Laload,
            Opcode::Land => Self::Land,
            Opcode::Lastore => Self::Lastore,
            Opcode::Lcmp => Self::Lcmp,
            Opcode::Lconst0 => Self::Lconst0,
            Opcode::Lconst1 => Self::Lconst1,
            Opcode::Ldc => Self::Ldc(cursor.u8()? as u16),
            Opcode::Ldc2W => Self::Ldc2W(cursor.u16()?),
            Opcode::LdcW => Self::LdcW(cursor.u16()?),
            Opcode::Ldiv => Self::Ldiv,
            Opcode::Lload => Self::Lload(cursor.u8()?),
            Opcode::Lload0 => Self::Lload0,
            Opcode::Lload1 => Self::Lload1,
            Opcode::Lload2 => Self::Lload2,
            Opcode::Lload3 => Self::Lload3,
            Opcode::Lmul => Self::Lmul,
            Opcode::Lneg => Self::Lneg,
            Opcode::Lookupswitch => {
                let padding = Self::switch_padding(pc);
                for _ in 0..padding {
                    cursor.u8()?;
                }

                let default_offset = cursor.i32()?;
                let npairs = cursor.i32()?;
                let mut pairs = Vec::with_capacity(npairs as usize);
                for _ in 0..npairs {
                    let match_value = cursor.i32()?;
                    let offset = cursor.i32()?;
                    pairs.push((match_value, offset));
                }
                Instruction::Lookupswitch(LookupSwitchData {
                    padding,
                    default_offset,
                    pairs,
                })
            }
            Opcode::Lor => Self::Lor,
            Opcode::Lrem => Self::Lrem,
            Opcode::Lreturn => Self::Lreturn,
            Opcode::Lshl => Self::Lshl,
            Opcode::Lshr => Self::Lshr,
            Opcode::Lstore => Self::Lstore(cursor.u8()?),
            Opcode::Lstore0 => Self::Lstore0,
            Opcode::Lstore1 => Self::Lstore1,
            Opcode::Lstore2 => Self::Lstore2,
            Opcode::Lstore3 => Self::Lstore3,
            Opcode::Lsub => Self::Lsub,
            Opcode::Lushr => Self::Lushr,
            Opcode::Lxor => Self::Lxor,
            Opcode::Monitorenter => Self::Monitorenter,
            Opcode::Monitorexit => Self::Monitorexit,
            Opcode::Multianewarray => Self::Multianewarray(cursor.u16()?, cursor.u8()?),
            Opcode::New => Self::New(cursor.u16()?),
            Opcode::Newarray => {
                let array_type_raw = cursor.u8()?;
                let array_type = ArrayType::try_from_primitive(array_type_raw)
                    .map_err(|_| InstructionErr::UnknownArrayType(array_type_raw))?;
                Self::Newarray(array_type)
            }
            Opcode::Nop => Self::Nop,
            Opcode::Pop => Self::Pop,
            Opcode::Pop2 => Self::Pop2,
            Opcode::Putfield => Self::Putfield(cursor.u16()?),
            Opcode::Putstatic => Self::Putstatic(cursor.u16()?),
            Opcode::Ret => Self::Ret(cursor.u8()?),
            Opcode::Return => Self::Return,
            Opcode::Saload => Self::Saload,
            Opcode::Sastore => Self::Sastore,
            Opcode::Sipush => Self::Sipush(cursor.i16()?),
            Opcode::Swap => Self::Swap,
            Opcode::TableSwitch => {
                let padding = Self::switch_padding(pc);
                for _ in 0..padding {
                    cursor.u8()?;
                }

                let default_offset = cursor.i32()?;
                let low = cursor.i32()?;
                let high = cursor.i32()?;
                let num_offsets = (high - low + 1) as usize;
                let mut offsets = Vec::with_capacity(num_offsets);
                for _ in 0..num_offsets {
                    offsets.push(cursor.i32()?);
                }
                Instruction::TableSwitch(TableSwitchData {
                    padding,
                    default_offset,
                    low,
                    high,
                    offsets,
                })
            }
        };

        Ok(instruction)
    }

    pub fn opcode(&self) -> Option<Opcode> {
        let op = match self {
            Self::Aaload => Opcode::Aaload,
            Self::Aastore => Opcode::Aastore,
            Self::AconstNull => Opcode::AconstNull,
            Self::Aload(_) => Opcode::Aload,
            Self::Aload0 => Opcode::Aload0,
            Self::Aload1 => Opcode::Aload1,
            Self::Aload2 => Opcode::Aload2,
            Self::Aload3 => Opcode::Aload3,
            Self::Anewarray(_) => Opcode::Anewarray,
            Self::Areturn => Opcode::Areturn,
            Self::ArrayLength => Opcode::ArrayLength,
            Self::Astore(_) => Opcode::Astore,
            Self::Astore0 => Opcode::Astore0,
            Self::Astore1 => Opcode::Astore1,
            Self::Astore2 => Opcode::Astore2,
            Self::Astore3 => Opcode::Astore3,
            Self::Athrow => Opcode::Athrow,
            Self::Baload => Opcode::Baload,
            Self::Bastore => Opcode::Bastore,
            Self::Bipush(_) => Opcode::Bipush,
            Self::Breakpoint => Opcode::Breakpoint,
            Self::Caload => Opcode::Caload,
            Self::Castore => Opcode::Castore,
            Self::Checkcast(_) => Opcode::Checkcast,
            Self::D2f => Opcode::D2f,
            Self::D2i => Opcode::D2i,
            Self::D2l => Opcode::D2l,
            Self::Dadd => Opcode::Dadd,
            Self::Daload => Opcode::Daload,
            Self::Dastore => Opcode::Dastore,
            Self::Dcmpg => Opcode::Dcmpg,
            Self::Dcmpl => Opcode::Dcmpl,
            Self::Dconst0 => Opcode::Dconst0,
            Self::Dconst1 => Opcode::Dconst1,
            Self::Ddiv => Opcode::Ddiv,
            Self::Dload(_) => Opcode::Dload,
            Self::Dload0 => Opcode::Dload0,
            Self::Dload1 => Opcode::Dload1,
            Self::Dload2 => Opcode::Dload2,
            Self::Dload3 => Opcode::Dload3,
            Self::Dmul => Opcode::Dmul,
            Self::Dneg => Opcode::Dneg,
            Self::Drem => Opcode::Drem,
            Self::Dreturn => Opcode::Dreturn,
            Self::Dstore(_) => Opcode::Dstore,
            Self::Dstore0 => Opcode::Dstore0,
            Self::Dstore1 => Opcode::Dstore1,
            Self::Dstore2 => Opcode::Dstore2,
            Self::Dstore3 => Opcode::Dstore3,
            Self::Dsub => Opcode::Dsub,
            Self::Dup => Opcode::Dup,
            Self::Dup2 => Opcode::Dup2,
            Self::Dup2X1 => Opcode::Dup2X1,
            Self::Dup2X2 => Opcode::Dup2X2,
            Self::DupX1 => Opcode::DupX1,
            Self::DupX2 => Opcode::DupX2,
            Self::F2d => Opcode::F2d,
            Self::F2i => Opcode::F2i,
            Self::F2l => Opcode::F2l,
            Self::Fadd => Opcode::Fadd,
            Self::Faload => Opcode::Faload,
            Self::Fastore => Opcode::Fastore,
            Self::Fcmpg => Opcode::Fcmpg,
            Self::Fcmpl => Opcode::Fcmpl,
            Self::Fconst0 => Opcode::Fconst0,
            Self::Fconst1 => Opcode::Fconst1,
            Self::Fconst2 => Opcode::Fconst2,
            Self::Fdiv => Opcode::Fdiv,
            Self::Fload(_) => Opcode::Fload,
            Self::Fload0 => Opcode::Fload0,
            Self::Fload1 => Opcode::Fload1,
            Self::Fload2 => Opcode::Fload2,
            Self::Fload3 => Opcode::Fload3,
            Self::Fmul => Opcode::Fmul,
            Self::Fneg => Opcode::Fneg,
            Self::Frem => Opcode::Frem,
            Self::Freturn => Opcode::Freturn,
            Self::Fstore(_) => Opcode::Fstore,
            Self::Fstore0 => Opcode::Fstore0,
            Self::Fstore1 => Opcode::Fstore1,
            Self::Fstore2 => Opcode::Fstore2,
            Self::Fstore3 => Opcode::Fstore3,
            Self::Fsub => Opcode::Fsub,
            Self::Getfield(_) => Opcode::Getfield,
            Self::Getstatic(_) => Opcode::Getstatic,
            Self::Goto(_) => Opcode::Goto,
            Self::GotoW(_) => Opcode::GotoW,
            Self::I2b => Opcode::I2b,
            Self::I2c => Opcode::I2c,
            Self::I2d => Opcode::I2d,
            Self::I2f => Opcode::I2f,
            Self::I2l => Opcode::I2l,
            Self::I2s => Opcode::I2s,
            Self::Iadd => Opcode::Iadd,
            Self::Iaload => Opcode::Iaload,
            Self::Iand => Opcode::Iand,
            Self::Iastore => Opcode::Iastore,
            Self::IconstM1 => Opcode::IconstM1,
            Self::Iconst0 => Opcode::Iconst0,
            Self::Iconst1 => Opcode::Iconst1,
            Self::Iconst2 => Opcode::Iconst2,
            Self::Iconst3 => Opcode::Iconst3,
            Self::Iconst4 => Opcode::Iconst4,
            Self::Iconst5 => Opcode::Iconst5,
            Self::Idiv => Opcode::Idiv,
            Self::IfAcmpEq(_) => Opcode::IfAcmpEq,
            Self::IfAcmpNe(_) => Opcode::IfAcmpNe,
            Self::IfEq(_) => Opcode::IfEq,
            Self::IfGe(_) => Opcode::IfGe,
            Self::IfGt(_) => Opcode::IfGt,
            Self::IfLe(_) => Opcode::IfLe,
            Self::IfLt(_) => Opcode::IfLt,
            Self::IfNe(_) => Opcode::IfNe,
            Self::Ifnonnull(_) => Opcode::Ifnonnull,
            Self::Ifnull(_) => Opcode::Ifnull,
            Self::IfIcmpeq(_) => Opcode::IfIcmpeq,
            Self::IfIcmpge(_) => Opcode::IfIcmpge,
            Self::IfIcmpgt(_) => Opcode::IfIcmpgt,
            Self::IfIcmple(_) => Opcode::IfIcmple,
            Self::IfIcmplt(_) => Opcode::IfIcmplt,
            Self::IfIcmpne(_) => Opcode::IfIcmpne,
            Self::Iinc(_, _) => Opcode::Iinc,
            Self::Iload(_) => Opcode::Iload,
            Self::Iload0 => Opcode::Iload0,
            Self::Iload1 => Opcode::Iload1,
            Self::Iload2 => Opcode::Iload2,
            Self::Iload3 => Opcode::Iload3,
            Self::Imul => Opcode::Imul,
            Self::Ineg => Opcode::Ineg,
            Self::Instanceof(_) => Opcode::Instanceof,
            Self::InvokeDynamic(_) => Opcode::InvokeDynamic,
            Self::InvokeInterface(_, _) => Opcode::InvokeInterface,
            Self::InvokeSpecial(_) => Opcode::InvokeSpecial,
            Self::InvokeStatic(_) => Opcode::InvokeStatic,
            Self::InvokeVirtual(_) => Opcode::InvokeVirtual,
            Self::Ior => Opcode::Ior,
            Self::Irem => Opcode::Irem,
            Self::Ireturn => Opcode::Ireturn,
            Self::Ishl => Opcode::Ishl,
            Self::Ishr => Opcode::Ishr,
            Self::Istore(_) => Opcode::Istore,
            Self::Istore0 => Opcode::Istore0,
            Self::Istore1 => Opcode::Istore1,
            Self::Istore2 => Opcode::Istore2,
            Self::Istore3 => Opcode::Istore3,
            Self::Isub => Opcode::Isub,
            Self::Iushr => Opcode::Iushr,
            Self::Ixor => Opcode::Ixor,
            Self::Jsr(_) => Opcode::Jsr,
            Self::JsrW(_) => Opcode::JsrW,
            Self::L2d => Opcode::L2d,
            Self::L2f => Opcode::L2f,
            Self::L2i => Opcode::L2i,
            Self::Ladd => Opcode::Ladd,
            Self::Laload => Opcode::Laload,
            Self::Land => Opcode::Land,
            Self::Lastore => Opcode::Lastore,
            Self::Lcmp => Opcode::Lcmp,
            Self::Lconst0 => Opcode::Lconst0,
            Self::Lconst1 => Opcode::Lconst1,
            Self::Ldc(_) => Opcode::Ldc,
            Self::Ldc2W(_) => Opcode::Ldc2W,
            Self::LdcW(_) => Opcode::LdcW,
            Self::Ldiv => Opcode::Ldiv,
            Self::Lload(_) => Opcode::Lload,
            Self::Lload0 => Opcode::Lload0,
            Self::Lload1 => Opcode::Lload1,
            Self::Lload2 => Opcode::Lload2,
            Self::Lload3 => Opcode::Lload3,
            Self::Lmul => Opcode::Lmul,
            Self::Lneg => Opcode::Lneg,
            Self::Lookupswitch(_) => Opcode::Lookupswitch,
            Self::Lor => Opcode::Lor,
            Self::Lrem => Opcode::Lrem,
            Self::Lreturn => Opcode::Lreturn,
            Self::Lshl => Opcode::Lshl,
            Self::Lshr => Opcode::Lshr,
            Self::Lstore(_) => Opcode::Lstore,
            Self::Lstore0 => Opcode::Lstore0,
            Self::Lstore1 => Opcode::Lstore1,
            Self::Lstore2 => Opcode::Lstore2,
            Self::Lstore3 => Opcode::Lstore3,
            Self::Lsub => Opcode::Lsub,
            Self::Lushr => Opcode::Lushr,
            Self::Lxor => Opcode::Lxor,
            Self::Monitorenter => Opcode::Monitorenter,
            Self::Monitorexit => Opcode::Monitorexit,
            Self::Multianewarray(_, _) => Opcode::Multianewarray,
            Self::New(_) => Opcode::New,
            Self::Newarray(_) => Opcode::Newarray,
            Self::Nop => Opcode::Nop,
            Self::Pop => Opcode::Pop,
            Self::Pop2 => Opcode::Pop2,
            Self::Putfield(_) => Opcode::Putfield,
            Self::Putstatic(_) => Opcode::Putstatic,
            Self::Ret(_) => Opcode::Ret,
            Self::Return => Opcode::Return,
            Self::Saload => Opcode::Saload,
            Self::Sastore => Opcode::Sastore,
            Self::Sipush(_) => Opcode::Sipush,
            Self::Swap => Opcode::Swap,
            Self::TableSwitch(_) => Opcode::TableSwitch,
            Self::Impdep1 | Self::Impdep2 => return None,
        };
        Some(op)
    }

    /// Returns the mnemonic name of this instruction (e.g. `"aload"`, `"invokespecial"`).
    ///
    /// Delegates to [`Opcode::as_str`] for standard instructions. Reserved
    /// instructions (`impdep1`, `impdep2`) are handled as special cases since
    /// they have no corresponding [`Opcode`] variant.
    pub fn get_name(&self) -> &'static str {
        match self {
            Self::Impdep1 => "impdep1",
            Self::Impdep2 => "impdep2",
            // SAFETY: all other variants have a corresponding Opcode
            _ => self.opcode().unwrap().as_str(),
        }
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.get_name())
    }
}
