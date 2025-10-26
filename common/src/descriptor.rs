use crate::error::MethodDescriptorErr;
use crate::jtype::Type;

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.3
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodDescriptor {
    pub params: Vec<Type>,
    pub ret: Type,
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
                    Type::try_recursive(&mut chars)
                        .map_err(|e| MethodDescriptorErr::Type(desc.to_string(), e))?,
                ),
                None => {
                    return Err(MethodDescriptorErr::MissingClosingParenthesis(
                        desc.to_string(),
                    ));
                }
            }
        }

        let ret = Type::try_recursive(&mut chars)
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

    // Java: void add(int, int)
    #[test]
    fn parse_two_ints_void() {
        // given
        let signature = "(II)V";
        let expected_param = vec![Type::Int, Type::Int];
        let expected_ret = Type::Void;

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
        let expected_param: Vec<Type> = Vec::new();
        let expected_ret = Type::Int;

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
        let expected_param = vec![Type::Instance("java/lang/String".into())];
        let expected_ret = Type::Instance("java/lang/String".into());

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
            Type::Int,
            Type::Array(Box::new(Type::Instance("java/lang/String".into()))),
        ];
        let expected_ret = Type::Array(Box::new(Type::Int));

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
        let obj = Type::Instance("java/lang/Object".into());
        let two_d_obj = Type::Array(Box::new(Type::Array(Box::new(obj.clone()))));
        let expected_param = vec![two_d_obj.clone()];
        let expected_ret = two_d_obj;

        // when
        let md = MethodDescriptor::try_from(signature).unwrap();

        // then
        assert_eq!(md.params, expected_param);
        assert_eq!(md.ret, expected_ret);
    }
}
