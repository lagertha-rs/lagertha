use crate::TypeDescriptorErr;
use core::fmt;
use std::fmt::Formatter;
use std::iter::Peekable;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeArg {
    Any,                // '*'
    Extends(Box<Type>), // '+'
    Super(Box<Type>),   // '-'
    Exact(Box<Type>),   // no prefix
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

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.3.2
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Void,
    Byte,
    Char,
    Double,
    Float,
    Int,
    Long,
    Instance(String), // obj or interface
    GenericInstance(ClassSignature),
    TypeVar(String),
    Short,
    Boolean,
    Array(Box<Type>),
}

//TODO: should be in this module?
pub type HeapAddr = usize;

//TODO: draft. refactor
//TODO: serializes right now only for runtime crate tests, but can't move it to dev deps
//TODO: the whole common crate should be rethought
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub enum Value {
    Integer(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Object(Option<HeapAddr>),
    Array(Option<HeapAddr>),
}

impl Type {
    pub fn get_default_value(&self) -> Value {
        match self {
            Type::Byte | Type::Char | Type::Short | Type::Int | Type::Boolean => Value::Integer(0),
            Type::Double => Value::Double(0.0),
            Type::Float => Value::Float(0.0),
            Type::Long => Value::Long(0),
            Type::Instance(_) => Value::Object(None),
            Type::Array(_) => Value::Array(None),
            _ => panic!("No default value for type: {:?}", self), //TODO
        }
    }

    pub fn is_compatible_with(&self, value: &Value) -> bool {
        match (self, value) {
            (Type::Byte, Value::Integer(_)) => true,
            (Type::Char, Value::Integer(_)) => true,
            (Type::Short, Value::Integer(_)) => true,
            (Type::Int, Value::Integer(_)) => true,
            (Type::Boolean, Value::Integer(_)) => true, // 0 or
            (Type::Long, Value::Long(_)) => true,
            (Type::Float, Value::Float(_)) => true,
            (Type::Double, Value::Double(_)) => true,
            (Type::Instance(_), Value::Object(_)) => true, //TODO: check class compatibility
            (Type::Array(_), Value::Array(_)) => true,     //TODO: check class compatibility
            _ => false,
        }
    }

    pub fn try_recursive<I>(it: &mut Peekable<I>) -> Result<Type, TypeDescriptorErr>
    where
        I: Iterator<Item = char>,
    {
        let c = it.next().ok_or(TypeDescriptorErr::UnexpectedEnd)?;

        if let Ok(base) = Type::try_from(c) {
            return Ok(base);
        }

        match c {
            'L' => Self::parse_class_type(it),
            'T' => Self::parse_type_var(it),
            '[' => {
                let elem = Type::try_recursive(it)?;
                Ok(Type::Array(Box::new(elem)))
            }
            unknown => Err(TypeDescriptorErr::InvalidType(unknown)),
        }
    }

    fn parse_type_var<I>(it: &mut Peekable<I>) -> Result<Type, TypeDescriptorErr>
    where
        I: Iterator<Item = char>,
    {
        let mut name = String::new();
        while let Some(&ch) = it.peek() {
            it.next();
            if ch == ';' {
                return Ok(Type::TypeVar(name));
            }
            name.push(ch);
        }
        Err(TypeDescriptorErr::UnexpectedEnd)
    }

    fn parse_class_type<I>(it: &mut Peekable<I>) -> Result<Type, TypeDescriptorErr>
    where
        I: Iterator<Item = char>,
    {
        let mut first_name = String::new();
        let mut first_args: Vec<TypeArg> = Vec::new();
        let mut suffix: Vec<ClassSignatureSegment> = Vec::new();

        loop {
            let ch = it.next().ok_or(TypeDescriptorErr::UnexpectedEnd)?;
            match ch {
                '<' => {
                    first_args = Self::parse_type_args(it)?;
                    match it.peek().copied() {
                        Some('.') => {
                            it.next();
                            break;
                        }
                        Some(';') => {
                            it.next();
                            return if first_args.is_empty() {
                                Ok(Type::Instance(first_name))
                            } else {
                                Ok(Type::GenericInstance(ClassSignature {
                                    first: ClassSignatureSegment {
                                        name: first_name,
                                        args: first_args,
                                    },
                                    suffix,
                                }))
                            };
                        }
                        Some(other) => return Err(TypeDescriptorErr::InvalidType(other)),
                        None => return Err(TypeDescriptorErr::UnexpectedEnd),
                    }
                }
                '.' => {
                    break;
                }
                ';' => {
                    return Ok(Type::Instance(first_name));
                }
                other => first_name.push(other),
            }
        }

        loop {
            let mut seg_name = String::new();
            let mut seg_args: Vec<TypeArg> = Vec::new();

            loop {
                let ch = it.next().ok_or(TypeDescriptorErr::UnexpectedEnd)?;
                match ch {
                    '<' => {
                        seg_args = Self::parse_type_args(it)?;
                        match it.peek().copied() {
                            Some('.') => {
                                it.next();
                                suffix.push(ClassSignatureSegment {
                                    name: seg_name,
                                    args: seg_args,
                                });
                                break;
                            }
                            Some(';') => {
                                it.next();
                                suffix.push(ClassSignatureSegment {
                                    name: seg_name,
                                    args: seg_args,
                                });
                                return Ok(Type::GenericInstance(ClassSignature {
                                    first: ClassSignatureSegment {
                                        name: first_name,
                                        args: first_args,
                                    },
                                    suffix,
                                }));
                            }
                            Some(other) => return Err(TypeDescriptorErr::InvalidType(other)),
                            None => return Err(TypeDescriptorErr::UnexpectedEnd),
                        }
                    }
                    '.' => {
                        suffix.push(ClassSignatureSegment {
                            name: seg_name,
                            args: seg_args,
                        });
                        break;
                    }
                    ';' => {
                        suffix.push(ClassSignatureSegment {
                            name: seg_name,
                            args: seg_args,
                        });
                        return Ok(Type::GenericInstance(ClassSignature {
                            first: ClassSignatureSegment {
                                name: first_name,
                                args: first_args,
                            },
                            suffix,
                        }));
                    }
                    other => seg_name.push(other),
                }
            }
        }
    }

    fn parse_type_args<I>(it: &mut Peekable<I>) -> Result<Vec<TypeArg>, TypeDescriptorErr>
    where
        I: Iterator<Item = char>,
    {
        let mut args = Vec::new();
        loop {
            let ch = *it.peek().ok_or(TypeDescriptorErr::UnexpectedEnd)?;
            match ch {
                '>' => {
                    it.next();
                    break;
                }
                '*' => {
                    it.next();
                    args.push(TypeArg::Any);
                }
                '+' => {
                    it.next();
                    args.push(TypeArg::Extends(Box::new(Self::parse_reference_type(it)?)));
                }
                '-' => {
                    it.next();
                    args.push(TypeArg::Super(Box::new(Self::parse_reference_type(it)?)));
                }
                _ => {
                    args.push(TypeArg::Exact(Box::new(Self::parse_reference_type(it)?)));
                }
            }
        }
        Ok(args)
    }

    fn parse_reference_type<I>(it: &mut Peekable<I>) -> Result<Type, TypeDescriptorErr>
    where
        I: Iterator<Item = char>,
    {
        let next = *it.peek().ok_or(TypeDescriptorErr::UnexpectedEnd)?;
        match next {
            'L' | 'T' | '[' => Type::try_recursive(it),
            other => Err(TypeDescriptorErr::InvalidType(other)),
        }
    }
}

impl TryFrom<&str> for Type {
    type Error = TypeDescriptorErr;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Type::try_recursive(&mut value.chars().peekable())
    }
}

