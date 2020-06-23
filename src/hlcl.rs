// auto-generated: "lalrpop 0.19.0"
// sha256: 31496f22ceb73dbe689ad0d948b492545ff868f0899bc0ccc9dd67c2a7b55b37
use std::str::FromStr;
use string_interner::{Sym, StringInterner};
use lalrpop_util::{ParseError, ErrorRecovery};
use crate::lexer::{Token, ParserError};
use crate::ast::{*, BinOpKind as BinOp};
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Expr {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use std::str::FromStr;
    use string_interner::{Sym, StringInterner};
    use lalrpop_util::{ParseError, ErrorRecovery};
    use crate::lexer::{Token, ParserError};
    use crate::ast::{*, BinOpKind as BinOp};
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Variant0(Token<'input>),
        Variant1(&'input str),
        Variant2(__lalrpop_util::ErrorRecovery<usize, Token<'input>, ParserError>),
        Variant3(::std::option::Option<Token<'input>>),
        Variant4(ComplexType),
        Variant5(::std::vec::Vec<ComplexType>),
        Variant6(Expr),
        Variant7(::std::vec::Vec<Expr>),
        Variant8(Type),
        Variant9(::std::option::Option<Type>),
        Variant10(Identifier),
        Variant11(::std::vec::Vec<Identifier>),
        Variant12((usize, usize)),
        Variant13(::std::option::Option<(usize, usize)>),
        Variant14(MemberVariable),
        Variant15(::std::vec::Vec<MemberVariable>),
        Variant16(usize),
        Variant17(Vec<MemberVariable>),
        Variant18(LitKind),
        Variant19(Vec<ComplexType>),
        Variant20(Vec<Expr>),
        Variant21(::std::option::Option<MemberVariable>),
        Variant22(UnOpKind),
        Variant23(BinOp),
        Variant24(Path),
        Variant25(Vec<ProgramPart>),
        Variant26(ProgramPart),
        Variant27(::std::vec::Vec<ProgramPart>),
        Variant28(Spanned<UnOpKind>),
        Variant29(Spanned<BinOp>),
        Variant30(Stmt),
        Variant31(StructDefinition),
    }
    const __ACTION: &[i16] = &[
        // State 0
        35, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 0, 0, 0, 39, 40, 41, 42,
        // State 1
        0, -62, 0, 44, -62, -62, -62, 0, -62, 45, -62, -62, -62, -62, -62, -62, 0, 0, 46, -62, 0, 0, 0, -62, -62, -62, -62, -62, -62, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -62, -62, 0, 0, 0, 0, 0,
        // State 2
        0, -63, 0, 0, -63, -63, -63, 0, -63, 0, -63, 48, -63, -63, 49, -63, 0, 0, 0, -63, 0, 0, 0, -63, -63, -63, -63, -63, -63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -63, -63, 0, 0, 0, 0, 0,
        // State 3
        0, -64, 0, 0, -64, -64, -64, 0, -64, 0, -64, 0, -64, -64, 0, -64, 0, 0, 0, -64, 0, 0, 0, 51, 52, -64, -64, 53, 54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -64, -64, 0, 0, 0, 0, 0,
        // State 4
        0, 56, 0, 0, -65, -65, -65, 0, -65, 0, -65, 0, -65, -65, 0, -65, 0, 0, 0, -65, 0, 0, 0, 0, 0, -65, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -65, -65, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, -66, 59, -66, 0, -66, 0, -66, 0, -66, -66, 0, -66, 0, 0, 0, -66, 0, 0, 0, 0, 0, -66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -66, -66, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, -67, 0, -67, 0, -67, 0, -67, 0, -67, -67, 0, -67, 0, 0, 0, -67, 0, 0, 0, 0, 0, -67, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 61, -67, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 63, 0, 64, 0, -60, 0, 65, 0, 66, -60, 0, 67, 0, 0, 0, 68, 0, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 69, 0, 0, 0, 0, 0,
        // State 8
        35, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 0, 0, 0, 39, 40, 41, 42,
        // State 9
        35, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 0, 0, 0, 39, 40, 41, 42,
        // State 10
        35, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 0, 0, 0, 39, 40, 41, 42,
        // State 11
        35, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 0, 0, 0, 39, 40, 41, 42,
        // State 12
        35, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 0, 0, 0, 39, 40, 41, 42,
        // State 13
        35, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 0, 0, 0, 39, 40, 41, 42,
        // State 14
        35, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 0, 0, 0, 39, 40, 41, 42,
        // State 15
        35, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 0, 0, 0, 39, 40, 41, 42,
        // State 16
        35, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 0, 0, 0, 39, 40, 41, 42,
        // State 17
        35, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 0, 0, 0, 39, 40, 41, 42,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, -75, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        35, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 0, 0, 0, 39, 40, 41, 42,
        // State 20
        35, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 0, 0, 0, 39, 40, 41, 42,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, -130, 0, -130, -130, -130, -130, 0, -130, -130, -130, -130, -130, -130, -130, -130, 0, 0, -130, -130, 0, 0, 0, -130, -130, -130, -130, -130, -130, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -130, -130, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, -53, 0, 0, 0, 0, -53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, -32, 0, -32, -32, -32, -32, 0, -32, -32, -32, -32, -32, -32, -32, -32, 0, 0, -32, -32, 0, 0, 0, -32, -32, -32, -32, -32, -32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -32, -32, 0, 0, 0, 0, 0,
        // State 25
        0, -34, 0, 0, -34, -34, -34, 0, -34, 0, -34, -34, -34, -34, -34, -34, 0, 0, 0, -34, 0, 0, 0, -34, -34, -34, -34, -34, -34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -34, -34, 0, 0, 0, 0, 0,
        // State 26
        0, -36, 0, 0, -36, -36, -36, 0, -36, 0, -36, 0, -36, -36, 0, -36, 0, 0, 0, -36, 0, 0, 0, -36, -36, -36, -36, -36, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -36, -36, 0, 0, 0, 0, 0,
        // State 27
        0, -38, 0, 0, -38, -38, -38, 0, -38, 0, -38, 0, -38, -38, 0, -38, 0, 0, 0, -38, 0, 0, 0, 0, 0, -38, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -38, -38, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, -40, -40, -40, 0, -40, 0, -40, 0, -40, -40, 0, -40, 0, 0, 0, -40, 0, 0, 0, 0, 0, -40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -40, -40, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, -42, 0, -42, 0, -42, 0, -42, 0, -42, -42, 0, -42, 0, 0, 0, -42, 0, 0, 0, 0, 0, -42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -42, -42, 0, 0, 0, 0, 0,
        // State 30
        0, -55, 0, -55, -55, -55, -55, 0, -55, -55, -55, -55, -55, -55, -55, -55, 0, 0, -55, -55, 0, 0, 0, -55, -55, -55, -55, -55, -55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -55, -55, 0, 0, 0, 0, 0,
        // State 31
        0, -54, 0, -54, -54, -54, -54, 0, -54, -54, -54, -54, -54, -54, -54, -54, 0, 0, -54, -54, 0, 0, 0, -54, -54, -54, -54, -54, -54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -54, -54, 0, 0, 0, 0, 0,
        // State 32
        -110, 0, 0, 0, 0, 0, 0, -110, 0, 0, 0, 0, 0, 0, -110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -110, 0, 0, 0, 0, 0, 0, 0, 0, 0, -110, 0, 0, 0, 0, 0, -110, -110, -110, -110,
        // State 33
        0, -61, 0, -61, -61, -61, -61, 0, -61, -61, -61, -61, -61, -61, -61, -61, 0, 0, -61, -61, 0, 0, 0, -61, -61, -61, -61, -61, -61, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -61, -61, 0, 0, 0, 0, 0,
        // State 34
        -81, 0, 0, 0, 0, 0, 0, -81, 0, 0, 0, 0, 0, 0, -81, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -81, 0, 0, 0, 0, 0, 0, 0, 0, 0, -81, 0, 0, 0, 0, 0, -81, -81, -81, -81,
        // State 35
        -80, 0, 0, 0, 0, 0, 0, -80, 0, 0, 0, 0, 0, 0, -80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -80, 0, 0, 0, 0, 0, 0, 0, 0, 0, -80, 0, 0, 0, 0, 0, -80, -80, -80, -80,
        // State 36
        0, -70, 0, -70, -70, -70, -70, 0, -70, -70, -70, -70, -70, -70, -70, -70, 0, 0, -70, -70, 0, 0, 0, -70, -70, -70, -70, -70, -70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -70, -70, 0, 0, 0, 0, 0,
        // State 37
        0, -69, 0, -69, -69, -69, -69, 0, -69, -69, -69, -69, -69, -69, -69, -69, 0, 0, -69, -69, 0, 0, 0, -69, -69, -69, -69, -69, -69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -69, -69, 0, 0, 0, 0, 0,
        // State 38
        0, -68, 0, -68, -68, -68, -68, 0, -68, -68, -68, -68, -68, -68, -68, -68, 0, 0, -68, -68, 0, 0, 0, -68, -68, -68, -68, -68, -68, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -68, -68, 0, 0, 0, 0, 0,
        // State 39
        0, -72, 0, -72, -72, -72, -72, 0, -72, -72, -72, -72, -72, -72, -72, -72, 0, 0, -72, -72, 0, 0, 0, -72, -72, -72, -72, -72, -72, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -72, -72, 0, 0, 0, 0, 0,
        // State 40
        0, -71, 0, -71, -71, -71, -71, 0, -71, -71, -71, -71, -71, -71, -71, -71, 0, 0, -71, -71, 0, 0, 0, -71, -71, -71, -71, -71, -71, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -71, -71, 0, 0, 0, 0, 0,
        // State 41
        0, -57, 0, -57, -57, -57, -57, 0, -57, -57, -57, -57, -57, -57, -57, -57, 0, 0, -57, -57, 0, 0, 0, -57, -57, -57, -57, -57, -57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -57, -57, 0, 0, 0, 0, 0,
        // State 42
        -111, 0, 0, 0, 0, 0, 0, -111, 0, 0, 0, 0, 0, 0, -111, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -111, 0, 0, 0, 0, 0, 0, 0, 0, 0, -111, 0, 0, 0, 0, 0, -111, -111, -111, -111,
        // State 43
        -84, 0, 0, 0, 0, 0, 0, -84, 0, 0, 0, 0, 0, 0, -84, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -84, 0, 0, 0, 0, 0, 0, 0, 0, 0, -84, 0, 0, 0, 0, 0, -84, -84, -84, -84,
        // State 44
        -82, 0, 0, 0, 0, 0, 0, -82, 0, 0, 0, 0, 0, 0, -82, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -82, 0, 0, 0, 0, 0, 0, 0, 0, 0, -82, 0, 0, 0, 0, 0, -82, -82, -82, -82,
        // State 45
        -83, 0, 0, 0, 0, 0, 0, -83, 0, 0, 0, 0, 0, 0, -83, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -83, 0, 0, 0, 0, 0, 0, 0, 0, 0, -83, 0, 0, 0, 0, 0, -83, -83, -83, -83,
        // State 46
        -112, 0, 0, 0, 0, 0, 0, -112, 0, 0, 0, 0, 0, 0, -112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -112, 0, 0, 0, 0, 0, 0, 0, 0, 0, -112, 0, 0, 0, 0, 0, -112, -112, -112, -112,
        // State 47
        -85, 0, 0, 0, 0, 0, 0, -85, 0, 0, 0, 0, 0, 0, -85, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -85, 0, 0, 0, 0, 0, 0, 0, 0, 0, -85, 0, 0, 0, 0, 0, -85, -85, -85, -85,
        // State 48
        -86, 0, 0, 0, 0, 0, 0, -86, 0, 0, 0, 0, 0, 0, -86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -86, 0, 0, 0, 0, 0, 0, 0, 0, 0, -86, 0, 0, 0, 0, 0, -86, -86, -86, -86,
        // State 49
        -113, 0, 0, 0, 0, 0, 0, -113, 0, 0, 0, 0, 0, 0, -113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -113, 0, 0, 0, 0, 0, 0, 0, 0, 0, -113, 0, 0, 0, 0, 0, -113, -113, -113, -113,
        // State 50
        -87, 0, 0, 0, 0, 0, 0, -87, 0, 0, 0, 0, 0, 0, -87, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -87, 0, 0, 0, 0, 0, 0, 0, 0, 0, -87, 0, 0, 0, 0, 0, -87, -87, -87, -87,
        // State 51
        -89, 0, 0, 0, 0, 0, 0, -89, 0, 0, 0, 0, 0, 0, -89, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -89, 0, 0, 0, 0, 0, 0, 0, 0, 0, -89, 0, 0, 0, 0, 0, -89, -89, -89, -89,
        // State 52
        -88, 0, 0, 0, 0, 0, 0, -88, 0, 0, 0, 0, 0, 0, -88, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -88, 0, 0, 0, 0, 0, 0, 0, 0, 0, -88, 0, 0, 0, 0, 0, -88, -88, -88, -88,
        // State 53
        -90, 0, 0, 0, 0, 0, 0, -90, 0, 0, 0, 0, 0, 0, -90, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -90, 0, 0, 0, 0, 0, 0, 0, 0, 0, -90, 0, 0, 0, 0, 0, -90, -90, -90, -90,
        // State 54
        -114, 0, 0, 0, 0, 0, 0, -114, 0, 0, 0, 0, 0, 0, -114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -114, 0, 0, 0, 0, 0, 0, 0, 0, 0, -114, 0, 0, 0, 0, 0, -114, -114, -114, -114,
        // State 55
        -92, 0, 0, 0, 0, 0, 0, -92, 0, 0, 0, 0, 0, 0, -92, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -92, 0, 0, 0, 0, 0, 0, 0, 0, 0, -92, 0, 0, 0, 0, 0, -92, -92, -92, -92,
        // State 56
        -91, 0, 0, 0, 0, 0, 0, -91, 0, 0, 0, 0, 0, 0, -91, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -91, 0, 0, 0, 0, 0, 0, 0, 0, 0, -91, 0, 0, 0, 0, 0, -91, -91, -91, -91,
        // State 57
        -115, 0, 0, 0, 0, 0, 0, -115, 0, 0, 0, 0, 0, 0, -115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -115, 0, 0, 0, 0, 0, 0, 0, 0, 0, -115, 0, 0, 0, 0, 0, -115, -115, -115, -115,
        // State 58
        -93, 0, 0, 0, 0, 0, 0, -93, 0, 0, 0, 0, 0, 0, -93, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -93, 0, 0, 0, 0, 0, 0, 0, 0, 0, -93, 0, 0, 0, 0, 0, -93, -93, -93, -93,
        // State 59
        -117, 0, 0, 0, 0, 0, 0, -117, 0, 0, 0, 0, 0, 0, -117, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -117, 0, 0, 0, 0, 0, 0, 0, 0, 0, -117, 0, 0, 0, 0, 0, -117, -117, -117, -117,
        // State 60
        -101, 0, 0, 0, 0, 0, 0, -101, 0, 0, 0, 0, 0, 0, -101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -101, 0, 0, 0, 0, 0, 0, 0, 0, 0, -101, 0, 0, 0, 0, 0, -101, -101, -101, -101,
        // State 61
        -116, 0, 0, 0, 0, 0, 0, -116, 0, 0, 0, 0, 0, 0, -116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -116, 0, 0, 0, 0, 0, 0, 0, 0, 0, -116, 0, 0, 0, 0, 0, -116, -116, -116, -116,
        // State 62
        -98, 0, 0, 0, 0, 0, 0, -98, 0, 0, 0, 0, 0, 0, -98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -98, 0, 0, 0, 0, 0, 0, 0, 0, 0, -98, 0, 0, 0, 0, 0, -98, -98, -98, -98,
        // State 63
        -99, 0, 0, 0, 0, 0, 0, -99, 0, 0, 0, 0, 0, 0, -99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -99, 0, 0, 0, 0, 0, 0, 0, 0, 0, -99, 0, 0, 0, 0, 0, -99, -99, -99, -99,
        // State 64
        -96, 0, 0, 0, 0, 0, 0, -96, 0, 0, 0, 0, 0, 0, -96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -96, 0, 0, 0, 0, 0, 0, 0, 0, 0, -96, 0, 0, 0, 0, 0, -96, -96, -96, -96,
        // State 65
        -94, 0, 0, 0, 0, 0, 0, -94, 0, 0, 0, 0, 0, 0, -94, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -94, 0, 0, 0, 0, 0, 0, 0, 0, 0, -94, 0, 0, 0, 0, 0, -94, -94, -94, -94,
        // State 66
        -95, 0, 0, 0, 0, 0, 0, -95, 0, 0, 0, 0, 0, 0, -95, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -95, 0, 0, 0, 0, 0, 0, 0, 0, 0, -95, 0, 0, 0, 0, 0, -95, -95, -95, -95,
        // State 67
        -97, 0, 0, 0, 0, 0, 0, -97, 0, 0, 0, 0, 0, 0, -97, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -97, 0, 0, 0, 0, 0, 0, 0, 0, 0, -97, 0, 0, 0, 0, 0, -97, -97, -97, -97,
        // State 68
        -100, 0, 0, 0, 0, 0, 0, -100, 0, 0, 0, 0, 0, 0, -100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -100, 0, 0, 0, 0, 0, 0, 0, 0, 0, -100, 0, 0, 0, 0, 0, -100, -100, -100, -100,
        // State 69
        0, -129, 0, -129, -129, -129, -129, 0, -129, -129, -129, -129, -129, -129, -129, -129, 0, 0, -129, -129, 0, 0, 0, -129, -129, -129, -129, -129, -129, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -129, -129, 0, 0, 0, 0, 0,
        // State 70
        0, 0, 0, 0, 0, 0, 0, 0, 81, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 71
        0, -31, 0, -31, -31, -31, -31, 0, -31, -31, -31, -31, -31, -31, -31, -31, 0, 0, -31, -31, 0, 0, 0, -31, -31, -31, -31, -31, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -31, -31, 0, 0, 0, 0, 0,
        // State 72
        0, -33, 0, 0, -33, -33, -33, 0, -33, 0, -33, -33, -33, -33, -33, -33, 0, 0, 0, -33, 0, 0, 0, -33, -33, -33, -33, -33, -33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -33, -33, 0, 0, 0, 0, 0,
        // State 73
        0, -35, 0, 0, -35, -35, -35, 0, -35, 0, -35, 0, -35, -35, 0, -35, 0, 0, 0, -35, 0, 0, 0, -35, -35, -35, -35, -35, -35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -35, -35, 0, 0, 0, 0, 0,
        // State 74
        0, -37, 0, 0, -37, -37, -37, 0, -37, 0, -37, 0, -37, -37, 0, -37, 0, 0, 0, -37, 0, 0, 0, 0, 0, -37, -37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -37, -37, 0, 0, 0, 0, 0,
        // State 75
        0, 0, 0, 0, -39, -39, -39, 0, -39, 0, -39, 0, -39, -39, 0, -39, 0, 0, 0, -39, 0, 0, 0, 0, 0, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -39, -39, 0, 0, 0, 0, 0,
        // State 76
        0, 0, 0, 0, -41, 0, -41, 0, -41, 0, -41, 0, -41, -41, 0, -41, 0, 0, 0, -41, 0, 0, 0, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -41, -41, 0, 0, 0, 0, 0,
        // State 77
        0, 0, 0, 0, 0, 0, 0, 0, -59, 0, 0, 0, 0, -59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 78
        0, 0, 0, 0, 0, 0, 0, 0, -58, 0, 0, 0, 0, -58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 79
        0, 0, 0, 0, 0, 0, 0, 0, -76, 0, 0, 0, 0, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 80
        0, -56, 0, -56, -56, -56, -56, 0, -56, -56, -56, -56, -56, -56, -56, -56, 0, 0, -56, -56, 0, 0, 0, -56, -56, -56, -56, -56, -56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -56, -56, 0, 0, 0, 0, 0,
        // State 81
        0, 0, 0, 0, 0, 0, 0, 0, -11, 0, 0, 0, 0, -11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 82
        0, 0, 0, 0, 0, 0, 0, 0, -12, 0, 0, 0, 0, -12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __action(state: i16, integer: usize) -> i16 {
        __ACTION[(state as usize) * 58 + integer]
    }
    const __EOF_ACTION: &[i16] = &[
        // State 0
        0,
        // State 1
        -62,
        // State 2
        -63,
        // State 3
        -64,
        // State 4
        -65,
        // State 5
        -66,
        // State 6
        -67,
        // State 7
        -60,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        -131,
        // State 22
        -130,
        // State 23
        -53,
        // State 24
        -32,
        // State 25
        -34,
        // State 26
        -36,
        // State 27
        -38,
        // State 28
        -40,
        // State 29
        -42,
        // State 30
        -55,
        // State 31
        -54,
        // State 32
        0,
        // State 33
        -61,
        // State 34
        0,
        // State 35
        0,
        // State 36
        -70,
        // State 37
        -69,
        // State 38
        -68,
        // State 39
        -72,
        // State 40
        -71,
        // State 41
        -57,
        // State 42
        0,
        // State 43
        0,
        // State 44
        0,
        // State 45
        0,
        // State 46
        0,
        // State 47
        0,
        // State 48
        0,
        // State 49
        0,
        // State 50
        0,
        // State 51
        0,
        // State 52
        0,
        // State 53
        0,
        // State 54
        0,
        // State 55
        0,
        // State 56
        0,
        // State 57
        0,
        // State 58
        0,
        // State 59
        0,
        // State 60
        0,
        // State 61
        0,
        // State 62
        0,
        // State 63
        0,
        // State 64
        0,
        // State 65
        0,
        // State 66
        0,
        // State 67
        0,
        // State 68
        0,
        // State 69
        -129,
        // State 70
        0,
        // State 71
        -31,
        // State 72
        -33,
        // State 73
        -35,
        // State 74
        -37,
        // State 75
        -39,
        // State 76
        -41,
        // State 77
        -59,
        // State 78
        -58,
        // State 79
        0,
        // State 80
        -56,
        // State 81
        0,
        // State 82
        0,
    ];
    fn __goto(state: i16, nt: usize) -> i16 {
        match nt {
            6 => 79,
            19 => 1,
            20 => 2,
            21 => 3,
            22 => 4,
            23 => 5,
            24 => 6,
            29 => match state {
                0 => 21,
                19 => 81,
                20 => 82,
                _ => 18,
            },
            30 => 22,
            31 => match state {
                16 => 77,
                17 => 78,
                _ => 23,
            },
            32 => match state {
                10 => 71,
                _ => 24,
            },
            33 => match state {
                11 => 72,
                _ => 25,
            },
            34 => match state {
                12 => 73,
                _ => 26,
            },
            35 => match state {
                13 => 74,
                _ => 27,
            },
            36 => match state {
                14 => 75,
                _ => 28,
            },
            37 => match state {
                15 => 76,
                _ => 29,
            },
            38 => 7,
            39 => 30,
            40 => 31,
            42 => 70,
            45 => 32,
            46 => 42,
            47 => 46,
            48 => 49,
            49 => 54,
            50 => 57,
            51 => 61,
            52 => 59,
            57 => 8,
            58 => 10,
            59 => 11,
            60 => 12,
            61 => 13,
            62 => 14,
            63 => 16,
            64 => 15,
            68 => match state {
                8 => 69,
                _ => 33,
            },
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i16) -> Vec<::std::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""!""###,
            r###""!=""###,
            r###""#""###,
            r###""%""###,
            r###""%=""###,
            r###""&""###,
            r###""&=""###,
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""*=""###,
            r###""+""###,
            r###""+=""###,
            r###"",""###,
            r###""-""###,
            r###""-=""###,
            r###"".""###,
            r###"".""###,
            r###""/""###,
            r###""/=""###,
            r###"":""###,
            r###""::""###,
            r###"";""###,
            r###""<""###,
            r###""<=""###,
            r###""=""###,
            r###""==""###,
            r###"">""###,
            r###"">=""###,
            r###""[""###,
            r###""]""###,
            r###""^""###,
            r###""bool""###,
            r###""builtin""###,
            r###""const""###,
            r###""do""###,
            r###""else""###,
            r###""enum""###,
            r###""false""###,
            r###""fn""###,
            r###""if""###,
            r###""int""###,
            r###""let""###,
            r###""mod""###,
            r###""return""###,
            r###""string""###,
            r###""struct""###,
            r###""trait""###,
            r###""true""###,
            r###""while""###,
            r###""{""###,
            r###""|""###,
            r###""|=""###,
            r###""}""###,
            r###"TokenIdentifier"###,
            r###"TokenInt"###,
            r###"TokenString"###,
        ];
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub struct __StateMachine<'input, 'err, '__2>
    where 'input: 'err
    {
        input: &'input str,
        intr: &'__2 mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __phantom: ::std::marker::PhantomData<(&'input (), &'err ())>,
    }
    impl<'input, 'err, '__2> __state_machine::ParserDefinition for __StateMachine<'input, 'err, '__2>
    where 'input: 'err
    {
        type Location = usize;
        type Error = ParserError;
        type Token = Token<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = Expr;
        type StateIndex = i16;
        type Action = i16;
        type ReduceIndex = i16;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, ::std::marker::PhantomData::<(&(), &())>)
        }

        #[inline]
        fn action(&self, state: i16, integer: usize) -> i16 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i16) -> i16 {
            __action(state, 58 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i16) -> i16 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i16, nt: usize) -> i16 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, ::std::marker::PhantomData::<(&(), &())>)
        }

        fn expected_tokens(&self, state: i16) -> Vec<String> {
            __expected_tokens(state)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            true
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            __Symbol::Variant2(recovery)
        }

        fn reduce(
            &mut self,
            action: i16,
            start_location: Option<&Self::Location>,
            states: &mut Vec<i16>,
            symbols: &mut Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.input,
                self.intr,
                self.errs,
                action,
                start_location,
                states,
                symbols,
                ::std::marker::PhantomData::<(&(), &())>,
            )
        }

        fn simulate_reduce(&self, action: i16) -> __state_machine::SimulatedReduce<Self> {
            __simulate_reduce(action, ::std::marker::PhantomData::<(&(), &())>)
        }
    }
    fn __token_to_integer<
        'input,
        'err,
    >(
        __token: &Token<'input>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> Option<usize>
    {
        match *__token {
            Token::Exclamation if true => Some(0),
            Token::Ne if true => Some(1),
            Token::Hash if true => Some(2),
            Token::Rem if true => Some(3),
            Token::RemAssign if true => Some(4),
            Token::And if true => Some(5),
            Token::AndAssign if true => Some(6),
            Token::OpenParen if true => Some(7),
            Token::CloseParen if true => Some(8),
            Token::Mul if true => Some(9),
            Token::MulAssign if true => Some(10),
            Token::Plus if true => Some(11),
            Token::AddAssign if true => Some(12),
            Token::Comma if true => Some(13),
            Token::Minus if true => Some(14),
            Token::SubAssign if true => Some(15),
            Token::Dot if true => Some(16),
            Token::Dot if true => Some(17),
            Token::Div if true => Some(18),
            Token::DivAssign if true => Some(19),
            Token::Colon if true => Some(20),
            Token::PathSeg if true => Some(21),
            Token::Semicolon if true => Some(22),
            Token::Lt if true => Some(23),
            Token::Le if true => Some(24),
            Token::Assign if true => Some(25),
            Token::Eq if true => Some(26),
            Token::Gt if true => Some(27),
            Token::Ge if true => Some(28),
            Token::OpenBracket if true => Some(29),
            Token::CloseBracket if true => Some(30),
            Token::Caret if true => Some(31),
            Token::Bool if true => Some(32),
            Token::Builtin if true => Some(33),
            Token::Const if true => Some(34),
            Token::Do if true => Some(35),
            Token::Else if true => Some(36),
            Token::Enum if true => Some(37),
            Token::False if true => Some(38),
            Token::Fn if true => Some(39),
            Token::If if true => Some(40),
            Token::Int if true => Some(41),
            Token::Let if true => Some(42),
            Token::Mod if true => Some(43),
            Token::Return if true => Some(44),
            Token::String if true => Some(45),
            Token::Struct if true => Some(46),
            Token::Trait if true => Some(47),
            Token::True if true => Some(48),
            Token::While if true => Some(49),
            Token::OpenBlock if true => Some(50),
            Token::Or if true => Some(51),
            Token::OrAssign if true => Some(52),
            Token::CloseBlock if true => Some(53),
            Token::Identifier(_) if true => Some(54),
            Token::IntLiteral(_) if true => Some(55),
            Token::StringLiteral(_) if true => Some(56),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'input,
        'err,
    >(
        __token_index: usize,
        __token: Token<'input>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> __Symbol<'input>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34 | 35 | 36 | 37 | 38 | 39 | 40 | 41 | 42 | 43 | 44 | 45 | 46 | 47 | 48 | 49 | 50 | 51 | 52 | 53 => __Symbol::Variant0(__token),
            54 | 55 | 56 => match __token {
                Token::Identifier(__tok0) | Token::IntLiteral(__tok0) | Token::StringLiteral(__tok0) if true => __Symbol::Variant1(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    fn __simulate_reduce<
        'input,
        'err,
        '__2,
    >(
        __reduce_index: i16,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> __state_machine::SimulatedReduce<__StateMachine<'input, 'err, '__2>>
    where
        'input: 'err,
    {
        match __reduce_index {
            0 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 0,
                }
            }
            1 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 0,
                }
            }
            2 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 1,
                }
            }
            3 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 2,
                }
            }
            4 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 2,
                }
            }
            5 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 3,
                }
            }
            6 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 3,
                }
            }
            7 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 4,
                }
            }
            8 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 5,
                }
            }
            9 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            10 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 6,
                }
            }
            11 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 6,
                }
            }
            12 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 7,
                }
            }
            13 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 8,
                }
            }
            14 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 8,
                }
            }
            15 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 9,
                }
            }
            16 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 10,
                }
            }
            17 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 10,
                }
            }
            18 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 11,
                }
            }
            19 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 11,
                }
            }
            20 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 12,
                }
            }
            21 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 13,
                }
            }
            22 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 13,
                }
            }
            23 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 14,
                }
            }
            24 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 15,
                }
            }
            25 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
                }
            }
            26 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 16,
                }
            }
            27 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 16,
                }
            }
            28 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 17,
                }
            }
            29 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 18,
                }
            }
            30 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 19,
                }
            }
            31 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 19,
                }
            }
            32 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 20,
                }
            }
            33 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 20,
                }
            }
            34 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 21,
                }
            }
            35 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 21,
                }
            }
            36 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 22,
                }
            }
            37 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 22,
                }
            }
            38 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 23,
                }
            }
            39 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
                }
            }
            40 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 24,
                }
            }
            41 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 24,
                }
            }
            42 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 25,
                }
            }
            43 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 25,
                }
            }
            44 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 25,
                }
            }
            45 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 25,
                }
            }
            46 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 26,
                }
            }
            47 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 27,
                }
            }
            48 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 27,
                }
            }
            49 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 28,
                }
            }
            50 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 28,
                }
            }
            51 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 28,
                }
            }
            52 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 29,
                }
            }
            53 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 30,
                }
            }
            54 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 30,
                }
            }
            55 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 30,
                }
            }
            56 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 30,
                }
            }
            57 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 31,
                }
            }
            58 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 31,
                }
            }
            59 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 31,
                }
            }
            60 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 32,
                }
            }
            61 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 33,
                }
            }
            62 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 34,
                }
            }
            63 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 35,
                }
            }
            64 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 36,
                }
            }
            65 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 37,
                }
            }
            66 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 38,
                }
            }
            67 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 39,
                }
            }
            68 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 40,
                }
            }
            69 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 40,
                }
            }
            70 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 40,
                }
            }
            71 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 40,
                }
            }
            72 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 41,
                }
            }
            73 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 41,
                }
            }
            74 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 42,
                }
            }
            75 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 42,
                }
            }
            76 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 43,
                }
            }
            77 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 44,
                }
            }
            78 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 44,
                }
            }
            79 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 45,
                }
            }
            80 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 45,
                }
            }
            81 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 46,
                }
            }
            82 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 46,
                }
            }
            83 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 46,
                }
            }
            84 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 47,
                }
            }
            85 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 47,
                }
            }
            86 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 48,
                }
            }
            87 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 48,
                }
            }
            88 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 48,
                }
            }
            89 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 48,
                }
            }
            90 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 49,
                }
            }
            91 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 49,
                }
            }
            92 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 50,
                }
            }
            93 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 51,
                }
            }
            94 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 51,
                }
            }
            95 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 51,
                }
            }
            96 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 51,
                }
            }
            97 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 51,
                }
            }
            98 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 51,
                }
            }
            99 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 51,
                }
            }
            100 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 52,
                }
            }
            101 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 53,
                }
            }
            102 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 53,
                }
            }
            103 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 54,
                }
            }
            104 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 55,
                }
            }
            105 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 55,
                }
            }
            106 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 55,
                }
            }
            107 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 56,
                }
            }
            108 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 56,
                }
            }
            109 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 57,
                }
            }
            110 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 58,
                }
            }
            111 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 59,
                }
            }
            112 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 60,
                }
            }
            113 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 61,
                }
            }
            114 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 62,
                }
            }
            115 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 63,
                }
            }
            116 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 64,
                }
            }
            117 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 65,
                }
            }
            118 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 65,
                }
            }
            119 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 8,
                    nonterminal_produced: 66,
                }
            }
            120 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 66,
                }
            }
            121 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 7,
                    nonterminal_produced: 66,
                }
            }
            122 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 66,
                }
            }
            123 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 67,
                }
            }
            124 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 67,
                }
            }
            125 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 67,
                }
            }
            126 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 67,
                }
            }
            127 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 67,
                }
            }
            128 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 68,
                }
            }
            129 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 68,
                }
            }
            130 => __state_machine::SimulatedReduce::Accept,
            131 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 70,
                }
            }
            _ => panic!("invalid reduction index {}", __reduce_index)
        }
    }
    pub struct ExprParser {
        _priv: (),
    }

    impl ExprParser {
        pub fn new() -> ExprParser {
            ExprParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
            'err,
            __TOKEN: __ToTriple<'input, 'err, >,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            input: &'input str,
            intr: &mut StringInterner<Sym>,
            errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
            __tokens0: __TOKENS,
        ) -> Result<Expr, __lalrpop_util::ParseError<usize, Token<'input>, ParserError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            __state_machine::Parser::drive(
                __StateMachine {
                    input,
                    intr,
                    errs,
                    __phantom: ::std::marker::PhantomData::<(&(), &())>,
                },
                __tokens,
            )
        }
    }
    fn __accepts<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __error_state: i16,
        __states: & [i16],
        __opt_integer: Option<usize>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> bool
    {
        let mut __states = __states.to_vec();
        __states.push(__error_state);
        loop {
            let mut __states_len = __states.len();
            let __top = __states[__states_len - 1];
            let __action = match __opt_integer {
                None => __EOF_ACTION[__top as usize],
                Some(__integer) => __action(__top, __integer),
            };
            if __action == 0 { return false; }
            if __action > 0 { return true; }
            let (__to_pop, __nt) = match __simulate_reduce(-(__action + 1), ::std::marker::PhantomData::<(&(), &())>) {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop, nonterminal_produced
                } => (states_to_pop, nonterminal_produced),
                __state_machine::SimulatedReduce::Accept => return true,
            };
            __states_len -= __to_pop;
            __states.truncate(__states_len);
            let __top = __states[__states_len - 1];
            let __next_state = __goto(__top, __nt);
            __states.push(__next_state);
        }
    }
    pub(crate) fn __reduce<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __action: i16,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i16>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> Option<Result<Expr,__lalrpop_util::ParseError<usize, Token<'input>, ParserError>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            1 => {
                __reduce1(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            2 => {
                __reduce2(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            3 => {
                __reduce3(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            4 => {
                __reduce4(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            5 => {
                __reduce5(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            6 => {
                __reduce6(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            7 => {
                __reduce7(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            8 => {
                __reduce8(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            9 => {
                __reduce9(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            10 => {
                __reduce10(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            11 => {
                __reduce11(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            12 => {
                __reduce12(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            13 => {
                __reduce13(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            14 => {
                __reduce14(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            15 => {
                __reduce15(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            16 => {
                __reduce16(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            17 => {
                __reduce17(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            18 => {
                __reduce18(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            19 => {
                __reduce19(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            20 => {
                __reduce20(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            21 => {
                __reduce21(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            22 => {
                __reduce22(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            23 => {
                __reduce23(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            24 => {
                __reduce24(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            25 => {
                __reduce25(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            26 => {
                __reduce26(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            27 => {
                __reduce27(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            28 => {
                __reduce28(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            29 => {
                __reduce29(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            30 => {
                __reduce30(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            31 => {
                __reduce31(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            32 => {
                __reduce32(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            33 => {
                __reduce33(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            34 => {
                __reduce34(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            35 => {
                __reduce35(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            36 => {
                __reduce36(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            37 => {
                __reduce37(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            38 => {
                __reduce38(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            39 => {
                __reduce39(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            40 => {
                __reduce40(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            41 => {
                __reduce41(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            42 => {
                __reduce42(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            43 => {
                __reduce43(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            44 => {
                __reduce44(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            45 => {
                __reduce45(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            46 => {
                __reduce46(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            47 => {
                __reduce47(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            48 => {
                __reduce48(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            49 => {
                __reduce49(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            50 => {
                __reduce50(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            51 => {
                __reduce51(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            52 => {
                __reduce52(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            53 => {
                __reduce53(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            54 => {
                __reduce54(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            55 => {
                __reduce55(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            56 => {
                __reduce56(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            57 => {
                __reduce57(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            58 => {
                __reduce58(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            59 => {
                __reduce59(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            60 => {
                __reduce60(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            61 => {
                __reduce61(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            62 => {
                __reduce62(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            63 => {
                __reduce63(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            64 => {
                __reduce64(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            65 => {
                __reduce65(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            66 => {
                __reduce66(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            67 => {
                __reduce67(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            68 => {
                __reduce68(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            69 => {
                __reduce69(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            70 => {
                __reduce70(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            71 => {
                __reduce71(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            72 => {
                __reduce72(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            73 => {
                __reduce73(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            74 => {
                __reduce74(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            75 => {
                __reduce75(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            76 => {
                __reduce76(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            77 => {
                __reduce77(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            78 => {
                __reduce78(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            79 => {
                __reduce79(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            80 => {
                __reduce80(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            81 => {
                __reduce81(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            82 => {
                __reduce82(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            83 => {
                __reduce83(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            84 => {
                __reduce84(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            85 => {
                __reduce85(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            86 => {
                __reduce86(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            87 => {
                __reduce87(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            88 => {
                __reduce88(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            89 => {
                __reduce89(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            90 => {
                __reduce90(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            91 => {
                __reduce91(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            92 => {
                __reduce92(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            93 => {
                __reduce93(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            94 => {
                __reduce94(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            95 => {
                __reduce95(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            96 => {
                __reduce96(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            97 => {
                __reduce97(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            98 => {
                __reduce98(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            99 => {
                __reduce99(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            100 => {
                __reduce100(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            101 => {
                __reduce101(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            102 => {
                __reduce102(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            103 => {
                __reduce103(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            104 => {
                __reduce104(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            105 => {
                __reduce105(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            106 => {
                __reduce106(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            107 => {
                __reduce107(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            108 => {
                __reduce108(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            109 => {
                __reduce109(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            110 => {
                __reduce110(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            111 => {
                __reduce111(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            112 => {
                __reduce112(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            113 => {
                __reduce113(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            114 => {
                __reduce114(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            115 => {
                __reduce115(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            116 => {
                __reduce116(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            117 => {
                __reduce117(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            118 => {
                __reduce118(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            119 => {
                __reduce119(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            120 => {
                __reduce120(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            121 => {
                __reduce121(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            122 => {
                __reduce122(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            123 => {
                __reduce123(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            124 => {
                __reduce124(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            125 => {
                __reduce125(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            126 => {
                __reduce126(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            127 => {
                __reduce127(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            128 => {
                __reduce128(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            129 => {
                __reduce129(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            130 => {
                // __Expr = Expr => ActionFn(1);
                let __sym0 = __pop_Variant6(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(input, intr, errs, __sym0);
                return Some(Ok(__nt));
            }
            131 => {
                __reduce131(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant12<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (usize, usize), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant12(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BinOp, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant23(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ComplexType, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant4(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expr, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant6(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Identifier, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant10(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant18<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, LitKind, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant18(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant14<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, MemberVariable, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant14(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant24<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Path, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant24(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant26<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ProgramPart, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant26(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Spanned<BinOp>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant29(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant28<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Spanned<UnOpKind>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant28(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant30<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Stmt, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant30(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant31<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, StructDefinition, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant31(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Token<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant0(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Type, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant8(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, UnOpKind, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant22(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant19<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<ComplexType>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant19(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant20<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Expr>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant20(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant17<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<MemberVariable>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant17(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant25<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<ProgramPart>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant25(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, __lalrpop_util::ErrorRecovery<usize, Token<'input>, ParserError>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant2(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant16<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant16(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant13<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<(usize, usize)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant13(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant21<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<MemberVariable>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant21(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Token<'input>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Type>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant9(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<ComplexType>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Expr>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant7(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant11<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Identifier>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant11(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant15<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<MemberVariable>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant15(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant27<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<ProgramPart>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant27(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant1(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // "builtin"? = "builtin" => ActionFn(90);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action90::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 0)
    }
    pub(crate) fn __reduce1<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // "builtin"? =  => ActionFn(91);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action91::<>(input, intr, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 0)
    }
    pub(crate) fn __reduce2<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ("+" <ComplexType1>) = "+", ComplexType1 => ActionFn(115);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action115::<>(input, intr, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 1)
    }
    pub(crate) fn __reduce3<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ("+" <ComplexType1>)* =  => ActionFn(113);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action113::<>(input, intr, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (0, 2)
    }
    pub(crate) fn __reduce4<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ("+" <ComplexType1>)* = ("+" <ComplexType1>)+ => ActionFn(114);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action114::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 2)
    }
    pub(crate) fn __reduce5<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ("+" <ComplexType1>)+ = "+", ComplexType1 => ActionFn(124);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action124::<>(input, intr, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce6<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ("+" <ComplexType1>)+ = ("+" <ComplexType1>)+, "+", ComplexType1 => ActionFn(125);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action125::<>(input, intr, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 3)
    }
    pub(crate) fn __reduce7<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ("," <Expr>) = ",", Expr => ActionFn(112);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action112::<>(input, intr, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 4)
    }
    pub(crate) fn __reduce8<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ("," <Expr>)* =  => ActionFn(110);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action110::<>(input, intr, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 5)
    }
    pub(crate) fn __reduce9<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ("," <Expr>)* = ("," <Expr>)+ => ActionFn(111);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action111::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce10<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ("," <Expr>)+ = ",", Expr => ActionFn(128);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action128::<>(input, intr, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 6)
    }
    pub(crate) fn __reduce11<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ("," <Expr>)+ = ("," <Expr>)+, ",", Expr => ActionFn(129);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action129::<>(input, intr, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 6)
    }
    pub(crate) fn __reduce12<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (":" <Type>) = ":", Type => ActionFn(89);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action89::<>(input, intr, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 7)
    }
    pub(crate) fn __reduce13<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (":" <Type>)? = ":", Type => ActionFn(132);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action132::<>(input, intr, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 8)
    }
    pub(crate) fn __reduce14<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (":" <Type>)? =  => ActionFn(88);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action88::<>(input, intr, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (0, 8)
    }
    pub(crate) fn __reduce15<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ("::" <Identifier>) = "::", Identifier => ActionFn(85);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action85::<>(input, intr, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (2, 9)
    }
    pub(crate) fn __reduce16<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ("::" <Identifier>)* =  => ActionFn(83);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action83::<>(input, intr, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (0, 10)
    }
    pub(crate) fn __reduce17<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ("::" <Identifier>)* = ("::" <Identifier>)+ => ActionFn(84);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action84::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce18<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ("::" <Identifier>)+ = "::", Identifier => ActionFn(137);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action137::<>(input, intr, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (2, 11)
    }
    pub(crate) fn __reduce19<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ("::" <Identifier>)+ = ("::" <Identifier>)+, "::", Identifier => ActionFn(138);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action138::<>(input, intr, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (3, 11)
    }
    pub(crate) fn __reduce20<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<@L> ";" <@R>) = ";" => ActionFn(177);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action177::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce21<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<@L> ";" <@R>)? = ";" => ActionFn(213);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action213::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce22<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<@L> ";" <@R>)? =  => ActionFn(81);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action81::<>(input, intr, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (0, 13)
    }
    pub(crate) fn __reduce23<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<MemberVariable> ",") = MemberVariable, "," => ActionFn(100);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action100::<>(input, intr, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (2, 14)
    }
    pub(crate) fn __reduce24<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<MemberVariable> ",")* =  => ActionFn(98);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action98::<>(input, intr, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (0, 15)
    }
    pub(crate) fn __reduce25<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<MemberVariable> ",")* = (<MemberVariable> ",")+ => ActionFn(99);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action99::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce26<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<MemberVariable> ",")+ = MemberVariable, "," => ActionFn(216);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action216::<>(input, intr, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (2, 16)
    }
    pub(crate) fn __reduce27<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<MemberVariable> ",")+ = (<MemberVariable> ",")+, MemberVariable, "," => ActionFn(217);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant14(__symbols);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action217::<>(input, intr, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce28<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // @L =  => ActionFn(93);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action93::<>(input, intr, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (0, 17)
    }
    pub(crate) fn __reduce29<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // @R =  => ActionFn(92);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action92::<>(input, intr, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (0, 18)
    }
    pub(crate) fn __reduce30<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // BinOpExpr<Op4, Expr3> = BinOpExpr<Op4, Expr3>, Spanned<Op4>, Expr3 => ActionFn(178);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant29(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action178::<>(input, intr, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 19)
    }
    pub(crate) fn __reduce31<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // BinOpExpr<Op4, Expr3> = Expr3 => ActionFn(68);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action68::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce32<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // BinOpExpr<Op5, Expr4> = BinOpExpr<Op5, Expr4>, Spanned<Op5>, Expr4 => ActionFn(179);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant29(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action179::<>(input, intr, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 20)
    }
    pub(crate) fn __reduce33<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // BinOpExpr<Op5, Expr4> = Expr4 => ActionFn(70);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action70::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce34<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // BinOpExpr<Op6, Expr5> = BinOpExpr<Op6, Expr5>, Spanned<Op6>, Expr5 => ActionFn(180);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant29(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action180::<>(input, intr, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 21)
    }
    pub(crate) fn __reduce35<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // BinOpExpr<Op6, Expr5> = Expr5 => ActionFn(72);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action72::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce36<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // BinOpExpr<Op7, Expr6> = BinOpExpr<Op7, Expr6>, Spanned<Op7>, Expr6 => ActionFn(181);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant29(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action181::<>(input, intr, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 22)
    }
    pub(crate) fn __reduce37<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // BinOpExpr<Op7, Expr6> = Expr6 => ActionFn(74);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action74::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 22)
    }
    pub(crate) fn __reduce38<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // BinOpExpr<OpAnd, Expr7> = BinOpExpr<OpAnd, Expr7>, Spanned<OpAnd>, Expr7 => ActionFn(182);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant29(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action182::<>(input, intr, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 23)
    }
    pub(crate) fn __reduce39<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // BinOpExpr<OpAnd, Expr7> = Expr7 => ActionFn(76);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action76::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 23)
    }
    pub(crate) fn __reduce40<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // BinOpExpr<OpOr, Expr8> = BinOpExpr<OpOr, Expr8>, Spanned<OpOr>, Expr8 => ActionFn(183);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant29(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action183::<>(input, intr, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 24)
    }
    pub(crate) fn __reduce41<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // BinOpExpr<OpOr, Expr8> = Expr8 => ActionFn(78);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action78::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce42<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Comma<MemberVariable> = MemberVariable => ActionFn(220);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action220::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce43<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Comma<MemberVariable> =  => ActionFn(221);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action221::<>(input, intr, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (0, 25)
    }
    pub(crate) fn __reduce44<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Comma<MemberVariable> = (<MemberVariable> ",")+, MemberVariable => ActionFn(222);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant14(__symbols);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action222::<>(input, intr, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (2, 25)
    }
    pub(crate) fn __reduce45<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Comma<MemberVariable> = (<MemberVariable> ",")+ => ActionFn(223);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action223::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce46<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ComplexType = Many1<"+", ComplexType1> => ActionFn(184);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action184::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 26)
    }
    pub(crate) fn __reduce47<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ComplexType0 = Path => ActionFn(185);
        let __sym0 = __pop_Variant24(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action185::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce48<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ComplexType0 = "(", ComplexType, ")" => ActionFn(186);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action186::<>(input, intr, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 27)
    }
    pub(crate) fn __reduce49<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ComplexType1 = "^", ComplexType0 => ActionFn(187);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action187::<>(input, intr, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 28)
    }
    pub(crate) fn __reduce50<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ComplexType1 = "!", ComplexType0 => ActionFn(188);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action188::<>(input, intr, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 28)
    }
    pub(crate) fn __reduce51<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ComplexType1 = ComplexType0 => ActionFn(60);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action60::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 28)
    }
    pub(crate) fn __reduce52<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr = Expr10 => ActionFn(11);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action11::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 29)
    }
    pub(crate) fn __reduce53<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr0 = Lit => ActionFn(189);
        let __sym0 = __pop_Variant18(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action189::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 30)
    }
    pub(crate) fn __reduce54<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr0 = Identifier => ActionFn(190);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action190::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 30)
    }
    pub(crate) fn __reduce55<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr0 = "(", Many1<",", Expr>, ")" => ActionFn(191);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant20(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action191::<>(input, intr, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 30)
    }
    pub(crate) fn __reduce56<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr0 = error => ActionFn(192);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action192::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 30)
    }
    pub(crate) fn __reduce57<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr10 = Expr9, "=", Expr10 => ActionFn(193);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action193::<>(input, intr, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 31)
    }
    pub(crate) fn __reduce58<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr10 = Expr9, Spanned<OpAssign>, Expr10 => ActionFn(194);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant29(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action194::<>(input, intr, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 31)
    }
    pub(crate) fn __reduce59<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr10 = Expr9 => ActionFn(14);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action14::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 31)
    }
    pub(crate) fn __reduce60<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr3 = UnOpExpr<Op3, Expr0> => ActionFn(41);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action41::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 32)
    }
    pub(crate) fn __reduce61<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr4 = BinOpExpr<Op4, Expr3> => ActionFn(37);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action37::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 33)
    }
    pub(crate) fn __reduce62<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr5 = BinOpExpr<Op5, Expr4> => ActionFn(34);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action34::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 34)
    }
    pub(crate) fn __reduce63<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr6 = BinOpExpr<Op6, Expr5> => ActionFn(29);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action29::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 35)
    }
    pub(crate) fn __reduce64<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr7 = BinOpExpr<Op7, Expr6> => ActionFn(26);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 36)
    }
    pub(crate) fn __reduce65<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr8 = BinOpExpr<OpAnd, Expr7> => ActionFn(24);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action24::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 37)
    }
    pub(crate) fn __reduce66<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr9 = BinOpExpr<OpOr, Expr8> => ActionFn(22);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 38)
    }
    pub(crate) fn __reduce67<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Identifier = TokenIdentifier => ActionFn(195);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action195::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 39)
    }
    pub(crate) fn __reduce68<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Lit = "true" => ActionFn(48);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action48::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 40)
    }
    pub(crate) fn __reduce69<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Lit = "false" => ActionFn(49);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action49::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 40)
    }
    pub(crate) fn __reduce70<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Lit = TokenString => ActionFn(50);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action50::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 40)
    }
    pub(crate) fn __reduce71<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Lit = TokenInt => ActionFn(196);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action196::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 40)
    }
    pub(crate) fn __reduce72<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Many1<"+", ComplexType1> = ComplexType1 => ActionFn(126);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action126::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 41)
    }
    pub(crate) fn __reduce73<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Many1<"+", ComplexType1> = ComplexType1, ("+" <ComplexType1>)+ => ActionFn(127);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action127::<>(input, intr, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (2, 41)
    }
    pub(crate) fn __reduce74<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Many1<",", Expr> = Expr => ActionFn(130);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action130::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 42)
    }
    pub(crate) fn __reduce75<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Many1<",", Expr> = Expr, ("," <Expr>)+ => ActionFn(131);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action131::<>(input, intr, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (2, 42)
    }
    pub(crate) fn __reduce76<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // MemberVariable = Identifier, ":", Type => ActionFn(7);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action7::<>(input, intr, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (3, 43)
    }
    pub(crate) fn __reduce77<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // MemberVariable? = MemberVariable => ActionFn(96);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action96::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 44)
    }
    pub(crate) fn __reduce78<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // MemberVariable? =  => ActionFn(97);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action97::<>(input, intr, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (0, 44)
    }
    pub(crate) fn __reduce79<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op3 = "-" => ActionFn(42);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action42::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 45)
    }
    pub(crate) fn __reduce80<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op3 = "!" => ActionFn(43);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action43::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 45)
    }
    pub(crate) fn __reduce81<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op4 = "*" => ActionFn(38);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action38::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 46)
    }
    pub(crate) fn __reduce82<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op4 = "/" => ActionFn(39);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action39::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 46)
    }
    pub(crate) fn __reduce83<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op4 = "%" => ActionFn(40);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action40::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 46)
    }
    pub(crate) fn __reduce84<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op5 = "+" => ActionFn(35);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action35::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 47)
    }
    pub(crate) fn __reduce85<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op5 = "-" => ActionFn(36);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action36::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 47)
    }
    pub(crate) fn __reduce86<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op6 = "<" => ActionFn(30);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action30::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 48)
    }
    pub(crate) fn __reduce87<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op6 = ">" => ActionFn(31);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action31::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 48)
    }
    pub(crate) fn __reduce88<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op6 = "<=" => ActionFn(32);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action32::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 48)
    }
    pub(crate) fn __reduce89<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op6 = ">=" => ActionFn(33);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action33::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 48)
    }
    pub(crate) fn __reduce90<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op7 = "==" => ActionFn(27);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 49)
    }
    pub(crate) fn __reduce91<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op7 = "!=" => ActionFn(28);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action28::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 49)
    }
    pub(crate) fn __reduce92<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // OpAnd = "&" => ActionFn(25);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action25::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 50)
    }
    pub(crate) fn __reduce93<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // OpAssign = "+=" => ActionFn(15);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action15::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 51)
    }
    pub(crate) fn __reduce94<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // OpAssign = "-=" => ActionFn(16);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action16::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 51)
    }
    pub(crate) fn __reduce95<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // OpAssign = "*=" => ActionFn(17);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 51)
    }
    pub(crate) fn __reduce96<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // OpAssign = "/=" => ActionFn(18);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action18::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 51)
    }
    pub(crate) fn __reduce97<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // OpAssign = "%=" => ActionFn(19);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 51)
    }
    pub(crate) fn __reduce98<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // OpAssign = "&=" => ActionFn(20);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action20::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 51)
    }
    pub(crate) fn __reduce99<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // OpAssign = "|=" => ActionFn(21);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action21::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 51)
    }
    pub(crate) fn __reduce100<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // OpOr = "|" => ActionFn(23);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action23::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 52)
    }
    pub(crate) fn __reduce101<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Path = Identifier => ActionFn(139);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action139::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant24(__nt), __end));
        (1, 53)
    }
    pub(crate) fn __reduce102<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Path = Identifier, ("::" <Identifier>)+ => ActionFn(140);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action140::<>(input, intr, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant24(__nt), __end));
        (2, 53)
    }
    pub(crate) fn __reduce103<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Program = ProgramPart+ => ActionFn(2);
        let __sym0 = __pop_Variant27(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant25(__nt), __end));
        (1, 54)
    }
    pub(crate) fn __reduce104<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ProgramPart = "mod", Identifier, "{", Program, "}" => ActionFn(3);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant25(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action3::<>(input, intr, errs, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant26(__nt), __end));
        (5, 55)
    }
    pub(crate) fn __reduce105<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ProgramPart = StructDefinition => ActionFn(4);
        let __sym0 = __pop_Variant31(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant26(__nt), __end));
        (1, 55)
    }
    pub(crate) fn __reduce106<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ProgramPart = error => ActionFn(197);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action197::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant26(__nt), __end));
        (1, 55)
    }
    pub(crate) fn __reduce107<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ProgramPart+ = ProgramPart => ActionFn(94);
        let __sym0 = __pop_Variant26(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action94::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant27(__nt), __end));
        (1, 56)
    }
    pub(crate) fn __reduce108<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ProgramPart+ = ProgramPart+, ProgramPart => ActionFn(95);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant26(__symbols);
        let __sym0 = __pop_Variant27(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action95::<>(input, intr, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant27(__nt), __end));
        (2, 56)
    }
    pub(crate) fn __reduce109<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Spanned<Op3> = Op3 => ActionFn(198);
        let __sym0 = __pop_Variant22(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action198::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant28(__nt), __end));
        (1, 57)
    }
    pub(crate) fn __reduce110<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Spanned<Op4> = Op4 => ActionFn(199);
        let __sym0 = __pop_Variant23(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action199::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant29(__nt), __end));
        (1, 58)
    }
    pub(crate) fn __reduce111<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Spanned<Op5> = Op5 => ActionFn(200);
        let __sym0 = __pop_Variant23(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action200::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant29(__nt), __end));
        (1, 59)
    }
    pub(crate) fn __reduce112<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Spanned<Op6> = Op6 => ActionFn(201);
        let __sym0 = __pop_Variant23(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action201::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant29(__nt), __end));
        (1, 60)
    }
    pub(crate) fn __reduce113<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Spanned<Op7> = Op7 => ActionFn(202);
        let __sym0 = __pop_Variant23(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action202::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant29(__nt), __end));
        (1, 61)
    }
    pub(crate) fn __reduce114<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Spanned<OpAnd> = OpAnd => ActionFn(203);
        let __sym0 = __pop_Variant23(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action203::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant29(__nt), __end));
        (1, 62)
    }
    pub(crate) fn __reduce115<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Spanned<OpAssign> = OpAssign => ActionFn(204);
        let __sym0 = __pop_Variant23(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action204::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant29(__nt), __end));
        (1, 63)
    }
    pub(crate) fn __reduce116<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Spanned<OpOr> = OpOr => ActionFn(205);
        let __sym0 = __pop_Variant23(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action205::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant29(__nt), __end));
        (1, 64)
    }
    pub(crate) fn __reduce117<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Stmt = Expr, ";" => ActionFn(214);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action214::<>(input, intr, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant30(__nt), __end));
        (2, 65)
    }
    pub(crate) fn __reduce118<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Stmt = Expr => ActionFn(215);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action215::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant30(__nt), __end));
        (1, 65)
    }
    pub(crate) fn __reduce119<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // StructDefinition = "builtin", "struct", Identifier, ":", Type, "{", Comma<MemberVariable>, "}" => ActionFn(133);
        assert!(__symbols.len() >= 8);
        let __sym7 = __pop_Variant0(__symbols);
        let __sym6 = __pop_Variant17(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant8(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym7.2.clone();
        let __nt = super::__action133::<>(input, intr, errs, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7);
        __symbols.push((__start, __Symbol::Variant31(__nt), __end));
        (8, 66)
    }
    pub(crate) fn __reduce120<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // StructDefinition = "builtin", "struct", Identifier, "{", Comma<MemberVariable>, "}" => ActionFn(134);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant17(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action134::<>(input, intr, errs, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant31(__nt), __end));
        (6, 66)
    }
    pub(crate) fn __reduce121<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // StructDefinition = "struct", Identifier, ":", Type, "{", Comma<MemberVariable>, "}" => ActionFn(135);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant17(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant8(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action135::<>(input, intr, errs, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant31(__nt), __end));
        (7, 66)
    }
    pub(crate) fn __reduce122<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // StructDefinition = "struct", Identifier, "{", Comma<MemberVariable>, "}" => ActionFn(136);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant17(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action136::<>(input, intr, errs, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant31(__nt), __end));
        (5, 66)
    }
    pub(crate) fn __reduce123<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Type = "bool" => ActionFn(207);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action207::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 67)
    }
    pub(crate) fn __reduce124<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Type = "string" => ActionFn(208);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action208::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 67)
    }
    pub(crate) fn __reduce125<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Type = "int" => ActionFn(209);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action209::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 67)
    }
    pub(crate) fn __reduce126<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Type = "(", ")" => ActionFn(210);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action210::<>(input, intr, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 67)
    }
    pub(crate) fn __reduce127<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Type = ComplexType => ActionFn(211);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action211::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 67)
    }
    pub(crate) fn __reduce128<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // UnOpExpr<Op3, Expr0> = Spanned<Op3>, UnOpExpr<Op3, Expr0> => ActionFn(212);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant28(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action212::<>(input, intr, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 68)
    }
    pub(crate) fn __reduce129<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // UnOpExpr<Op3, Expr0> = Expr0 => ActionFn(66);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action66::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 68)
    }
    pub(crate) fn __reduce131<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // __Program = Program => ActionFn(0);
        let __sym0 = __pop_Variant25(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant25(__nt), __end));
        (1, 70)
    }
}
pub use self::__parse__Expr::ExprParser;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Program {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use std::str::FromStr;
    use string_interner::{Sym, StringInterner};
    use lalrpop_util::{ParseError, ErrorRecovery};
    use crate::lexer::{Token, ParserError};
    use crate::ast::{*, BinOpKind as BinOp};
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Variant0(Token<'input>),
        Variant1(&'input str),
        Variant2(__lalrpop_util::ErrorRecovery<usize, Token<'input>, ParserError>),
        Variant3(::std::option::Option<Token<'input>>),
        Variant4(ComplexType),
        Variant5(::std::vec::Vec<ComplexType>),
        Variant6(Expr),
        Variant7(::std::vec::Vec<Expr>),
        Variant8(Type),
        Variant9(::std::option::Option<Type>),
        Variant10(Identifier),
        Variant11(::std::vec::Vec<Identifier>),
        Variant12((usize, usize)),
        Variant13(::std::option::Option<(usize, usize)>),
        Variant14(MemberVariable),
        Variant15(::std::vec::Vec<MemberVariable>),
        Variant16(usize),
        Variant17(Vec<MemberVariable>),
        Variant18(LitKind),
        Variant19(Vec<ComplexType>),
        Variant20(Vec<Expr>),
        Variant21(::std::option::Option<MemberVariable>),
        Variant22(UnOpKind),
        Variant23(BinOp),
        Variant24(Path),
        Variant25(Vec<ProgramPart>),
        Variant26(ProgramPart),
        Variant27(::std::vec::Vec<ProgramPart>),
        Variant28(Spanned<UnOpKind>),
        Variant29(Spanned<BinOp>),
        Variant30(Stmt),
        Variant31(StructDefinition),
    }
    const __ACTION: &[i16] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 4, 0, 0, 0, 0, 0, 0, -104, 0, 0, 0, 29,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29,
        // State 6
        13, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 41, 0, 0, 0, 0, 0, 0, 0, 0, 42, 0, 0, 0, 43, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -44, 32, 0, 0, 0,
        // State 8
        13, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 41, 0, 0, 0, 0, 0, 0, 0, 0, 42, 0, 0, 0, 43, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -44, 32, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, -73, 0, 0, 17, 0, -73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -73, 0, 0, -73, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, -102, 0, 0, -102, 0, -102, 0, 0, 0, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -102, 0, 0, -102, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0,
        // State 13
        13, 0, 0, 0, 0, 0, 0, 20, 54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -46, 32, 0, 0, 0,
        // State 16
        13, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -44, 32, 0, 0, 0,
        // State 19
        13, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0,
        // State 20
        13, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 41, 0, 0, 0, 0, 0, 0, 0, 0, 42, 0, 0, 0, 43, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -44, 32, 0, 0, 0,
        // State 22
        13, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -108, 0, 0, 0, 0, 0, 0, 0, 0, 0, -108, 0, 0, -108, 0, 0, 0, 0, 0, 0, -108, 0, 0, 0, -108,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -106, 0, 0, 0, 0, 0, 0, 0, 0, 0, -106, 0, 0, -106, 0, 0, 0, 0, 0, 0, -106, 0, 0, 0, -106,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -107, 0, 0, 0, 0, 0, 0, 0, 0, 0, -107, 0, 0, -107, 0, 0, 0, 0, 0, 0, -107, 0, 0, 0, -107,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -109, 0, 0, 0, 0, 0, 0, 0, 0, 0, -109, 0, 0, -109, 0, 0, 0, 0, 0, 0, -109, 0, 0, 0, -109,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, -68, 0, 0, -68, 0, -68, 0, 0, 0, 0, 0, 0, -68, -68, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -68, 0, 0, -68, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 49, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -128, 0, 0, -128, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, -52, 0, 0, -52, 0, -52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -52, 0, 0, -52, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, -47, 0, 0, 0, 0, -47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -47, 0, 0, -47, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, -48, 0, 0, -48, 0, -48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -48, 0, 0, -48, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -124, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -124, 0, 0, -124, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -126, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -126, 0, 0, -126, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -125, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -125, 0, 0, -125, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 57, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -43, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 59, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -105, 0, 0, 0, 0, 0, 0, 0, 0, 0, -105, 0, 0, -105, 0, 0, 0, 0, 0, 0, -105, 0, 0, 0, -105,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, -74, 0, 0, 23, 0, -74, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -74, 0, 0, -74, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, -103, 0, 0, -103, 0, -103, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -103, 0, 0, -103, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, 0, 0, -51, 0, 0, -51, 0, -51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -51, 0, 0, -51, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -127, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -127, 0, 0, -127, 0, 0, 0, 0,
        // State 54
        0, 0, 0, 0, 0, 0, 0, 0, -50, 0, 0, -50, 0, -50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -50, 0, 0, -50, 0, 0, 0, 0,
        // State 55
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -45, 0, 0, 0, 0,
        // State 56
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -123, 0, 0, 0, 0, 0, 0, 0, 0, 0, -123, 0, 0, -123, 0, 0, 0, 0, 0, 0, -123, 0, 0, 0, -123,
        // State 57
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -27, -27, 0, 0, 0,
        // State 58
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -121, 0, 0, 0, 0, 0, 0, 0, 0, 0, -121, 0, 0, -121, 0, 0, 0, 0, 0, 0, -121, 0, 0, 0, -121,
        // State 59
        0, 0, 0, 0, 0, 0, 0, 0, -6, 0, 0, -6, 0, -6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -6, 0, 0, -6, 0, 0, 0, 0,
        // State 60
        0, 0, 0, 0, 0, 0, 0, 0, -19, 0, 0, -19, 0, -19, 0, 0, 0, 0, 0, 0, 0, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19, 0, 0, -19, 0, 0, 0, 0,
        // State 61
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 69, 0, 0, 0, 0,
        // State 62
        0, 0, 0, 0, 0, 0, 0, 0, -49, 0, 0, -49, 0, -49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -49, 0, 0, -49, 0, 0, 0, 0,
        // State 63
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -28, -28, 0, 0, 0,
        // State 64
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -77, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -77, 0, 0, 0, 0,
        // State 65
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 70, 0, 0, 0, 0,
        // State 66
        0, 0, 0, 0, 0, 0, 0, 0, -7, 0, 0, -7, 0, -7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -7, 0, 0, -7, 0, 0, 0, 0,
        // State 67
        0, 0, 0, 0, 0, 0, 0, 0, -20, 0, 0, -20, 0, -20, 0, 0, 0, 0, 0, 0, 0, -20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -20, 0, 0, -20, 0, 0, 0, 0,
        // State 68
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -122, 0, 0, 0, 0, 0, 0, 0, 0, 0, -122, 0, 0, -122, 0, 0, 0, 0, 0, 0, -122, 0, 0, 0, -122,
        // State 69
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -120, 0, 0, 0, 0, 0, 0, 0, 0, 0, -120, 0, 0, -120, 0, 0, 0, 0, 0, 0, -120, 0, 0, 0, -120,
    ];
    fn __action(state: i16, integer: usize) -> i16 {
        __ACTION[(state as usize) * 58 + integer]
    }
    const __EOF_ACTION: &[i16] = &[
        // State 0
        0,
        // State 1
        -104,
        // State 2
        0,
        // State 3
        0,
        // State 4
        0,
        // State 5
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        -132,
        // State 25
        -108,
        // State 26
        -106,
        // State 27
        0,
        // State 28
        -107,
        // State 29
        -109,
        // State 30
        0,
        // State 31
        0,
        // State 32
        0,
        // State 33
        0,
        // State 34
        0,
        // State 35
        0,
        // State 36
        0,
        // State 37
        0,
        // State 38
        0,
        // State 39
        0,
        // State 40
        0,
        // State 41
        0,
        // State 42
        0,
        // State 43
        0,
        // State 44
        0,
        // State 45
        0,
        // State 46
        0,
        // State 47
        0,
        // State 48
        -105,
        // State 49
        0,
        // State 50
        0,
        // State 51
        0,
        // State 52
        0,
        // State 53
        0,
        // State 54
        0,
        // State 55
        0,
        // State 56
        -123,
        // State 57
        0,
        // State 58
        -121,
        // State 59
        0,
        // State 60
        0,
        // State 61
        0,
        // State 62
        0,
        // State 63
        0,
        // State 64
        0,
        // State 65
        0,
        // State 66
        0,
        // State 67
        0,
        // State 68
        -122,
        // State 69
        -120,
    ];
    fn __goto(state: i16, nt: usize) -> i16 {
        match nt {
            3 => 49,
            11 => 50,
            16 => 15,
            25 => match state {
                9 => 47,
                18 => 61,
                21 => 65,
                _ => 43,
            },
            26 => match state {
                13 | 19 => 52,
                _ => 35,
            },
            27 => match state {
                12 => 51,
                14 => 54,
                _ => 36,
            },
            28 => match state {
                16 => 59,
                22 => 66,
                _ => 10,
            },
            39 => match state {
                2 => 30,
                3 => 32,
                4 => 33,
                7 | 9 | 15 | 18 | 21 => 44,
                17 => 60,
                23 => 67,
                _ => 11,
            },
            41 => 37,
            43 => match state {
                15 => 55,
                _ => 45,
            },
            53 => 38,
            54 => match state {
                5 => 34,
                _ => 24,
            },
            55 => match state {
                1 => 29,
                _ => 25,
            },
            56 => 1,
            66 => 26,
            67 => match state {
                8 => 46,
                20 => 64,
                _ => 39,
            },
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i16) -> Vec<::std::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""!""###,
            r###""!=""###,
            r###""#""###,
            r###""%""###,
            r###""%=""###,
            r###""&""###,
            r###""&=""###,
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""*=""###,
            r###""+""###,
            r###""+=""###,
            r###"",""###,
            r###""-""###,
            r###""-=""###,
            r###"".""###,
            r###"".""###,
            r###""/""###,
            r###""/=""###,
            r###"":""###,
            r###""::""###,
            r###"";""###,
            r###""<""###,
            r###""<=""###,
            r###""=""###,
            r###""==""###,
            r###"">""###,
            r###"">=""###,
            r###""[""###,
            r###""]""###,
            r###""^""###,
            r###""bool""###,
            r###""builtin""###,
            r###""const""###,
            r###""do""###,
            r###""else""###,
            r###""enum""###,
            r###""false""###,
            r###""fn""###,
            r###""if""###,
            r###""int""###,
            r###""let""###,
            r###""mod""###,
            r###""return""###,
            r###""string""###,
            r###""struct""###,
            r###""trait""###,
            r###""true""###,
            r###""while""###,
            r###""{""###,
            r###""|""###,
            r###""|=""###,
            r###""}""###,
            r###"TokenIdentifier"###,
            r###"TokenInt"###,
            r###"TokenString"###,
        ];
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub struct __StateMachine<'input, 'err, '__2>
    where 'input: 'err
    {
        input: &'input str,
        intr: &'__2 mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __phantom: ::std::marker::PhantomData<(&'input (), &'err ())>,
    }
    impl<'input, 'err, '__2> __state_machine::ParserDefinition for __StateMachine<'input, 'err, '__2>
    where 'input: 'err
    {
        type Location = usize;
        type Error = ParserError;
        type Token = Token<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = Vec<ProgramPart>;
        type StateIndex = i16;
        type Action = i16;
        type ReduceIndex = i16;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, ::std::marker::PhantomData::<(&(), &())>)
        }

        #[inline]
        fn action(&self, state: i16, integer: usize) -> i16 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i16) -> i16 {
            __action(state, 58 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i16) -> i16 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i16, nt: usize) -> i16 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, ::std::marker::PhantomData::<(&(), &())>)
        }

        fn expected_tokens(&self, state: i16) -> Vec<String> {
            __expected_tokens(state)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            true
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            __Symbol::Variant2(recovery)
        }

        fn reduce(
            &mut self,
            action: i16,
            start_location: Option<&Self::Location>,
            states: &mut Vec<i16>,
            symbols: &mut Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.input,
                self.intr,
                self.errs,
                action,
                start_location,
                states,
                symbols,
                ::std::marker::PhantomData::<(&(), &())>,
            )
        }

        fn simulate_reduce(&self, action: i16) -> __state_machine::SimulatedReduce<Self> {
            __simulate_reduce(action, ::std::marker::PhantomData::<(&(), &())>)
        }
    }
    fn __token_to_integer<
        'input,
        'err,
    >(
        __token: &Token<'input>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> Option<usize>
    {
        match *__token {
            Token::Exclamation if true => Some(0),
            Token::Ne if true => Some(1),
            Token::Hash if true => Some(2),
            Token::Rem if true => Some(3),
            Token::RemAssign if true => Some(4),
            Token::And if true => Some(5),
            Token::AndAssign if true => Some(6),
            Token::OpenParen if true => Some(7),
            Token::CloseParen if true => Some(8),
            Token::Mul if true => Some(9),
            Token::MulAssign if true => Some(10),
            Token::Plus if true => Some(11),
            Token::AddAssign if true => Some(12),
            Token::Comma if true => Some(13),
            Token::Minus if true => Some(14),
            Token::SubAssign if true => Some(15),
            Token::Dot if true => Some(16),
            Token::Dot if true => Some(17),
            Token::Div if true => Some(18),
            Token::DivAssign if true => Some(19),
            Token::Colon if true => Some(20),
            Token::PathSeg if true => Some(21),
            Token::Semicolon if true => Some(22),
            Token::Lt if true => Some(23),
            Token::Le if true => Some(24),
            Token::Assign if true => Some(25),
            Token::Eq if true => Some(26),
            Token::Gt if true => Some(27),
            Token::Ge if true => Some(28),
            Token::OpenBracket if true => Some(29),
            Token::CloseBracket if true => Some(30),
            Token::Caret if true => Some(31),
            Token::Bool if true => Some(32),
            Token::Builtin if true => Some(33),
            Token::Const if true => Some(34),
            Token::Do if true => Some(35),
            Token::Else if true => Some(36),
            Token::Enum if true => Some(37),
            Token::False if true => Some(38),
            Token::Fn if true => Some(39),
            Token::If if true => Some(40),
            Token::Int if true => Some(41),
            Token::Let if true => Some(42),
            Token::Mod if true => Some(43),
            Token::Return if true => Some(44),
            Token::String if true => Some(45),
            Token::Struct if true => Some(46),
            Token::Trait if true => Some(47),
            Token::True if true => Some(48),
            Token::While if true => Some(49),
            Token::OpenBlock if true => Some(50),
            Token::Or if true => Some(51),
            Token::OrAssign if true => Some(52),
            Token::CloseBlock if true => Some(53),
            Token::Identifier(_) if true => Some(54),
            Token::IntLiteral(_) if true => Some(55),
            Token::StringLiteral(_) if true => Some(56),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'input,
        'err,
    >(
        __token_index: usize,
        __token: Token<'input>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> __Symbol<'input>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34 | 35 | 36 | 37 | 38 | 39 | 40 | 41 | 42 | 43 | 44 | 45 | 46 | 47 | 48 | 49 | 50 | 51 | 52 | 53 => __Symbol::Variant0(__token),
            54 | 55 | 56 => match __token {
                Token::Identifier(__tok0) | Token::IntLiteral(__tok0) | Token::StringLiteral(__tok0) if true => __Symbol::Variant1(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    fn __simulate_reduce<
        'input,
        'err,
        '__2,
    >(
        __reduce_index: i16,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> __state_machine::SimulatedReduce<__StateMachine<'input, 'err, '__2>>
    where
        'input: 'err,
    {
        match __reduce_index {
            0 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 0,
                }
            }
            1 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 0,
                }
            }
            2 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 1,
                }
            }
            3 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 2,
                }
            }
            4 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 2,
                }
            }
            5 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 3,
                }
            }
            6 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 3,
                }
            }
            7 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 4,
                }
            }
            8 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 5,
                }
            }
            9 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            10 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 6,
                }
            }
            11 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 6,
                }
            }
            12 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 7,
                }
            }
            13 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 8,
                }
            }
            14 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 8,
                }
            }
            15 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 9,
                }
            }
            16 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 10,
                }
            }
            17 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 10,
                }
            }
            18 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 11,
                }
            }
            19 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 11,
                }
            }
            20 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 12,
                }
            }
            21 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 13,
                }
            }
            22 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 13,
                }
            }
            23 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 14,
                }
            }
            24 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 15,
                }
            }
            25 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
                }
            }
            26 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 16,
                }
            }
            27 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 16,
                }
            }
            28 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 17,
                }
            }
            29 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 18,
                }
            }
            30 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 19,
                }
            }
            31 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 19,
                }
            }
            32 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 20,
                }
            }
            33 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 20,
                }
            }
            34 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 21,
                }
            }
            35 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 21,
                }
            }
            36 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 22,
                }
            }
            37 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 22,
                }
            }
            38 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 23,
                }
            }
            39 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
                }
            }
            40 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 24,
                }
            }
            41 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 24,
                }
            }
            42 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 25,
                }
            }
            43 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 25,
                }
            }
            44 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 25,
                }
            }
            45 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 25,
                }
            }
            46 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 26,
                }
            }
            47 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 27,
                }
            }
            48 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 27,
                }
            }
            49 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 28,
                }
            }
            50 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 28,
                }
            }
            51 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 28,
                }
            }
            52 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 29,
                }
            }
            53 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 30,
                }
            }
            54 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 30,
                }
            }
            55 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 30,
                }
            }
            56 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 30,
                }
            }
            57 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 31,
                }
            }
            58 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 31,
                }
            }
            59 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 31,
                }
            }
            60 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 32,
                }
            }
            61 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 33,
                }
            }
            62 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 34,
                }
            }
            63 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 35,
                }
            }
            64 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 36,
                }
            }
            65 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 37,
                }
            }
            66 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 38,
                }
            }
            67 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 39,
                }
            }
            68 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 40,
                }
            }
            69 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 40,
                }
            }
            70 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 40,
                }
            }
            71 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 40,
                }
            }
            72 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 41,
                }
            }
            73 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 41,
                }
            }
            74 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 42,
                }
            }
            75 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 42,
                }
            }
            76 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 43,
                }
            }
            77 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 44,
                }
            }
            78 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 44,
                }
            }
            79 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 45,
                }
            }
            80 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 45,
                }
            }
            81 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 46,
                }
            }
            82 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 46,
                }
            }
            83 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 46,
                }
            }
            84 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 47,
                }
            }
            85 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 47,
                }
            }
            86 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 48,
                }
            }
            87 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 48,
                }
            }
            88 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 48,
                }
            }
            89 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 48,
                }
            }
            90 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 49,
                }
            }
            91 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 49,
                }
            }
            92 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 50,
                }
            }
            93 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 51,
                }
            }
            94 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 51,
                }
            }
            95 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 51,
                }
            }
            96 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 51,
                }
            }
            97 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 51,
                }
            }
            98 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 51,
                }
            }
            99 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 51,
                }
            }
            100 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 52,
                }
            }
            101 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 53,
                }
            }
            102 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 53,
                }
            }
            103 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 54,
                }
            }
            104 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 55,
                }
            }
            105 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 55,
                }
            }
            106 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 55,
                }
            }
            107 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 56,
                }
            }
            108 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 56,
                }
            }
            109 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 57,
                }
            }
            110 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 58,
                }
            }
            111 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 59,
                }
            }
            112 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 60,
                }
            }
            113 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 61,
                }
            }
            114 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 62,
                }
            }
            115 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 63,
                }
            }
            116 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 64,
                }
            }
            117 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 65,
                }
            }
            118 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 65,
                }
            }
            119 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 8,
                    nonterminal_produced: 66,
                }
            }
            120 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 66,
                }
            }
            121 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 7,
                    nonterminal_produced: 66,
                }
            }
            122 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 66,
                }
            }
            123 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 67,
                }
            }
            124 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 67,
                }
            }
            125 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 67,
                }
            }
            126 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 67,
                }
            }
            127 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 67,
                }
            }
            128 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 68,
                }
            }
            129 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 68,
                }
            }
            130 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 69,
                }
            }
            131 => __state_machine::SimulatedReduce::Accept,
            _ => panic!("invalid reduction index {}", __reduce_index)
        }
    }
    pub struct ProgramParser {
        _priv: (),
    }

    impl ProgramParser {
        pub fn new() -> ProgramParser {
            ProgramParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
            'err,
            __TOKEN: __ToTriple<'input, 'err, >,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            input: &'input str,
            intr: &mut StringInterner<Sym>,
            errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
            __tokens0: __TOKENS,
        ) -> Result<Vec<ProgramPart>, __lalrpop_util::ParseError<usize, Token<'input>, ParserError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            __state_machine::Parser::drive(
                __StateMachine {
                    input,
                    intr,
                    errs,
                    __phantom: ::std::marker::PhantomData::<(&(), &())>,
                },
                __tokens,
            )
        }
    }
    fn __accepts<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __error_state: i16,
        __states: & [i16],
        __opt_integer: Option<usize>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> bool
    {
        let mut __states = __states.to_vec();
        __states.push(__error_state);
        loop {
            let mut __states_len = __states.len();
            let __top = __states[__states_len - 1];
            let __action = match __opt_integer {
                None => __EOF_ACTION[__top as usize],
                Some(__integer) => __action(__top, __integer),
            };
            if __action == 0 { return false; }
            if __action > 0 { return true; }
            let (__to_pop, __nt) = match __simulate_reduce(-(__action + 1), ::std::marker::PhantomData::<(&(), &())>) {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop, nonterminal_produced
                } => (states_to_pop, nonterminal_produced),
                __state_machine::SimulatedReduce::Accept => return true,
            };
            __states_len -= __to_pop;
            __states.truncate(__states_len);
            let __top = __states[__states_len - 1];
            let __next_state = __goto(__top, __nt);
            __states.push(__next_state);
        }
    }
    pub(crate) fn __reduce<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __action: i16,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i16>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> Option<Result<Vec<ProgramPart>,__lalrpop_util::ParseError<usize, Token<'input>, ParserError>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            1 => {
                __reduce1(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            2 => {
                __reduce2(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            3 => {
                __reduce3(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            4 => {
                __reduce4(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            5 => {
                __reduce5(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            6 => {
                __reduce6(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            7 => {
                __reduce7(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            8 => {
                __reduce8(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            9 => {
                __reduce9(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            10 => {
                __reduce10(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            11 => {
                __reduce11(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            12 => {
                __reduce12(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            13 => {
                __reduce13(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            14 => {
                __reduce14(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            15 => {
                __reduce15(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            16 => {
                __reduce16(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            17 => {
                __reduce17(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            18 => {
                __reduce18(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            19 => {
                __reduce19(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            20 => {
                __reduce20(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            21 => {
                __reduce21(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            22 => {
                __reduce22(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            23 => {
                __reduce23(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            24 => {
                __reduce24(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            25 => {
                __reduce25(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            26 => {
                __reduce26(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            27 => {
                __reduce27(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            28 => {
                __reduce28(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            29 => {
                __reduce29(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            30 => {
                __reduce30(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            31 => {
                __reduce31(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            32 => {
                __reduce32(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            33 => {
                __reduce33(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            34 => {
                __reduce34(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            35 => {
                __reduce35(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            36 => {
                __reduce36(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            37 => {
                __reduce37(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            38 => {
                __reduce38(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            39 => {
                __reduce39(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            40 => {
                __reduce40(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            41 => {
                __reduce41(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            42 => {
                __reduce42(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            43 => {
                __reduce43(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            44 => {
                __reduce44(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            45 => {
                __reduce45(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            46 => {
                __reduce46(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            47 => {
                __reduce47(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            48 => {
                __reduce48(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            49 => {
                __reduce49(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            50 => {
                __reduce50(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            51 => {
                __reduce51(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            52 => {
                __reduce52(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            53 => {
                __reduce53(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            54 => {
                __reduce54(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            55 => {
                __reduce55(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            56 => {
                __reduce56(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            57 => {
                __reduce57(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            58 => {
                __reduce58(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            59 => {
                __reduce59(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            60 => {
                __reduce60(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            61 => {
                __reduce61(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            62 => {
                __reduce62(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            63 => {
                __reduce63(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            64 => {
                __reduce64(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            65 => {
                __reduce65(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            66 => {
                __reduce66(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            67 => {
                __reduce67(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            68 => {
                __reduce68(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            69 => {
                __reduce69(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            70 => {
                __reduce70(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            71 => {
                __reduce71(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            72 => {
                __reduce72(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            73 => {
                __reduce73(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            74 => {
                __reduce74(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            75 => {
                __reduce75(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            76 => {
                __reduce76(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            77 => {
                __reduce77(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            78 => {
                __reduce78(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            79 => {
                __reduce79(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            80 => {
                __reduce80(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            81 => {
                __reduce81(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            82 => {
                __reduce82(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            83 => {
                __reduce83(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            84 => {
                __reduce84(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            85 => {
                __reduce85(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            86 => {
                __reduce86(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            87 => {
                __reduce87(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            88 => {
                __reduce88(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            89 => {
                __reduce89(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            90 => {
                __reduce90(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            91 => {
                __reduce91(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            92 => {
                __reduce92(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            93 => {
                __reduce93(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            94 => {
                __reduce94(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            95 => {
                __reduce95(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            96 => {
                __reduce96(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            97 => {
                __reduce97(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            98 => {
                __reduce98(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            99 => {
                __reduce99(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            100 => {
                __reduce100(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            101 => {
                __reduce101(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            102 => {
                __reduce102(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            103 => {
                __reduce103(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            104 => {
                __reduce104(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            105 => {
                __reduce105(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            106 => {
                __reduce106(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            107 => {
                __reduce107(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            108 => {
                __reduce108(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            109 => {
                __reduce109(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            110 => {
                __reduce110(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            111 => {
                __reduce111(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            112 => {
                __reduce112(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            113 => {
                __reduce113(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            114 => {
                __reduce114(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            115 => {
                __reduce115(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            116 => {
                __reduce116(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            117 => {
                __reduce117(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            118 => {
                __reduce118(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            119 => {
                __reduce119(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            120 => {
                __reduce120(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            121 => {
                __reduce121(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            122 => {
                __reduce122(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            123 => {
                __reduce123(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            124 => {
                __reduce124(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            125 => {
                __reduce125(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            126 => {
                __reduce126(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            127 => {
                __reduce127(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            128 => {
                __reduce128(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            129 => {
                __reduce129(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            130 => {
                __reduce130(input, intr, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            131 => {
                // __Program = Program => ActionFn(0);
                let __sym0 = __pop_Variant25(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, intr, errs, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant12<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (usize, usize), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant12(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BinOp, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant23(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ComplexType, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant4(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expr, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant6(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Identifier, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant10(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant18<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, LitKind, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant18(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant14<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, MemberVariable, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant14(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant24<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Path, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant24(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant26<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ProgramPart, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant26(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Spanned<BinOp>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant29(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant28<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Spanned<UnOpKind>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant28(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant30<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Stmt, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant30(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant31<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, StructDefinition, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant31(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Token<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant0(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Type, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant8(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, UnOpKind, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant22(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant19<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<ComplexType>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant19(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant20<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Expr>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant20(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant17<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<MemberVariable>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant17(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant25<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<ProgramPart>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant25(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, __lalrpop_util::ErrorRecovery<usize, Token<'input>, ParserError>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant2(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant16<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant16(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant13<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<(usize, usize)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant13(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant21<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<MemberVariable>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant21(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Token<'input>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Type>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant9(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<ComplexType>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Expr>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant7(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant11<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Identifier>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant11(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant15<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<MemberVariable>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant15(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant27<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<ProgramPart>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant27(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant1(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // "builtin"? = "builtin" => ActionFn(90);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action90::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 0)
    }
    pub(crate) fn __reduce1<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // "builtin"? =  => ActionFn(91);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action91::<>(input, intr, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 0)
    }
    pub(crate) fn __reduce2<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ("+" <ComplexType1>) = "+", ComplexType1 => ActionFn(115);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action115::<>(input, intr, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 1)
    }
    pub(crate) fn __reduce3<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ("+" <ComplexType1>)* =  => ActionFn(113);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action113::<>(input, intr, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (0, 2)
    }
    pub(crate) fn __reduce4<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ("+" <ComplexType1>)* = ("+" <ComplexType1>)+ => ActionFn(114);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action114::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 2)
    }
    pub(crate) fn __reduce5<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ("+" <ComplexType1>)+ = "+", ComplexType1 => ActionFn(124);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action124::<>(input, intr, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce6<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ("+" <ComplexType1>)+ = ("+" <ComplexType1>)+, "+", ComplexType1 => ActionFn(125);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action125::<>(input, intr, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 3)
    }
    pub(crate) fn __reduce7<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ("," <Expr>) = ",", Expr => ActionFn(112);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action112::<>(input, intr, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 4)
    }
    pub(crate) fn __reduce8<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ("," <Expr>)* =  => ActionFn(110);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action110::<>(input, intr, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 5)
    }
    pub(crate) fn __reduce9<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ("," <Expr>)* = ("," <Expr>)+ => ActionFn(111);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action111::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce10<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ("," <Expr>)+ = ",", Expr => ActionFn(128);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action128::<>(input, intr, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 6)
    }
    pub(crate) fn __reduce11<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ("," <Expr>)+ = ("," <Expr>)+, ",", Expr => ActionFn(129);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action129::<>(input, intr, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 6)
    }
    pub(crate) fn __reduce12<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (":" <Type>) = ":", Type => ActionFn(89);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action89::<>(input, intr, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 7)
    }
    pub(crate) fn __reduce13<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (":" <Type>)? = ":", Type => ActionFn(132);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action132::<>(input, intr, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 8)
    }
    pub(crate) fn __reduce14<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (":" <Type>)? =  => ActionFn(88);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action88::<>(input, intr, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (0, 8)
    }
    pub(crate) fn __reduce15<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ("::" <Identifier>) = "::", Identifier => ActionFn(85);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action85::<>(input, intr, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (2, 9)
    }
    pub(crate) fn __reduce16<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ("::" <Identifier>)* =  => ActionFn(83);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action83::<>(input, intr, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (0, 10)
    }
    pub(crate) fn __reduce17<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ("::" <Identifier>)* = ("::" <Identifier>)+ => ActionFn(84);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action84::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce18<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ("::" <Identifier>)+ = "::", Identifier => ActionFn(137);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action137::<>(input, intr, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (2, 11)
    }
    pub(crate) fn __reduce19<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ("::" <Identifier>)+ = ("::" <Identifier>)+, "::", Identifier => ActionFn(138);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action138::<>(input, intr, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (3, 11)
    }
    pub(crate) fn __reduce20<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<@L> ";" <@R>) = ";" => ActionFn(177);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action177::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce21<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<@L> ";" <@R>)? = ";" => ActionFn(213);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action213::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce22<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<@L> ";" <@R>)? =  => ActionFn(81);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action81::<>(input, intr, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (0, 13)
    }
    pub(crate) fn __reduce23<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<MemberVariable> ",") = MemberVariable, "," => ActionFn(100);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action100::<>(input, intr, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (2, 14)
    }
    pub(crate) fn __reduce24<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<MemberVariable> ",")* =  => ActionFn(98);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action98::<>(input, intr, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (0, 15)
    }
    pub(crate) fn __reduce25<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<MemberVariable> ",")* = (<MemberVariable> ",")+ => ActionFn(99);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action99::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce26<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<MemberVariable> ",")+ = MemberVariable, "," => ActionFn(216);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action216::<>(input, intr, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (2, 16)
    }
    pub(crate) fn __reduce27<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<MemberVariable> ",")+ = (<MemberVariable> ",")+, MemberVariable, "," => ActionFn(217);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant14(__symbols);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action217::<>(input, intr, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce28<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // @L =  => ActionFn(93);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action93::<>(input, intr, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (0, 17)
    }
    pub(crate) fn __reduce29<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // @R =  => ActionFn(92);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action92::<>(input, intr, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (0, 18)
    }
    pub(crate) fn __reduce30<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // BinOpExpr<Op4, Expr3> = BinOpExpr<Op4, Expr3>, Spanned<Op4>, Expr3 => ActionFn(178);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant29(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action178::<>(input, intr, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 19)
    }
    pub(crate) fn __reduce31<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // BinOpExpr<Op4, Expr3> = Expr3 => ActionFn(68);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action68::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce32<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // BinOpExpr<Op5, Expr4> = BinOpExpr<Op5, Expr4>, Spanned<Op5>, Expr4 => ActionFn(179);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant29(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action179::<>(input, intr, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 20)
    }
    pub(crate) fn __reduce33<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // BinOpExpr<Op5, Expr4> = Expr4 => ActionFn(70);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action70::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce34<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // BinOpExpr<Op6, Expr5> = BinOpExpr<Op6, Expr5>, Spanned<Op6>, Expr5 => ActionFn(180);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant29(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action180::<>(input, intr, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 21)
    }
    pub(crate) fn __reduce35<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // BinOpExpr<Op6, Expr5> = Expr5 => ActionFn(72);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action72::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce36<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // BinOpExpr<Op7, Expr6> = BinOpExpr<Op7, Expr6>, Spanned<Op7>, Expr6 => ActionFn(181);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant29(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action181::<>(input, intr, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 22)
    }
    pub(crate) fn __reduce37<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // BinOpExpr<Op7, Expr6> = Expr6 => ActionFn(74);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action74::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 22)
    }
    pub(crate) fn __reduce38<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // BinOpExpr<OpAnd, Expr7> = BinOpExpr<OpAnd, Expr7>, Spanned<OpAnd>, Expr7 => ActionFn(182);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant29(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action182::<>(input, intr, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 23)
    }
    pub(crate) fn __reduce39<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // BinOpExpr<OpAnd, Expr7> = Expr7 => ActionFn(76);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action76::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 23)
    }
    pub(crate) fn __reduce40<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // BinOpExpr<OpOr, Expr8> = BinOpExpr<OpOr, Expr8>, Spanned<OpOr>, Expr8 => ActionFn(183);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant29(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action183::<>(input, intr, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 24)
    }
    pub(crate) fn __reduce41<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // BinOpExpr<OpOr, Expr8> = Expr8 => ActionFn(78);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action78::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce42<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Comma<MemberVariable> = MemberVariable => ActionFn(220);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action220::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce43<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Comma<MemberVariable> =  => ActionFn(221);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action221::<>(input, intr, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (0, 25)
    }
    pub(crate) fn __reduce44<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Comma<MemberVariable> = (<MemberVariable> ",")+, MemberVariable => ActionFn(222);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant14(__symbols);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action222::<>(input, intr, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (2, 25)
    }
    pub(crate) fn __reduce45<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Comma<MemberVariable> = (<MemberVariable> ",")+ => ActionFn(223);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action223::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce46<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ComplexType = Many1<"+", ComplexType1> => ActionFn(184);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action184::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 26)
    }
    pub(crate) fn __reduce47<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ComplexType0 = Path => ActionFn(185);
        let __sym0 = __pop_Variant24(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action185::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce48<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ComplexType0 = "(", ComplexType, ")" => ActionFn(186);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action186::<>(input, intr, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 27)
    }
    pub(crate) fn __reduce49<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ComplexType1 = "^", ComplexType0 => ActionFn(187);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action187::<>(input, intr, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 28)
    }
    pub(crate) fn __reduce50<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ComplexType1 = "!", ComplexType0 => ActionFn(188);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action188::<>(input, intr, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 28)
    }
    pub(crate) fn __reduce51<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ComplexType1 = ComplexType0 => ActionFn(60);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action60::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 28)
    }
    pub(crate) fn __reduce52<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr = Expr10 => ActionFn(11);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action11::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 29)
    }
    pub(crate) fn __reduce53<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr0 = Lit => ActionFn(189);
        let __sym0 = __pop_Variant18(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action189::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 30)
    }
    pub(crate) fn __reduce54<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr0 = Identifier => ActionFn(190);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action190::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 30)
    }
    pub(crate) fn __reduce55<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr0 = "(", Many1<",", Expr>, ")" => ActionFn(191);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant20(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action191::<>(input, intr, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 30)
    }
    pub(crate) fn __reduce56<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr0 = error => ActionFn(192);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action192::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 30)
    }
    pub(crate) fn __reduce57<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr10 = Expr9, "=", Expr10 => ActionFn(193);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action193::<>(input, intr, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 31)
    }
    pub(crate) fn __reduce58<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr10 = Expr9, Spanned<OpAssign>, Expr10 => ActionFn(194);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant29(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action194::<>(input, intr, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 31)
    }
    pub(crate) fn __reduce59<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr10 = Expr9 => ActionFn(14);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action14::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 31)
    }
    pub(crate) fn __reduce60<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr3 = UnOpExpr<Op3, Expr0> => ActionFn(41);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action41::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 32)
    }
    pub(crate) fn __reduce61<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr4 = BinOpExpr<Op4, Expr3> => ActionFn(37);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action37::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 33)
    }
    pub(crate) fn __reduce62<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr5 = BinOpExpr<Op5, Expr4> => ActionFn(34);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action34::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 34)
    }
    pub(crate) fn __reduce63<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr6 = BinOpExpr<Op6, Expr5> => ActionFn(29);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action29::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 35)
    }
    pub(crate) fn __reduce64<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr7 = BinOpExpr<Op7, Expr6> => ActionFn(26);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 36)
    }
    pub(crate) fn __reduce65<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr8 = BinOpExpr<OpAnd, Expr7> => ActionFn(24);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action24::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 37)
    }
    pub(crate) fn __reduce66<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr9 = BinOpExpr<OpOr, Expr8> => ActionFn(22);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 38)
    }
    pub(crate) fn __reduce67<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Identifier = TokenIdentifier => ActionFn(195);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action195::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 39)
    }
    pub(crate) fn __reduce68<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Lit = "true" => ActionFn(48);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action48::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 40)
    }
    pub(crate) fn __reduce69<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Lit = "false" => ActionFn(49);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action49::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 40)
    }
    pub(crate) fn __reduce70<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Lit = TokenString => ActionFn(50);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action50::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 40)
    }
    pub(crate) fn __reduce71<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Lit = TokenInt => ActionFn(196);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action196::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 40)
    }
    pub(crate) fn __reduce72<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Many1<"+", ComplexType1> = ComplexType1 => ActionFn(126);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action126::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 41)
    }
    pub(crate) fn __reduce73<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Many1<"+", ComplexType1> = ComplexType1, ("+" <ComplexType1>)+ => ActionFn(127);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action127::<>(input, intr, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (2, 41)
    }
    pub(crate) fn __reduce74<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Many1<",", Expr> = Expr => ActionFn(130);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action130::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 42)
    }
    pub(crate) fn __reduce75<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Many1<",", Expr> = Expr, ("," <Expr>)+ => ActionFn(131);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action131::<>(input, intr, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (2, 42)
    }
    pub(crate) fn __reduce76<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // MemberVariable = Identifier, ":", Type => ActionFn(7);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action7::<>(input, intr, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (3, 43)
    }
    pub(crate) fn __reduce77<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // MemberVariable? = MemberVariable => ActionFn(96);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action96::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 44)
    }
    pub(crate) fn __reduce78<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // MemberVariable? =  => ActionFn(97);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action97::<>(input, intr, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (0, 44)
    }
    pub(crate) fn __reduce79<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op3 = "-" => ActionFn(42);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action42::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 45)
    }
    pub(crate) fn __reduce80<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op3 = "!" => ActionFn(43);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action43::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 45)
    }
    pub(crate) fn __reduce81<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op4 = "*" => ActionFn(38);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action38::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 46)
    }
    pub(crate) fn __reduce82<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op4 = "/" => ActionFn(39);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action39::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 46)
    }
    pub(crate) fn __reduce83<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op4 = "%" => ActionFn(40);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action40::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 46)
    }
    pub(crate) fn __reduce84<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op5 = "+" => ActionFn(35);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action35::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 47)
    }
    pub(crate) fn __reduce85<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op5 = "-" => ActionFn(36);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action36::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 47)
    }
    pub(crate) fn __reduce86<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op6 = "<" => ActionFn(30);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action30::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 48)
    }
    pub(crate) fn __reduce87<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op6 = ">" => ActionFn(31);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action31::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 48)
    }
    pub(crate) fn __reduce88<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op6 = "<=" => ActionFn(32);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action32::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 48)
    }
    pub(crate) fn __reduce89<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op6 = ">=" => ActionFn(33);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action33::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 48)
    }
    pub(crate) fn __reduce90<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op7 = "==" => ActionFn(27);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 49)
    }
    pub(crate) fn __reduce91<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op7 = "!=" => ActionFn(28);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action28::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 49)
    }
    pub(crate) fn __reduce92<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // OpAnd = "&" => ActionFn(25);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action25::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 50)
    }
    pub(crate) fn __reduce93<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // OpAssign = "+=" => ActionFn(15);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action15::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 51)
    }
    pub(crate) fn __reduce94<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // OpAssign = "-=" => ActionFn(16);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action16::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 51)
    }
    pub(crate) fn __reduce95<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // OpAssign = "*=" => ActionFn(17);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 51)
    }
    pub(crate) fn __reduce96<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // OpAssign = "/=" => ActionFn(18);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action18::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 51)
    }
    pub(crate) fn __reduce97<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // OpAssign = "%=" => ActionFn(19);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 51)
    }
    pub(crate) fn __reduce98<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // OpAssign = "&=" => ActionFn(20);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action20::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 51)
    }
    pub(crate) fn __reduce99<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // OpAssign = "|=" => ActionFn(21);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action21::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 51)
    }
    pub(crate) fn __reduce100<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // OpOr = "|" => ActionFn(23);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action23::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 52)
    }
    pub(crate) fn __reduce101<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Path = Identifier => ActionFn(139);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action139::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant24(__nt), __end));
        (1, 53)
    }
    pub(crate) fn __reduce102<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Path = Identifier, ("::" <Identifier>)+ => ActionFn(140);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action140::<>(input, intr, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant24(__nt), __end));
        (2, 53)
    }
    pub(crate) fn __reduce103<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Program = ProgramPart+ => ActionFn(2);
        let __sym0 = __pop_Variant27(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant25(__nt), __end));
        (1, 54)
    }
    pub(crate) fn __reduce104<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ProgramPart = "mod", Identifier, "{", Program, "}" => ActionFn(3);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant25(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action3::<>(input, intr, errs, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant26(__nt), __end));
        (5, 55)
    }
    pub(crate) fn __reduce105<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ProgramPart = StructDefinition => ActionFn(4);
        let __sym0 = __pop_Variant31(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant26(__nt), __end));
        (1, 55)
    }
    pub(crate) fn __reduce106<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ProgramPart = error => ActionFn(197);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action197::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant26(__nt), __end));
        (1, 55)
    }
    pub(crate) fn __reduce107<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ProgramPart+ = ProgramPart => ActionFn(94);
        let __sym0 = __pop_Variant26(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action94::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant27(__nt), __end));
        (1, 56)
    }
    pub(crate) fn __reduce108<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ProgramPart+ = ProgramPart+, ProgramPart => ActionFn(95);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant26(__symbols);
        let __sym0 = __pop_Variant27(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action95::<>(input, intr, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant27(__nt), __end));
        (2, 56)
    }
    pub(crate) fn __reduce109<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Spanned<Op3> = Op3 => ActionFn(198);
        let __sym0 = __pop_Variant22(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action198::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant28(__nt), __end));
        (1, 57)
    }
    pub(crate) fn __reduce110<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Spanned<Op4> = Op4 => ActionFn(199);
        let __sym0 = __pop_Variant23(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action199::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant29(__nt), __end));
        (1, 58)
    }
    pub(crate) fn __reduce111<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Spanned<Op5> = Op5 => ActionFn(200);
        let __sym0 = __pop_Variant23(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action200::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant29(__nt), __end));
        (1, 59)
    }
    pub(crate) fn __reduce112<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Spanned<Op6> = Op6 => ActionFn(201);
        let __sym0 = __pop_Variant23(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action201::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant29(__nt), __end));
        (1, 60)
    }
    pub(crate) fn __reduce113<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Spanned<Op7> = Op7 => ActionFn(202);
        let __sym0 = __pop_Variant23(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action202::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant29(__nt), __end));
        (1, 61)
    }
    pub(crate) fn __reduce114<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Spanned<OpAnd> = OpAnd => ActionFn(203);
        let __sym0 = __pop_Variant23(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action203::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant29(__nt), __end));
        (1, 62)
    }
    pub(crate) fn __reduce115<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Spanned<OpAssign> = OpAssign => ActionFn(204);
        let __sym0 = __pop_Variant23(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action204::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant29(__nt), __end));
        (1, 63)
    }
    pub(crate) fn __reduce116<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Spanned<OpOr> = OpOr => ActionFn(205);
        let __sym0 = __pop_Variant23(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action205::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant29(__nt), __end));
        (1, 64)
    }
    pub(crate) fn __reduce117<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Stmt = Expr, ";" => ActionFn(214);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action214::<>(input, intr, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant30(__nt), __end));
        (2, 65)
    }
    pub(crate) fn __reduce118<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Stmt = Expr => ActionFn(215);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action215::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant30(__nt), __end));
        (1, 65)
    }
    pub(crate) fn __reduce119<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // StructDefinition = "builtin", "struct", Identifier, ":", Type, "{", Comma<MemberVariable>, "}" => ActionFn(133);
        assert!(__symbols.len() >= 8);
        let __sym7 = __pop_Variant0(__symbols);
        let __sym6 = __pop_Variant17(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant8(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym7.2.clone();
        let __nt = super::__action133::<>(input, intr, errs, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7);
        __symbols.push((__start, __Symbol::Variant31(__nt), __end));
        (8, 66)
    }
    pub(crate) fn __reduce120<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // StructDefinition = "builtin", "struct", Identifier, "{", Comma<MemberVariable>, "}" => ActionFn(134);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant17(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action134::<>(input, intr, errs, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant31(__nt), __end));
        (6, 66)
    }
    pub(crate) fn __reduce121<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // StructDefinition = "struct", Identifier, ":", Type, "{", Comma<MemberVariable>, "}" => ActionFn(135);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant17(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant8(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action135::<>(input, intr, errs, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant31(__nt), __end));
        (7, 66)
    }
    pub(crate) fn __reduce122<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // StructDefinition = "struct", Identifier, "{", Comma<MemberVariable>, "}" => ActionFn(136);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant17(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action136::<>(input, intr, errs, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant31(__nt), __end));
        (5, 66)
    }
    pub(crate) fn __reduce123<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Type = "bool" => ActionFn(207);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action207::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 67)
    }
    pub(crate) fn __reduce124<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Type = "string" => ActionFn(208);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action208::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 67)
    }
    pub(crate) fn __reduce125<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Type = "int" => ActionFn(209);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action209::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 67)
    }
    pub(crate) fn __reduce126<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Type = "(", ")" => ActionFn(210);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action210::<>(input, intr, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 67)
    }
    pub(crate) fn __reduce127<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Type = ComplexType => ActionFn(211);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action211::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 67)
    }
    pub(crate) fn __reduce128<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // UnOpExpr<Op3, Expr0> = Spanned<Op3>, UnOpExpr<Op3, Expr0> => ActionFn(212);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant28(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action212::<>(input, intr, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 68)
    }
    pub(crate) fn __reduce129<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // UnOpExpr<Op3, Expr0> = Expr0 => ActionFn(66);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action66::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 68)
    }
    pub(crate) fn __reduce130<
        'input,
        'err,
    >(
        input: &'input str,
        intr: &mut StringInterner<Sym>,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // __Expr = Expr => ActionFn(1);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(input, intr, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 69)
    }
}
pub use self::__parse__Program::ProgramParser;

#[allow(unused_variables)]
fn __action0<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Vec<ProgramPart>, usize),
) -> Vec<ProgramPart>
{
    __0
}

#[allow(unused_variables)]
fn __action1<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(unused_variables)]
fn __action2<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, ::std::vec::Vec<ProgramPart>, usize),
) -> Vec<ProgramPart>
{
    __0
}

#[allow(unused_variables)]
fn __action3<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, _, _): (usize, Token<'input>, usize),
    (_, name, _): (usize, Identifier, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, p, _): (usize, Vec<ProgramPart>, usize),
    (_, _, _): (usize, Token<'input>, usize),
) -> ProgramPart
{
    ProgramPart::Module(name, p)
}

#[allow(unused_variables)]
fn __action4<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, StructDefinition, usize),
) -> ProgramPart
{
    ProgramPart::StructDefinition(__0)
}

#[allow(unused_variables)]
fn __action5<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, l, _): (usize, usize, usize),
    (_, err, _): (usize, __lalrpop_util::ErrorRecovery<usize, Token<'input>, ParserError>, usize),
    (_, r, _): (usize, usize, usize),
) -> ProgramPart
{
    { errs.push(err); ProgramPart::Err }
}

#[allow(unused_variables)]
fn __action6<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, b, _): (usize, ::std::option::Option<Token<'input>>, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, name, _): (usize, Identifier, usize),
    (_, bounds, _): (usize, ::std::option::Option<Type>, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, members, _): (usize, Vec<MemberVariable>, usize),
    (_, _, _): (usize, Token<'input>, usize),
) -> StructDefinition
{
    StructDefinition{name, builtin: b.is_some(), bounds, members}
}

#[allow(unused_variables)]
fn __action7<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, name, _): (usize, Identifier, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, ty, _): (usize, Type, usize),
) -> MemberVariable
{
    MemberVariable{name, ty}
}

#[allow(unused_variables)]
fn __action8<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, l, _): (usize, usize, usize),
    (_, n, _): (usize, &'input str, usize),
    (_, r, _): (usize, usize, usize),
) -> Identifier
{
    Identifier::new(l, r, intr.get_or_intern(n))
}

#[allow(unused_variables)]
fn __action9<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, one, _): (usize, Identifier, usize),
    (_, v, _): (usize, ::std::vec::Vec<Identifier>, usize),
) -> Path
{
    {
        let mut v = v;
        v.insert(0, one);

        Path {
            items: v
        }
    }
}

#[allow(unused_variables)]
fn __action10<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, l, _): (usize, usize, usize),
    (_, e, _): (usize, Expr, usize),
    (_, semi, _): (usize, ::std::option::Option<(usize, usize)>, usize),
    (_, r, _): (usize, usize, usize),
) -> Stmt
{
    {
        let val = if let Some((l, r)) = semi {
            StmtKind::Semi(Box::new(e), Span{l,r})
        } else {
            StmtKind::Expr(Box::new(e))
        };

        Stmt::new(l, r, val)
    }
}

#[allow(unused_variables)]
fn __action11<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(unused_variables)]
fn __action12<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, lo, _): (usize, usize, usize),
    (_, lhs, _): (usize, Expr, usize),
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, r, _): (usize, usize, usize),
    (_, rhs, _): (usize, Expr, usize),
    (_, ro, _): (usize, usize, usize),
) -> Expr
{
    Expr::new(lo, ro, ExprKind::Assign(Span{l,r}, bx(lhs), bx(rhs)))
}

#[allow(unused_variables)]
fn __action13<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, l, _): (usize, usize, usize),
    (_, lhs, _): (usize, Expr, usize),
    (_, op, _): (usize, Spanned<BinOp>, usize),
    (_, rhs, _): (usize, Expr, usize),
    (_, r, _): (usize, usize, usize),
) -> Expr
{
    Expr::new(l, r, ExprKind::AssignOp(op, bx(lhs), bx(rhs)))
}

#[allow(unused_variables)]
fn __action14<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(unused_variables)]
fn __action15<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> BinOp
{
    BinOp::Add
}

#[allow(unused_variables)]
fn __action16<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> BinOp
{
    BinOp::Sub
}

#[allow(unused_variables)]
fn __action17<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> BinOp
{
    BinOp::Mul
}

#[allow(unused_variables)]
fn __action18<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> BinOp
{
    BinOp::Div
}

#[allow(unused_variables)]
fn __action19<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> BinOp
{
    BinOp::Rem
}

#[allow(unused_variables)]
fn __action20<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> BinOp
{
    BinOp::Rem
}

#[allow(unused_variables)]
fn __action21<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> BinOp
{
    BinOp::Rem
}

#[allow(unused_variables)]
fn __action22<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(unused_variables)]
fn __action23<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> BinOp
{
    BinOp::Or
}

#[allow(unused_variables)]
fn __action24<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(unused_variables)]
fn __action25<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> BinOp
{
    BinOp::And
}

#[allow(unused_variables)]
fn __action26<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(unused_variables)]
fn __action27<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> BinOp
{
    BinOp::Eq
}

#[allow(unused_variables)]
fn __action28<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> BinOp
{
    BinOp::Ne
}

#[allow(unused_variables)]
fn __action29<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(unused_variables)]
fn __action30<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> BinOp
{
    BinOp::Lt
}

#[allow(unused_variables)]
fn __action31<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> BinOp
{
    BinOp::Gt
}

#[allow(unused_variables)]
fn __action32<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> BinOp
{
    BinOp::Le
}

#[allow(unused_variables)]
fn __action33<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> BinOp
{
    BinOp::Ge
}

#[allow(unused_variables)]
fn __action34<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(unused_variables)]
fn __action35<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> BinOp
{
    BinOp::Add
}

#[allow(unused_variables)]
fn __action36<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> BinOp
{
    BinOp::Sub
}

#[allow(unused_variables)]
fn __action37<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(unused_variables)]
fn __action38<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> BinOp
{
    BinOp::Mul
}

#[allow(unused_variables)]
fn __action39<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> BinOp
{
    BinOp::Div
}

#[allow(unused_variables)]
fn __action40<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> BinOp
{
    BinOp::Rem
}

#[allow(unused_variables)]
fn __action41<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(unused_variables)]
fn __action42<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> UnOpKind
{
    UnOpKind::Neg
}

#[allow(unused_variables)]
fn __action43<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> UnOpKind
{
    UnOpKind::Not
}

#[allow(unused_variables)]
fn __action44<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, l, _): (usize, usize, usize),
    (_, lit, _): (usize, LitKind, usize),
    (_, r, _): (usize, usize, usize),
) -> Expr
{
    Expr::new(l, r, ExprKind::Lit(lit))
}

#[allow(unused_variables)]
fn __action45<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, l, _): (usize, usize, usize),
    (_, ident, _): (usize, Identifier, usize),
    (_, r, _): (usize, usize, usize),
) -> Expr
{
    Expr::new(l, r, ExprKind::Variable(ident))
}

#[allow(unused_variables)]
fn __action46<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, exprs, _): (usize, Vec<Expr>, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, r, _): (usize, usize, usize),
) -> Expr
{
    {
        if exprs.len() == 1 {
            let mut exprs = exprs;
            exprs.remove(0)
        } else {
            Expr::new(l, r, ExprKind::Tuple(exprs))
        }
    }
}

#[allow(unused_variables)]
fn __action47<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, l, _): (usize, usize, usize),
    (_, err, _): (usize, __lalrpop_util::ErrorRecovery<usize, Token<'input>, ParserError>, usize),
    (_, r, _): (usize, usize, usize),
) -> Expr
{
    { errs.push(err); Expr::new(l, r, ExprKind::Err) }
}

#[allow(unused_variables)]
fn __action48<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> LitKind
{
    LitKind::Bool(true)
}

#[allow(unused_variables)]
fn __action49<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> LitKind
{
    LitKind::Bool(false)
}

#[allow(unused_variables)]
fn __action50<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, s, _): (usize, &'input str, usize),
) -> LitKind
{
    LitKind::String(s.to_string())
}

#[allow(unused_variables)]
fn __action51<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, l, _): (usize, usize, usize),
    (_, tok, _): (usize, &'input str, usize),
    (_, r, _): (usize, usize, usize),
) -> LitKind
{
    {
        let s: String = tok.chars().filter(|v| *v != '_').collect();
        match i32::from_str(&s) {
            Ok(i) => LitKind::Int(i),
            Err(_) => {
                errs.push(ErrorRecovery {
                    error: ParseError::User{ 
                        error: ParserError::IntTooBig(l, r)
                    },
                    dropped_tokens: vec![(l, Token::IntLiteral(tok.clone()), r)]
                });

                LitKind::Err
            }
        }
    }
}

#[allow(unused_variables)]
fn __action52<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, r, _): (usize, usize, usize),
) -> Type
{
    Type::new(l, r, TypeKind::Bool)
}

#[allow(unused_variables)]
fn __action53<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, r, _): (usize, usize, usize),
) -> Type
{
    Type::new(l, r, TypeKind::String)
}

#[allow(unused_variables)]
fn __action54<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, r, _): (usize, usize, usize),
) -> Type
{
    Type::new(l, r, TypeKind::Int)
}

#[allow(unused_variables)]
fn __action55<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, r, _): (usize, usize, usize),
) -> Type
{
    Type::new(l, r, TypeKind::Unit)
}

#[allow(unused_variables)]
fn __action56<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, l, _): (usize, usize, usize),
    (_, ty, _): (usize, ComplexType, usize),
    (_, r, _): (usize, usize, usize),
) -> Type
{
    Type::new(l, r, TypeKind::Complex(ty))
}

#[allow(unused_variables)]
fn __action57<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, l, _): (usize, usize, usize),
    (_, ty, _): (usize, Vec<ComplexType>, usize),
    (_, r, _): (usize, usize, usize),
) -> ComplexType
{
    {
        if ty.len() == 1 {
            let mut ty = ty;
            ty.remove(0)
        } else {
            ComplexType::new(l, r, ComplexTypeKind::Compound(ty))
        }
    }
}

#[allow(unused_variables)]
fn __action58<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, ty, _): (usize, ComplexType, usize),
    (_, r, _): (usize, usize, usize),
) -> ComplexType
{
    ComplexType::new(l, r, ComplexTypeKind::Above(Box::new(ty)))
}

#[allow(unused_variables)]
fn __action59<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, ty, _): (usize, ComplexType, usize),
    (_, r, _): (usize, usize, usize),
) -> ComplexType
{
    ComplexType::new(l, r, ComplexTypeKind::Not(Box::new(ty)))
}

#[allow(unused_variables)]
fn __action60<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, ComplexType, usize),
) -> ComplexType
{
    __0
}

#[allow(unused_variables)]
fn __action61<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, l, _): (usize, usize, usize),
    (_, name, _): (usize, Path, usize),
    (_, r, _): (usize, usize, usize),
) -> ComplexType
{
    ComplexType::new(l, r, ComplexTypeKind::Base(name, ComplexReferent::Infer))
}

#[allow(unused_variables)]
fn __action62<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, ty, _): (usize, ComplexType, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, r, _): (usize, usize, usize),
) -> ComplexType
{
    ty
}

#[allow(unused_variables)]
fn __action63<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, e, _): (usize, ComplexType, usize),
    (_, v, _): (usize, ::std::vec::Vec<ComplexType>, usize),
) -> Vec<ComplexType>
{
    {
        let mut v = v;
        v.insert(0, e);
        v
    }
}

#[allow(unused_variables)]
fn __action64<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, e, _): (usize, Expr, usize),
    (_, v, _): (usize, ::std::vec::Vec<Expr>, usize),
) -> Vec<Expr>
{
    {
        let mut v = v;
        v.insert(0, e);
        v
    }
}

#[allow(unused_variables)]
fn __action65<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, l, _): (usize, usize, usize),
    (_, op, _): (usize, Spanned<UnOpKind>, usize),
    (_, rhs, _): (usize, Expr, usize),
    (_, r, _): (usize, usize, usize),
) -> Expr
{
    Expr::new(l, r, ExprKind::UnOp(op, bx(rhs)))
}

#[allow(unused_variables)]
fn __action66<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(unused_variables)]
fn __action67<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, l, _): (usize, usize, usize),
    (_, lhs, _): (usize, Expr, usize),
    (_, op, _): (usize, Spanned<BinOp>, usize),
    (_, rhs, _): (usize, Expr, usize),
    (_, r, _): (usize, usize, usize),
) -> Expr
{
    Expr::new(l, r, ExprKind::BinOp(op, bx(lhs), bx(rhs)))
}

#[allow(unused_variables)]
fn __action68<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(unused_variables)]
fn __action69<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, l, _): (usize, usize, usize),
    (_, lhs, _): (usize, Expr, usize),
    (_, op, _): (usize, Spanned<BinOp>, usize),
    (_, rhs, _): (usize, Expr, usize),
    (_, r, _): (usize, usize, usize),
) -> Expr
{
    Expr::new(l, r, ExprKind::BinOp(op, bx(lhs), bx(rhs)))
}

#[allow(unused_variables)]
fn __action70<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(unused_variables)]
fn __action71<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, l, _): (usize, usize, usize),
    (_, lhs, _): (usize, Expr, usize),
    (_, op, _): (usize, Spanned<BinOp>, usize),
    (_, rhs, _): (usize, Expr, usize),
    (_, r, _): (usize, usize, usize),
) -> Expr
{
    Expr::new(l, r, ExprKind::BinOp(op, bx(lhs), bx(rhs)))
}

#[allow(unused_variables)]
fn __action72<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(unused_variables)]
fn __action73<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, l, _): (usize, usize, usize),
    (_, lhs, _): (usize, Expr, usize),
    (_, op, _): (usize, Spanned<BinOp>, usize),
    (_, rhs, _): (usize, Expr, usize),
    (_, r, _): (usize, usize, usize),
) -> Expr
{
    Expr::new(l, r, ExprKind::BinOp(op, bx(lhs), bx(rhs)))
}

#[allow(unused_variables)]
fn __action74<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(unused_variables)]
fn __action75<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, l, _): (usize, usize, usize),
    (_, lhs, _): (usize, Expr, usize),
    (_, op, _): (usize, Spanned<BinOp>, usize),
    (_, rhs, _): (usize, Expr, usize),
    (_, r, _): (usize, usize, usize),
) -> Expr
{
    Expr::new(l, r, ExprKind::BinOp(op, bx(lhs), bx(rhs)))
}

#[allow(unused_variables)]
fn __action76<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(unused_variables)]
fn __action77<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, l, _): (usize, usize, usize),
    (_, lhs, _): (usize, Expr, usize),
    (_, op, _): (usize, Spanned<BinOp>, usize),
    (_, rhs, _): (usize, Expr, usize),
    (_, r, _): (usize, usize, usize),
) -> Expr
{
    Expr::new(l, r, ExprKind::BinOp(op, bx(lhs), bx(rhs)))
}

#[allow(unused_variables)]
fn __action78<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(unused_variables)]
fn __action79<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, l, _): (usize, usize, usize),
    (_, val, _): (usize, BinOp, usize),
    (_, r, _): (usize, usize, usize),
) -> Spanned<BinOp>
{
    Spanned::new(l, r, val)
}

#[allow(unused_variables)]
fn __action80<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, (usize, usize), usize),
) -> ::std::option::Option<(usize, usize)>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action81<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<(usize, usize)>
{
    None
}

#[allow(unused_variables)]
fn __action82<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, usize, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, __1, _): (usize, usize, usize),
) -> (usize, usize)
{
    (__0, __1)
}

#[allow(unused_variables)]
fn __action83<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Identifier>
{
    vec![]
}

#[allow(unused_variables)]
fn __action84<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, v, _): (usize, ::std::vec::Vec<Identifier>, usize),
) -> ::std::vec::Vec<Identifier>
{
    v
}

#[allow(unused_variables)]
fn __action85<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, _, _): (usize, Token<'input>, usize),
    (_, __0, _): (usize, Identifier, usize),
) -> Identifier
{
    __0
}

#[allow(unused_variables)]
fn __action86<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, v, _): (usize, ::std::vec::Vec<MemberVariable>, usize),
    (_, e, _): (usize, ::std::option::Option<MemberVariable>, usize),
) -> Vec<MemberVariable>
{
    match e {
        None => v,
        Some(e) => {
            let mut v = v;
            v.push(e);
            v
        }
    }
}

#[allow(unused_variables)]
fn __action87<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Type, usize),
) -> ::std::option::Option<Type>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action88<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<Type>
{
    None
}

#[allow(unused_variables)]
fn __action89<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, _, _): (usize, Token<'input>, usize),
    (_, __0, _): (usize, Type, usize),
) -> Type
{
    __0
}

#[allow(unused_variables)]
fn __action90<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> ::std::option::Option<Token<'input>>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action91<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<Token<'input>>
{
    None
}

#[allow(unused_variables)]
fn __action92<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    __lookbehind.clone()
}

#[allow(unused_variables)]
fn __action93<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    __lookahead.clone()
}

#[allow(unused_variables)]
fn __action94<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, ProgramPart, usize),
) -> ::std::vec::Vec<ProgramPart>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action95<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, v, _): (usize, ::std::vec::Vec<ProgramPart>, usize),
    (_, e, _): (usize, ProgramPart, usize),
) -> ::std::vec::Vec<ProgramPart>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action96<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, MemberVariable, usize),
) -> ::std::option::Option<MemberVariable>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action97<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<MemberVariable>
{
    None
}

#[allow(unused_variables)]
fn __action98<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<MemberVariable>
{
    vec![]
}

#[allow(unused_variables)]
fn __action99<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, v, _): (usize, ::std::vec::Vec<MemberVariable>, usize),
) -> ::std::vec::Vec<MemberVariable>
{
    v
}

#[allow(unused_variables)]
fn __action100<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, MemberVariable, usize),
    (_, _, _): (usize, Token<'input>, usize),
) -> MemberVariable
{
    __0
}

#[allow(unused_variables)]
fn __action101<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Identifier, usize),
) -> ::std::vec::Vec<Identifier>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action102<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, v, _): (usize, ::std::vec::Vec<Identifier>, usize),
    (_, e, _): (usize, Identifier, usize),
) -> ::std::vec::Vec<Identifier>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action103<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, l, _): (usize, usize, usize),
    (_, val, _): (usize, BinOp, usize),
    (_, r, _): (usize, usize, usize),
) -> Spanned<BinOp>
{
    Spanned::new(l, r, val)
}

#[allow(unused_variables)]
fn __action104<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, l, _): (usize, usize, usize),
    (_, val, _): (usize, BinOp, usize),
    (_, r, _): (usize, usize, usize),
) -> Spanned<BinOp>
{
    Spanned::new(l, r, val)
}

#[allow(unused_variables)]
fn __action105<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, l, _): (usize, usize, usize),
    (_, val, _): (usize, BinOp, usize),
    (_, r, _): (usize, usize, usize),
) -> Spanned<BinOp>
{
    Spanned::new(l, r, val)
}

#[allow(unused_variables)]
fn __action106<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, l, _): (usize, usize, usize),
    (_, val, _): (usize, BinOp, usize),
    (_, r, _): (usize, usize, usize),
) -> Spanned<BinOp>
{
    Spanned::new(l, r, val)
}

#[allow(unused_variables)]
fn __action107<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, l, _): (usize, usize, usize),
    (_, val, _): (usize, BinOp, usize),
    (_, r, _): (usize, usize, usize),
) -> Spanned<BinOp>
{
    Spanned::new(l, r, val)
}

#[allow(unused_variables)]
fn __action108<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, l, _): (usize, usize, usize),
    (_, val, _): (usize, BinOp, usize),
    (_, r, _): (usize, usize, usize),
) -> Spanned<BinOp>
{
    Spanned::new(l, r, val)
}

