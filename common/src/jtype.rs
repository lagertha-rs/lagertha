use crate::error::TypeDescriptorErr;
use crate::{HeapRef, Value};
use core::fmt;
use num_enum::TryFromPrimitive;
use std::fmt::{Display, Formatter};
use std::iter::Peekable;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeArg {
    Any,                    // '*'
    Extends(Box<JavaType>), // '+'
    Super(Box<JavaType>),   // '-'
    Exact(Box<JavaType>),   // no prefix
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassSignatureSegment {
    pub name: String,
    pub args: Vec<TypeArg>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassSignature {
    pub first: ClassSignatureSegment,
    pub suffix: Vec<ClassSignatureSegment>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveType {
    Byte,
    Char,
    Double,
    Float,
    Int,
    Long,
    Short,
    Boolean,
}

impl PrimitiveType {
    pub fn values() -> &'static [PrimitiveType] {
        static PRIMITIVE_TYPES: [PrimitiveType; 8] = [
            PrimitiveType::Byte,
            PrimitiveType::Char,
            PrimitiveType::Double,
            PrimitiveType::Float,
            PrimitiveType::Int,
            PrimitiveType::Long,
            PrimitiveType::Short,
            PrimitiveType::Boolean,
        ];
        &PRIMITIVE_TYPES
    }
}

impl Display for PrimitiveType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PrimitiveType::Byte => write!(f, "byte"),
            PrimitiveType::Char => write!(f, "char"),
            PrimitiveType::Double => write!(f, "double"),
            PrimitiveType::Float => write!(f, "float"),
            PrimitiveType::Int => write!(f, "int"),
            PrimitiveType::Long => write!(f, "long"),
            PrimitiveType::Short => write!(f, "short"),
            PrimitiveType::Boolean => write!(f, "boolean"),
        }
    }
}

impl TryFrom<char> for PrimitiveType {
    type Error = (); // todo

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'B' => Ok(PrimitiveType::Byte),
            'C' => Ok(PrimitiveType::Char),
            'D' => Ok(PrimitiveType::Double),
            'F' => Ok(PrimitiveType::Float),
            'I' => Ok(PrimitiveType::Int),
            'J' => Ok(PrimitiveType::Long),
            'S' => Ok(PrimitiveType::Short),
            'Z' => Ok(PrimitiveType::Boolean),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum AllocationType {
    Byte,
    Char,
    Double,
    Float,
    Int,
    Long,
    Short,
    Boolean,
    Reference,
}

impl AllocationType {
    pub fn byte_size(&self) -> usize {
        match self {
            AllocationType::Byte | AllocationType::Boolean => 1,
            AllocationType::Char | AllocationType::Short => 2,
            AllocationType::Int | AllocationType::Float => 4,
            AllocationType::Long | AllocationType::Double => 8,
            AllocationType::Reference => size_of::<HeapRef>(),
        }
    }
}

/// Represents any actual Java type (cannot be void)
/// Used for field descriptors and method parameters
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JavaType {
    Primitive(PrimitiveType),
    Instance(String), // TODO: should be interned?
    GenericInstance(ClassSignature),
    TypeVar(String),
    Array(Box<JavaType>),
}

/// Represents a method return type (can be void or any JavaType)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReturnType {
    Void,
    Type(JavaType),
}

impl PrimitiveType {
    pub fn get_default_value(&self) -> Value {
        match self {
            PrimitiveType::Byte
            | PrimitiveType::Char
            | PrimitiveType::Short
            | PrimitiveType::Int
            | PrimitiveType::Boolean => Value::Integer(0),
            PrimitiveType::Double => Value::Double(0.0),
            PrimitiveType::Float => Value::Float(0.0),
            PrimitiveType::Long => Value::Long(0),
        }
    }

    pub fn is_compatible_with(&self, value: &Value) -> bool {
        matches!(
            (self, value),
            (PrimitiveType::Byte, Value::Integer(_))
                | (PrimitiveType::Char, Value::Integer(_))
                | (PrimitiveType::Short, Value::Integer(_))
                | (PrimitiveType::Int, Value::Integer(_))
                | (PrimitiveType::Boolean, Value::Integer(_))
                | (PrimitiveType::Long, Value::Long(_))
                | (PrimitiveType::Float, Value::Float(_))
                | (PrimitiveType::Double, Value::Double(_))
        )
    }
}

impl TryFrom<&str> for JavaType {
    type Error = TypeDescriptorErr;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        JavaType::try_recursive(&mut value.chars().peekable())
    }
}

