use crate::heap::method_area::MethodArea;
use crate::rt::constant_pool::rt_cp_deprecated::RuntimeConstantPoolDeprecated;
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
    pub fn new(cf: ClassFile, method_area: &mut MethodArea) -> Result<Self, JvmError> {
        let cp = Arc::new(RuntimeConstantPoolDeprecated::new(cf.cp.inner));
        let mut declared_index = HashMap::new();
        for method in cf.methods {
            let name = cp.get_utf8(&method.name_index)?;
            let desc = cp.get_utf8(&method.descriptor_index)?;
            let name_id = method_area.string_interner.get_or_intern(name);
            let desc_id = method_area.string_interner.get_or_intern(desc);

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
            // register methods
        }
        todo!()
    }
}
