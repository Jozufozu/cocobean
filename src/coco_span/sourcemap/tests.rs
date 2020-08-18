use crate::sourcemap::{InputSource, SpanError, SourceMap, FileInfo};
use crate::{Span, BytePos};

/// Test if it can successfully insert multiple files
#[test]
fn multiple_files() {
    let source1 = "blah blah blah blah";
    let file1 = InputSource::new(source1, "test1".to_string(), Span::new(0, source1.len()));

    let source2 = "yadda yadda yadda yadda yadda yadda";
    let file2 = InputSource::new(
        source2,
        "test2".to_string(),
        Span::new(source1.len(), source1.len() + source2.len()),
    );

    let mut map = SourceMap::with_length(source1.len() + source2.len());

    map.insert_source(file1);
    map.insert_source(file2);

    assert_eq!(Some("ah bl"), map.get_span(Span::new(2, 7)));
    assert_eq!(Some("yadda"), map.get_span(Span::new(19, 24)));
}

/// Test if it actually panics when source files have overlapping map_spans
#[test]
#[should_panic(expected = "overlapping source files")]
fn overlapping_files() {
    let source1 = "blah blah blah blah";
    let file1 = InputSource::new(source1, "test1".to_string(), Span::new(0, source1.len()));

    let source2 = "yadda yadda yadda yadda yadda yadda";
    let file2 = InputSource::new(
        source2,
        "test2".to_string(),
        Span::new(source1.len() - 10, source1.len() + source2.len() - 10),
    );

    let mut map = SourceMap::with_length(source1.len() + source2.len());

    map.insert_source(file1);
    map.insert_source(file2);
}

/// Test if it actually panics when a source file's content and span have different lengths
#[test]
#[should_panic(expected = "content and span have different lengths")]
fn invalid_span() {
    let source1 = "blah blah blah blah";
    let file1 = InputSource::new(
        source1,
        "test1".to_string(),
        Span::new(0, source1.len() + 20),
    );

    let mut map = SourceMap::with_length(source1.len());

    map.insert_source(file1);
}

/// Test if it returns the correct parent span
#[test]
fn parent_span() {
    let source1 = "blah blah blah blah";
    let span1 = Span::new(0, source1.len());
    let file1 = InputSource::new(source1, "test1".to_string(), span1);

    let source2 = "yadda yadda yadda yadda yadda yadda";
    let span2 = Span::new(source1.len(), source1.len() + source2.len());
    let file2 = InputSource::new(source2, "test2".to_string(), span2);

    let mut map = SourceMap::with_length(source1.len() + source2.len());

    assert!(matches!(
        map.file_info_for(Span::new(2, 7)),
        Err(SpanError::EmptyMap)
    ));

    map.insert_source(file1);
    map.insert_source(file2);

    assert_eq!(span1, map.file_info_for(Span::new(2, 7)).unwrap().span);
    assert_eq!(span1, map.file_info_for(span1).unwrap().span);
    assert_eq!(span2, map.file_info_for(Span::new(19, 24)).unwrap().span);
    assert_eq!(span2, map.file_info_for(span2).unwrap().span);
    assert!(matches!(
        map.file_info_for(Span::new(200, 300)),
        Err(SpanError::OutOfRange)
    ));
    assert!(matches!(
        map.file_info_for(Span::new(7, 20)),
        Err(SpanError::SpanningMultiple)
    ));
}

#[test]
fn line_lookup() {
    let data = "line 1\nline2\nline3\r\nline4\n";
    let info = FileInfo::new(Span::new(0, data.len()), data, "test/mod.coco".to_string());

    assert_eq!(1, info.lookup_line(BytePos(0)).unwrap().get());
    assert_eq!(1, info.lookup_line(BytePos(6)).unwrap().get());
    assert_eq!(2, info.lookup_line(BytePos(7)).unwrap().get());
    assert_eq!(3, info.lookup_line(BytePos(18)).unwrap().get());
    assert_eq!(3, info.lookup_line(BytePos(19)).unwrap().get());
    assert_eq!(4, info.lookup_line(BytePos(20)).unwrap().get());
    assert_eq!(4, info.lookup_line(BytePos(25)).unwrap().get());
    assert_eq!(5, info.lookup_line(BytePos(26)).unwrap().get());
}

#[test]
fn loc_lookup() {
    let data = "line 1\nline2\nline3\r\nline4\n";
    let info = FileInfo::new(Span::new(0, data.len()), data, "test/mod.coco".to_string());

    let loc = info.get_loc(BytePos(0)).unwrap();
    assert_eq!(1, loc.line);
    assert_eq!(0, loc.col);

    let loc = info.get_loc(BytePos(8)).unwrap();
    assert_eq!(2, loc.line);
    assert_eq!(1, loc.col);

    let loc = info.get_loc(BytePos(10)).unwrap();
    assert_eq!(2, loc.line);
    assert_eq!(3, loc.col);
}