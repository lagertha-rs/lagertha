use crate::rt::class::LinkageError;
use crate::rt::class::class::Class;
use crate::rt::constant_pool::RuntimeConstantPool;
use crate::rt::constant_pool::reference::MethodDescriptorReference;
use class_file::attribute::Annotation;
use class_file::flags::MethodFlags;
use class_file::method::MethodInfo;
use once_cell::sync::OnceCell as SyncOnceCell;
use std::cell::OnceCell;
use std::sync::Arc;

pub struct NativeMethod {
    name: Arc<str>,
    flags: MethodFlags,
    class: SyncOnceCell<Arc<Class>>,
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
            class: SyncOnceCell::new(),
            signature: signature.into_inner(),
            rt_visible_annotations: rt_vis_ann.into_inner(),
        })
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
