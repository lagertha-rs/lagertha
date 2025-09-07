use crate::rt::class::LinkageError;
use crate::rt::constant_pool::RuntimeConstantPool;
use crate::rt::constant_pool::reference::MethodDescriptorReference;
use class_file::attribute::method::code::{
    CodeAttributeInfo, LineNumberEntry, LocalVariableEntry, StackMapFrame,
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
    pub name: Arc<String>,
    pub flags: MethodFlags,
    pub descriptor: Arc<MethodDescriptorReference>,
    pub code_context: CodeContext,
    pub signature: Option<Arc<String>>,
    pub rt_visible_annotations: Option<Vec<Annotation>>,
    pub is_deprecated: bool,
}

impl Method {
    pub fn new(
        method_info: MethodInfo,
        cp: Arc<RuntimeConstantPool>,
    ) -> Result<Self, LinkageError> {
        let name = cp.get_utf8(&method_info.name_index)?.clone();
        let flags = method_info.access_flags;
        let descriptor = cp.get_method_descriptor(&method_info.descriptor_index)?;

        let mut code_ctx = OnceCell::<CodeContext>::new();
        let signature = OnceCell::<Arc<String>>::new();
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
                        .set(cp.get_utf8(&idx)?.clone())
                        .map_err(|_| LinkageError::DuplicatedSignatureAttr)?,
                    SharedAttribute::RuntimeVisibleAnnotations(v) => rt_vis_ann
                        .set(v)
                        .map_err(|_| LinkageError::DuplicatedRuntimeVisibleAnnotationsAttr)?,
                    SharedAttribute::RuntimeInvisibleAnnotations => unimplemented!(),
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

    pub fn is_main(&self) -> bool {
        self.name.as_str() == "main"
            && self.flags.is_public()
            && self.flags.is_static()
            && self.descriptor.raw().as_str() == "([Ljava/lang/String;)V"
    }

    pub fn instructions(&self) -> &Vec<Instruction> {
        &self.code_context.instructions
    }

    pub fn max_stack(&self) -> usize {
        self.code_context.max_stack as usize
    }

    pub fn max_locals(&self) -> usize {
        self.code_context.max_locals as usize
    }
}

impl TryFrom<CodeAttribute> for CodeContext {
    type Error = LinkageError;

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
                    .map_err(|_| LinkageError::DuplicatedStackMapTable)?,
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
