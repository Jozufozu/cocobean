use crate::parse::ParsingSession;

pub mod ast;
pub mod parse;
pub mod span;

fn main() {
    let session = ParsingSession::new("./test_prog/main.hlcl");

    let project = session.parse_project();
}
