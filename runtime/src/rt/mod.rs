use crate::rt::class::InstanceClass;
use crate::{ClassId, TypeDescriptorId, Symbol};
use crate::rt::field::InstanceField;

pub mod class;
pub mod class_deprecated;
pub mod constant_pool;
mod field;
pub mod field_deprecated;
pub mod method;
pub mod method_deprecated;

pub enum JvmClass {
    Instance(InstanceClass),
    PrimitiveArray(PrimitiveArrayClass),
    InstanceArray(ObjectArrayClass),
}

impl JvmClass {
    pub fn get_name(&self) -> &Symbol {
        match self {
            JvmClass::Instance(ic) => &ic.name,
            JvmClass::PrimitiveArray(pac) => &pac.name,
            JvmClass::InstanceArray(oac) => &oac.name,
        }
    }

    pub fn get_instance_fields(&self) -> &[InstanceField] {
        match self {
            JvmClass::Instance(ic) => {
                ic.instance_fields
                    .get()
                    .map_or(&[], |fields_vec| fields_vec.as_slice())
            }
            _ => &[],
        }
    }
}

pub struct PrimitiveArrayClass {
    pub name: Symbol,
    pub super_id: ClassId,
    pub element_type: TypeDescriptorId
}

pub struct ObjectArrayClass {
    pub name: Symbol,
    pub super_id: ClassId,
    pub element_class_id: ClassId,
}
