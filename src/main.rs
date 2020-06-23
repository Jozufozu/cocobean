use string_interner::{Sym, StringInterner};

#[allow(clippy::all)]
#[cfg_attr(rustfmt, rustfmt_skip)]
mod hlcl;
mod ast;
mod lexer;

fn main() {
    let mut errs = Vec::new();

    let mut interner = StringInterner::default();

    let input =
r#"builtin struct player: entity {
    wow: int,
    amazing: bool
}
"#;

    let lex = lexer::Lexer::new(input);
    let program = hlcl::ProgramParser::new().parse(input, &mut interner, &mut errs, lex);

    println!("{:?}", program);

    let input = r#"namehage += 20000000 * false"#;

    let lex = lexer::Lexer::new(input);
    let program = hlcl::ExprParser::new().parse(input, &mut interner, &mut errs, lex);

    println!("{:?}", errs);
    println!("{}", program.unwrap());

    println!("{:?}", interner)
}
