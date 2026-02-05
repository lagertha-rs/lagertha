use common::error::ClassFormatErr;
use num_enum::TryFromPrimitive;

/// Reference to a method, field, or interface method in the constant pool.
///
/// https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.2
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reference {
    pub class_index: u16,
    pub name_and_type_index: u16,
}

impl Reference {
    pub fn new(class_index: u16, name_and_type_index: u16) -> Self {
        Self {
            class_index,
            name_and_type_index,
        }
    }
}

/// Name and type descriptor pair in the constant pool.
///
/// https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.6
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NameAndType {
    pub name_index: u16,
    pub descriptor_index: u16,
}

impl NameAndType {
    pub fn new(name_index: u16, descriptor_index: u16) -> Self {
        Self {
            name_index,
            descriptor_index,
        }
    }
}

/// Dynamic constant or invoke dynamic info.
///
/// https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.10
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dynamic {
    pub bootstrap_method_attr_index: u16,
    pub name_and_type_index: u16,
}

impl Dynamic {
    pub fn new(bootstrap_method_attr_index: u16, name_and_type_index: u16) -> Self {
        Self {
            bootstrap_method_attr_index,
            name_and_type_index,
        }
    }
}

/// Method handle reference in the constant pool.
///
/// https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.8
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodHandle {
    pub reference_kind: u8,
    pub reference_index: u16,
}

impl MethodHandle {
    pub fn new(reference_kind: u8, reference_index: u16) -> Self {
        Self {
            reference_kind,
            reference_index,
        }
    }

    pub fn get_kind(&self) -> Result<MethodHandleKind, ClassFormatErr> {
        MethodHandleKind::try_from_primitive(self.reference_kind)
            .map_err(|_| ClassFormatErr::InvalidMethodHandleKind(self.reference_kind))
    }
}

/// Kind of method handle reference.
///
/// https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.8
#[derive(Debug, Clone, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum MethodHandleKind {
    GetField = 1,
    GetStatic = 2,
    PutField = 3,
    PutStatic = 4,
    InvokeVirtual = 5,
    InvokeStatic = 6,
    InvokeSpecial = 7,
    NewInvokeSpecial = 8,
    InvokeInterface = 9,
}

#[cfg(feature = "javap_print")]
impl MethodHandleKind {
    pub(crate) fn get_javap_value(&self) -> Result<&str, ClassFormatErr> {
        Ok(match self {
            MethodHandleKind::GetField => "REF_getField",
            MethodHandleKind::GetStatic => "REF_getStatic",
            MethodHandleKind::PutField => "REF_putField",
            MethodHandleKind::PutStatic => "REF_putStatic",
            MethodHandleKind::InvokeVirtual => "REF_invokeVirtual",
            MethodHandleKind::InvokeStatic => "REF_invokeStatic",
            MethodHandleKind::InvokeSpecial => "REF_invokeSpecial",
            MethodHandleKind::NewInvokeSpecial => "REF_newInvokeSpecial",
            MethodHandleKind::InvokeInterface => "REF_invokeInterface",
        })
    }
}
