use crate::session::Session;

pub mod session;
pub mod parse;
pub mod ast;

fn main() {
    let mut session = Session::new();

    session.parse("./src/test_prog/main.hlcl");
}
