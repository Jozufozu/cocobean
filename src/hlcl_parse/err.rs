use std::fmt;

use hlcl_span::Span;

#[derive(Debug)]
pub enum ParserError {
    EofInString(Span),
    UnrecognisedToken(Span, String),
    IntTooBig(Span),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::EofInString(_) => write!(f, "EofInString"),
            ParserError::UnrecognisedToken(_, _) => write!(f, "UnrecognisedToken"),
            ParserError::IntTooBig(_) => write!(f, "IntTooBig"),
        }
    }
}
