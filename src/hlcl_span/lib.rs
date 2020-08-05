use derive_more::*;

use std::mem;
pub use lasso;

pub mod kw;

use std::fmt;
use std::fmt::{Display, Formatter};
use std::collections::{HashMap, BTreeSet};
use std::ops::{RangeBounds, Bound, Range};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Span {
    pub l: BytePos,
    pub r: BytePos,
}

#[derive(Debug, Copy, Clone, Add, Sub, Ord, PartialOrd, Eq, PartialEq, From, Into, Hash)]
pub struct BytePos(pub u32);

#[allow(clippy::len_without_is_empty)]
impl Span {
    pub const DUMMY: Span = Span { l: BytePos(0), r: BytePos(0) };

    #[inline]
    pub fn new(mut l: usize, mut r: usize) -> Span {
        if l > r {
            mem::swap(&mut l, &mut r);
        }
        Span {
            l: BytePos::from(l as u32),
            r: BytePos::from(r as u32),
        }
    }

    #[inline]
    pub const fn len(&self) -> u32 {
        self.r.0 - self.l.0
    }

    #[inline]
    pub const fn lo_idx(&self) -> usize {
        self.l.0 as usize
    }

    #[inline]
    pub const fn hi_idx(&self) -> usize {
        self.l.0 as usize
    }
}

impl From<Span> for Range<usize> {
    fn from(sp: Span) -> Self {
        Range {
            start: sp.lo_idx(),
            end: sp.hi_idx(),
        }
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
}

impl<T: Eq + PartialEq> Eq for Spanned<T> {}

impl<T: Display> Display for Spanned<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.val.fmt(f)
    }
}

#[derive(Debug)]
pub struct SourceMap {
    data: String,
    file_names: HashMap<Span, String>,
    spans: BTreeSet<Span>,
}

impl SourceMap {
    pub fn with_capacity(cap: usize) -> Self {
        SourceMap {
            data: String::with_capacity(cap),
            file_names: HashMap::new(),
            spans: BTreeSet::new(),
        }
    }

    pub fn insert_source(&mut self, SourceFile { source, file_name, map_span }: SourceFile) {
        self.data.replace_range(Range::from(map_span), &source);
        self.spans.insert(map_span);
        self.file_names.insert(map_span, file_name);
    }
}

#[derive(Debug)]
pub struct SourceFile {
    pub source: String,
    pub file_name: String,
    pub map_span: Span,
}

impl SourceFile {
    pub fn new(source: String, file_name: String, map_span: Span) -> Self {
        SourceFile { source, file_name, map_span }
    }
}
