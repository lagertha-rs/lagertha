// TODO: Right now I plan to use it in runtime as well. idk if it's a good idea or not.

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.1-200-E.1
/// Table 4.1-B. Class access and property modifiers
#[derive(Debug, Clone, Copy)]
pub struct ClassFlags(u16);

/// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.6-300-D.1-D.1
/// Table 4.7.6-A. Nested class access and property flags
#[derive(Debug, Clone, Copy)]
pub struct InnerClassFlags(u16);

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.6-200-A.1
/// Table 4.6-A. Method access and property flags
#[derive(Debug, Clone, Copy)]
pub struct MethodFlags(u16);

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.7.24
#[derive(Debug, Clone, Copy)]
pub struct MethodParamFlags(u16);

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.5-200-A.1
/// Table 4.5-A. Field access and property flags
#[derive(Debug, Clone, Copy)]
pub struct FieldFlags(u16);

impl ClassFlags {
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
}

impl InnerClassFlags {
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

    pub fn get_raw(&self) -> &u16 {
        &self.0
    }
}

impl MethodFlags {
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

impl MethodParamFlags {
    pub fn new(value: u16) -> Self {
        Self(value)
    }

    pub fn is_final(&self) -> bool {
        self.0 & 0x0010 != 0
    }

    pub fn is_synthetic(&self) -> bool {
        self.0 & 0x1000 != 0
    }

    pub fn is_mandated(&self) -> bool {
        self.0 & 0x8000 != 0
    }

    pub fn get_raw(&self) -> &u16 {
        &self.0
    }
}

impl FieldFlags {
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

    pub fn is_volatile(&self) -> bool {
        self.0 & 0x0040 != 0
    }

    pub fn is_transient(&self) -> bool {
        self.0 & 0x0080 != 0
    }

    pub fn is_synthetic(&self) -> bool {
        self.0 & 0x1000 != 0
    }

    pub fn is_enum(&self) -> bool {
        self.0 & 0x4000 != 0
    }

    pub fn get_raw(&self) -> &u16 {
        &self.0
    }
}

#[cfg(feature = "pretty_print")]
impl InnerClassFlags {
    ///  prints java like class prefix: "public abstract class", "public interface"...
    pub fn pretty_java_like_prefix(&self) -> String {
        let mut flags = vec![];

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

        let is_iface_like = self.is_interface() || self.is_annotation();
        if self.is_abstract() && !is_iface_like {
            flags.push("abstract");
        }
        if self.is_final() && !is_iface_like {
            flags.push("final");
        }

        flags.join(" ")
    }
}

#[cfg(feature = "pretty_print")]
impl ClassFlags {
    ///  prints java like class prefix: "public abstract class", "public interface"...
    pub fn fmt_pretty_java_like_prefix(
        &self,
        ind: &mut common::utils::indent_write::Indented<'_>,
    ) -> std::fmt::Result {
        use std::fmt::Write as _;

        if self.is_public() {
            write!(ind, "public ")?;
        }

        let is_iface_like = self.is_interface() || self.is_annotation() || self.is_module();

        if self.is_abstract() && !is_iface_like {
            write!(ind, "abstract ")?;
        }
        if self.is_final() && !is_iface_like {
            write!(ind, "final ")?;
        }

        if self.is_module() {
            write!(ind, "module ")
        } else if self.is_annotation() {
            write!(ind, "@interface ")
        } else if self.is_interface() {
            write!(ind, "interface ")
        } else if self.is_enum() {
            write!(ind, "enum ")
        } else {
            write!(ind, "class ")
        }
    }

    /// prints javap-like list of flags like "ACC_PUBLIC, ACC_FINAL, ACC_SUPER"
    pub fn fmt_class_javap_like_list(
        &self,
        ind: &mut common::utils::indent_write::Indented<'_>,
    ) -> std::fmt::Result {
        use std::fmt::Write as _;

        let flags = [
            (self.is_public(), "ACC_PUBLIC"),
            (self.is_final(), "ACC_FINAL"),
            (self.is_super(), "ACC_SUPER"),
            (self.is_interface(), "ACC_INTERFACE"),
            (self.is_abstract(), "ACC_ABSTRACT"),
            (self.is_synthetic(), "ACC_SYNTHETIC"),
            (self.is_annotation(), "ACC_ANNOTATION"),
            (self.is_enum(), "ACC_ENUM"),
            (self.is_module(), "ACC_MODULE"),
        ];

        let mut first = true;
        for (on, name) in flags {
            if on {
                if !first {
                    write!(ind, ", ")?;
                }
                write!(ind, "{name}")?;
                first = false;
            }
        }
        Ok(())
    }
}

#[cfg(feature = "pretty_print")]
impl MethodFlags {
    ///  prints java like class prefix: "public static final"...
    pub fn fmt_pretty_java_like_prefix(
        &self,
        ind: &mut common::utils::indent_write::Indented<'_>,
    ) -> std::fmt::Result {
        use std::fmt::Write as _;

        if self.is_public() {
            write!(ind, "public ")?;
        } else if self.is_protected() {
            write!(ind, "protected ")?;
        } else if self.is_private() {
            write!(ind, "private ")?;
        }

        if self.is_static() {
            write!(ind, "static ")?;
        }
        if self.is_final() {
            write!(ind, "final ")?;
        }
        if self.is_synchronized() {
            write!(ind, "synchronized ")?;
        }
        if self.is_native() {
            write!(ind, "native ")?;
        }
        if self.is_abstract() {
            write!(ind, "abstract ")?;
        }
        if self.is_strict() {
            write!(ind, "strictfp ")?;
        }

        Ok(())
    }

