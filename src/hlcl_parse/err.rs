use hlcl_span::Span;

#[derive(Debug)]
pub enum ParserError {
    EofInString(Span),
    UnrecognisedToken(Span, String),
    IntTooBig(Span),
}
