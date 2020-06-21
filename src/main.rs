#[allow(clippy::all)]
#[cfg_attr(rustfmt, rustfmt_skip)]
mod hlcl;
mod ast;
mod lexer;

fn main() {
    let mut errs = Vec::new();
    let input =
r#"builtin struct player: entity {
    wow: int,
    amazing: bool
}
"#;

    let lex = lexer::Lexer::new(input);
    let program = hlcl::ProgramParser::new().parse(input, &mut errs, lex);

    println!("{:?}", program);

    let input =
        r#"true + false * (2000, heh, "yeet", FUCK SHIT DAMMIT) + 4 % 20"#;

    let lex = lexer::Lexer::new(input);
    let program = hlcl::ExprParser::new().parse(input, &mut errs, lex);

    println!("{:?}", errs);
    println!("{}", program.unwrap());
}
