use std::fmt;

/// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.1
/// Table 4.1-B. Class access and property modifiers
#[derive(Debug, Clone, Copy)]
pub struct ClassAccessFlag(pub u16);

/// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.6
/// Table 4.6-A. Method access and property flags
#[derive(Debug, Clone, Copy)]
pub struct MethodAccessFlag(pub u16);

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
}

impl MethodAccessFlag {
    pub fn new(value: u16) -> Self {
        Self(value)
    }

    pub fn is_public(&self) -> bool {
        self.0 & 0x0001 != 0
    }

    pub fn is_private(&self) -> bool {
        self.0 & 0x0001 != 0
    }

    pub fn is_protected(&self) -> bool {
        self.0 & 0x0001 != 0
    }

    pub fn is_static(&self) -> bool {
        self.0 & 0x0001 != 0
    }

    pub fn is_final(&self) -> bool {
        self.0 & 0x0010 != 0
    }

    pub fn is_synchronized(&self) -> bool {
        self.0 & 0x0020 != 0
    }

    pub fn is_bridge(&self) -> bool {
        self.0 & 0x0400 != 0
    }

    pub fn is_varargs(&self) -> bool {
        self.0 & 0x1000 != 0
    }

    pub fn is_native(&self) -> bool {
        self.0 & 0x2000 != 0
    }

    pub fn is_abstract(&self) -> bool {
        self.0 & 0x4000 != 0
    }

    pub fn is_strict(&self) -> bool {
        self.0 & 0x8000 != 0
    }

    pub fn is_synthetic(&self) -> bool {
        self.0 & 0x8000 != 0
    }
}

impl fmt::Display for MethodAccessFlag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut flags = Vec::new();

        if self.is_public() {
            flags.push("public");
        }
        if self.is_private() {
            flags.push("private");
        }
        if self.is_protected() {
            flags.push("protected");
        }
        if self.is_static() {
            flags.push("static");
        }
        if self.is_final() {
            flags.push("final");
        }
        if self.is_synchronized() {
            flags.push("synchronized");
        }
        if self.is_bridge() {
            flags.push("bridge");
        }
        if self.is_varargs() {
            flags.push("varargs");
        }
        if self.is_native() {
            flags.push("native");
        }
        if self.is_abstract() {
            flags.push("abstract");
        }
        if self.is_strict() {
            flags.push("strict");
        }
        if self.is_synthetic() {
            flags.push("synthetic");
        }

        write!(f, "({})", flags.join(" "))
    }
}

impl fmt::Display for ClassAccessFlag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut flags = Vec::new();

        if self.is_public() {
            flags.push("public");
        }
        if self.is_final() {
            flags.push("final");
        }
        if self.is_super() {
            flags.push("super");
        }
        if self.is_interface() {
            flags.push("interface");
        }
        if self.is_abstract() {
            flags.push("abstract");
        }
        if self.is_synthetic() {
            flags.push("synthetic");
        }
        if self.is_annotation() {
            flags.push("annotation");
        }
        if self.is_enum() {
            flags.push("enum");
        }
        if self.is_module() {
            flags.push("module");
        }

        write!(f, "({})", flags.join(" "))
    }
}
