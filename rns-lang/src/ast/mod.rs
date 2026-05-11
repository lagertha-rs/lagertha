pub mod flag;

use crate::ast::flag::RnsMethodFlag;
use crate::diagnostic::Diagnostic;
use crate::instruction::InstructionSpec;
use crate::token::type_hint::TypeHint;
use crate::token::{Span, Spanned};
use flag::RnsClassFlag;
use std::collections::{BTreeMap, HashMap};

pub struct RnsModule {
    pub class_dir: ClassDirective,
    pub super_dir: Option<SuperDirective>,
    pub diagnostics: Vec<Diagnostic>,
    pub methods: Vec<MethodDirective>,
}

pub struct SuperDirective {
    pub dir_span: Option<Span>,
    pub name: TypeHint,
}

pub struct ClassDirective {
    pub dir_span: Span,
    pub name: Option<TypeHint>,
    // TODO: BTreeMap because I need it to be sorted for my snapshot test. investigate impact
    pub flags: BTreeMap<RnsClassFlag, Span>,
}

pub struct MethodDirective {
    pub dir_span: Span,
    pub name: Option<TypeHint>,
    pub descriptor: Option<TypeHint>,
    pub flags: BTreeMap<RnsMethodFlag, Span>,
    pub code_dir: Option<CodeDirective>,
}

impl MethodDirective {
    pub fn new(dir_span: Span, flags: BTreeMap<RnsMethodFlag, Span>) -> Self {
        Self {
            dir_span,
            flags,
            name: None,
            descriptor: None,
            code_dir: None,
        }
    }
}

pub enum RnsOperand {
    CpRef(TypeHint),
    Byte(Spanned<u8>),
    Label(Spanned<String>),
}

pub struct RnsInstruction {
    pub ins_span: Span,
    pub spec: InstructionSpec,
    pub operand: Option<RnsOperand>,
}

impl RnsInstruction {
    pub fn new(ins_span: Span, spec: InstructionSpec, operand: RnsOperand) -> Self {
        Self {
            ins_span,
            spec,
            operand: Some(operand),
        }
    }

    pub fn new_without_operand(ins_span: Span, spec: InstructionSpec) -> Self {
        Self {
            ins_span,
            spec,
            operand: None,
        }
    }
}

pub struct CodeDirective {
    pub dir_span: Span,
    pub instructions: Vec<RnsInstruction>,
    pub labels: HashMap<String, u32>,
    pub max_stack: u16,
    pub max_locals: u16,
}
