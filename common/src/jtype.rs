use crate::error::{JavaExceptionFromJvm, JvmError, TypeDescriptorErr};
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

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.3.2
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Void,
    Primitive(PrimitiveType),
    Instance(String), // TODO: should be interned?
    GenericInstance(ClassSignature),
    TypeVar(String),
    Array(Box<Type>),
}

//TODO: should be in this module?
pub type HeapRef = usize;

//TODO: draft. refactor
//TODO: serializes right now only for runtime crate tests, but can't move it to dev deps
//TODO: the whole common crate should be rethought
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize)]
pub enum Value {
    Integer(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Ref(HeapRef),
    Null,
}

impl Value {
    pub fn as_obj_ref(&self) -> Result<HeapRef, JvmError> {
        match self {
            Value::Ref(addr) => Ok(*addr),
            Value::Null => Err(JvmError::JavaException(
                JavaExceptionFromJvm::NullPointerException(None),
            )),
            _ => Err(JvmError::Todo(
                "Value::as_obj_ref called on non-reference value".to_string(),
            )),
        }
    }

    pub fn as_int(&self) -> Result<i32, JvmError> {
        match self {
            Value::Integer(v) => Ok(*v),
            _ => Err(JvmError::Todo(
                "Value::as_int called on non-integer value".to_string(),
            )),
        }
    }
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
        match (self, value) {
            (PrimitiveType::Byte, Value::Integer(_)) => true,
            (PrimitiveType::Char, Value::Integer(_)) => true,
            (PrimitiveType::Short, Value::Integer(_)) => true,
            (PrimitiveType::Int, Value::Integer(_)) => true,
            (PrimitiveType::Boolean, Value::Integer(_)) => true,
            (PrimitiveType::Long, Value::Long(_)) => true,
            (PrimitiveType::Float, Value::Float(_)) => true,
            (PrimitiveType::Double, Value::Double(_)) => true,
            _ => false,
        }
    }
}

impl Type {
    // TODO: work only for one-dimensional arrays for now
    pub fn get_primitive_array_element_type(&self) -> Option<PrimitiveType> {
        match self {
            Type::Array(elem) => match **elem {
                Type::Primitive(prim) => Some(prim),
                _ => None,
            },
            _ => None,
        }
    }

    // TODO: work only for one-dimensional arrays for now
    pub fn get_instance_array_element_type(&self) -> Option<&str> {
        match self {
            Type::Array(elem) => match **elem {
                Type::Instance(ref name) => Some(name.as_str()),
                _ => None,
            },
            _ => None,
        }
    }


    pub fn is_primitive_array(&self) -> bool {
        match self {
            Type::Array(elem) => matches!(**elem, Type::Primitive(_)),
            _ => false,
        }
    }

    pub fn get_default_value(&self) -> Value {
        match self {
            Type::Primitive(prim) => prim.get_default_value(),
            Type::Instance(_) | Type::GenericInstance(_) | Type::TypeVar(_) | Type::Array(_) => {
                Value::Null
            }
            Type::Void => panic!("No default value for type: {:?}", self),
        }
    }

    pub fn is_compatible_with(&self, value: &Value) -> bool {
        match (self, value) {
            (Type::Primitive(prim), val) => prim.is_compatible_with(val), // Delegate
            (Type::Instance(_), Value::Ref(_) | Value::Null) => true, // TODO: check class
            (Type::Array(_), Value::Ref(_) | Value::Null) => true,    // TODO: check class
            (Type::GenericInstance(_), Value::Ref(_) | Value::Null) => true, // TODO: check class
            (Type::TypeVar(_), Value::Ref(_) | Value::Null) => true, // TODO: check class
            _ => false,
        }
    }

    pub fn try_recursive<I>(it: &mut Peekable<I>) -> Result<Type, TypeDescriptorErr>
    where
        I: Iterator<Item = char>,
    {
        let c = it.next().ok_or(TypeDescriptorErr::UnexpectedEnd)?;

        if c == 'V' {
            return Ok(Type::Void);
        }

        if let Ok(prim) = PrimitiveType::try_from(c) {
            return Ok(Type::Primitive(prim));
        }

        match c {
            'L' => Self::parse_class_type(it),
            'T' => Self::parse_type_var(it),
            '[' => {
                let elem = Type::try_recursive(it)?;
                if matches!(elem, Type::Void) {
                    return Err(TypeDescriptorErr::InvalidType('V'));
                }
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

impl fmt::Display for PrimitiveType {
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

impl fmt::Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Type::Void => write!(f, "void"),
            Type::Primitive(p) => write!(f, "{}", p),

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
        assert_eq!(
            parse_one("[I").unwrap(),
            Type::Array(Box::new(Type::Primitive(PrimitiveType::Int))) // <-- CHANGED
        );
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
            Type::Array(Box::new(Type::Array(Box::new(Type::Primitive(
                PrimitiveType::Int
            )))))
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
        assert_eq!(res.unwrap(), Type::Primitive(PrimitiveType::Int)); // <-- CHANGED
        assert_eq!(rest, "[Ljava/lang/String;".to_string()); // untouched remainder
    }

    // --- All Generic Signature tests below are unchanged ---

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