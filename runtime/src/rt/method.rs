use crate::{ClassId, ClassIdDeprecated, MethodId, Symbol, throw_exception};
use common::error::JvmError;
use jclass::attribute::method::MethodAttribute;
use jclass::flags::MethodFlags;
use jclass::method::MethodInfo;

pub struct CodeBody {
    pub code: Box<[u8]>,
    max_stack: u16,
    max_locals: u16,
}

pub enum MethodBody {
    Interpreted(CodeBody),
    Native,
    Abstract,
}

pub struct Method {
    class_id: ClassId,
    //name: Sym,
    //descriptor: Sym,
    flags: MethodFlags,
    body: MethodBody,
}

impl Method {
    pub fn new(method_info: MethodInfo, class_id: ClassId) -> Self {
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
                max_stack: code_attr.max_stack,
                max_locals: code_attr.max_locals,
            })
        };
        Method {
            class_id,
            //name:
            //descriptor:
            flags,
            body,
        }
    }

    pub fn class_id(&self) -> ClassId {
        self.class_id
    }

    pub fn is_static(&self) -> bool {
        self.flags.is_static()
    }

    pub fn get_frame_attributes(&self) -> Result<(u16, u16), JvmError> {
        match &self.body {
            MethodBody::Interpreted(code_body) => {
                // For simplicity, we return fixed values here.
                // In a real implementation, you would parse the code attributes.
                Ok((256, 256))
            }
            _ => throw_exception!(InternalError, "Method is not interpretable"), //TODO
        }
    }

    pub fn get_code(&self) -> Result<&[u8], JvmError> {
        match &self.body {
            MethodBody::Interpreted(code_body) => Ok(&code_body.code),
            _ => throw_exception!(InternalError, "Method is not interpretable"), //TODO
        }
    }
}
