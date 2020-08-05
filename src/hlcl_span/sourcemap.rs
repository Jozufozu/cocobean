use std::collections::{BTreeSet, HashMap};
use std::ops::Range;

use crate::Span;

#[derive(Debug)]
pub struct SourceMap {
    pub data: String,
    pub file_names: HashMap<Span, String>,
    pub spans: BTreeSet<Span>,
}

impl SourceMap {
    pub fn with_capacity(cap: usize) -> Self {
        SourceMap {
            data: {
                let cap = cap.next_power_of_two();
                let mut string = String::with_capacity(cap);

                for _ in 0..cap {
                    string.push(' ');
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