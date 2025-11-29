use crate::error::MethodDescriptorErr;
use crate::jtype::{JavaType, ReturnType};

/// Field descriptor - represents a field type (cannot be void)
/// https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.3.2
pub type FieldDescriptor = JavaType;

/// Method descriptor - represents a method signature
/// https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.3.3
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodDescriptor {
    pub params: Vec<JavaType>,
    pub ret: ReturnType,
}

impl MethodDescriptor {
    pub fn to_java_signature(&self, class_name: &str, method_name: &str) -> String {
        let mut buffer = String::with_capacity(
            class_name.len() + method_name.len() + self.params.len() * 10 + 10,
        );

        use std::fmt::Write;
        write!(buffer, "{} ", self.ret).unwrap();

        for c in class_name.chars() {
            buffer.push(if c == '/' { '.' } else { c });
        }

        write!(buffer, ".{}(", method_name).unwrap();

        for (i, p) in self.params.iter().enumerate() {
            if i > 0 {
                buffer.push_str(", ");
            }
            write!(buffer, "{}", p).unwrap();
        }

        buffer.push(')');
        buffer
    }
}

impl TryFrom<&str> for MethodDescriptor {
    type Error = MethodDescriptorErr;

    fn try_from(desc: &str) -> Result<Self, Self::Error> {
        let mut chars = desc.chars().peekable();

        if chars.next() != Some('(') {
            return Err(MethodDescriptorErr::ShouldStartWithParentheses(
                desc.to_string(),
            ));
        }

        let mut params = Vec::new();
        loop {
            match chars.peek() {
                Some(')') => {
                    chars.next();
                    break;
                }
                Some(_) => params.push(
                    JavaType::try_recursive(&mut chars)
                        .map_err(|e| MethodDescriptorErr::Type(desc.to_string(), e))?,
                ),
                None => {
                    return Err(MethodDescriptorErr::MissingClosingParenthesis(
                        desc.to_string(),
                    ));
                }
            }
        }

        let ret = ReturnType::try_recursive(&mut chars)
            .map_err(|e| MethodDescriptorErr::Type(desc.to_string(), e))?;

        if chars.next().is_some() {
            return Err(MethodDescriptorErr::TrailingCharacters);
        }

        Ok(MethodDescriptor { params, ret })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jtype::PrimitiveType;

    // Java: void add(int, int)
    #[test]
    fn parse_two_ints_void() {
        // given
        let signature = "(II)V";
        let expected_param = vec![
            JavaType::Primitive(PrimitiveType::Int),
            JavaType::Primitive(PrimitiveType::Int),
        ];
        let expected_ret = ReturnType::Void;

        // when
        let md = MethodDescriptor::try_from(signature).unwrap();

        // then
        assert_eq!(md.params, expected_param);
        assert_eq!(md.ret, expected_ret);
    }

    // Java: int getCount()
    #[test]
    fn parse_no_params_int_return() {
        // given
        let signature = "()I";
        let expected_param: Vec<JavaType> = Vec::new();
        let expected_ret = ReturnType::Type(JavaType::Primitive(PrimitiveType::Int));

        // when
        let md = MethodDescriptor::try_from(signature).unwrap();

        // then
        assert_eq!(md.params, expected_param);
        assert_eq!(md.ret, expected_ret);
    }

    // Java: String echo(String)
    #[test]
    fn parse_string_param_string_return() {
        // given
        let signature = "(Ljava/lang/String;)Ljava/lang/String;";
        let expected_param = vec![JavaType::Instance("java/lang/String".into())];
        let expected_ret = ReturnType::Type(JavaType::Instance("java/lang/String".into()));

        // when
        let md = MethodDescriptor::try_from(signature).unwrap();

        // then
        assert_eq!(md.params, expected_param);
        assert_eq!(md.ret, expected_ret);
    }

    // Java: int[] process(int, String[])
    #[test]
    fn parse_array_param_and_return() {
        // given
        let signature = "(I[Ljava/lang/String;)[I";
        let expected_param = vec![
            JavaType::Primitive(PrimitiveType::Int),
            JavaType::Array(Box::new(JavaType::Instance("java/lang/String".into()))),
        ];
        let expected_ret = ReturnType::Type(JavaType::Array(Box::new(JavaType::Primitive(
            PrimitiveType::Int,
        ))));

        // when
        let md = MethodDescriptor::try_from(signature).unwrap();

        // then
        assert_eq!(md.params, expected_param);
        assert_eq!(md.ret, expected_ret);
    }

    // Java: Object[][] flatten(Object[][])
    #[test]
    fn parse_multi_dimensional_arrays() {
        // given
        let signature = "([[Ljava/lang/Object;)[[Ljava/lang/Object;";
        let obj = JavaType::Instance("java/lang/Object".into());
        let two_d_obj = JavaType::Array(Box::new(JavaType::Array(Box::new(obj.clone()))));
        let expected_param = vec![two_d_obj.clone()];
        let expected_ret = ReturnType::Type(two_d_obj);

        // when
        let md = MethodDescriptor::try_from(signature).unwrap();

        // then
        assert_eq!(md.params, expected_param);
        assert_eq!(md.ret, expected_ret);
    }

    // Test that void parameters are rejected
    #[test]
    fn parse_void_param_should_fail() {
        let signature = "(V)I";
        let result = MethodDescriptor::try_from(signature);
        assert!(result.is_err());
    }

    // Test field descriptor parsing
    #[test]
    fn parse_field_descriptor_int() {
        let mut chars = "I".chars().peekable();
        let field = JavaType::try_recursive(&mut chars).unwrap();
        assert_eq!(field, JavaType::Primitive(PrimitiveType::Int));
    }

    #[test]
    fn parse_field_descriptor_string() {
        let mut chars = "Ljava/lang/String;".chars().peekable();
        let field = JavaType::try_recursive(&mut chars).unwrap();
        assert_eq!(field, JavaType::Instance("java/lang/String".into()));
    }

    #[test]
    fn parse_field_descriptor_array() {
        let mut chars = "[I".chars().peekable();
        let field = JavaType::try_recursive(&mut chars).unwrap();
        assert_eq!(
            field,
            JavaType::Array(Box::new(JavaType::Primitive(PrimitiveType::Int)))
        );
    }

    // Test that void field descriptor is rejected
    #[test]
    fn parse_field_descriptor_void_should_fail() {
        let mut chars = "V".chars().peekable();
        let result = JavaType::try_recursive(&mut chars);
        assert!(result.is_err());
    }
}