impl JavaType {
    pub fn as_allocation_type(&self) -> AllocationType {
        match self {
            JavaType::Primitive(prim) => match prim {
                PrimitiveType::Byte => AllocationType::Byte,
                PrimitiveType::Char => AllocationType::Char,
                PrimitiveType::Double => AllocationType::Double,
                PrimitiveType::Float => AllocationType::Float,
                PrimitiveType::Int => AllocationType::Int,
                PrimitiveType::Long => AllocationType::Long,
                PrimitiveType::Short => AllocationType::Short,
                PrimitiveType::Boolean => AllocationType::Boolean,
            },
            JavaType::Instance(_) | JavaType::Array(_) => AllocationType::Reference,
            _ => panic!("Cannot get allocation type for non-primitive, non-instance JavaType"),
        }
    }

    // TODO: work only for one-dimensional arrays for now
    pub fn get_primitive_array_element_type(&self) -> Option<PrimitiveType> {
        match self {
            JavaType::Array(elem) => match **elem {
                JavaType::Primitive(prim) => Some(prim),
                _ => None,
            },
            _ => None,
        }
    }

    // TODO: work only for one-dimensional arrays for now
    pub fn get_instance_array_element_type(&self) -> Option<&str> {
        match self {
            JavaType::Array(elem) => match **elem {
                JavaType::Instance(ref name) => Some(name.as_str()),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn is_primitive_array(&self) -> bool {
        match self {
            JavaType::Array(elem) => matches!(**elem, JavaType::Primitive(_)),
            _ => false,
        }
    }

    pub fn get_default_value(&self) -> Value {
        match self {
            JavaType::Primitive(prim) => prim.get_default_value(),
            JavaType::Instance(_)
            | JavaType::GenericInstance(_)
            | JavaType::TypeVar(_)
            | JavaType::Array(_) => Value::Null,
        }
    }

    pub fn is_compatible_with(&self, value: &Value) -> bool {
        match (self, value) {
            (JavaType::Primitive(prim), val) => prim.is_compatible_with(val), // Delegate
            (JavaType::Instance(_), Value::Ref(_) | Value::Null) => true,     // TODO: check class
            (JavaType::Array(_), Value::Ref(_) | Value::Null) => true,        // TODO: check class
            (JavaType::GenericInstance(_), Value::Ref(_) | Value::Null) => true, // TODO: check class
            (JavaType::TypeVar(_), Value::Ref(_) | Value::Null) => true, // TODO: check class
            _ => false,
        }
    }

    /// Parse a JavaType from a character iterator
    /// This cannot parse 'V' (void) - will return an error
    pub fn try_recursive<I>(it: &mut Peekable<I>) -> Result<JavaType, TypeDescriptorErr>
    where
        I: Iterator<Item = char>,
    {
        let c = it.peek().ok_or(TypeDescriptorErr::UnexpectedEnd)?;

        // Void is not allowed in JavaType
        if *c == 'V' {
            return Err(TypeDescriptorErr::InvalidType('V'));
        }

        // Try to parse primitive
        if let Ok(prim) = PrimitiveType::try_from(*c) {
            it.next();
            return Ok(JavaType::Primitive(prim));
        }

        match c {
            '[' => {
                it.next();
                let elem = JavaType::try_recursive(it)?;
                Ok(JavaType::Array(Box::new(elem)))
            }
            'L' => {
                it.next();
                let mut name = String::new();
                let mut has_type_args = false;
                let mut has_dot = false;
                let mut depth = 0;

                loop {
                    let ch = it.next().ok_or(TypeDescriptorErr::UnexpectedEnd)?;
                    match ch {
                        '<' => {
                            has_type_args = true;
                            depth += 1;
                            name.push(ch);
                        }
                        '>' => {
                            depth -= 1;
                            name.push(ch);
                            if depth == 0 {
                                // After closing type args, only '.' or ';' are valid
                                match it.peek() {
                                    Some('.') | Some(';') => {}
                                    Some(&c) => return Err(TypeDescriptorErr::InvalidType(c)),
                                    None => return Err(TypeDescriptorErr::UnexpectedEnd),
                                }
                            }
                        }
                        '.' => {
                            has_dot = true;
                            name.push(ch);
                        }
                        ';' => {
                            if depth == 0 {
                                break;
                            }
                            name.push(ch);
                        }
                        _ => {
                            name.push(ch);
                        }
                    }
                }

                if has_type_args || has_dot {
                    let sig = Self::parse_class_signature(&name)?;
                    Ok(JavaType::GenericInstance(sig))
                } else {
                    Ok(JavaType::Instance(name))
                }
            }
            'T' => {
                it.next();
                let mut name = String::new();
                loop {
                    let ch = it.next().ok_or(TypeDescriptorErr::UnexpectedEnd)?;
                    if ch == ';' {
                        break;
                    }
                    name.push(ch);
                }
                Ok(JavaType::TypeVar(name))
            }
            _ => Err(TypeDescriptorErr::InvalidType(*c)),
        }
    }

    fn parse_class_signature(s: &str) -> Result<ClassSignature, TypeDescriptorErr> {
        let mut it = s.chars().peekable();
        let mut segments = Vec::new();

        loop {
            let mut seg_name = String::new();

            // Read until '<' or '.' or end
            while let Some(&ch) = it.peek() {
                if ch == '<' || ch == '.' {
                    break;
                }
                seg_name.push(ch);
                it.next();
            }

            let mut args = Vec::new();
            if it.peek() == Some(&'<') {
                it.next(); // consume '<'
                loop {
                    match it.peek() {
                        Some('>') => {
                            it.next(); // consume '>'
                            break;
                        }
                        Some('*') => {
                            it.next();
                            args.push(TypeArg::Any);
                        }
                        Some('+') => {
                            it.next();
                            let ty = JavaType::try_recursive(&mut it)?;
                            args.push(TypeArg::Extends(Box::new(ty)));
                        }
                        Some('-') => {
                            it.next();
                            let ty = JavaType::try_recursive(&mut it)?;
                            args.push(TypeArg::Super(Box::new(ty)));
                        }
                        Some(_) => {
                            let ty = JavaType::try_recursive(&mut it)?;
                            args.push(TypeArg::Exact(Box::new(ty)));
                        }
                        None => return Err(TypeDescriptorErr::UnexpectedEnd),
                    }
                }
            }

            segments.push(ClassSignatureSegment {
                name: seg_name,
                args,
            });

            if it.peek() == Some(&'.') {
                it.next(); // consume '.'
            } else {
                break;
            }
        }

        if segments.is_empty() {
            return Err(TypeDescriptorErr::UnexpectedEnd);
        }

        let first = segments.remove(0);
        Ok(ClassSignature {
            first,
            suffix: segments,
        })
    }
}

impl ReturnType {
    /// Parse a ReturnType from a character iterator
    /// This can parse both 'V' (void) and any JavaType
    pub fn try_recursive<I>(it: &mut Peekable<I>) -> Result<ReturnType, TypeDescriptorErr>
    where
        I: Iterator<Item = char>,
    {
        let c = it.peek().ok_or(TypeDescriptorErr::UnexpectedEnd)?;

        if *c == 'V' {
            it.next();
            return Ok(ReturnType::Void);
        }

        let java_type = JavaType::try_recursive(it)?;
        Ok(ReturnType::Type(java_type))
    }

    pub fn get_default_value(&self) -> Value {
        match self {
            ReturnType::Void => panic!("No default value for void"),
            ReturnType::Type(java_type) => java_type.get_default_value(),
        }
    }

    pub fn is_compatible_with(&self, value: &Value) -> bool {
        match self {
            ReturnType::Void => false,
            ReturnType::Type(java_type) => java_type.is_compatible_with(value),
        }
    }
}

impl Display for JavaType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            JavaType::Primitive(prim) => {
                let name = match prim {
                    PrimitiveType::Byte => "byte",
                    PrimitiveType::Char => "char",
                    PrimitiveType::Double => "double",
                    PrimitiveType::Float => "float",
                    PrimitiveType::Int => "int",
                    PrimitiveType::Long => "long",
                    PrimitiveType::Short => "short",
                    PrimitiveType::Boolean => "boolean",
                };
                write!(f, "{}", name)
            }

            JavaType::Instance(name) => write!(f, "{}", name.replace('/', ".")),

            JavaType::GenericInstance(sig) => {
                // first segment with args
                write!(f, "{}", sig.first.name.replace('/', "."))?;
                if !sig.first.args.is_empty() {
                    write!(f, "<")?;
                    for (i, a) in sig.first.args.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{a}")?;
                    }
                    write!(f, ">")?;
                }
                // suffix segments with args
                for seg in &sig.suffix {
                    write!(f, ".{}", seg.name)?;
                    if !seg.args.is_empty() {
                        write!(f, "<")?;
                        for (i, a) in seg.args.iter().enumerate() {
                            if i > 0 {
                                write!(f, ", ")?;
                            }
                            write!(f, "{a}")?;
                        }
                        write!(f, ">")?;
                    }
                }
                Ok(())
            }

            JavaType::TypeVar(name) => write!(f, "{}", name),

            JavaType::Array(elem) => write!(f, "{}[]", elem),
        }
    }
}

