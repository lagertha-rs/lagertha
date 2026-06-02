use jclass::ClassFile;

mod indent_write;
mod attribute;
mod class;
mod constant_pool;
mod flags;
mod instruction;
mod method;

pub fn disassemble(class: &ClassFile) -> Result<String, std::fmt::Error> {
    class::fmt_rns(&class)
}