use crate::rt::class::LinkageError;
use crate::rt::constant_pool::RuntimeConstantPool;
use crate::rt::constant_pool::reference::MethodDescriptorReference;
use class_file::attribute::{Annotation, SharedAttribute};
use class_file::flags::MethodFlags;
use class_file::method::MethodInfo;
use std::cell::OnceCell;
use std::sync::Arc;

// TODO:
#[derive(Debug)]
pub struct NativeMethod {
    name: Arc<str>,
    flags: MethodFlags,
    descriptor: Arc<MethodDescriptorReference>,
    signature: Option<Arc<str>>,
    rt_visible_annotations: Option<Vec<Annotation>>,
}

impl NativeMethod {
    pub fn new(method_info: MethodInfo, cp: &RuntimeConstantPool) -> Result<Self, LinkageError> {
        let name = cp.get_utf8_arc(&method_info.name_index)?;
        let flags = method_info.access_flags;
        let descriptor = cp.get_method_descriptor(&method_info.descriptor_index)?;

        let signature = OnceCell::<Arc<str>>::new();
        let rt_vis_ann = OnceCell::<Vec<Annotation>>::new();

        Ok(NativeMethod {
            name,
            flags,
            descriptor,
            signature: signature.into_inner(),
            rt_visible_annotations: rt_vis_ann.into_inner(),
        })
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
