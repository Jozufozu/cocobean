use std::collections::{BTreeSet, HashMap};
use std::ops::{Range, Div};

use crate::Span;

#[derive(Debug)]
pub struct SourceMap {
    pub data: String,
    pub file_names: HashMap<Span, String>,
    pub spans: BTreeSet<Span>,
}

impl SourceMap {
    pub fn with_length(len: usize) -> Self {
        SourceMap {
            data: {
                let len = len + len % 8;
                let mut string = String::with_capacity(len);

                // fill the string with empty bytes so we can randomly index into the entire thing
                for _ in 0..len.div(8) {
                    string.push_str("        ");
                }

                string
            },
            file_names: HashMap::new(),
            spans: BTreeSet::new(),
        }
    }

    pub fn insert_source(
        &mut self,
        SourceFile {
            source,
            file_name,
            map_span,
        }: SourceFile,
    ) {
        self.data.replace_range(Range::from(map_span), &source);
        self.spans.insert(map_span);
        self.file_names.insert(map_span, file_name);
    }

    #[inline]
    pub fn get_span(&self, span: Span) -> Option<&str> {
        self.data.get(Range::from(span))
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
        SourceFile {
            source,
            file_name,
            map_span,
        }
    }
}

#[cfg(test)]
mod tests {

}