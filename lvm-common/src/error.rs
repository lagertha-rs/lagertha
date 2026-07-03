use std::fmt::{Display, Formatter};

// TODO: looks like a trash bin, needs refactoring
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignatureErr {
    UnexpectedEnd,
    MissingParamsOpenParen,
    MissingParamsCloseParen,
    TrailingCharacters,
    InvalidIdentifier,
    MissingSuper,
    InvalidBound,
    Type(TypeDescriptorErr),
    InvalidSuperClassType,
}

impl Display for SignatureErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SignatureErr::UnexpectedEnd => write!(f, "Unexpected end of signature"),
            SignatureErr::MissingParamsOpenParen => {
                write!(f, "Missing opening parenthesis for method parameters")
            }
            SignatureErr::MissingParamsCloseParen => {
                write!(f, "Missing closing parenthesis for method parameters")
            }
            SignatureErr::TrailingCharacters => write!(f, "Trailing characters after signature"),
            SignatureErr::InvalidIdentifier => write!(f, "Invalid identifier in signature"),
            SignatureErr::MissingSuper => write!(f, "Missing 'super' in class signature"),
            SignatureErr::InvalidBound => write!(f, "Invalid bound in type variable"),
            SignatureErr::Type(err) => write!(f, "Type descriptor error: {}", err),
            SignatureErr::InvalidSuperClassType => {
                write!(f, "Invalid superclass type in class signature")
            }
        }
    }
}

impl From<TypeDescriptorErr> for SignatureErr {
    fn from(value: TypeDescriptorErr) -> Self {
        SignatureErr::Type(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeDescriptorErr {
    UnexpectedEnd,
    InvalidType(char),
    InvalidObjectRef,
}

impl Display for TypeDescriptorErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeDescriptorErr::UnexpectedEnd => write!(f, "Unexpected end of type descriptor"),
            TypeDescriptorErr::InvalidType(c) => write!(f, "Invalid type character: {}", c),
            TypeDescriptorErr::InvalidObjectRef => write!(f, "Invalid object reference type"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MethodDescriptorErr {
    ShouldStartWithParentheses(String),
    MissingClosingParenthesis(String),
    TrailingCharacters,
    Type(String, TypeDescriptorErr),
}

impl Display for MethodDescriptorErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MethodDescriptorErr::ShouldStartWithParentheses(desc) => {
                write!(f, "Method descriptor should start with '(': {}", desc)
            }
            MethodDescriptorErr::MissingClosingParenthesis(desc) => {
                write!(
                    f,
                    "Missing closing parenthesis in method descriptor: {}",
                    desc
                )
            }
            MethodDescriptorErr::TrailingCharacters => {
                write!(f, "Trailing characters after method descriptor")
            }
            MethodDescriptorErr::Type(desc, err) => {
                write!(f, "Type descriptor error in '{}': {}", desc, err)
            }
        }
    }
}
