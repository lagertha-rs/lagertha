use crate::rt::class::LoadingError;
use crate::rt::constant_pool::RuntimeConstantPool;
use crate::rt::constant_pool::reference::MethodDescriptorReference;
use class_file::attribute::method::code::{
    CodeAttributeInfo, LineNumberEntry, LocalVariableEntry, StackMapFrame,
};
use class_file::attribute::method::{CodeAttribute, MethodAttribute};
use class_file::attribute::{Annotation, SharedAttribute};
use class_file::method::MethodInfo;
use common::access::MethodAccessFlag;
use common::instruction::Instruction;
use std::cell::OnceCell;
use std::rc::Rc;

///https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.7.3
#[derive(Debug)]
pub struct CodeContext {
    max_stack: u16,
    max_locals: u16,
    instructions: Vec<Instruction>,
    // TODO: Create a dedicated struct? (now struct from class_file)
    line_numbers: Option<Vec<LineNumberEntry>>,
    // TODO: Create a dedicated struct? (now struct from class_file)
    local_variables: Option<Vec<LocalVariableEntry>>,
    stack_map_table: Option<Vec<StackMapFrame>>,
}

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.6
#[derive(Debug)]
pub struct Method {
    pub name: Rc<String>,
    pub flags: MethodAccessFlag,
    pub descriptor: Rc<MethodDescriptorReference>,
    pub code_context: Option<CodeContext>,
    pub signature: Option<Rc<String>>,
    pub rt_visible_annotations: Option<Vec<Annotation>>,
    pub is_deprecated: bool,
}

impl Method {
    pub fn new(method_info: MethodInfo, cp: Rc<RuntimeConstantPool>) -> Result<Self, LoadingError> {
        let name = cp.get_utf8(&method_info.name_index)?.clone();
        let flags = MethodAccessFlag::new(method_info.access_flags);
        let descriptor = cp.get_method_descriptor(&method_info.descriptor_index)?;

        let code_ctx = OnceCell::<CodeContext>::new();
        let signature = OnceCell::<Rc<String>>::new();
        let rt_vis_ann = OnceCell::<Vec<Annotation>>::new();
        let exceptions = OnceCell::<Vec<u16>>::new();
        let mut is_deprecated = false;

        for attr in method_info.attributes {
            match attr {
                MethodAttribute::Code(code) => code_ctx
                    .set(CodeContext::try_from(code)?)
                    .map_err(|_| LoadingError::DuplicatedCodeAttr)?,
                MethodAttribute::Shared(shared) => match shared {
                    SharedAttribute::Synthetic => unimplemented!(),
                    SharedAttribute::Deprecated => is_deprecated = true,
                    SharedAttribute::Signature(idx) => signature
                        .set(cp.get_utf8(&idx)?.clone())
                        .map_err(|_| LoadingError::DuplicatedSignatureAttr)?,
                    SharedAttribute::RuntimeVisibleAnnotations(v) => rt_vis_ann
                        .set(v)
                        .map_err(|_| LoadingError::DuplicatedRuntimeVisibleAnnotationsAttr)?,
                    SharedAttribute::RuntimeInvisibleAnnotations => unimplemented!(),
                    SharedAttribute::RuntimeVisibleTypeAnnotations => unimplemented!(),
                    SharedAttribute::RuntimeInvisibleTypeAnnotations => unimplemented!(),
                },
                MethodAttribute::Exceptions(v) => exceptions
                    .set(v)
                    .map_err(|_| LoadingError::DuplicatedExceptionAttribute)?,
                other => unimplemented!(),
            }
        }

        let code_context = match (flags.is_native(), code_ctx.into_inner()) {
            (true, Some(_)) => return Err(LoadingError::CodeAttrIsAmbiguousForNative),
            (true, None) => None,
            (false, Some(c)) => Some(c),
            (false, None) => return Err(LoadingError::MissingCodeAttr),
        };

        Ok(Method {
            name,
            flags,
            descriptor,
            code_context,
            is_deprecated,
            signature: signature.into_inner(),
            rt_visible_annotations: rt_vis_ann.into_inner(),
        })
    }
}

impl TryFrom<CodeAttribute> for CodeContext {
    type Error = LoadingError;

    fn try_from(code: CodeAttribute) -> Result<Self, Self::Error> {
        let mut all_line_numbers: Option<Vec<LineNumberEntry>> = None;
        let mut all_local_vars: Option<Vec<LocalVariableEntry>> = None;
        let mut stack_map_table = OnceCell::<Vec<StackMapFrame>>::new();

        for code_attr in code.attributes {
            match code_attr {
                CodeAttributeInfo::LineNumberTable(v) => {
                    if let Some(cur) = &mut all_line_numbers {
                        cur.extend(v);
                    } else {
                        all_line_numbers = Some(v);
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
                    .map_err(|_| LoadingError::DuplicatedStackMapTable)?,
                other => unimplemented!("Unknown code attr {:?}", other),
            }
        }

        Ok(CodeContext {
            max_stack: code.max_stack,
            max_locals: code.max_locals,
            instructions: Instruction::new_instruction_set(&code.code)?,
            line_numbers: all_line_numbers,
            local_variables: all_local_vars,
            stack_map_table: stack_map_table.take(),
        })
    }
}
