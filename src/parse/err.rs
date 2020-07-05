use crate::span::Span;

#[derive(Debug)]
pub enum ParserError {
    EofInString(Span),
    UnrecognisedToken(Span, String),
    IntTooBig(Span),
}
