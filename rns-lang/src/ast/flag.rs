use crate::token::flag::{
    JVMS_ABSTRACT_FLAG_NAME, JVMS_ANNOTATION_FLAG_NAME, JVMS_BRIDGE_FLAG_NAME, JVMS_ENUM_FLAG_NAME,
    JVMS_FINAL_FLAG_NAME, JVMS_INTERFACE_FLAG_NAME, JVMS_MODULE_FLAG_NAME, JVMS_NATIVE_FLAG_NAME,
    JVMS_PRIVATE_FLAG_NAME, JVMS_PROTECTED_FLAG_NAME, JVMS_PUBLIC_FLAG_NAME, JVMS_STATIC_FLAG_NAME,
    JVMS_STRICT_FLAG_NAME, JVMS_SUPER_FLAG_NAME, JVMS_SYNCHRONIZED_FLAG_NAME,
    JVMS_SYNTHETIC_FLAG_NAME, JVMS_VARARGS_FLAG_NAME,
};
use crate::token::{
    FLAG_ABSTRACT, FLAG_ANNOTATION, FLAG_BRIDGE, FLAG_ENUM, FLAG_FINAL, FLAG_INTERFACE,
    FLAG_MODULE, FLAG_NATIVE, FLAG_PRIVATE, FLAG_PROTECTED, FLAG_PUBLIC, FLAG_STATIC, FLAG_STRICT,
    FLAG_SUPER, FLAG_SYNCHRONIZED, FLAG_SYNTHETIC, FLAG_VARARGS,
};

#[derive(Debug, Eq, PartialEq, Clone, Copy, Ord, PartialOrd)]
pub enum RnsClassFlag {
    Public,
    Static,
    Final,
    Super,
    Interface,
    Abstract,
    Enum,
    Synthetic,
    Annotation,
    Module,
}

impl RnsClassFlag {
    pub fn jvm_spec_name(&self) -> &'static str {
        match self {
            RnsClassFlag::Interface => JVMS_INTERFACE_FLAG_NAME,
            RnsClassFlag::Abstract => JVMS_ABSTRACT_FLAG_NAME,
            RnsClassFlag::Enum => JVMS_ENUM_FLAG_NAME,
            RnsClassFlag::Module => JVMS_MODULE_FLAG_NAME,
            RnsClassFlag::Public => JVMS_PUBLIC_FLAG_NAME,
            RnsClassFlag::Static => JVMS_STATIC_FLAG_NAME,
            RnsClassFlag::Final => JVMS_FINAL_FLAG_NAME,
            RnsClassFlag::Super => JVMS_SUPER_FLAG_NAME,
            RnsClassFlag::Synthetic => JVMS_SYNTHETIC_FLAG_NAME,
            RnsClassFlag::Annotation => JVMS_ANNOTATION_FLAG_NAME,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            RnsClassFlag::Interface => FLAG_INTERFACE,
            RnsClassFlag::Abstract => FLAG_ABSTRACT,
            RnsClassFlag::Enum => FLAG_ENUM,
            RnsClassFlag::Module => FLAG_MODULE,
            RnsClassFlag::Public => FLAG_PUBLIC,
            RnsClassFlag::Static => FLAG_STATIC,
            RnsClassFlag::Final => FLAG_FINAL,
            RnsClassFlag::Super => FLAG_SUPER,
            RnsClassFlag::Synthetic => FLAG_SYNTHETIC,
            RnsClassFlag::Annotation => FLAG_ANNOTATION,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Ord, PartialOrd)]
pub enum RnsMethodFlag {
    Public,
    Private,
    Protected,
    Static,
    Final,
    Synchronized,
    Bridge,
    Varargs,
    Native,
    Abstract,
    Strict,
    Synthetic,
}

impl RnsMethodFlag {
    pub fn jvm_spec_name(&self) -> &'static str {
        match self {
            RnsMethodFlag::Public => JVMS_PUBLIC_FLAG_NAME,
            RnsMethodFlag::Private => JVMS_PRIVATE_FLAG_NAME,
            RnsMethodFlag::Protected => JVMS_PROTECTED_FLAG_NAME,
            RnsMethodFlag::Static => JVMS_STATIC_FLAG_NAME,
            RnsMethodFlag::Final => JVMS_FINAL_FLAG_NAME,
            RnsMethodFlag::Synchronized => JVMS_SYNCHRONIZED_FLAG_NAME,
            RnsMethodFlag::Bridge => JVMS_BRIDGE_FLAG_NAME,
            RnsMethodFlag::Varargs => JVMS_VARARGS_FLAG_NAME,
            RnsMethodFlag::Native => JVMS_NATIVE_FLAG_NAME,
            RnsMethodFlag::Abstract => JVMS_ABSTRACT_FLAG_NAME,
            RnsMethodFlag::Strict => JVMS_STRICT_FLAG_NAME,
            RnsMethodFlag::Synthetic => JVMS_SYNTHETIC_FLAG_NAME,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            RnsMethodFlag::Public => FLAG_PUBLIC,
            RnsMethodFlag::Private => FLAG_PRIVATE,
            RnsMethodFlag::Protected => FLAG_PROTECTED,
            RnsMethodFlag::Static => FLAG_STATIC,
            RnsMethodFlag::Final => FLAG_FINAL,
            RnsMethodFlag::Synchronized => FLAG_SYNCHRONIZED,
            RnsMethodFlag::Bridge => FLAG_BRIDGE,
            RnsMethodFlag::Varargs => FLAG_VARARGS,
            RnsMethodFlag::Native => FLAG_NATIVE,
            RnsMethodFlag::Abstract => FLAG_ABSTRACT,
            RnsMethodFlag::Strict => FLAG_STRICT,
            RnsMethodFlag::Synthetic => FLAG_SYNTHETIC,
        }
    }
}
