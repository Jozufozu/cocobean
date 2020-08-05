use std::fmt;
use std::fmt::{Display, Formatter};
use std::mem;
use std::ops::Range;

use derive_more::*;
pub use lasso;

pub mod kw;
pub mod sourcemap;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Span {
    pub l: BytePos,
    pub r: BytePos,
}

#[derive(Debug, Copy, Clone, Add, Sub, Ord, PartialOrd, Eq, PartialEq, From, Into, Hash)]
pub struct BytePos(pub u32);

#[allow(clippy::len_without_is_empty)]
impl Span {
    pub const DUMMY: Span = Span {
        l: BytePos(0),
        r: BytePos(0),
    };

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
        self.r.0 as usize
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

#[cfg(test)]
mod tests {
    use crate::Span;

    #[test]
    fn reverse() {
        assert_eq!(Span::new(0, 20), Span::new(20, 0));
    }

    #[test]
    fn lo_hi() {
        let span = Span::new(0, 20);
        assert_eq!(span.hi_idx(), 20);
        assert_eq!(span.lo_idx(), 0);
    }

    #[test]
    fn len() {
        let span = Span::new(0, 20);
        assert_eq!(span.len(), 20);
    }
}
