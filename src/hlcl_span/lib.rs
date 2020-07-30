#[macro_use]
extern crate derive_more;

use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone)]
pub struct Span {
    pub l: usize,
    pub r: usize,
}

#[derive(Debug, Copy, Clone, Add, Sub, Ord, PartialOrd, Eq, PartialEq, From)]
pub struct BytePos(pub u32);

pub struct SpanData {
    pub l: BytePos,
    pub r: BytePos,
}

impl Span {
    pub const DUMMY: Span = Span::new(0, 0);

    #[inline]
    pub const fn new(l: usize, r: usize) -> Span {
        Span { l, r }
    }
}

#[derive(Debug, Clone)]
pub struct Spanned<T> {
    pub span: Span,
    pub val: T,
}

impl<T> Spanned<T> {
    #[inline(always)]
    pub fn new(l: usize, r: usize, val: T) -> Self {
        Spanned {
            span: Span::new(l, r),
            val,
        }
    }

    pub fn dummy(val: T) -> Self {
        Spanned {
            span: Span::DUMMY,
            val,
        }
    }
}

impl<T: PartialEq> PartialEq for Spanned<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }

    #[inline]
    fn ne(&self, other: &Self) -> bool {
        self.val != other.val
    }
}

impl<T: Eq + PartialEq> Eq for Spanned<T> {}

impl<T: Display> Display for Spanned<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.val.fmt(f)
    }
}

pub struct SourceMap {
    data: String,
}

#[derive(Debug)]
pub struct SourceFile {
    pub source: String,
    pub file_name: String,
}

impl SourceFile {
    pub fn new(source: String, file_name: String) -> Self {
        SourceFile { source, file_name }
    }
}