impl Display for ReturnType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ReturnType::Void => write!(f, "void"),
            ReturnType::Type(java_type) => write!(f, "{}", java_type),
        }
    }
}

impl fmt::Display for TypeArg {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TypeArg::Any => write!(f, "?"),
            TypeArg::Extends(t) => write!(f, "? extends {}", t),
            TypeArg::Super(t) => write!(f, "? super {}", t),
            TypeArg::Exact(t) => write!(f, "{t}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_one_descriptor(s: &str) -> Result<ReturnType, TypeDescriptorErr> {
        let mut it = s.chars().peekable();
        ReturnType::try_recursive(&mut it)
    }

    fn parse_one_java(s: &str) -> Result<JavaType, TypeDescriptorErr> {
        let mut it = s.chars().peekable();
        JavaType::try_recursive(&mut it)
    }

    fn parse_one_return(s: &str) -> Result<ReturnType, TypeDescriptorErr> {
        let mut it = s.chars().peekable();
        ReturnType::try_recursive(&mut it)
    }

    fn parse_and_rest(s: &str) -> (Result<ReturnType, TypeDescriptorErr>, String) {
        let mut it = s.chars().peekable();
        let res = ReturnType::try_recursive(&mut it);
        let rest: String = it.collect();
        (res, rest)
    }

