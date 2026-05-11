use crate::diagnostic::{
    ERR_CODE_CLASS_DEF_TRAILING_TOK, ERR_CODE_CLASS_END_TRAILING_TOK, ERR_CODE_INVALID_CLASS_FLAG,
    ERR_CODE_INVALID_METHOD_FLAG, ERR_CODE_METHOD_TRAILING_TOK, ERR_CODE_SUPER_TRAILING_TOK,
    ERR_CODE_TH_TRAILING_TOK, ERR_CODE_TOKEN_OUTSIDE_CLASS, ERR_CODE_UNEXPECTED_TOKEN_IN_CLASS,
    ERR_CODE_UNEXPECTED_TOKEN_IN_METHOD,
};
use crate::instruction::InstructionSpec;
use crate::token::Spanned;
use crate::token::type_hint::TypeHintKind;
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub(in crate::parser) enum AccessFlagContext {
    Class,
    Method,
}

impl Display for AccessFlagContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AccessFlagContext::Class => write!(f, "class"),
            AccessFlagContext::Method => write!(f, "method"),
        }
    }
}

impl AccessFlagContext {
    pub(in crate::parser) fn error_code(&self) -> &'static str {
        match self {
            AccessFlagContext::Class => ERR_CODE_INVALID_CLASS_FLAG,
            AccessFlagContext::Method => ERR_CODE_INVALID_METHOD_FLAG,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub(in crate::parser) enum UnexpectedTokenContext {
    BeforeClassDefinition,
    AfterClassDefinition,
    ClassBody,
    MethodBody,
}

impl Display for UnexpectedTokenContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UnexpectedTokenContext::BeforeClassDefinition => {
                write!(f, "before class definition")
            }
            UnexpectedTokenContext::AfterClassDefinition => {
                write!(f, "after class definition")
            }
            UnexpectedTokenContext::ClassBody => write!(f, "class body"),
            UnexpectedTokenContext::MethodBody => write!(f, "method body"),
        }
    }
}

impl UnexpectedTokenContext {
    pub(in crate::parser) fn error_code(&self) -> &'static str {
        match self {
            UnexpectedTokenContext::BeforeClassDefinition
            | UnexpectedTokenContext::AfterClassDefinition => ERR_CODE_TOKEN_OUTSIDE_CLASS,
            UnexpectedTokenContext::ClassBody => ERR_CODE_UNEXPECTED_TOKEN_IN_CLASS,
            UnexpectedTokenContext::MethodBody => ERR_CODE_UNEXPECTED_TOKEN_IN_METHOD,
        }
    }
}

// TODO: Rename because not only about operands, for example instruction name...
#[derive(Debug, Eq, PartialEq, Clone)]
pub(in crate::parser) enum OperandErrPosContext {
    ClassName,
    SuperName,
    MethodName,
    MethodDescriptor,
    InstructionName,
    InstructionOperand(InstructionSpec),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub(in crate::parser) enum TrailingTokensErrContext {
    Class,
    Super,
    Method,
    TypeHint(Spanned<TypeHintKind>),
    ClassEnd,
}

impl Display for TrailingTokensErrContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TrailingTokensErrContext::Class => write!(f, "class definition"),
            TrailingTokensErrContext::Super => write!(f, "super class definition"),
            TrailingTokensErrContext::Method => write!(f, "method definition"),
            TrailingTokensErrContext::TypeHint(kind) => {
                write!(f, "type hint '{}'", kind.value.token_name())
            }
            TrailingTokensErrContext::ClassEnd => write!(f, "class definition end"),
        }
    }
}

impl TrailingTokensErrContext {
    pub(in crate::parser) fn error_code(&self) -> &'static str {
        match self {
            TrailingTokensErrContext::Class => ERR_CODE_CLASS_DEF_TRAILING_TOK,
            TrailingTokensErrContext::Super => ERR_CODE_SUPER_TRAILING_TOK,
            TrailingTokensErrContext::TypeHint(_) => ERR_CODE_TH_TRAILING_TOK,
            TrailingTokensErrContext::Method => ERR_CODE_METHOD_TRAILING_TOK,
            TrailingTokensErrContext::ClassEnd => ERR_CODE_CLASS_END_TRAILING_TOK,
        }
    }
}

impl Display for OperandErrPosContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OperandErrPosContext::ClassName => write!(f, "class name"),
            OperandErrPosContext::SuperName => write!(f, "super class name"),
            OperandErrPosContext::MethodName => write!(f, "method name"),
            OperandErrPosContext::MethodDescriptor => write!(f, "method descriptor"),
            OperandErrPosContext::InstructionName => write!(f, "instruction name"),
            OperandErrPosContext::InstructionOperand(spec) => {
                write!(f, "instruction '{}' operand", spec.opcode)
            }
        }
    }
}

impl OperandErrPosContext {
    pub(in crate::parser) fn directive_name(&self) -> &'static str {
        match self {
            //TODO: use something like TokenKind::DotClass.name() to not hardcode here
            OperandErrPosContext::ClassName => ".class",
            OperandErrPosContext::SuperName => ".super",
            OperandErrPosContext::MethodName | OperandErrPosContext::MethodDescriptor => ".method",
            OperandErrPosContext::InstructionName => "instruction",
            OperandErrPosContext::InstructionOperand(spec) => spec.opcode.as_str(),
        }
    }
}
