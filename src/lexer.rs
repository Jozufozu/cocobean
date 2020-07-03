
use phf::phf_map;
use std::iter::Peekable;
use std::str::{CharIndices, FromStr};

pub type Spanned<Token, Loc, Error> = Result<(Loc, Token, Loc), Error>;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Token<'input> {
    Identifier(&'input str),
    StringLiteral(&'input str),
    IntLiteral(&'input str),
    DocComment(&'input str),

    True,
    False,

    Const,

    Do,
    While,
    Loop,

    If,
    Else,

    Is,

    String,
    Bool,
    Int,

    Pub,
    Fn,
    Let,
    Return,
    Break,
    Continue,

    Mod,
    Trait,
    Struct,
    Class,
    Branch,
    Enum,
    Builtin,
    Player,
    Entity,

    Colon,
    PathSeg,
    Dot,
    Caret,

    Semicolon,
    Comma,

    OpenBlock,
    CloseBlock,

    OpenBracket,
    CloseBracket,

    OpenParen,
    CloseParen,

    Hash,
    At,

    Plus,
    Minus,
    Mul,
    Div,
    Rem,

    And,
    Or,
    Exclamation,

    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,

    Assign,

    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    RemAssign,

    AndAssign,
    OrAssign,

    PlusPlus,
    MinusMinus,
}

static KEYWORDS: phf::Map<&'static str, Token> = phf_map! {
    "true" => Token::True,
    "false" => Token::False,
    "const" => Token::Const,
    "pub" => Token::Pub,
    "fn" => Token::Fn,
    "do" => Token::Do,
    "while" => Token::While,
    "loop" => Token::Loop,
    "if" => Token::If,
    "else" => Token::Else,
    "is" => Token::Is,
    "string" => Token::String,
    "bool" => Token::Bool,
    "int" => Token::Int,
    "let" => Token::Let,
    "return" => Token::Return,
    "break" => Token::Break,
    "continue" => Token::Continue,
    "trait" => Token::Trait,
    "struct" => Token::Struct,
    "class" => Token::Class,
    "branch" => Token::Branch,
    "enum" => Token::Enum,
    "builtin" => Token::Builtin,
    // "player" => Token::Player,
    // "entity" => Token::Entity,
    "mod" => Token::Mod,
};

pub struct Lexer<'input> {
    input: &'input str,
    chars: Peekable<CharIndices<'input>>,
    last_tokens: [Option<Token<'input>>; 2],
}

#[derive(Debug)]
pub enum ParserError {
    EofInString(usize, usize),
    UnrecognisedToken(usize, usize, String),
    IntTooBig(usize, usize)
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer {
            input,
            chars: input.char_indices().peekable(),
            last_tokens: [None, None],
        }
    }

    pub fn next(&mut self) -> Option<Spanned<Token<'input>, usize, ParserError>> {
        loop {
            match self.chars.next() {
                Some((start, ch)) if ch == '_' || ch.is_ascii_alphabetic() => {
                    let end;

                    loop {
                        if let Some((i, ch)) = self.chars.peek() {
                            if *ch != '_' && !ch.is_ascii_alphanumeric() {
                                end = *i;
                                break;
                            }
                            self.chars.next();
                        } else {
                            end = self.input.len();
                            break;
                        }
                    }

                    let id = &self.input[start..end];

                    return if let Some(w) = KEYWORDS.get(id) {
                        Some(Ok((start, *w, end)))
                    } else {
                        Some(Ok((start, Token::Identifier(id), end)))
                    };
                },
                Some((start, '"')) => {
                    let mut end;

                    let mut last_was_escape = false;

                    loop {
                        if let Some((i, ch)) = self.chars.next() {
                            end = i;
                            if !last_was_escape {
                                if ch == '"' {
                                    break;
                                }
                                last_was_escape = ch == '\\';
                            } else {
                                last_was_escape = false;
                            }
                        } else {
                            return Some(Err(ParserError::EofInString(
                                start,
                                self.input.len(),
                            )));
                        }
                    }

                    return Some(Ok((start, Token::StringLiteral(&self.input[start+1..end]), end)))
                },
                Some((start, ch)) if ch.is_ascii_digit() => {
                    let mut end = start;

                    while let Some((i, ch)) = self.chars.peek() {
                        if ch.is_ascii_digit() || *ch == '_' {
                            end = *i;
                            self.chars.next();
                        } else {
                            break;
                        }
                    }

                    return Some(Ok((start, Token::IntLiteral(&self.input[start..=end]), end + 1)))
                },
                Some((i, '+')) => return match self.chars.peek() {
                    Some((_, '=')) => {
                        self.chars.next();
                        Some(Ok((i, Token::AddAssign, i + 2)))
                    },
                    Some((_, '+')) => {
                        self.chars.next();
                        Some(Ok((i, Token::PlusPlus, i + 2)))
                    },
                    _ => Some(Ok((i, Token::Plus, i + 1)))
                },
                Some((i, '-')) => return match self.chars.peek() {
                    Some((_, '=')) => {
                        self.chars.next();
                        Some(Ok((i, Token::SubAssign, i + 2)))
                    },
                    Some((_, '-')) => {
                        self.chars.next();
                        Some(Ok((i, Token::MinusMinus, i + 2)))
                    },
                    _ => Some(Ok((i, Token::Minus, i + 1)))
                },
                Some((i, '*')) => return match self.chars.peek() {
                    Some((_, '=')) => {
                        self.chars.next();
                        Some(Ok((i, Token::MulAssign, i + 2)))
                    },
                    _ => Some(Ok((i, Token::Mul, i + 1)))
                },
                Some((i, '/')) => return match self.chars.peek() {
                    Some((_, '=')) => {
                        self.chars.next();
                        Some(Ok((i, Token::DivAssign, i + 2)))
                    },
                    _ => Some(Ok((i, Token::Div, i + 1)))
                },
                Some((i, '%')) => return match self.chars.peek() {
                    Some((_, '=')) => {
                        self.chars.next();
                        Some(Ok((i, Token::RemAssign, i + 2)))
                    },
                    _ => Some(Ok((i, Token::Rem, i + 1)))
                },
                Some((i, '!')) => return match self.chars.peek() {
                    Some((_, '=')) => {
                        self.chars.next();
                        Some(Ok((i, Token::Ne, i + 2)))
                    },
                    _ => Some(Ok((i, Token::Exclamation, i + 1)))
                },
                Some((i, '&')) => return match self.chars.peek() {
                    Some((_, '=')) => {
                        self.chars.next();
                        Some(Ok((i, Token::AndAssign, i + 2)))
                    },
                    _ => Some(Ok((i, Token::And, i + 1)))
                },
                Some((i, '|')) => return match self.chars.peek() {
                    Some((_, '=')) => {
                        self.chars.next();
                        Some(Ok((i, Token::OrAssign, i + 2)))
                    },
                    _ => Some(Ok((i, Token::Or, i + 1)))
                },
                Some((i, '=')) => return match self.chars.peek() {
                    Some((_, '=')) => {
                        self.chars.next();
                        Some(Ok((i, Token::Eq, i + 2)))
                    },
                    _ => Some(Ok((i, Token::Assign, i + 1)))
                },
                Some((i, '>')) => return match self.chars.peek() {
                    Some((_, '=')) => {
                        self.chars.next();
                        Some(Ok((i, Token::Ge, i + 2)))
                    },
                    _ => Some(Ok((i, Token::Gt, i + 1)))
                },
                Some((i, '<')) => return match self.chars.peek() {
                    Some((_, '=')) => {
                        self.chars.next();
                        Some(Ok((i, Token::Le, i + 2)))
                    },
                    _ => Some(Ok((i, Token::Lt, i + 1)))
                },
                Some((i, ':')) => return match self.chars.peek() {
                    Some((_, ':')) => {
                        self.chars.next();
                        Some(Ok((i, Token::PathSeg, i + 2)))
                    },
                    _ => Some(Ok((i, Token::Colon, i + 1)))
                },
                Some((i, '.')) => return Some(Ok((i, Token::Dot, i + 1))),
                Some((i, '^')) => return Some(Ok((i, Token::Caret, i + 1))),
                Some((i, ';')) => return Some(Ok((i, Token::Semicolon, i + 1))),
                Some((i, ',')) => return Some(Ok((i, Token::Comma, i + 1))),
                Some((i, '(')) => return Some(Ok((i, Token::OpenParen, i + 1))),
                Some((i, ')')) => return Some(Ok((i, Token::CloseParen, i + 1))),
                Some((i, '{')) => return Some(Ok((i, Token::OpenBlock, i + 1))),
                Some((i, '}')) => return Some(Ok((i, Token::CloseBlock, i + 1))),
                Some((i, '[')) => return Some(Ok((i, Token::OpenBracket, i + 1))),
                Some((i, ']')) => return Some(Ok((i, Token::CloseBracket, i + 1))),
                Some((i, '#')) => return Some(Ok((i, Token::Hash, i + 1))),
                Some((i, '@')) => return Some(Ok((i, Token::At, i + 1))),
                Some((_, ch)) if ch.is_whitespace() => (),
                Some((start, _)) => {
                    let mut end;

                    loop {
                        if let Some((i, ch)) = self.chars.next() {
                            end = i;

                            if ch.is_whitespace() {
                                break;
                            }
                        } else {
                            end = self.input.len();
                            break;
                        }
                    }

                    return Some(Err(ParserError::UnrecognisedToken(
                        start,
                        end,
                        self.input[start..end].to_owned(),
                    )));
                }
                None => return None, // End of file
            }
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Token<'input>, usize, ParserError>;

    /// Return the next token
    fn next(&mut self) -> Option<Self::Item> {
        let token = self.next();

        self.last_tokens = [
            self.last_tokens[1],
            match token {
                Some(Ok((_, n, _))) => Some(n),
                _ => None,
            },
        ];

        token
    }
}