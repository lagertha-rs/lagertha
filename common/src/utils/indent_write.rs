use std::fmt::{self};

/// Allows to avoid boilerplate error handling when pretty printing and
/// printing errors to the given indented writer.
#[macro_export]
macro_rules! pretty_try {
    ($ind:expr, $expr:expr) => {
        match $expr {
            Ok(v) => v,
            Err(e) => {
                let _ = $ind.with_specific_indent(0, |ind| {
                    ::std::writeln!(ind, "\n****************************************")?;
                    let err_str = format!("{}", e);
                    if err_str.is_empty() {
                        ::std::writeln!(ind, "print error (no message): debug {e:?}")
                    } else {
                        ::std::writeln!(ind, "print error: {}", err_str)
                    }
                });
                return Err(::std::fmt::Error);
            }
        }
    };
}

// TODO: next 2 macros are specific to class files, move them to jclass crate?

#[doc(hidden)]
#[macro_export]
macro_rules! pretty_class_name_try {
    ($ind:expr, $expr:expr) => {
        match $expr {
            Ok(class) => class
                .trim_start_matches('L')
                .trim_end_matches(';')
                .replace('/', "."),
            Err(e) => {
                let _ = $ind.with_specific_indent(0, |ind| {
                    ::std::writeln!(ind, "\n****************************************")?;
                    let err_str = format!("{}", e);
                    if err_str.is_empty() {
                        ::std::writeln!(ind, "print error (no message): debug {e:?}")
                    } else {
                        ::std::writeln!(ind, "print error: {}", err_str)
                    }
                });
                return Err(::std::fmt::Error);
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! pretty_method_name_try {
    ($ind:expr, $expr:expr) => {
        match $expr {
            Ok(method_name) => match method_name {
                "<init>" => format!("\"{}\"", method_name),
                _ => method_name.to_string(),
            },
            Err(e) => {
                let _ = $ind.with_specific_indent(0, |ind| {
                    ::std::writeln!(ind, "\n****************************************")?;
                    let err_str = format!("{}", e);
                    if err_str.is_empty() {
                        ::std::writeln!(ind, "print error (no message): debug {e:?}")
                    } else {
                        ::std::writeln!(ind, "print error: {}", err_str)
                    }
                });
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

    pub fn with_specific_indent<F>(&mut self, level: usize, f: F) -> fmt::Result
    where
        F: FnOnce(&mut Self) -> fmt::Result,
    {
        let prev_level = self.level;
        self.level = level;
        let res = f(self);
        self.level = prev_level;
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
