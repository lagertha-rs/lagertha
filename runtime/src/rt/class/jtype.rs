use class_file::DescriptorErr;
use std::iter::Peekable;

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
    Short,
    Boolean,
    Array(Box<Type>),
}

impl Type {
    pub fn try_recursive<I>(it: &mut Peekable<I>) -> Result<Type, DescriptorErr>
    where
        I: Iterator<Item = char>,
    {
        let c = it.next().ok_or(DescriptorErr::UnexpectedEnd)?;

        if let Ok(base) = Type::try_from(c) {
            return Ok(base);
        }

        match c {
            'L' => {
                let mut instance_name = String::new();
                while let Some(&next) = it.peek() {
                    it.next();
                    if next == ';' {
                        return Ok(Type::Instance(instance_name));
                    }
                    instance_name.push(next);
                }
                Err(DescriptorErr::UnexpectedEnd)
            }
            '[' => {
                let elem = Type::try_recursive(it)?;
                Ok(Type::Array(Box::new(elem)))
            }
            _ => Err(DescriptorErr::InvalidType),
        }
    }
}

impl TryFrom<&str> for Type {
    type Error = DescriptorErr;

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

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_one(s: &str) -> Result<Type, DescriptorErr> {
        let mut it = s.chars().peekable();
        Type::try_recursive(&mut it)
    }

    fn parse_and_rest(s: &str) -> (Result<Type, DescriptorErr>, String) {
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
        assert!(matches!(err, DescriptorErr::UnexpectedEnd));
    }

    #[test]
    fn error_unexpected_end_after_array_prefix() {
        let err = parse_one("[").unwrap_err();
        assert!(matches!(err, DescriptorErr::UnexpectedEnd));
    }

    #[test]
    fn error_invalid_type_tag() {
        let err = parse_one("Q").unwrap_err();
        assert!(matches!(err, DescriptorErr::InvalidType));
    }

    #[test]
    fn consumes_exactly_one_type() {
        let (res, rest) = parse_and_rest("I[Ljava/lang/String;");
        assert_eq!(res.unwrap(), Type::Int);
        assert_eq!(rest, "[Ljava/lang/String;".to_string()); // untouched remainder
    }
}
