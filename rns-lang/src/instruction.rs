use jclass::bytecode::Opcode;
use phf::phf_map;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionOperand {
    None,
    // Constant pool references (parsed via type hints)
    MethodRef,
    FieldRef,
    TypeHint,
    // Raw values
    Byte,
    // Branch targets
    Label,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InstructionSpec {
    pub opcode: Opcode,
    pub operand: InstructionOperand,
}

macro_rules! define_instructions {
    (
        $(
            $variant:ident => {
                name: $name:literal,
                operand: $operand:ident,
            }
        ),* $(,)?
    ) => {
        pub static INSTRUCTION_SPECS: phf::Map<&'static str, InstructionSpec> = phf_map! {
            $(
                $name => InstructionSpec {
                    opcode: Opcode::$variant,
                    operand: InstructionOperand::$operand,
                },
            )*
        };
    };
}

define_instructions! {
    // No operand
    Aload0 => {
        name: "aload_0",
        operand: None,
    },
    Return => {
        name: "return",
        operand: None,
    },
    Iload0 => {
        name: "iload_0",
        operand: None,
    },
    Iconst0 => {
        name: "iconst_0",
        operand: None,
    },

    // Constant pool references
    InvokeSpecial => {
        name: "invokespecial",
        operand: MethodRef,
    },
    InvokeVirtual => {
        name: "invokevirtual",
        operand: MethodRef,
    },
    InvokeStatic => {
        name: "invokestatic",
        operand: MethodRef,
    },
    Getstatic => {
        name: "getstatic",
        operand: FieldRef,
    },
    Ldc => {
        name: "ldc",
        operand: TypeHint,
    },

    // Byte operand
    Bipush => {
        name: "bipush",
        operand: Byte,
    },

    // Label operand
    Goto => {
        name: "goto",
        operand: Label,
    },
    IfIcmpeq => {
        name: "if_icmpeq",
        operand: Label,
    },
}
