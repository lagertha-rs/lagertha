use std::ops::Range;

// TODO: use u32 instead of usize?
#[derive(Debug, Eq, PartialEq, Clone, Copy, Default)]
pub struct Span {
    // Global position
    pub byte_start: usize,
    pub byte_end: usize, // is exclusive

    // Relative positions
    pub line: usize,
    pub col_start: usize,
    pub col_end: usize, // is exclusive
}

impl Span {
    pub(crate) fn as_range(&self) -> Range<usize> {
        self.byte_start..self.byte_end
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Spanned<T> {
    pub value: T,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub fn new(value: T, span: Span) -> Self {
        Self { value, span }
    }
}
