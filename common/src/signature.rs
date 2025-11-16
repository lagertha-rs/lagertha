use crate::error::SignatureErr;
use crate::jtype::{JavaType, ReturnType};
use std::fmt;
use std::fmt::Formatter;
use std::iter::Peekable;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodSignature {
    pub type_params: Vec<FormalTypeParam>,
    pub params: Vec<JavaType>,
    pub ret: ReturnType,
    pub throws: Vec<JavaType>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassSignature {
    pub type_params: Vec<FormalTypeParam>,
    pub super_class: JavaType,
    pub interfaces: Vec<JavaType>,
    pub is_interface: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormalTypeParam {
    pub name: String,
    pub class_bound: Option<JavaType>,
    pub interface_bounds: Vec<JavaType>,
}

impl TryFrom<&str> for MethodSignature {
    type Error = SignatureErr;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut it = s.chars().peekable();

        let type_params = if it.peek() == Some(&'<') {
            it.next(); // consume '<'
            parse_formal_type_params(&mut it)?
        } else {
            Vec::new()
        };

        if it.next() != Some('(') {
            return Err(SignatureErr::MissingParamsOpenParen);
        }
        let mut params = Vec::new();
        loop {
            match it.peek().copied() {
                Some(')') => {
                    it.next();
                    break;
                }
                Some(_) => params.push(JavaType::try_recursive(&mut it)?),
                None => return Err(SignatureErr::MissingParamsCloseParen),
            }
        }

        let ret = ReturnType::try_recursive(&mut it)?;

        let mut throws = Vec::new();
        while it.peek() == Some(&'^') {
            it.next();
            let t = JavaType::try_recursive(&mut it)?;
            throws.push(t);
        }

        if it.next().is_some() {
            return Err(SignatureErr::TrailingCharacters);
        }

        Ok(MethodSignature {
            type_params,
            params,
            ret,
            throws,
        })
    }
}

fn parse_formal_type_params<I>(it: &mut Peekable<I>) -> Result<Vec<FormalTypeParam>, SignatureErr>
where
    I: Iterator<Item = char>,
{
    let mut res = Vec::new();
    loop {
        let mut name = String::new();
        loop {
            let ch = it.next().ok_or(SignatureErr::UnexpectedEnd)?;
            if ch == ':' {
                break;
            }
            if ch == '>' {
                return Err(SignatureErr::InvalidIdentifier);
            }
            name.push(ch);
        }
        if name.is_empty() {
            return Err(SignatureErr::InvalidIdentifier);
        }

        let class_bound = match it.peek().copied() {
            Some(':') => None,
            Some(_) => Some(JavaType::try_recursive(it)?),
            None => return Err(SignatureErr::UnexpectedEnd),
        };

        let mut interface_bounds = Vec::new();
        while it.peek() == Some(&':') {
            it.next();
            if matches!(it.peek(), Some('>')) {
                return Err(SignatureErr::InvalidBound);
            }
            interface_bounds.push(JavaType::try_recursive(it)?);
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
            None => return Err(SignatureErr::UnexpectedEnd),
        }
    }
    Ok(res)
}

impl ClassSignature {
    pub fn new(s: &str, is_interface: bool) -> Result<Self, SignatureErr> {
        let mut it = s.chars().peekable();

        let type_params = if it.peek() == Some(&'<') {
            it.next();
            parse_formal_type_params(&mut it)?
        } else {
            Vec::new()
        };

        let super_class = parse_class_type(&mut it)?.ok_or(SignatureErr::MissingSuper)?;

        let mut interfaces = Vec::new();
        while it.peek().is_some() {
            let iface = parse_class_type(&mut it)?.ok_or(SignatureErr::UnexpectedEnd)?;
            interfaces.push(iface);
        }

        Ok(ClassSignature {
            is_interface,
            type_params,
            super_class,
            interfaces,
        })
    }
}

fn parse_class_type<I>(it: &mut Peekable<I>) -> Result<Option<JavaType>, SignatureErr>
where
    I: Iterator<Item = char>,
{
    if it.peek().is_none() {
        return Ok(None);
    }
    let t = JavaType::try_recursive(it)?;
    match t {
        JavaType::Instance(_) | JavaType::GenericInstance(_) => Ok(Some(t)),
        _ => Err(SignatureErr::InvalidSuperClassType),
    }
}

impl fmt::Display for ClassSignature {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if !self.type_params.is_empty() {
            write!(f, "<")?;
            for (i, tp) in self.type_params.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{tp}")?;
            }
            write!(f, "> ")?;
        }

        // extends Super
        let is_object = self.super_class == JavaType::Instance("java/lang/Object".to_string());
        if !(self.is_interface && is_object) {
            write!(f, "extends {}", self.super_class)?;
        }

        if !self.interfaces.is_empty() {
            //TODO: probably makes sense to split interfaces and classes structures?
            if self.is_interface {
                write!(f, " extends ")?;
            } else {
                write!(f, " implements ")?;
            }
            for (i, itf) in self.interfaces.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{itf}")?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for MethodSignature {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if !self.type_params.is_empty() {
            write!(f, "<")?;
            for (i, tp) in self.type_params.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{tp}")?;
            }
            write!(f, "> ")?;
        }

        write!(f, "{}", self.ret)?;

        if !self.throws.is_empty() {
            write!(f, " throws ")?;
            for (i, t) in self.throws.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{t}")?;
            }
        }

        Ok(())
    }
}

impl fmt::Display for FormalTypeParam {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)?;
        let mut printed_any_bound = false;

        if let Some(cb) = &self.class_bound {
            write!(f, " extends {}", cb)?;
            printed_any_bound = true;
        }

        for ib in &self.interface_bounds {
            if printed_any_bound {
                write!(f, " & {}", ib)?;
            } else {
                write!(f, " extends {}", ib)?;
                printed_any_bound = true;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn method_sig_typevar_return_and_bounds() {
        let sig = MethodSignature::try_from(
            "<R:Ljava/lang/Object;>(Ljava/util/function/Function<-Ljava/lang/String;+TR;>;)TR;",
        )
        .unwrap();

        assert_eq!(sig.type_params.len(), 1);
        assert_eq!(sig.type_params[0].to_string(), "R extends java.lang.Object");

        assert_eq!(sig.params.len(), 1);
        assert_eq!(
            sig.params[0].to_string(),
            "java.util.function.Function<? super java.lang.String, ? extends R>"
        );

        assert_eq!(sig.ret.to_string(), "R");
    }
}
