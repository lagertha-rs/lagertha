use crate::flags::{ClassFlags, MethodFlags};
use common::error::ClassFormatErr;
use common::utils::indent_write::Indented;
use std::fmt::Write as _;

impl ClassFlags {
    pub(super) fn fmt_rns(&self, ind: &mut Indented) -> Result<(), ClassFormatErr> {
        if self.is_module() {
            unimplemented!();
            //write!(ind, "module ")?;
        } else if self.is_annotation() {
            unimplemented!();
            //write!(ind, "@interface ")?;
        } else if self.is_interface() {
            unimplemented!();
            //write!(ind, "interface ")?;
        } else {
            write!(ind, ".class ")?;
        }

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

        Ok(())
    }
}

impl MethodFlags {
    pub(super) fn fmt_rns(&self, ind: &mut Indented) -> Result<(), ClassFormatErr> {
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
}
