use crate::access::class::ClassAccessFlag;
use crate::field::Field;
use crate::method::Method;
use crate::runtime_constant_pool::RuntimeConstantPool;
use crate::{ClassReference, JvmError};
use class_file::ClassFile;
use std::rc::Rc;

#[derive(Debug)]
pub struct Class {
    this: Rc<ClassReference>,
    access: ClassAccessFlag,
    super_class: Rc<ClassReference>,
    fields: Vec<Field>,
    methods: Vec<Method>,
    cp: RuntimeConstantPool,
    initialized: bool,
}

impl Class {
    pub fn new(cf: ClassFile) -> Result<Self, JvmError> {
        let cp = RuntimeConstantPool::new(cf.constant_pool);
        let this = cp.get_class(cf.this_class)?.clone();
        let super_class = cp.get_class(cf.super_class)?.clone();
        let access = ClassAccessFlag(cf.access_flags);
        let methods = cf
            .methods
            .iter()
            .map(|method| Method::new(method, &cp))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Class {
            this,
            access,
            super_class,
            fields: vec![],
            methods,
            cp,
            initialized: false,
        })
    }
}
