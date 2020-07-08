#[macro_use]
extern crate derive_more;

use crate::parse::ParsingSession;

pub mod ast;
pub mod parse;
pub mod project;
pub mod resolve;
pub mod span;

/// TODO: Collect names and item signatures
/// TODO: Perform sanity check
/// TODO: Desugar/lower AST
/// TODO: Type checking oh god
///
/// TODO: ERROR MESSAGES
/// TODO: Attributes
/// TODO: Pretty print ast (fmt command)
///
/// long term
/// TODO: STANDARD LIBRARY
/// TODO: better project management (like Cargo.toml)
/// TODO: DOCUMENTATION (both language support and documenting the compiler)
///
fn main() {
    let session = ParsingSession::new("./test_prog", "test_prog".into()).unwrap();

    let project = session.parse_project().unwrap();
}
