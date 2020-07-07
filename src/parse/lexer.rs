use std::iter::Peekable;
use std::str::CharIndices;

use phf::phf_map;

use crate::ast::BinOpKind;
use crate::parse::err::ParserError;
use crate::span::Span;

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
};

pub struct Lexer<'input> {
    input: &'input str,
    chars: Peekable<CharIndices<'input>>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer {
            input,
            chars: input.char_indices().peekable(),
        }
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
                        Some(Ok((start, *w, end)))
                    } else {
                        Some(Ok((start, Token::Identifier(id), end)))
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
                            return Some(Err(ParserError::EofInString(Span {
                                l: start,
                                r: self.input.len(),
                            })));
                        }
                    }

                    return Some(Ok((
                        start,
                        Token::StringLiteral(&self.input[start + 1..end]),
                        end,
                    )));
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

                    return Some(Ok((
                        start,
                        Token::IntLiteral(&self.input[start..=end]),
                        end + 1,
                    )));
                }
                Some((i, '+')) => {
                    return match self.chars.peek() {
                        Some((_, '=')) => {
                            self.chars.next();
                            Some(Ok((i, Token::AssignOp(BinOpKind::Add), i + 2)))
                        }
                        Some((_, '+')) => {
                            self.chars.next();
                            Some(Ok((i, Token::PlusPlus, i + 2)))
                        }
                        _ => Some(Ok((i, Token::BinOp(BinOpKind::Add), i + 1))),
                    }
                }
                Some((i, '-')) => {
                    return match self.chars.peek() {
                        Some((_, '=')) => {
                            self.chars.next();
                            Some(Ok((i, Token::AssignOp(BinOpKind::Sub), i + 2)))
                        }
                        Some((_, '-')) => {
                            self.chars.next();
                            Some(Ok((i, Token::MinusMinus, i + 2)))
                        }
                        _ => Some(Ok((i, Token::BinOp(BinOpKind::Sub), i + 1))),
                    }
                }
                Some((i, '*')) => {
                    return match self.chars.peek() {
                        Some((_, '=')) => {
                            self.chars.next();
                            Some(Ok((i, Token::AssignOp(BinOpKind::Mul), i + 2)))
                        }
                        _ => Some(Ok((i, Token::BinOp(BinOpKind::Mul), i + 1))),
                    }
                }
                Some((i, '/')) => {
                    match self.chars.peek() {
                        Some((_, '/')) => {
                            loop {
                                if let Some((_, '\n')) = self.chars.next() {
                                    break
                                }
                            }
                        }
                        Some((_, '=')) => {
                            self.chars.next();
                            return Some(Ok((i, Token::AssignOp(BinOpKind::Div), i + 2)))
                        }
                        _ => return Some(Ok((i, Token::BinOp(BinOpKind::Div), i + 1))),
                    }
                }
                Some((i, '%')) => {
                    return match self.chars.peek() {
                        Some((_, '=')) => {
                            self.chars.next();
                            Some(Ok((i, Token::AssignOp(BinOpKind::Rem), i + 2)))
                        }
                        _ => Some(Ok((i, Token::BinOp(BinOpKind::Rem), i + 1))),
                    }
                }
                Some((i, '!')) => {
                    return match self.chars.peek() {
                        Some((_, '=')) => {
                            self.chars.next();
                            Some(Ok((i, Token::BinOp(BinOpKind::Ne), i + 2)))
                        }
                        _ => Some(Ok((i, Token::Exclamation, i + 1))),
                    }
                }
                Some((i, '&')) => {
                    return match self.chars.peek() {
                        Some((_, '=')) => {
                            self.chars.next();
                            Some(Ok((i, Token::AssignOp(BinOpKind::And), i + 2)))
                        }
                        _ => Some(Ok((i, Token::BinOp(BinOpKind::And), i + 1))),
                    }
                }
                Some((i, '|')) => {
                    return match self.chars.peek() {
                        Some((_, '=')) => {
                            self.chars.next();
                            Some(Ok((i, Token::AssignOp(BinOpKind::Or), i + 2)))
                        }
                        _ => Some(Ok((i, Token::BinOp(BinOpKind::Or), i + 1))),
                    }
                }
                Some((i, '=')) => {
                    return match self.chars.peek() {
                        Some((_, '=')) => {
                            self.chars.next();
                            Some(Ok((i, Token::BinOp(BinOpKind::Eq), i + 2)))
                        }
                        _ => Some(Ok((i, Token::Assign, i + 1))),
                    }
                }
                Some((i, '>')) => {
                    return match self.chars.peek() {
                        Some((_, '=')) => {
                            self.chars.next();
                            Some(Ok((i, Token::BinOp(BinOpKind::Ge), i + 2)))
                        }
                        _ => Some(Ok((i, Token::BinOp(BinOpKind::Gt), i + 1))),
                    }
                }
                Some((i, '<')) => {
                    return match self.chars.peek() {
                        Some((_, '=')) => {
                            self.chars.next();
                            Some(Ok((i, Token::BinOp(BinOpKind::Le), i + 2)))
                        }
                        _ => Some(Ok((i, Token::BinOp(BinOpKind::Lt), i + 1))),
                    }
                }
                Some((i, ':')) => {
                    return match self.chars.peek() {
                        Some((_, ':')) => {
                            self.chars.next();
                            Some(Ok((i, Token::PathSeg, i + 2)))
                        }
                        _ => Some(Ok((i, Token::Colon, i + 1))),
                    }
                }
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
                // Some((i, ch)) if ch.is_whitespace() => {
                //     loop {
                //         match self.chars.peek() {
                //             Some((_, ch)) if ch.is_whitespace() => {
                //                 self.chars.next();
                //             }
                //             Some((end, _)) => {
                //                 return Some(Ok((i, Token::WhiteSpace, *end)))
                //             }
                //             None => {
                //                 return Some(Ok((i, Token::WhiteSpace, self.input.len())))
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
                        Span { l: start, r: end },
                        self.input[start..end].to_owned(),
                    )));
                }
                None => return None, // End of file
            }
        }
    }
}