#[allow(unused_variables)]
fn __action109<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, l, _): (usize, usize, usize),
    (_, val, _): (usize, UnOpKind, usize),
    (_, r, _): (usize, usize, usize),
) -> Spanned<UnOpKind>
{
    Spanned::new(l, r, val)
}

#[allow(unused_variables)]
fn __action110<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Expr>
{
    vec![]
}

#[allow(unused_variables)]
fn __action111<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, v, _): (usize, ::std::vec::Vec<Expr>, usize),
) -> ::std::vec::Vec<Expr>
{
    v
}

#[allow(unused_variables)]
fn __action112<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, _, _): (usize, Token<'input>, usize),
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(unused_variables)]
fn __action113<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<ComplexType>
{
    vec![]
}

#[allow(unused_variables)]
fn __action114<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, v, _): (usize, ::std::vec::Vec<ComplexType>, usize),
) -> ::std::vec::Vec<ComplexType>
{
    v
}

#[allow(unused_variables)]
fn __action115<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, _, _): (usize, Token<'input>, usize),
    (_, __0, _): (usize, ComplexType, usize),
) -> ComplexType
{
    __0
}

#[allow(unused_variables)]
fn __action116<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, ComplexType, usize),
) -> ::std::vec::Vec<ComplexType>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action117<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, v, _): (usize, ::std::vec::Vec<ComplexType>, usize),
    (_, e, _): (usize, ComplexType, usize),
) -> ::std::vec::Vec<ComplexType>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action118<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Expr, usize),
) -> ::std::vec::Vec<Expr>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action119<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, v, _): (usize, ::std::vec::Vec<Expr>, usize),
    (_, e, _): (usize, Expr, usize),
) -> ::std::vec::Vec<Expr>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action120<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, MemberVariable, usize),
) -> ::std::vec::Vec<MemberVariable>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action121<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, v, _): (usize, ::std::vec::Vec<MemberVariable>, usize),
    (_, e, _): (usize, MemberVariable, usize),
) -> ::std::vec::Vec<MemberVariable>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action122<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, Identifier, usize),
    __3: (usize, ::std::option::Option<Type>, usize),
    __4: (usize, Token<'input>, usize),
    __5: (usize, Vec<MemberVariable>, usize),
    __6: (usize, Token<'input>, usize),
) -> StructDefinition
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action90(
        input,
        intr,
        errs,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action6(
        input,
        intr,
        errs,
        __temp0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
    )
}

#[allow(unused_variables)]
fn __action123<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, Identifier, usize),
    __2: (usize, ::std::option::Option<Type>, usize),
    __3: (usize, Token<'input>, usize),
    __4: (usize, Vec<MemberVariable>, usize),
    __5: (usize, Token<'input>, usize),
) -> StructDefinition
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action91(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action6(
        input,
        intr,
        errs,
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
    )
}

#[allow(unused_variables)]
fn __action124<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, ComplexType, usize),
) -> ::std::vec::Vec<ComplexType>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action115(
        input,
        intr,
        errs,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action116(
        input,
        intr,
        errs,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action125<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, ::std::vec::Vec<ComplexType>, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, ComplexType, usize),
) -> ::std::vec::Vec<ComplexType>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action115(
        input,
        intr,
        errs,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action117(
        input,
        intr,
        errs,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action126<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, ComplexType, usize),
) -> Vec<ComplexType>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action113(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action63(
        input,
        intr,
        errs,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action127<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, ComplexType, usize),
    __1: (usize, ::std::vec::Vec<ComplexType>, usize),
) -> Vec<ComplexType>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action114(
        input,
        intr,
        errs,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action63(
        input,
        intr,
        errs,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action128<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, Expr, usize),
) -> ::std::vec::Vec<Expr>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action112(
        input,
        intr,
        errs,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action118(
        input,
        intr,
        errs,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action129<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, ::std::vec::Vec<Expr>, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, Expr, usize),
) -> ::std::vec::Vec<Expr>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action112(
        input,
        intr,
        errs,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action119(
        input,
        intr,
        errs,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action130<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Expr, usize),
) -> Vec<Expr>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action110(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action64(
        input,
        intr,
        errs,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action131<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Expr, usize),
    __1: (usize, ::std::vec::Vec<Expr>, usize),
) -> Vec<Expr>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action111(
        input,
        intr,
        errs,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action64(
        input,
        intr,
        errs,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action132<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, Type, usize),
) -> ::std::option::Option<Type>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action89(
        input,
        intr,
        errs,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action87(
        input,
        intr,
        errs,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action133<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, Identifier, usize),
    __3: (usize, Token<'input>, usize),
    __4: (usize, Type, usize),
    __5: (usize, Token<'input>, usize),
    __6: (usize, Vec<MemberVariable>, usize),
    __7: (usize, Token<'input>, usize),
) -> StructDefinition
{
    let __start0 = __3.0.clone();
    let __end0 = __4.2.clone();
    let __temp0 = __action132(
        input,
        intr,
        errs,
        __3,
        __4,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action122(
        input,
        intr,
        errs,
        __0,
        __1,
        __2,
        __temp0,
        __5,
        __6,
        __7,
    )
}

#[allow(unused_variables)]
fn __action134<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, Identifier, usize),
    __3: (usize, Token<'input>, usize),
    __4: (usize, Vec<MemberVariable>, usize),
    __5: (usize, Token<'input>, usize),
) -> StructDefinition
{
    let __start0 = __2.2.clone();
    let __end0 = __3.0.clone();
    let __temp0 = __action88(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action122(
        input,
        intr,
        errs,
        __0,
        __1,
        __2,
        __temp0,
        __3,
        __4,
        __5,
    )
}

#[allow(unused_variables)]
fn __action135<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, Identifier, usize),
    __2: (usize, Token<'input>, usize),
    __3: (usize, Type, usize),
    __4: (usize, Token<'input>, usize),
    __5: (usize, Vec<MemberVariable>, usize),
    __6: (usize, Token<'input>, usize),
) -> StructDefinition
{
    let __start0 = __2.0.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action132(
        input,
        intr,
        errs,
        __2,
        __3,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action123(
        input,
        intr,
        errs,
        __0,
        __1,
        __temp0,
        __4,
        __5,
        __6,
    )
}

#[allow(unused_variables)]
fn __action136<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, Identifier, usize),
    __2: (usize, Token<'input>, usize),
    __3: (usize, Vec<MemberVariable>, usize),
    __4: (usize, Token<'input>, usize),
) -> StructDefinition
{
    let __start0 = __1.2.clone();
    let __end0 = __2.0.clone();
    let __temp0 = __action88(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action123(
        input,
        intr,
        errs,
        __0,
        __1,
        __temp0,
        __2,
        __3,
        __4,
    )
}

#[allow(unused_variables)]
fn __action137<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, Identifier, usize),
) -> ::std::vec::Vec<Identifier>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action85(
        input,
        intr,
        errs,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action101(
        input,
        intr,
        errs,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action138<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, ::std::vec::Vec<Identifier>, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, Identifier, usize),
) -> ::std::vec::Vec<Identifier>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action85(
        input,
        intr,
        errs,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action102(
        input,
        intr,
        errs,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action139<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Identifier, usize),
) -> Path
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action83(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action9(
        input,
        intr,
        errs,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action140<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Identifier, usize),
    __1: (usize, ::std::vec::Vec<Identifier>, usize),
) -> Path
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action84(
        input,
        intr,
        errs,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action9(
        input,
        intr,
        errs,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action141<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, Token<'input>, usize),
) -> (usize, usize)
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action92(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action82(
        input,
        intr,
        errs,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action142<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, Expr, usize),
    __2: (usize, Spanned<BinOp>, usize),
    __3: (usize, Expr, usize),
) -> Expr
{
    let __start0 = __3.2.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action92(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action67(
        input,
        intr,
        errs,
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action143<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, Expr, usize),
    __2: (usize, Spanned<BinOp>, usize),
    __3: (usize, Expr, usize),
) -> Expr
{
    let __start0 = __3.2.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action92(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action69(
        input,
        intr,
        errs,
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action144<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, Expr, usize),
    __2: (usize, Spanned<BinOp>, usize),
    __3: (usize, Expr, usize),
) -> Expr
{
    let __start0 = __3.2.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action92(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action71(
        input,
        intr,
        errs,
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action145<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, Expr, usize),
    __2: (usize, Spanned<BinOp>, usize),
    __3: (usize, Expr, usize),
) -> Expr
{
    let __start0 = __3.2.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action92(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action73(
        input,
        intr,
        errs,
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action146<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, Expr, usize),
    __2: (usize, Spanned<BinOp>, usize),
    __3: (usize, Expr, usize),
) -> Expr
{
    let __start0 = __3.2.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action92(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action75(
        input,
        intr,
        errs,
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action147<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, Expr, usize),
    __2: (usize, Spanned<BinOp>, usize),
    __3: (usize, Expr, usize),
) -> Expr
{
    let __start0 = __3.2.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action92(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action77(
        input,
        intr,
        errs,
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action148<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, Vec<ComplexType>, usize),
) -> ComplexType
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action92(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action57(
        input,
        intr,
        errs,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action149<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, Path, usize),
) -> ComplexType
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action92(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action61(
        input,
        intr,
        errs,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action150<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, ComplexType, usize),
    __3: (usize, Token<'input>, usize),
) -> ComplexType
{
    let __start0 = __3.2.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action92(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action62(
        input,
        intr,
        errs,
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action151<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, ComplexType, usize),
) -> ComplexType
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action92(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action58(
        input,
        intr,
        errs,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action152<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, ComplexType, usize),
) -> ComplexType
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action92(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action59(
        input,
        intr,
        errs,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action153<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, LitKind, usize),
) -> Expr
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action92(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action44(
        input,
        intr,
        errs,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action154<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, Identifier, usize),
) -> Expr
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action92(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action45(
        input,
        intr,
        errs,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action155<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, Vec<Expr>, usize),
    __3: (usize, Token<'input>, usize),
) -> Expr
{
    let __start0 = __3.2.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action92(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action46(
        input,
        intr,
        errs,
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action156<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, __lalrpop_util::ErrorRecovery<usize, Token<'input>, ParserError>, usize),
) -> Expr
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action92(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action47(
        input,
        intr,
        errs,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action157<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, Expr, usize),
    __2: (usize, usize, usize),
    __3: (usize, Token<'input>, usize),
    __4: (usize, Expr, usize),
) -> Expr
{
    let __start0 = __3.2.clone();
    let __end0 = __4.0.clone();
    let __start1 = __4.2.clone();
    let __end1 = __4.2.clone();
    let __temp0 = __action92(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    let __temp1 = __action92(
        input,
        intr,
        errs,
        &__start1,
        &__end1,
    );
    let __temp1 = (__start1, __temp1, __end1);
    __action12(
        input,
        intr,
        errs,
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __4,
        __temp1,
    )
}

#[allow(unused_variables)]
fn __action158<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, Expr, usize),
    __2: (usize, Spanned<BinOp>, usize),
    __3: (usize, Expr, usize),
) -> Expr
{
    let __start0 = __3.2.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action92(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action13(
        input,
        intr,
        errs,
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action159<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, &'input str, usize),
) -> Identifier
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action92(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action8(
        input,
        intr,
        errs,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action160<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, &'input str, usize),
) -> LitKind
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action92(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action51(
        input,
        intr,
        errs,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action161<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, __lalrpop_util::ErrorRecovery<usize, Token<'input>, ParserError>, usize),
) -> ProgramPart
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action92(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action5(
        input,
        intr,
        errs,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action162<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, UnOpKind, usize),
) -> Spanned<UnOpKind>
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action92(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action109(
        input,
        intr,
        errs,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action163<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, BinOp, usize),
) -> Spanned<BinOp>
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action92(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action108(
        input,
        intr,
        errs,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action164<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, BinOp, usize),
) -> Spanned<BinOp>
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action92(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action107(
        input,
        intr,
        errs,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action165<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, BinOp, usize),
) -> Spanned<BinOp>
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action92(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action106(
        input,
        intr,
        errs,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action166<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, BinOp, usize),
) -> Spanned<BinOp>
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action92(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action105(
        input,
        intr,
        errs,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action167<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, BinOp, usize),
) -> Spanned<BinOp>
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action92(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action104(
        input,
        intr,
        errs,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action168<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, BinOp, usize),
) -> Spanned<BinOp>
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action92(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action79(
        input,
        intr,
        errs,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action169<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, BinOp, usize),
) -> Spanned<BinOp>
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action92(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action103(
        input,
        intr,
        errs,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action170<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, Expr, usize),
    __2: (usize, ::std::option::Option<(usize, usize)>, usize),
) -> Stmt
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action92(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action10(
        input,
        intr,
        errs,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action171<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, Token<'input>, usize),
) -> Type
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action92(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action52(
        input,
        intr,
        errs,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action172<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, Token<'input>, usize),
) -> Type
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action92(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action53(
        input,
        intr,
        errs,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action173<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, Token<'input>, usize),
) -> Type
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action92(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action54(
        input,
        intr,
        errs,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action174<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, Token<'input>, usize),
) -> Type
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action92(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action55(
        input,
        intr,
        errs,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action175<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, ComplexType, usize),
) -> Type
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action92(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action56(
        input,
        intr,
        errs,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action176<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, Spanned<UnOpKind>, usize),
    __2: (usize, Expr, usize),
) -> Expr
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action92(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action65(
        input,
        intr,
        errs,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action177<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Token<'input>, usize),
) -> (usize, usize)
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action93(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action141(
        input,
        intr,
        errs,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action178<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Expr, usize),
    __1: (usize, Spanned<BinOp>, usize),
    __2: (usize, Expr, usize),
) -> Expr
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action93(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action142(
        input,
        intr,
        errs,
        __temp0,
        __0,
        __1,
        __2,
    )
}

#[allow(unused_variables)]
fn __action179<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Expr, usize),
    __1: (usize, Spanned<BinOp>, usize),
    __2: (usize, Expr, usize),
) -> Expr
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action93(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action143(
        input,
        intr,
        errs,
        __temp0,
        __0,
        __1,
        __2,
    )
}

#[allow(unused_variables)]
fn __action180<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Expr, usize),
    __1: (usize, Spanned<BinOp>, usize),
    __2: (usize, Expr, usize),
) -> Expr
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action93(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action144(
        input,
        intr,
        errs,
        __temp0,
        __0,
        __1,
        __2,
    )
}

#[allow(unused_variables)]
fn __action181<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Expr, usize),
    __1: (usize, Spanned<BinOp>, usize),
    __2: (usize, Expr, usize),
) -> Expr
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action93(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action145(
        input,
        intr,
        errs,
        __temp0,
        __0,
        __1,
        __2,
    )
}

#[allow(unused_variables)]
fn __action182<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Expr, usize),
    __1: (usize, Spanned<BinOp>, usize),
    __2: (usize, Expr, usize),
) -> Expr
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action93(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action146(
        input,
        intr,
        errs,
        __temp0,
        __0,
        __1,
        __2,
    )
}

#[allow(unused_variables)]
fn __action183<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Expr, usize),
    __1: (usize, Spanned<BinOp>, usize),
    __2: (usize, Expr, usize),
) -> Expr
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action93(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action147(
        input,
        intr,
        errs,
        __temp0,
        __0,
        __1,
        __2,
    )
}

#[allow(unused_variables)]
fn __action184<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Vec<ComplexType>, usize),
) -> ComplexType
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action93(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action148(
        input,
        intr,
        errs,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action185<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Path, usize),
) -> ComplexType
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action93(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action149(
        input,
        intr,
        errs,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action186<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, ComplexType, usize),
    __2: (usize, Token<'input>, usize),
) -> ComplexType
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action93(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action150(
        input,
        intr,
        errs,
        __temp0,
        __0,
        __1,
        __2,
    )
}

#[allow(unused_variables)]
fn __action187<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, ComplexType, usize),
) -> ComplexType
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action93(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action151(
        input,
        intr,
        errs,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action188<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, ComplexType, usize),
) -> ComplexType
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action93(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action152(
        input,
        intr,
        errs,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action189<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, LitKind, usize),
) -> Expr
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action93(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action153(
        input,
        intr,
        errs,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action190<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Identifier, usize),
) -> Expr
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action93(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action154(
        input,
        intr,
        errs,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action191<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, Vec<Expr>, usize),
    __2: (usize, Token<'input>, usize),
) -> Expr
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action93(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action155(
        input,
        intr,
        errs,
        __temp0,
        __0,
        __1,
        __2,
    )
}

#[allow(unused_variables)]
fn __action192<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, __lalrpop_util::ErrorRecovery<usize, Token<'input>, ParserError>, usize),
) -> Expr
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action93(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action156(
        input,
        intr,
        errs,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action193<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Expr, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, Expr, usize),
) -> Expr
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __start1 = __0.2.clone();
    let __end1 = __1.0.clone();
    let __temp0 = __action93(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    let __temp1 = __action93(
        input,
        intr,
        errs,
        &__start1,
        &__end1,
    );
    let __temp1 = (__start1, __temp1, __end1);
    __action157(
        input,
        intr,
        errs,
        __temp0,
        __0,
        __temp1,
        __1,
        __2,
    )
}

#[allow(unused_variables)]
fn __action194<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Expr, usize),
    __1: (usize, Spanned<BinOp>, usize),
    __2: (usize, Expr, usize),
) -> Expr
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action93(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action158(
        input,
        intr,
        errs,
        __temp0,
        __0,
        __1,
        __2,
    )
}

#[allow(unused_variables)]
fn __action195<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, &'input str, usize),
) -> Identifier
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action93(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action159(
        input,
        intr,
        errs,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action196<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, &'input str, usize),
) -> LitKind
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action93(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action160(
        input,
        intr,
        errs,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action197<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, __lalrpop_util::ErrorRecovery<usize, Token<'input>, ParserError>, usize),
) -> ProgramPart
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action93(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action161(
        input,
        intr,
        errs,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action198<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, UnOpKind, usize),
) -> Spanned<UnOpKind>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action93(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action162(
        input,
        intr,
        errs,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action199<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, BinOp, usize),
) -> Spanned<BinOp>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action93(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action163(
        input,
        intr,
        errs,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action200<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, BinOp, usize),
) -> Spanned<BinOp>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action93(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action164(
        input,
        intr,
        errs,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action201<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, BinOp, usize),
) -> Spanned<BinOp>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action93(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action165(
        input,
        intr,
        errs,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action202<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, BinOp, usize),
) -> Spanned<BinOp>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action93(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action166(
        input,
        intr,
        errs,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action203<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, BinOp, usize),
) -> Spanned<BinOp>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action93(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action167(
        input,
        intr,
        errs,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action204<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, BinOp, usize),
) -> Spanned<BinOp>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action93(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action168(
        input,
        intr,
        errs,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action205<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, BinOp, usize),
) -> Spanned<BinOp>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action93(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action169(
        input,
        intr,
        errs,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action206<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Expr, usize),
    __1: (usize, ::std::option::Option<(usize, usize)>, usize),
) -> Stmt
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action93(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action170(
        input,
        intr,
        errs,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action207<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Token<'input>, usize),
) -> Type
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action93(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action171(
        input,
        intr,
        errs,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action208<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Token<'input>, usize),
) -> Type
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action93(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action172(
        input,
        intr,
        errs,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action209<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Token<'input>, usize),
) -> Type
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action93(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action173(
        input,
        intr,
        errs,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action210<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, Token<'input>, usize),
) -> Type
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action93(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action174(
        input,
        intr,
        errs,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action211<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, ComplexType, usize),
) -> Type
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action93(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action175(
        input,
        intr,
        errs,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action212<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Spanned<UnOpKind>, usize),
    __1: (usize, Expr, usize),
) -> Expr
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action93(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action176(
        input,
        intr,
        errs,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action213<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Token<'input>, usize),
) -> ::std::option::Option<(usize, usize)>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action177(
        input,
        intr,
        errs,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action80(
        input,
        intr,
        errs,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action214<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Expr, usize),
    __1: (usize, Token<'input>, usize),
) -> Stmt
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action213(
        input,
        intr,
        errs,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action206(
        input,
        intr,
        errs,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action215<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Expr, usize),
) -> Stmt
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action81(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action206(
        input,
        intr,
        errs,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action216<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, MemberVariable, usize),
    __1: (usize, Token<'input>, usize),
) -> ::std::vec::Vec<MemberVariable>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action100(
        input,
        intr,
        errs,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action120(
        input,
        intr,
        errs,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action217<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, ::std::vec::Vec<MemberVariable>, usize),
    __1: (usize, MemberVariable, usize),
    __2: (usize, Token<'input>, usize),
) -> ::std::vec::Vec<MemberVariable>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action100(
        input,
        intr,
        errs,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action121(
        input,
        intr,
        errs,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action218<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, ::std::option::Option<MemberVariable>, usize),
) -> Vec<MemberVariable>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action98(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action86(
        input,
        intr,
        errs,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action219<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, ::std::vec::Vec<MemberVariable>, usize),
    __1: (usize, ::std::option::Option<MemberVariable>, usize),
) -> Vec<MemberVariable>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action99(
        input,
        intr,
        errs,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action86(
        input,
        intr,
        errs,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action220<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, MemberVariable, usize),
) -> Vec<MemberVariable>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action96(
        input,
        intr,
        errs,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action218(
        input,
        intr,
        errs,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action221<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<MemberVariable>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action97(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action218(
        input,
        intr,
        errs,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action222<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, ::std::vec::Vec<MemberVariable>, usize),
    __1: (usize, MemberVariable, usize),
) -> Vec<MemberVariable>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action96(
        input,
        intr,
        errs,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action219(
        input,
        intr,
        errs,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action223<
    'input,
    'err,
>(
    input: &'input str,
    intr: &mut StringInterner<Sym>,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, ::std::vec::Vec<MemberVariable>, usize),
) -> Vec<MemberVariable>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action97(
        input,
        intr,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action219(
        input,
        intr,
        errs,
        __0,
        __temp0,
    )
}

pub trait __ToTriple<'input, 'err, > {
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, ParserError>>;
}

impl<'input, 'err, > __ToTriple<'input, 'err, > for (usize, Token<'input>, usize) {
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, ParserError>> {
        Ok(value)
    }
}
impl<'input, 'err, > __ToTriple<'input, 'err, > for Result<(usize, Token<'input>, usize), ParserError> {
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, ParserError>> {
        match value {
            Ok(v) => Ok(v),
            Err(error) => Err(__lalrpop_util::ParseError::User { error }),
        }
    }
}