    /// prints javap-like list of flags like "ACC_PUBLIC, ACC_FINAL, ACC_SUPER"
    pub fn fmt_class_javap_like_list(
        &self,
        ind: &mut common::utils::indent_write::Indented<'_>,
    ) -> std::fmt::Result {
        use std::fmt::Write as _;

        let flags = [
            (self.is_public(), "ACC_PUBLIC"),
            (self.is_private(), "ACC_PRIVATE"),
            (self.is_protected(), "ACC_PROTECTED"),
            (self.is_static(), "ACC_STATIC"),
            (self.is_final(), "ACC_FINAL"),
            (self.is_synchronized(), "ACC_SYNCHRONIZED"),
            (self.is_bridge(), "ACC_BRIDGE"),
            (self.is_varargs(), "ACC_VARARGS"),
            (self.is_native(), "ACC_NATIVE"),
            (self.is_abstract(), "ACC_ABSTRACT"),
            (self.is_strict(), "ACC_STRICT"),
            (self.is_synthetic(), "ACC_SYNTHETIC"),
        ];

        let mut first = true;
        for (on, name) in flags {
            if on {
                if !first {
                    write!(ind, ", ")?;
                }
                write!(ind, "{name}")?;
                first = false;
            }
        }
        Ok(())
    }
}

#[cfg(feature = "pretty_print")]
impl MethodParamFlags {
    pub fn fmt_pretty(
        &self,
        ind: &mut common::utils::indent_write::Indented<'_>,
    ) -> std::fmt::Result {
        use std::fmt::Write as _;

        if self.is_final() {
            write!(ind, "final ")?;
        }
        if self.is_synthetic() {
            write!(ind, "synthetic ")?;
        }
        if self.is_mandated() {
            write!(ind, "mandated ")?;
        }

        Ok(())
    }
}

#[cfg(feature = "pretty_print")]
impl FieldFlags {
    /// Java-like modifier prefix for a field header
    pub fn fmt_field_pretty_java_like_prefix(
        &self,
        ind: &mut common::utils::indent_write::Indented<'_>,
    ) -> std::fmt::Result {
        use std::fmt::Write as _;

        if self.is_public() {
            write!(ind, "public ")?;
        } else if self.is_protected() {
            write!(ind, "protected ")?;
        } else if self.is_private() {
            write!(ind, "private ")?;
        }

        if self.is_static() {
            write!(ind, "static ")?;
        }
        if self.is_final() {
            write!(ind, "final ")?;
        }
        if self.is_volatile() {
            write!(ind, "volatile ")?;
        }
        if self.is_transient() {
            write!(ind, "transient ")?;
        }
        if self.is_enum() {
            write!(ind, "enum ")?;
        }

        Ok(())
    }

    /// prints javap-like list of flags like "ACC_PUBLIC, ACC_FINAL, ACC_SUPER"
    pub fn fmt_class_javap_like_list(
        &self,
        ind: &mut common::utils::indent_write::Indented<'_>,
    ) -> std::fmt::Result {
        use std::fmt::Write as _;

        let flags = [
            (self.is_public(), "ACC_PUBLIC"),
            (self.is_private(), "ACC_PRIVATE"),
            (self.is_protected(), "ACC_PROTECTED"),
            (self.is_static(), "ACC_STATIC"),
            (self.is_final(), "ACC_FINAL"),
            (self.is_volatile(), "ACC_VOLATILE"),
            (self.is_transient(), "ACC_TRANSIENT"),
            (self.is_synthetic(), "ACC_SYNTHETIC"),
            (self.is_enum(), "ACC_ENUM"),
        ];

        let mut first = true;
        for (on, name) in flags {
            if on {
                if !first {
                    write!(ind, ", ")?;
                }
                write!(ind, "{name}")?;
                first = false;
            }
        }
        Ok(())
    }
}
