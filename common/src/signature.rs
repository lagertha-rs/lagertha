use crate::TypeDescriptorErr;
use crate::jtype::Type;
use std::iter::Peekable;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodSignature {
    pub type_params: Vec<FormalTypeParam>,
    pub params: Vec<Type>,
    pub ret: Type,
    pub throws: Vec<Type>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormalTypeParam {
    pub name: String,
    pub class_bound: Option<Type>,
    pub interface_bounds: Vec<Type>,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum MethodSignatureErr {
    #[error("Unexpected end of signature")]
    UnexpectedEnd,
    #[error("Expected '(' after formal type parameters")]
    MissingParamsOpenParen,
    #[error("Expected ')' to close parameter list")]
    MissingParamsCloseParen,
    #[error("Trailing characters after method signature")]
    TrailingCharacters,
    #[error("Invalid identifier for type parameter")]
    InvalidIdentifier,
    #[error("Invalid type after ':' in bound")]
    InvalidBound,
    #[error("Type parse error: {0}")]
    Type(#[from] TypeDescriptorErr),
}

impl TryFrom<&str> for MethodSignature {
    type Error = MethodSignatureErr;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut it = s.chars().peekable();

        let type_params = if it.peek() == Some(&'<') {
            it.next(); // consume '<'
            parse_formal_type_params(&mut it)?
        } else {
            Vec::new()
        };

        if it.next() != Some('(') {
            return Err(MethodSignatureErr::MissingParamsOpenParen);
        }
        let mut params = Vec::new();
        loop {
            match it.peek().copied() {
                Some(')') => {
                    it.next();
                    break;
                }
                Some(_) => params.push(Type::try_recursive(&mut it)?),
                None => return Err(MethodSignatureErr::MissingParamsCloseParen),
            }
        }

        let ret = Type::try_recursive(&mut it)?;

        let mut throws = Vec::new();
        while it.peek() == Some(&'^') {
            it.next(); // consume '^'
            let t = Type::try_recursive(&mut it)?;
            throws.push(t);
        }

        if it.next().is_some() {
            return Err(MethodSignatureErr::TrailingCharacters);
        }

        Ok(MethodSignature {
            type_params,
            params,
            ret,
            throws,
        })
    }
}

fn parse_formal_type_params<I>(
    it: &mut Peekable<I>,
) -> Result<Vec<FormalTypeParam>, MethodSignatureErr>
where
    I: Iterator<Item = char>,
{
    let mut res = Vec::new();
    loop {
        let mut name = String::new();
        loop {
            let ch = it.next().ok_or(MethodSignatureErr::UnexpectedEnd)?;
            if ch == ':' {
                break;
            }
            if ch == '>' {
                return Err(MethodSignatureErr::InvalidIdentifier);
            }
            name.push(ch);
        }
        if name.is_empty() {
            return Err(MethodSignatureErr::InvalidIdentifier);
        }

        let class_bound = match it.peek().copied() {
            Some(':') => None,
            Some(_) => Some(Type::try_recursive(it)?),
            None => return Err(MethodSignatureErr::UnexpectedEnd),
        };

        let mut interface_bounds = Vec::new();
        while it.peek() == Some(&':') {
            it.next();
            if matches!(it.peek(), Some('>')) {
                return Err(MethodSignatureErr::InvalidBound);
            }
            interface_bounds.push(Type::try_recursive(it)?);
        }

        res.push(FormalTypeParam {
            name,
            class_bound,
            interface_bounds,
        });

        match it.peek().copied() {
            Some('>') => {
                it.next();
                break;
            }
            Some(_) => {
                continue;
            }
            None => return Err(MethodSignatureErr::UnexpectedEnd),
        }
    }
    Ok(res)
}
