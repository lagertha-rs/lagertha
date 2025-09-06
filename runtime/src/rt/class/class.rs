use crate::JvmError;
use crate::rt::class::field::Field;
use crate::rt::class::method::Method;
use crate::rt::constant_pool::RuntimeConstantPool;
use crate::rt::constant_pool::reference::ClassReference;
use class_file::ClassFile;
use class_file::attribute::class::ClassAttribute;
use class_file::flags::ClassFlags;
use std::sync::Arc;
use tracing_log::log::debug;

#[derive(Debug)]
pub struct Class {
    this: Arc<ClassReference>,
    access: ClassFlags,
    minor_version: u16,
    major_version: u16,
    super_class: Option<Arc<ClassReference>>,
    fields: Vec<Field>,
    methods: Vec<Method>,
    interfaces: Vec<String>,
    attributes: Vec<ClassAttribute>,
    cp: Arc<RuntimeConstantPool>,
    class_loader_name: Arc<String>, // TODO: spec says I need it. Check it later
    initialized: bool,
}

impl Class {
    pub fn new(cf: ClassFile, class_loader_name: Arc<String>) -> Result<Self, JvmError> {
        let cp = Arc::new(RuntimeConstantPool::new(cf.cp.inner));
        let minor_version = cf.minor_version;
        let major_version = cf.major_version;
        let this = cp.get_class(&cf.this_class)?.clone();
        let super_class = if cf.super_class != 0 {
            Some(cp.get_class(&cf.super_class)?.clone())
        } else {
            None
        };
        let access = cf.access_flags;
        let methods = cf
            .methods
            .into_iter()
            .map(|method| Method::new(method, cp.clone()))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Class {
            class_loader_name,
            this,
            access,
            super_class,
            minor_version,
            major_version,
            fields: vec![],
            methods,
            interfaces: vec![],
            attributes: cf.attributes,
            cp,
            initialized: false,
        })
    }

    pub fn get_name(&self) -> Result<&Arc<String>, JvmError> {
        self.this.name().map_err(Into::into)
    }
}
