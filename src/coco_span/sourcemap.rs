use std::ops::{Div, Range};

use crate::{Span, BytePos};
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use std::num::NonZeroUsize;

#[cfg(test)]
mod tests;

/// Contains information about a given location in the `SourceMap`.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Loc<'s> {
    /// The project relative file name this location is in.
    pub file_name: &'s str,
    /// The (0-based) column number of the location.
    pub col: usize,
    /// The (1-based) line number of the location.
    pub line: usize,
}

#[derive(Debug)]
pub struct FileInfo {
    /// The part of the `SourceMap` that contains the data annotated by this `FileInfo`.
    pub span: Span,
    /// The project relative file name of the file.
    pub name: String,
    /// The start positions of each line in the file.
    lines: Vec<BytePos>,
}

impl FileInfo {
    pub fn new<S: AsRef<str>>(source_span: Span, data: S, name: String) -> Self {
        let mut lines = vec![source_span.lo];
        for (idx, ch) in data.as_ref().char_indices() {
            if ch == '\n' {
                lines.push(BytePos::from(idx + 1) + source_span.lo);
            }
        }

        FileInfo {
            span: source_span,
            name,
            lines,
        }
    }

    /// Gets the (1-based) line number for the given byte pos.
    ///
    /// Returns `None` if this file is empty.
    pub fn lookup_line(&self, pos: BytePos) -> Option<NonZeroUsize> {
        if self.lines.is_empty() {
            return None
        }
        let idx = self.lines
            .binary_search(&pos)
            .map(|idx| idx + 1)
            .unwrap_or_else(|idx| idx);

        NonZeroUsize::new(idx)
    }

    #[inline]
    pub fn line_start(&self, line: usize) -> Option<BytePos> {
        self.lines.get(line.saturating_sub(1)).map(|b| *b)
    }

    pub fn get_loc(&self, pos: BytePos) -> Option<Loc> {
        let line = self.lookup_line(pos)?.get();
        let line_start = self.line_start(line)?;

        let col = (pos.0 - line_start.0) as usize;

        Some(Loc {
            file_name: &self.name,
            line,
            col,
        })
    }
}

#[derive(Debug)]
pub struct SourceMap {
    actual_len: usize,
    data: String,
    files: Vec<(Span, FileInfo)>,
}

impl SourceMap {
    pub fn with_length(len: usize) -> Self {
        SourceMap {
            actual_len: len,
            data: SourceMap::fill_string(len),
            files: Vec::new(),
        }
    }

    fn fill_string(len: usize) -> String {
        let len = len + len % 8;
        let mut string = String::with_capacity(len);

        // fill the string with empty bytes so we can randomly index into the entire thing
        for _ in 0..len.div(8) {
            string.push_str("        ");
        }

        string
    }

    pub fn insert_source<S: AsRef<str>>(
        &mut self,
        InputSource {
            source,
            file_name,
            map_span,
        }: InputSource<S>,
    ) {
        assert_eq!(
            map_span.len() as usize,
            source.as_ref().len(),
            "content and span have different lengths"
        );
        if !self.files.is_empty() {
            let idx = self
                .files
                .binary_search_by_key(&map_span.lo, |k| k.0.lo)
                .unwrap_or_else(|k| k.saturating_sub(1));
            if map_span.lo < self.files[idx].0.hi {
                panic!("overlapping source files")
            }
        }

        self.data.replace_range(Range::from(map_span), source.as_ref());

        let info = FileInfo::new(map_span, source, file_name);
        self.files.push((map_span, info));

        self.files.sort_unstable_by_key(|k| k.0.lo);
    }

    pub fn file_info_for(&self, span: Span) -> Result<&FileInfo, SpanError> {
        if self.files.is_empty() {
            return Err(SpanError::EmptyMap);
        }

        if self.files.last().unwrap().0.hi < span.hi {
            return Err(SpanError::OutOfRange);
        }

        let idx = self
            .files
            .binary_search_by_key(&span.lo, |k| k.0.lo)
            .unwrap_or_else(|k| k.saturating_sub(1));

        let (file_span, info) = &self.files[idx];

        if span.is_subspan_of(*file_span) {
            Ok(info)
        } else {
            Err(SpanError::SpanningMultiple)
        }
    }

    #[inline]
    fn span_in_one_file(&self, span: Span) -> bool {
        self.file_info_for(span).is_ok()
    }

    #[inline]
    pub fn get_span(&self, span: Span) -> Option<&str> {
        if self.span_in_one_file(span) {
            self.data.get(Range::from(span))
        } else {
            None
        }
    }

    pub fn get_loc(&self, span: Span) -> Option<Loc> {
        let parent = self.file_info_for(span).ok()?;
        parent.get_loc(span.lo)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SpanError {
    EmptyMap,
    SpanningMultiple,
    OutOfRange,
}

impl fmt::Display for SpanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct InputSource<S: AsRef<str> = String> {
    pub source: S,
    pub file_name: String,
    pub map_span: Span,
}

impl<S: AsRef<str>> InputSource<S> {
    pub fn new(source: S, file_name: String, map_span: Span) -> Self {
        InputSource {
            source,
            file_name,
            map_span,
        }
    }
}
