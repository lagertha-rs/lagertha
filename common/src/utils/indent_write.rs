use std::fmt::{self};

/// Allows to avoid boilerplate error handling when pretty printing and
/// printing errors to the given indented writer.
#[macro_export]
macro_rules! pretty_try {
    ($ind:expr, $expr:expr) => {
        match $expr {
            Ok(v) => v,
            Err(e) => {
                let _ = ::std::writeln!($ind, "\nprint error: {}", e);
                return Err(::std::fmt::Error);
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! pretty_class_name_try {
    ($ind:expr, $expr:expr) => {
        match $expr {
            Ok(class) => class.replace("/", "."),
            Err(e) => {
                let _ = ::std::writeln!($ind, "\nprint error: {}", e);
                return Err(::std::fmt::Error);
            }
        }
    };
}

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

    pub fn with_indent<F>(&mut self, f: F) -> fmt::Result
    where
        F: FnOnce(&mut Self) -> fmt::Result,
    {
        self.level += 1;
        let res = f(self);
        self.level -= 1;
        res
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
