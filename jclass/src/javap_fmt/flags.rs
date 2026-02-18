use crate::flags::{ClassFlags, FieldFlags, InnerClassFlags, MethodFlags, MethodParamFlags};
use common::error::ClassFormatErr;
use common::utils::indent_write::Indented;
use std::fmt::Write as _;

impl InnerClassFlags {
    /// java like class prefix: "public abstract class", "public interface"...
    pub fn javap_java_like_prefix(&self) -> String {
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

impl ClassFlags {
    /// java like class prefix: "public abstract class", "public interface"...
    pub fn javap_fmt_java_like_prefix(&self, ind: &mut Indented) -> Result<(), ClassFormatErr> {
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
            write!(ind, "module ")?;
        } else if self.is_annotation() {
            write!(ind, "@interface ")?;
        } else if self.is_interface() {
            write!(ind, "interface ")?;
        } else {
            write!(ind, "class ")?;
        }
        Ok(())
    }

    /// Prints javap-like list of flags like "ACC_PUBLIC, ACC_FINAL, ACC_SUPER"
    pub fn fmt_class_javap_like_list(&self, ind: &mut Indented) -> Result<(), ClassFormatErr> {
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

impl MethodFlags {
    /// Prints java like class prefix: "public static final"...
    pub fn javap_fmt_java_like_prefix(&self, ind: &mut Indented) -> Result<(), ClassFormatErr> {
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

    /// Prints javap-like list of flags like "ACC_PUBLIC, ACC_FINAL, ACC_SUPER"
    pub fn fmt_class_javap_like_list(&self, ind: &mut Indented) -> Result<(), ClassFormatErr> {
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

impl MethodParamFlags {
    pub fn javap_fmt(&self, ind: &mut Indented) -> Result<(), ClassFormatErr> {
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

impl FieldFlags {
    /// Java-like modifier prefix for a field header
    pub fn fmt_field_javap_java_like_prefix(
        &self,
        ind: &mut Indented,
    ) -> Result<(), ClassFormatErr> {
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

        Ok(())
    }

    /// Prints javap-like list of flags like "ACC_PUBLIC, ACC_FINAL, ACC_SUPER"
    pub fn fmt_class_javap_like_list(&self, ind: &mut Indented) -> Result<(), ClassFormatErr> {
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
