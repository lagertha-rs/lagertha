use crate::disassembler::DisasmResult;
use std::fmt;

pub struct Indented<'a> {
    inner: &'a mut dyn fmt::Write,
    unit: &'static str,
    level: usize,
    at_line_start: bool,
}

impl<'a> Indented<'a> {
    pub fn new(inner: &'a mut dyn fmt::Write) -> Self {
        Self {
            inner,
            unit: "  ",
            level: 0,
            at_line_start: true,
        }
    }

    pub fn with_indent<F>(&mut self, f: F) -> DisasmResult<()>
    where
        F: FnOnce(&mut Self) -> DisasmResult<()>,
    {
        self.level += 1;
        let result = f(self);
        self.level -= 1;
        result
    }
}

impl fmt::Write for Indented<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for chunk in s.split_inclusive('\n') {
            if self.at_line_start {
                for _ in 0..self.level {
                    self.inner.write_str(self.unit)?;
                }
                self.at_line_start = false;
            }
            self.inner.write_str(chunk)?;
            self.at_line_start = chunk.ends_with('\n');
        }
        Ok(())
    }
}
