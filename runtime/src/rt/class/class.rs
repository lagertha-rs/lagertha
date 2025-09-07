use crate::JvmError;
use crate::method_area::MethodArea;
use crate::rt::class::field::Field;
use crate::rt::class::method::Method;
use crate::rt::class::native_method::NativeMethod;
use crate::rt::constant_pool::RuntimeConstantPool;
use crate::rt::constant_pool::reference::ClassReference;
use class_file::ClassFile;
use class_file::attribute::class::ClassAttribute;
use class_file::flags::ClassFlags;
use std::sync::Arc;

#[derive(Debug)]
pub struct Class {
    this: Arc<ClassReference>,
    access: ClassFlags,
    minor_version: u16,
    major_version: u16,
    super_class: Option<Arc<Class>>,
    fields: Vec<Field>,
    methods: Vec<Method>,
    native_methods: Vec<NativeMethod>,
    interfaces: Vec<String>,
    attributes: Vec<ClassAttribute>,
    cp: Arc<RuntimeConstantPool>,
    initialized: bool,
}

impl Class {
    pub fn new(cf: ClassFile, method_area: &MethodArea) -> Result<Self, JvmError> {
        let cp = Arc::new(RuntimeConstantPool::new(cf.cp.inner));
        let minor_version = cf.minor_version;
        let major_version = cf.major_version;
        let this = cp.get_class(&cf.this_class)?.clone();
        let access = cf.access_flags;
        let mut methods = vec![];
        let mut native_methods = vec![];
        for method in cf.methods.into_iter() {
            if method.access_flags.is_native() {
                let native_method = NativeMethod::new(method, cp.clone())?;
                native_methods.push(native_method);
            } else {
                let method = Method::new(method, cp.clone())?;
                methods.push(method);
            }
        }
        let super_class = if cf.super_class != 0 {
            Some(method_area.get_class(cp.get_class_name(&cf.super_class)?)?)
        } else {
            None
        };

        Ok(Class {
            this,
            access,
            super_class,
            minor_version,
            major_version,
            fields: vec![],
            methods,
            native_methods,
            interfaces: vec![],
            attributes: cf.attributes,
            cp,
            initialized: false,
        })
    }

    pub fn get_name(&self) -> Result<&Arc<String>, JvmError> {
        self.this.name().map_err(Into::into)
    }

    pub fn get_main_method(&self) -> Option<&Method> {
        self.methods.iter().find(|m| m.is_main())
    }

    pub fn get_cp(&self) -> &Arc<RuntimeConstantPool> {
        &self.cp
    }
}
