use crate::ClassFile;
use common::error::ClassFormatErr;
use common::utils::indent_write::Indented;
use std::fmt::Write as _;

impl ClassFile {
    fn fmt_signature(&self, ind: &mut Indented) -> Result<(), ClassFormatErr> {
        self.access_flags.fmt_jasm(ind)?;
        self.cp
            .get_raw_entry(self.this_class)?
            .fmt_jasm(ind, &self.cp)?;
        writeln!(ind)?;
        Ok(())
    }

    fn fmt_super_class(&self, ind: &mut Indented) -> Result<(), ClassFormatErr> {
        if self.super_class == 0 {
            return Ok(());
        }
        write!(ind, ".super ",)?;
        self.cp
            .get_raw_entry(self.super_class)?
            .fmt_jasm(ind, &self.cp)?;
        writeln!(ind)?;
        Ok(())
    }

    pub fn fmt_jasm(&self) -> Result<String, ClassFormatErr> {
        let mut out = String::new();
        let mut ind = Indented::new(&mut out);
        self.fmt_signature(&mut ind)?;

        self.fmt_super_class(&mut ind)?;
        writeln!(ind)?;
        for (i, method) in self.methods.iter().enumerate() {
            method.fmt_jasm(&mut ind, &self.cp)?;
            if i < self.methods.len() - 1 {
                writeln!(ind)?;
            }
        }

        writeln!(ind, ".end class")?;
        Ok(out)
    }
}
