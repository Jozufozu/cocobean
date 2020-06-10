#[allow(clippy::all)]
#[cfg_attr(rustfmt, rustfmt_skip)]
mod hlcl;
mod ast;
mod lexer;

fn main() {
    let input =
r#"type Thing: !Other {
    wow: int,
    amazing: bool
}
"#;

    let lex = lexer::Lexer::new(input);
    let program = hlcl::ProgramParser::new().parse(input, lex);

    println!("{:?}", program)
}
