#[allow(clippy::all)]
#[cfg_attr(rustfmt, rustfmt_skip)]
mod hlcl;
mod ast;
mod lexer;

fn main() {
    let mut errs = Vec::new();
    let input =
r#"struct Thing: !Other {
    wow: int,
    amazing: bool
}
"#;

    let lex = lexer::Lexer::new(input);
    let program = hlcl::ProgramParser::new().parse(input, &mut errs, lex);

    println!("{:?}", program);

    let input =
        r#"(true, fals, 204103____21__5426103);"#;

    let lex = lexer::Lexer::new(input);
    let program = hlcl::StmtParser::new().parse(input, &mut errs, lex);

    println!("{:?}", errs);
    println!("{:?}", program);
}
