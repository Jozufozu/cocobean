extern crate static_assertions as sa;

use string_interner::{StringInterner, Sym};

#[allow(clippy::all)]
#[cfg_attr(rustfmt, rustfmt_skip)]
mod hlcl;
mod ast;
mod lexer;

fn main() {
    let mut errs = Vec::new();

    let mut interner = StringInterner::default();

    let input =
        r#"
pub branch GameMode: player {
    InGame {
        pub kills: int,
        pub cooldown: int,
        thing = true,
        pub health: int,
    },
    Spectator {

    },
}

pub branch Class: GameMode::InGame {
    Scout {

    },
    Heavy {

    }
}

fn test(thing: int, other: player) {

    if other is GameMode::Spectator { return; }

    let other = GameMode::InGame(other);

    while { thing += 2; thing != 0 } {
        loop {
            if other@player.health >= 0 {
                other@InGame.health = 2;
            } else if other.cooldown == 2 {
                other@InGame.health = thing;
            } else if true {
            }
        }
    }
}
"#;

    let lex = lexer::Lexer::new(input);
    let program = hlcl::ProgramParser::new().parse(input, &mut interner, &mut errs, lex);

    println!("{:?}", program);
    println!("{:?}", errs);
    for (_, name) in interner {
        print!("{:}, ", name)
    }
}
