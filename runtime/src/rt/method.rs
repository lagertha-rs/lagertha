use crate::error::JvmError;
use crate::keys::{ClassId, MethodDescriptorId};
use crate::{Symbol, throw_exception};
use common::error::LinkageError;
use jclass::attribute::method::code::{
    CodeAttributeInfo, LineNumberEntry, LocalVariableEntry, LocalVariableTypeEntry, StackMapFrame,
};
use jclass::attribute::method::{CodeAttribute, ExceptionTableEntry, MethodAttribute};
use jclass::flags::MethodFlags;
use jclass::method::MethodInfo;
use std::cell::OnceCell;

pub struct CodeBody {
    pub code: Box<[u8]>,
    max_stack: u16,
    max_locals: u16,
    // TODO: Create a dedicated struct? (now struct from jclass)
    line_numbers: Option<Vec<LineNumberEntry>>,
    pub exception_table: Vec<ExceptionTableEntry>,
}

pub enum MethodBody {
    Interpreted(CodeBody),
    Native,
    Abstract,
}

pub struct Method {
    class_id: ClassId,
    pub name: Symbol,
    pub desc: Symbol,
    descriptor_id: MethodDescriptorId,
    flags: MethodFlags,
    body: MethodBody,
}

impl Method {
    pub fn new(
        method_info: MethodInfo,
        class_id: ClassId,
        descriptor_id: MethodDescriptorId,
        name: Symbol,
        desc: Symbol,
    ) -> Self {
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
            MethodBody::Interpreted(CodeBody::try_from(code_attr).unwrap())
        };
        Method {
            name,
            desc,
            class_id,
            descriptor_id,
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

    pub fn is_abstract(&self) -> bool {
        self.flags.is_abstract()
    }

    pub fn is_native(&self) -> bool {
        self.flags.is_native()
    }

    pub fn descriptor_id(&self) -> MethodDescriptorId {
        self.descriptor_id
    }

    pub fn get_frame_attributes(&self) -> Result<(u16, u16), JvmError> {
        match &self.body {
            MethodBody::Interpreted(code_body) => {
                // TODO: For simplicity, we return fixed values here.
                Ok((256, 256))
            }
            _ => throw_exception!(InternalError, "Method is not interpretable"), //TODO
        }
    }

    pub fn get_exception_table(&self) -> Result<&[ExceptionTableEntry], JvmError> {
        match &self.body {
            MethodBody::Interpreted(code_body) => Ok(&code_body.exception_table),
            _ => throw_exception!(InternalError, "Method is not interpretable"), //TODO
        }
    }

    pub fn get_code(&self) -> Result<&[u8], JvmError> {
        match &self.body {
            MethodBody::Interpreted(code_body) => Ok(&code_body.code),
            _ => throw_exception!(InternalError, "Method is not interpretable"), //TODO
        }
    }

    pub fn get_line_number_by_cp(&self, cp: i32) -> Option<i32> {
        if cp == -2 {
            return Some(-2);
        }

        let cp = cp as usize;
        let MethodBody::Interpreted(ctx) = &self.body else {
            return None;
        };
        let ln_table = ctx.line_numbers.as_ref()?;

        if ln_table.is_empty() {
            return None;
        }

        let mut result = None;
        for entry in ln_table.iter() {
            if entry.start_pc as usize <= cp {
                result = Some(entry.line_number as i32);
            } else {
                break;
            }
        }

        result.or_else(|| Some(ln_table[0].line_number as i32))
    }
}

impl TryFrom<CodeAttribute> for CodeBody {
    type Error = LinkageError;

    fn try_from(code_attr: CodeAttribute) -> Result<Self, Self::Error> {
        let mut all_line_numbers: Option<Vec<LineNumberEntry>> = None;
        let mut all_local_vars: Option<Vec<LocalVariableEntry>> = None;
        let mut all_local_types: Option<Vec<LocalVariableTypeEntry>> = None;
        let exception_table = code_attr.exception_table;
        let stack_map_table = OnceCell::<Vec<StackMapFrame>>::new();

        for code_attr in code_attr.attributes {
            match code_attr {
                CodeAttributeInfo::LineNumberTable(v) => {
                    if let Some(cur) = &mut all_line_numbers {
                        cur.extend(v);
                    } else {
                        all_line_numbers = Some(v);
                    }
                }
                CodeAttributeInfo::LocalVariableTypeTable(v) => {
                    if let Some(cur) = &mut all_local_types {
                        cur.extend(v);
                    } else {
                        all_local_types = Some(v);
                    }
                }
                CodeAttributeInfo::LocalVariableTable(v) => {
                    if let Some(cur) = &mut all_local_vars {
                        cur.extend(v);
                    } else {
                        all_local_vars = Some(v);
                    }
                    // TODO: JVMS §4.7.13: ensure no more than one entry per *local variable* across tables.
                }
                CodeAttributeInfo::StackMapTable(table) => stack_map_table
                    .set(table)
                    .map_err(|_| LinkageError::DuplicatedStackMapTable)?,
                other => unimplemented!("Unknown code attr {:?}", other),
            }
        }

        Ok(CodeBody {
            code: code_attr.code.into_boxed_slice(),
            max_stack: code_attr.max_stack,
            max_locals: code_attr.max_locals,
            line_numbers: all_line_numbers,
            exception_table,
        })
    }
}
