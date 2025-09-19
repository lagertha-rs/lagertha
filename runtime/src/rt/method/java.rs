use crate::rt::class::LinkageError;
use crate::rt::constant_pool::RuntimeConstantPool;
use crate::rt::constant_pool::reference::MethodDescriptorReference;
use class_file::attribute::method::code::{
    CodeAttributeInfo, LineNumberEntry, LocalVariableEntry, LocalVariableTypeEntry, StackMapFrame,
};
use class_file::attribute::method::{CodeAttribute, MethodAttribute};
use class_file::attribute::{Annotation, SharedAttribute};
use class_file::flags::MethodFlags;
use class_file::method::MethodInfo;
use common::instruction::Instruction;
use std::cell::OnceCell;
use std::sync::Arc;

///https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.7.3
#[derive(Debug)]
pub struct CodeContext {
    max_stack: u16,
    max_locals: u16,
    instructions: Vec<u8>,
    // TODO: Create a dedicated struct? (now struct from class_file)
    line_numbers: Option<Vec<LineNumberEntry>>,
    // TODO: Create a dedicated struct? (now struct from class_file)
    local_variables: Option<Vec<LocalVariableEntry>>,
    // TODO: Create a dedicated struct? (now struct from class_file)
    local_variable_types: Option<Vec<LocalVariableTypeEntry>>,
    stack_map_table: Option<Vec<StackMapFrame>>,
}

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.6
#[derive(Debug)]
pub struct Method {
    name: Arc<str>,
    flags: MethodFlags,
    descriptor: Arc<MethodDescriptorReference>,
    code_context: CodeContext,
    signature: Option<Arc<str>>,
    rt_visible_annotations: Option<Vec<Annotation>>,
    is_deprecated: bool,
}

impl Method {
    pub fn new(method_info: MethodInfo, cp: &RuntimeConstantPool) -> Result<Self, LinkageError> {
        let name = cp.get_utf8_arc(&method_info.name_index)?;
        let flags = method_info.access_flags;
        let descriptor = cp.get_method_descriptor(&method_info.descriptor_index)?;

        let mut code_ctx = OnceCell::<CodeContext>::new();
        let signature = OnceCell::<Arc<str>>::new();
        let rt_vis_ann = OnceCell::<Vec<Annotation>>::new();
        let exceptions = OnceCell::<Vec<u16>>::new();
        let mut is_deprecated = false;

        for attr in method_info.attributes {
            match attr {
                MethodAttribute::Code(code) => code_ctx
                    .set(CodeContext::try_from(code)?)
                    .map_err(|_| LinkageError::DuplicatedCodeAttr)?,
                MethodAttribute::Shared(shared) => match shared {
                    SharedAttribute::Synthetic => unimplemented!(),
                    SharedAttribute::Deprecated => is_deprecated = true,
                    SharedAttribute::Signature(idx) => signature
                        .set(cp.get_utf8_arc(&idx)?)
                        .map_err(|_| LinkageError::DuplicatedSignatureAttr)?,
                    SharedAttribute::RuntimeVisibleAnnotations(v) => rt_vis_ann
                        .set(v)
                        .map_err(|_| LinkageError::DuplicatedRuntimeVisibleAnnotationsAttr)?,
                    SharedAttribute::RuntimeInvisibleAnnotations(_) => unimplemented!(),
                    SharedAttribute::RuntimeVisibleTypeAnnotations => unimplemented!(),
                    SharedAttribute::RuntimeInvisibleTypeAnnotations => unimplemented!(),
                },
                MethodAttribute::Exceptions(v) => exceptions
                    .set(v)
                    .map_err(|_| LinkageError::DuplicatedExceptionAttribute)?,
                other => unimplemented!(),
            }
        }

        let code_context = code_ctx.take().ok_or(LinkageError::MissingCodeAttr)?;

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

    pub fn instructions(&self) -> &Vec<u8> {
        &self.code_context.instructions
    }

    pub fn max_stack(&self) -> usize {
        self.code_context.max_stack as usize
    }

    pub fn max_locals(&self) -> usize {
        self.code_context.max_locals as usize
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn name_arc(&self) -> Arc<str> {
        self.name.clone()
    }

    pub fn descriptor(&self) -> &Arc<MethodDescriptorReference> {
        &self.descriptor
    }
}

impl TryFrom<CodeAttribute> for CodeContext {
    type Error = LinkageError;

    fn try_from(code: CodeAttribute) -> Result<Self, Self::Error> {
        let mut all_line_numbers: Option<Vec<LineNumberEntry>> = None;
        let mut all_local_vars: Option<Vec<LocalVariableEntry>> = None;
        let mut all_local_types: Option<Vec<LocalVariableTypeEntry>> = None;
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

        Ok(CodeContext {
            max_stack: code.max_stack,
            max_locals: code.max_locals,
            instructions: code.code,
            line_numbers: all_line_numbers,
            local_variables: all_local_vars,
            local_variable_types: all_local_types,
            stack_map_table: stack_map_table.take(),
        })
    }
}