    #[test]
    fn primitives_try_from_char() {
        let cases = vec![
            ('B', PrimitiveType::Byte),
            ('C', PrimitiveType::Char),
            ('D', PrimitiveType::Double),
            ('F', PrimitiveType::Float),
            ('I', PrimitiveType::Int),
            ('J', PrimitiveType::Long),
            ('S', PrimitiveType::Short),
            ('Z', PrimitiveType::Boolean),
        ];
        for (ch, ty) in cases {
            assert_eq!(PrimitiveType::try_from(ch), Ok(ty));
        }
        // invalid primitive
        assert!(PrimitiveType::try_from('Q').is_err());
        // 'V' (Void) is handled by the `Type` enum directly, it's not a `PrimitiveType`
        assert!(PrimitiveType::try_from('V').is_err());
    }

    #[test]
    fn parse_void_descriptor() {
        assert_eq!(parse_one_descriptor("V").unwrap(), ReturnType::Void);
    }

    #[test]
    fn parse_void_return() {
        assert_eq!(parse_one_return("V").unwrap(), ReturnType::Void);
    }

    #[test]
    fn parse_void_java_type_should_fail() {
        assert!(parse_one_java("V").is_err());
    }

    #[test]
    fn parse_instance_object() {
        assert_eq!(
            parse_one_descriptor("Ljava/lang/String;").unwrap(),
            ReturnType::Type(JavaType::Instance("java/lang/String".to_string()))
        );
        assert_eq!(
            parse_one_java("Ljava/lang/String;").unwrap(),
            JavaType::Instance("java/lang/String".to_string())
        );
    }

    #[test]
    fn parse_array_of_primitive() {
        assert_eq!(
            parse_one_descriptor("[I").unwrap(),
            ReturnType::Type(JavaType::Array(Box::new(JavaType::Primitive(
                PrimitiveType::Int
            ))))
        );
        assert_eq!(
            parse_one_java("[I").unwrap(),
            JavaType::Array(Box::new(JavaType::Primitive(PrimitiveType::Int)))
        );
    }

    #[test]
    fn parse_array_of_object() {
        assert_eq!(
            parse_one_descriptor("[Ljava/util/List;").unwrap(),
            ReturnType::Type(JavaType::Array(Box::new(JavaType::Instance(
                "java/util/List".to_string()
            ))))
        );
        assert_eq!(
            parse_one_java("[Ljava/util/List;").unwrap(),
            JavaType::Array(Box::new(JavaType::Instance("java/util/List".to_string())))
        );
    }

