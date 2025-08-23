/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.1-200-E.1
/// Table 4.1-B. Class access and property modifiers
#[derive(Debug, Clone, Copy)]
pub struct ClassAccessFlag(u16);

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.6-200-A.1
/// Table 4.6-A. Method access and property flags
#[derive(Debug, Clone, Copy)]
pub struct MethodAccessFlag(u16);

impl ClassAccessFlag {
    pub fn new(value: u16) -> Self {
        Self(value)
    }

    pub fn is_public(&self) -> bool {
        self.0 & 0x0001 != 0
    }

    pub fn is_final(&self) -> bool {
        self.0 & 0x0010 != 0
    }

    pub fn is_super(&self) -> bool {
        self.0 & 0x0020 != 0
    }

    pub fn is_interface(&self) -> bool {
        self.0 & 0x0200 != 0
    }

    pub fn is_abstract(&self) -> bool {
        self.0 & 0x0400 != 0
    }

    pub fn is_synthetic(&self) -> bool {
        self.0 & 0x1000 != 0
    }

    pub fn is_annotation(&self) -> bool {
        self.0 & 0x2000 != 0
    }

    pub fn is_enum(&self) -> bool {
        self.0 & 0x4000 != 0
    }

    pub fn is_module(&self) -> bool {
        self.0 & 0x8000 != 0
    }

    pub fn get_raw(&self) -> &u16 {
        &self.0
    }

    #[cfg(feature = "pretty_print")]
    pub fn get_pretty_java_like_prefix(&self) -> String {
        let mut flags = Vec::new();
        if self.is_public() {
            flags.push("public");
        }
        if self.is_abstract() && !self.is_interface() {
            flags.push("abstract");
        }
        if self.is_final() {
            flags.push("final");
        }

        if self.is_interface() {
            if self.is_annotation() {
                flags.push("@interface");
            } else {
                flags.push("interface");
            }
        } else if self.is_enum() {
            flags.push("enum");
        } else if self.is_module() {
            flags.push("module");
        } else {
            flags.push("class");
        }

        flags.join(" ")
    }

    #[cfg(feature = "pretty_print")]
    pub fn get_javap_like_list(&self) -> String {
        let mut flags = Vec::new();
        if self.is_public() {
            flags.push("ACC_PUBLIC");
        }
        if self.is_final() {
            flags.push("ACC_FINAL");
        }
        if self.is_super() {
            flags.push("ACC_SUPER");
        }
        if self.is_interface() {
            flags.push("ACC_INTERFACE");
        }
        if self.is_abstract() {
            flags.push("ACC_ABSTRACT");
        }
        if self.is_synthetic() {
            flags.push("ACC_SYNTHETIC");
        }
        if self.is_annotation() {
            flags.push("ACC_ANNOTATION");
        }
        if self.is_enum() {
            flags.push("ACC_ENUM");
        }
        if self.is_module() {
            flags.push("ACC_MODULE");
        }
        flags.join(", ")
    }
}

impl MethodAccessFlag {
    pub fn new(value: u16) -> Self {
        Self(value)
    }

    pub fn is_public(&self) -> bool {
        self.0 & 0x0001 != 0
    }

    pub fn is_private(&self) -> bool {
        self.0 & 0x0002 != 0
    }

    pub fn is_protected(&self) -> bool {
        self.0 & 0x0004 != 0
    }

    pub fn is_static(&self) -> bool {
        self.0 & 0x0008 != 0
    }

    pub fn is_final(&self) -> bool {
        self.0 & 0x0010 != 0
    }

    pub fn is_synchronized(&self) -> bool {
        self.0 & 0x0020 != 0
    }

    pub fn is_bridge(&self) -> bool {
        self.0 & 0x0040 != 0
    }

    pub fn is_varargs(&self) -> bool {
        self.0 & 0x0080 != 0
    }

    pub fn is_native(&self) -> bool {
        self.0 & 0x0100 != 0
    }

    pub fn is_abstract(&self) -> bool {
        self.0 & 0x0400 != 0
    }

    pub fn is_strict(&self) -> bool {
        self.0 & 0x0800 != 0
    }

    pub fn is_synthetic(&self) -> bool {
        self.0 & 0x1000 != 0
    }

    pub fn get_raw(&self) -> &u16 {
        &self.0
    }
}
