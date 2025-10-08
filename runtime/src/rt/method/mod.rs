use crate::ClassId;
use crate::error::JvmError;
use crate::rt::class::LinkageError;
use crate::rt::class::class::Class;
use crate::rt::constant_pool::RuntimeConstantPool;
use crate::rt::constant_pool::reference::MethodDescriptorReference;
use class_file::attribute::method::code::{
    CodeAttributeInfo, LineNumberEntry, LocalVariableEntry, LocalVariableTypeEntry, StackMapFrame,
};
use class_file::attribute::method::{CodeAttribute, MethodAttribute};
use class_file::attribute::{Annotation, SharedAttribute};
use class_file::flags::MethodFlags;
use class_file::method::MethodInfo;
use log::warn;
use once_cell::sync::OnceCell as SyncOnceCell;
use std::cell::OnceCell;
use std::sync::Arc;

pub enum MethodType {
    Abstract,
    Java,
    Native,
}

///https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.7.3
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
pub struct Method {
    name_idx: u16,
    name: Arc<str>,
    flags: MethodFlags,
    class: SyncOnceCell<Arc<Class>>,
    descriptor: Arc<MethodDescriptorReference>,
    code_context: Option<CodeContext>,
    signature: Option<Arc<str>>,
    rt_visible_annotations: Option<Vec<Annotation>>,
    rt_invisible_annotations: Option<Vec<Annotation>>,
    is_deprecated: bool,
    method_type: MethodType,
}

impl Method {
    pub fn new_native(
        name: Arc<str>,
        descriptor: Arc<MethodDescriptorReference>,
        flags: MethodFlags,
    ) -> Self {
        Method {
            name_idx: 0,
            name,
            flags,
            class: SyncOnceCell::new(),
            descriptor,
            code_context: None,
            signature: None,
            rt_visible_annotations: None,
            rt_invisible_annotations: None,
            is_deprecated: false,
            method_type: MethodType::Native,
        }
    }

    pub fn new(
        method_info: MethodInfo,
        method_type: MethodType,
        cp: &RuntimeConstantPool,
    ) -> Result<Self, LinkageError> {
        let name_idx = method_info.name_index;
        let name = cp.get_utf8_arc(&name_idx)?;
        let flags = method_info.access_flags;
        let descriptor = cp.get_method_descriptor(&method_info.descriptor_index)?;

        let mut code_ctx = OnceCell::<CodeContext>::new();
        let signature = OnceCell::<Arc<str>>::new();
        let rt_vis_ann = OnceCell::<Vec<Annotation>>::new();
        let rt_invis_ann = OnceCell::<Vec<Annotation>>::new();
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
                    SharedAttribute::RuntimeInvisibleAnnotations(v) => rt_invis_ann
                        .set(v)
                        .map_err(|_| LinkageError::DuplicatedRuntimeInvisibleAnnotationsAttr)?,
                    SharedAttribute::RuntimeVisibleTypeAnnotations => unimplemented!(),
                    SharedAttribute::RuntimeInvisibleTypeAnnotations => unimplemented!(),
                },
                MethodAttribute::Exceptions(v) => exceptions
                    .set(v)
                    .map_err(|_| LinkageError::DuplicatedExceptionAttribute)?,
                other => warn!("Unimplemented method attribute is ignored: {:?}", other),
            }
        }

        let code_context = code_ctx.take();

        Ok(Method {
            method_type,
            name_idx,
            class: SyncOnceCell::new(),
            name,
            flags,
            descriptor,
            code_context,
            is_deprecated,
            signature: signature.into_inner(),
            rt_visible_annotations: rt_vis_ann.into_inner(),
            rt_invisible_annotations: rt_invis_ann.into_inner(),
        })
    }

    pub fn type_of(&self) -> &MethodType {
        &self.method_type
    }

    pub fn name_idx(&self) -> u16 {
        self.name_idx
    }

    pub fn set_class(&self, class: Arc<Class>) -> Result<(), LinkageError> {
        self.class
            .set(class)
            .map_err(|_| LinkageError::DuplicatedClassInMethod)?;
        Ok(())
    }

    pub fn class(&self) -> Result<Arc<Class>, LinkageError> {
        self.class
            .get()
            .cloned()
            .ok_or(LinkageError::MethodClassIsNotSet)
    }

    pub fn class_id(&self) -> Result<ClassId, JvmError> {
        self.class()?.id()
    }

    pub fn instructions(&self) -> Result<&Vec<u8>, JvmError> {
        self.code_context
            .as_ref()
            .map(|ctx| &ctx.instructions)
            .ok_or_else(|| JvmError::MethodIsAbstract(self.name.to_string()))
    }

    pub fn max_stack(&self) -> Result<usize, JvmError> {
        self.code_context
            .as_ref()
            .map(|ctx| ctx.max_stack as usize)
            .ok_or_else(|| JvmError::MethodIsAbstract(self.name.to_string()))
    }

    pub fn max_locals(&self) -> Result<usize, JvmError> {
        self.code_context
            .as_ref()
            .map(|ctx| ctx.max_locals as usize)
            .ok_or_else(|| JvmError::MethodIsAbstract(self.name.to_string()))
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

    pub fn flags(&self) -> &MethodFlags {
        &self.flags
    }

    pub fn params_count(&self) -> usize {
        self.descriptor.resolved().params.len() + if !self.flags.is_static() { 1 } else { 0 }
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