    #[test]
    fn parse_multi_dimensional_array() {
        assert_eq!(
            parse_one_descriptor("[[I").unwrap(),
            ReturnType::Type(JavaType::Array(Box::new(JavaType::Array(Box::new(
                JavaType::Primitive(PrimitiveType::Int)
            )))))
        );
        assert_eq!(
            parse_one_java("[[I").unwrap(),
            JavaType::Array(Box::new(JavaType::Array(Box::new(JavaType::Primitive(
                PrimitiveType::Int
            )))))
        );
        assert_eq!(
            parse_one_descriptor("[[Ljava/lang/String;").unwrap(),
            ReturnType::Type(JavaType::Array(Box::new(JavaType::Array(Box::new(
                JavaType::Instance("java/lang/String".to_string())
            )))))
        );
        assert_eq!(
            parse_one_java("[[Ljava/lang/String;").unwrap(),
            JavaType::Array(Box::new(JavaType::Array(Box::new(JavaType::Instance(
                "java/lang/String".to_string()
            )))))
        );
    }

    #[test]
    fn error_unexpected_end_after_l() {
        // Missing ';' terminator
        let err = parse_one_descriptor("Ljava/lang/String").unwrap_err();
        assert!(matches!(err, TypeDescriptorErr::UnexpectedEnd));
    }

    #[test]
    fn error_unexpected_end_after_array_prefix() {
        let err = parse_one_descriptor("[").unwrap_err();
        assert!(matches!(err, TypeDescriptorErr::UnexpectedEnd));
    }

    #[test]
    fn error_invalid_type_tag() {
        let err = parse_one_descriptor("Q").unwrap_err();
        assert!(matches!(err, TypeDescriptorErr::InvalidType('Q')));
    }

    #[test]
    fn consumes_exactly_one_type() {
        let (res, rest) = parse_and_rest("I[Ljava/lang/String;");
        assert_eq!(
            res.unwrap(),
            ReturnType::Type(JavaType::Primitive(PrimitiveType::Int))
        );
        assert_eq!(rest, "[Ljava/lang/String;".to_string()); // untouched remainder
    }

    // --- Generic Signature tests ---

    #[test]
    fn generic_first_segment_only() {
        let s = "Ljava/util/List<+Ljava/lang/CharSequence;>;";
        let t = parse_one_java(s).unwrap();
        assert_eq!(
            t,
            JavaType::GenericInstance(ClassSignature {
                first: ClassSignatureSegment {
                    name: "java/util/List".into(),
                    args: vec![TypeArg::Extends(Box::new(JavaType::Instance(
                        "java/lang/CharSequence".into()
                    )))],
                },
                suffix: vec![],
            })
        );
    }

    #[test]
    fn generic_with_suffix_and_args() {
        let s = "Ljava/util/Map<Ljava/lang/String;Ljava/lang/Integer;>.Entry<Ljava/lang/String;>;";
        let t = parse_one_java(s).unwrap();
        assert_eq!(
            t,
            JavaType::GenericInstance(ClassSignature {
                first: ClassSignatureSegment {
                    name: "java/util/Map".into(),
                    args: vec![
                        TypeArg::Exact(Box::new(JavaType::Instance("java/lang/String".into()))),
                        TypeArg::Exact(Box::new(JavaType::Instance("java/lang/Integer".into()))),
                    ],
                },
                suffix: vec![ClassSignatureSegment {
                    name: "Entry".into(),
                    args: vec![TypeArg::Exact(Box::new(JavaType::Instance(
                        "java/lang/String".into()
                    )))],
                }],
            })
        );
    }

    #[test]
    fn suffix_without_args() {
        let s = "Lpkg/Outer.Inner;";
        let t = parse_one_java(s).unwrap();
        assert_eq!(
            t,
            JavaType::GenericInstance(ClassSignature {
                first: ClassSignatureSegment {
                    name: "pkg/Outer".into(),
                    args: vec![]
                },
                suffix: vec![ClassSignatureSegment {
                    name: "Inner".into(),
                    args: vec![]
                }],
            })
        );
    }

    #[test]
    fn error_unexpected_end_in_type_args() {
        // Missing '>'
        let err = parse_one_java("Ljava/util/List<").unwrap_err();
        assert!(matches!(err, TypeDescriptorErr::UnexpectedEnd));
    }

    #[test]
    fn error_invalid_after_type_args() {
        // After '<...>' must be '.' or ';'
        let err = parse_one_java("Ljava/util/List<Ljava/lang/String;>X").unwrap_err();
        assert!(matches!(err, TypeDescriptorErr::InvalidType('X')));
    }
}
