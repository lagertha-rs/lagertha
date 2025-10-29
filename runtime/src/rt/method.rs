use crate::{ClassIdDeprecated, MethodId, Symbol};
use jclass::attribute::method::MethodAttribute;
use jclass::flags::MethodFlags;
use jclass::method::MethodInfo;

pub struct CodeBody {
    pub code: Box<[u8]>,
}

pub enum MethodBody {
    Interpreted(CodeBody),
    Native,
    Abstract,
}

pub struct Method {
    class_id: ClassIdDeprecated,
    //name: Sym,
    //descriptor: Sym,
    flags: MethodFlags,
    body: MethodBody,
}

impl Method {
    pub fn new(method_info: MethodInfo, class_id: ClassIdDeprecated) -> Self {
        let flags = method_info.access_flags;
        let body = if flags.is_abstract() {
            MethodBody::Abstract
        } else if flags.is_native() {
            MethodBody::Native
        } else {
            let code_attr = method_info
                .attributes
                .iter()
                .find_map(|e| match e {
                    MethodAttribute::Code(code) => Some(code.to_owned()),
                    _ => None,
                })
                .unwrap();
            MethodBody::Interpreted(CodeBody {
                code: code_attr.code.into_boxed_slice(),
            })
        };
        Method {
            class_id,
            //name: Sym::default(), // placeholder
            //descriptor: Sym::default(), // placeholder
            flags,
            body,
        }
    }
}
