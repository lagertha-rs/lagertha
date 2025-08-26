use crate::JvmError;
use crate::rt::class::field::Field;
use crate::rt::class::method::Method;
use crate::rt::constant_pool::RuntimeConstantPool;
use crate::rt::constant_pool::reference::ClassReference;
use class_file::ClassFile;
use class_file::attribute::class::ClassAttribute;
use common::access::ClassAccessFlag;
use std::rc::Rc;

#[derive(Debug)]
pub struct Class {
    this: Rc<ClassReference>,
    access: ClassAccessFlag,
    minor_version: u16,
    major_version: u16,
    super_class: Option<Rc<ClassReference>>,
    fields: Vec<Field>,
    methods: Vec<Method>,
    interfaces: Vec<String>,
    attributes: Vec<ClassAttribute>,
    cp: Rc<RuntimeConstantPool>,
    initialized: bool,
}

impl Class {
    pub fn new(cf: ClassFile) -> Result<Self, JvmError> {
        let cp = Rc::new(RuntimeConstantPool::new(cf.cp.cp));
        let minor_version = cf.minor_version;
        let major_version = cf.major_version;
        let this = cp.get_class(&cf.this_class)?.clone();
        let super_class = if cf.super_class != 0 {
            Some(cp.get_class(&cf.super_class)?.clone())
        } else {
            None
        };
        let access = ClassAccessFlag::new(cf.access_flags);
        let methods = cf
            .methods
            .into_iter()
            .map(|method| Method::new(method, cp.clone()))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Class {
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

    pub fn get_name(&self) -> Result<&Rc<String>, JvmError> {
        self.this.name().map_err(Into::into)
    }
}