impl TryFrom<char> for Type {
    type Error = (); // todo

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'V' => Ok(Type::Void),
            'B' => Ok(Type::Byte),
            'C' => Ok(Type::Char),
            'D' => Ok(Type::Double),
            'F' => Ok(Type::Float),
            'I' => Ok(Type::Int),
            'J' => Ok(Type::Long),
            'S' => Ok(Type::Short),
            'Z' => Ok(Type::Boolean),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Type::Void => write!(f, "void"),
            Type::Byte => write!(f, "byte"),
            Type::Char => write!(f, "char"),
            Type::Double => write!(f, "double"),
            Type::Float => write!(f, "float"),
            Type::Int => write!(f, "int"),
            Type::Long => write!(f, "long"),
            Type::Short => write!(f, "short"),
            Type::Boolean => write!(f, "boolean"),

            Type::Instance(name) => write!(f, "{}", name.replace('/', ".")),

            Type::GenericInstance(sig) => {
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

            Type::TypeVar(name) => write!(f, "{}", name),

            Type::Array(elem) => write!(f, "{}[]", elem),
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

    fn parse_one(s: &str) -> Result<Type, TypeDescriptorErr> {
        let mut it = s.chars().peekable();
        Type::try_recursive(&mut it)
    }

    fn parse_and_rest(s: &str) -> (Result<Type, TypeDescriptorErr>, String) {
        let mut it = s.chars().peekable();
        let res = Type::try_recursive(&mut it);
        let rest: String = it.collect();
        (res, rest)
    }

    #[test]
    fn primitives_try_from_char() {
        let cases = vec![
            ('V', Type::Void),
            ('B', Type::Byte),
            ('C', Type::Char),
            ('D', Type::Double),
            ('F', Type::Float),
            ('I', Type::Int),
            ('J', Type::Long),
            ('S', Type::Short),
            ('Z', Type::Boolean),
        ];
        for (ch, ty) in cases {
            assert_eq!(Type::try_from(ch), Ok(ty));
        }
        // invalid primitive
        assert!(Type::try_from('Q').is_err());
    }

    #[test]
    fn parse_void() {
        assert_eq!(parse_one("V").unwrap(), Type::Void);
    }

    #[test]
    fn parse_instance_object() {
        assert_eq!(
            parse_one("Ljava/lang/String;").unwrap(),
            Type::Instance("java/lang/String".to_string())
        );
    }

    #[test]
    fn parse_array_of_primitive() {
        assert_eq!(parse_one("[I").unwrap(), Type::Array(Box::new(Type::Int)));
    }

    #[test]
    fn parse_array_of_object() {
        assert_eq!(
            parse_one("[Ljava/util/List;").unwrap(),
            Type::Array(Box::new(Type::Instance("java/util/List".to_string())))
        );
    }

    #[test]
    fn parse_multi_dimensional_array() {
        assert_eq!(
            parse_one("[[I").unwrap(),
            Type::Array(Box::new(Type::Array(Box::new(Type::Int))))
        );
        assert_eq!(
            parse_one("[[Ljava/lang/String;").unwrap(),
            Type::Array(Box::new(Type::Array(Box::new(Type::Instance(
                "java/lang/String".to_string()
            )))))
        );
    }

    #[test]
    fn error_unexpected_end_after_l() {
        // Missing ';' terminator
        let err = parse_one("Ljava/lang/String").unwrap_err();
        assert!(matches!(err, TypeDescriptorErr::UnexpectedEnd));
    }

    #[test]
    fn error_unexpected_end_after_array_prefix() {
        let err = parse_one("[").unwrap_err();
        assert!(matches!(err, TypeDescriptorErr::UnexpectedEnd));
    }

    #[test]
    fn error_invalid_type_tag() {
        let err = parse_one("Q").unwrap_err();
        assert!(matches!(err, TypeDescriptorErr::InvalidType('Q')));
    }

    #[test]
    fn consumes_exactly_one_type() {
        let (res, rest) = parse_and_rest("I[Ljava/lang/String;");
        assert_eq!(res.unwrap(), Type::Int);
        assert_eq!(rest, "[Ljava/lang/String;".to_string()); // untouched remainder
    }

    #[test]
    fn generic_first_segment_only() {
        let s = "Ljava/util/List<+Ljava/lang/CharSequence;>;";
        let t = parse_one(s).unwrap();
        assert_eq!(
            t,
            Type::GenericInstance(ClassSignature {
                first: ClassSignatureSegment {
                    name: "java/util/List".into(),
                    args: vec![TypeArg::Extends(Box::new(Type::Instance(
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
        let t = parse_one(s).unwrap();
        assert_eq!(
            t,
            Type::GenericInstance(ClassSignature {
                first: ClassSignatureSegment {
                    name: "java/util/Map".into(),
                    args: vec![
                        TypeArg::Exact(Box::new(Type::Instance("java/lang/String".into()))),
                        TypeArg::Exact(Box::new(Type::Instance("java/lang/Integer".into()))),
                    ],
                },
                suffix: vec![ClassSignatureSegment {
                    name: "Entry".into(),
                    args: vec![TypeArg::Exact(Box::new(Type::Instance(
                        "java/lang/String".into()
                    )))],
                }],
            })
        );
    }

    #[test]
    fn suffix_without_args() {
        let s = "Lpkg/Outer.Inner;";
        let t = parse_one(s).unwrap();
        assert_eq!(
            t,
            Type::GenericInstance(ClassSignature {
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
        let err = parse_one("Ljava/util/List<").unwrap_err();
        assert!(matches!(err, TypeDescriptorErr::UnexpectedEnd));
    }

    #[test]
    fn error_invalid_after_type_args() {
        // After '<...>' must be '.' or ';'
        let err = parse_one("Ljava/util/List<Ljava/lang/String;>X").unwrap_err();
        assert!(matches!(err, TypeDescriptorErr::InvalidType('X')));
    }
}
