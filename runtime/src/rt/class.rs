use crate::heap::method_area::MethodArea;
use crate::heap::method_area_deprecated::MethodAreaDeprecated;
use crate::rt::constant_pool::RuntimeConstantPool;
use crate::rt::method::Method;
use crate::{ClassId, MethodId, MethodKey};
use common::error::JvmError;
use jclass::ClassFile;
use std::collections::HashMap;
use std::sync::Arc;

pub struct Class {
    //pub id: ClassId,
    pub super_id: Option<ClassId>,
    pub declared_index: HashMap<MethodKey, MethodId>,
    // fields
    // access
}

impl Class {
    pub fn new(
        cf: ClassFile,
        method_area: &mut MethodArea,
        super_id: Option<ClassId>,
    ) -> Result<Self, JvmError> {
        let cp = Arc::new(RuntimeConstantPool::new(cf.cp.inner));
        //let mut declared_index = HashMap::new();
        for method in cf.methods {
            let name = cp.get_utf8(&method.name_index, &method_area.string_interner)?;
            let desc = cp.get_utf8(&method.descriptor_index, &method_area.string_interner)?;
            /*
            let method = Method::new(method, )

            if method.access_flags.is_static()
                || method.access_flags.is_private()
                || name == "<init>"
                || name == "<clinit>"
            {
                let rt_method = Method::new();
                let method_id = method_area.push_method(rt_method);
                declared_index.insert(
                    MethodKey {
                        name: name_id,
                        desc: desc_id,
                    },
                    method_id,
                );
                continue;
            }

             */
            // register methods
        }
        Ok(Self {
            super_id,
            declared_index: HashMap::new(),
        })
    }
}
