use std::fmt;
use std::iter::Peekable;
use std::str::CharIndices;

use phf::phf_map;

use coco_ast::BinOpKind;
use coco_span::Span;

use crate::err::ParserError;

pub type Spanned<Token, Loc, Error> = Result<(Loc, Token, Loc), Error>;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Token<'input> {
    Identifier(&'input str),
    StringLiteral(&'input str),
    IntLiteral(&'input str),
    DocComment(&'input str),
    WhiteSpace,

    True,
    False,

    Const,

    Do,
    While,
    Loop,

    If,
    Else,

    Is,

    Pub,
    Fn,
    Let,
    Return,
    Break,
    Continue,

    Use,
    As,
    Mod,
    Trait,
    Struct,
    Class,
    Branch,
    Enum,
    Builtin,

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

    Exclamation,

    Assign,
    BinOp(BinOpKind),
    AssignOp(BinOpKind),

    PlusPlus,
    MinusMinus,
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Identifier(_) => write!(f, "Identifier"),
            Token::StringLiteral(_) => write!(f, "StringLiteral"),
            Token::IntLiteral(_) => write!(f, "IntLiteral"),
            Token::DocComment(_) => write!(f, "DocComment"),
            Token::WhiteSpace => write!(f, "WhiteSpace"),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            Token::Const => write!(f, "const"),
            Token::Do => write!(f, "do"),
            Token::While => write!(f, "while"),
            Token::Loop => write!(f, "loop"),
            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
            Token::Is => write!(f, "is"),
            Token::Pub => write!(f, "pub"),
            Token::Fn => write!(f, "fn"),
            Token::Let => write!(f, "let"),
            Token::Return => write!(f, "return"),
            Token::Break => write!(f, "break"),
            Token::Continue => write!(f, "continue"),
            Token::Use => write!(f, "use"),
            Token::As => write!(f, "as"),
            Token::Mod => write!(f, "mod"),
            Token::Trait => write!(f, "trait"),
            Token::Struct => write!(f, "struct"),
            Token::Class => write!(f, "class"),
            Token::Branch => write!(f, "branch"),
            Token::Enum => write!(f, "enum"),
            Token::Builtin => write!(f, "builtin"),
            Token::Colon => write!(f, "Colon"),
            Token::PathSeg => write!(f, "PathSeg"),
            Token::Dot => write!(f, "Dot"),
            Token::Caret => write!(f, "Caret"),
            Token::Semicolon => write!(f, "Semicolon"),
            Token::Comma => write!(f, "Comma"),
            Token::OpenBlock => write!(f, "OpenBlock"),
            Token::CloseBlock => write!(f, "CloseBlock"),
            Token::OpenBracket => write!(f, "OpenBracket"),
            Token::CloseBracket => write!(f, "CloseBracket"),
            Token::OpenParen => write!(f, "OpenParen"),
            Token::CloseParen => write!(f, "CloseParen"),
            Token::Hash => write!(f, "Hash"),
            Token::At => write!(f, "At"),
            Token::Exclamation => write!(f, "Exclamation"),
            Token::Assign => write!(f, "Assign"),
            Token::BinOp(_) => write!(f, "BinOp"),
            Token::AssignOp(_) => write!(f, "AssignOp"),
            Token::PlusPlus => write!(f, "PlusPlus"),
            Token::MinusMinus => write!(f, "MinusMinus"),
        }
    }
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
    "mod" => Token::Mod,
    "use" => Token::Use,
    "as" => Token::As,
};

pub struct Lexer<'input> {
    input: &'input str,
    chars: Peekable<CharIndices<'input>>,
    start_pos: usize,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str, start_pos: usize) -> Self {
        Lexer {
            input,
            chars: input.char_indices().peekable(),
            start_pos,
        }
    }

    #[inline]
    fn token<'a>(
        &self,
        l: usize,
        r: usize,
        tok: Token<'a>,
    ) -> Option<Spanned<Token<'a>, usize, ParserError>> {
        let l = self.start_pos + l;
        let r = self.start_pos + r;
        Some(Ok((l, tok, r)))
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Token<'input>, usize, ParserError>;

    /// Return the next token
    fn next(&mut self) -> Option<Self::Item> {
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
                        self.token(start, end, *w)
                    } else {
                        self.token(start, end, Token::Identifier(id))
                    };
                }
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
                            return Some(Err(ParserError::EofInString(Span::new(
                                start,
                                self.input.len(),
                            ))));
                        }
                    }

                    return self.token(
                        start,
                        end,
                        Token::StringLiteral(&self.input[start + 1..end]),
                    );
                }
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

                    return self.token(start, end + 1, Token::IntLiteral(&self.input[start..=end]));
                }
                Some((i, '+')) => {
                    return match self.chars.peek() {
                        Some((_, '=')) => {
                            self.chars.next();
                            self.token(i, i + 2, Token::AssignOp(BinOpKind::Add))
                        }
                        Some((_, '+')) => {
                            self.chars.next();
                            self.token(i, i + 2, Token::PlusPlus)
                        }
                        _ => self.token(i, i + 1, Token::BinOp(BinOpKind::Add)),
                    };
                }
                Some((i, '-')) => {
                    return match self.chars.peek() {
                        Some((_, '=')) => {
                            self.chars.next();
                            self.token(i, i + 2, Token::AssignOp(BinOpKind::Sub))
                        }
                        Some((_, '-')) => {
                            self.chars.next();
                            self.token(i, i + 2, Token::MinusMinus)
                        }
                        _ => self.token(i, i + 1, Token::BinOp(BinOpKind::Sub)),
                    };
                }
                Some((i, '*')) => {
                    return match self.chars.peek() {
                        Some((_, '=')) => {
                            self.chars.next();
                            self.token(i, i + 2, Token::AssignOp(BinOpKind::Mul))
                        }
                        _ => self.token(i, i + 1, Token::BinOp(BinOpKind::Mul)),
                    };
                }
                Some((i, '/')) => match self.chars.peek() {
                    Some((_, '/')) => loop {
                        if let Some((_, '\n')) = self.chars.next() {
                            break;
                        }
                    },
                    Some((_, '=')) => {
                        self.chars.next();
                        return self.token(i, i + 2, Token::AssignOp(BinOpKind::Div));
                    }
                    _ => return self.token(i, i + 1, Token::BinOp(BinOpKind::Div)),
                },
                Some((i, '%')) => {
                    return match self.chars.peek() {
                        Some((_, '=')) => {
                            self.chars.next();
                            self.token(i, i + 2, Token::AssignOp(BinOpKind::Rem))
                        }
                        _ => self.token(i, i + 1, Token::BinOp(BinOpKind::Rem)),
                    };
                }
                Some((i, '!')) => {
                    return match self.chars.peek() {
                        Some((_, '=')) => {
                            self.chars.next();
                            self.token(i, i + 2, Token::BinOp(BinOpKind::Ne))
                        }
                        _ => self.token(i, i + 1, Token::Exclamation),
                    };
                }
                Some((i, '&')) => {
                    return match self.chars.peek() {
                        Some((_, '=')) => {
                            self.chars.next();
                            self.token(i, i + 2, Token::AssignOp(BinOpKind::And))
                        }
                        _ => self.token(i, i + 1, Token::BinOp(BinOpKind::And)),
                    };
                }
                Some((i, '|')) => {
                    return match self.chars.peek() {
                        Some((_, '=')) => {
                            self.chars.next();
                            self.token(i, i + 2, Token::AssignOp(BinOpKind::Or))
                        }
                        _ => self.token(i, i + 1, Token::BinOp(BinOpKind::Or)),
                    };
                }
                Some((i, '=')) => {
                    return match self.chars.peek() {
                        Some((_, '=')) => {
                            self.chars.next();
                            self.token(i, i + 2, Token::BinOp(BinOpKind::Eq))
                        }
                        _ => self.token(i, i + 1, Token::Assign),
                    };
                }
                Some((i, '>')) => {
                    return match self.chars.peek() {
                        Some((_, '=')) => {
                            self.chars.next();
                            self.token(i, i + 2, Token::BinOp(BinOpKind::Ge))
                        }
                        _ => self.token(i, i + 1, Token::BinOp(BinOpKind::Gt)),
                    };
                }
                Some((i, '<')) => {
                    return match self.chars.peek() {
                        Some((_, '=')) => {
                            self.chars.next();
                            self.token(i, i + 2, Token::BinOp(BinOpKind::Le))
                        }
                        _ => self.token(i, i + 1, Token::BinOp(BinOpKind::Lt)),
                    };
                }
                Some((i, ':')) => {
                    return match self.chars.peek() {
                        Some((_, ':')) => {
                            self.chars.next();
                            self.token(i, i + 2, Token::PathSeg)
                        }
                        _ => self.token(i, i + 1, Token::Colon),
                    };
                }
                Some((i, '.')) => return self.token(i, i + 1, Token::Dot),
                Some((i, '^')) => return self.token(i, i + 1, Token::Caret),
                Some((i, ';')) => return self.token(i, i + 1, Token::Semicolon),
                Some((i, ',')) => return self.token(i, i + 1, Token::Comma),
                Some((i, '(')) => return self.token(i, i + 1, Token::OpenParen),
                Some((i, ')')) => return self.token(i, i + 1, Token::CloseParen),
                Some((i, '{')) => return self.token(i, i + 1, Token::OpenBlock),
                Some((i, '}')) => return self.token(i, i + 1, Token::CloseBlock),
                Some((i, '[')) => return self.token(i, i + 1, Token::OpenBracket),
                Some((i, ']')) => return self.token(i, i + 1, Token::CloseBracket),
                Some((i, '#')) => return self.token(i, i + 1, Token::Hash),
                Some((i, '@')) => return self.token(i, i + 1, Token::At),
                Some((_, ch)) if ch.is_whitespace() => (),
                // Some((i, ch)) if ch.is_whitespace() => {
                //     loop {
                //         match self.chars.peek() {
                //             Some((_, ch)) if ch.is_whitespace() => {
                //                 self.chars.next();
                //             }
                //             Some((end, _)) => {
                //                 return self.token(i, Token::WhiteSpace, *end)
                //             }
                //             None => {
                //                 return self.token(i, Token::WhiteSpace, self.input.len())
                //             }
                //         }
                //     }
                // }
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
                        Span::new(start, end),
                        self.input[start..end].to_owned(),
                    )));
                }
                None => return None, // End of file
            }
        }
    }
}
