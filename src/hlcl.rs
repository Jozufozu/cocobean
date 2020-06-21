// auto-generated: "lalrpop 0.19.0"
// sha256: 8fa8c35bed7ce3de89ce3adf27f34dacb1e8c5d396ecd16964f7d1dc6eaa2ad
use std::str::FromStr;
use lalrpop_util::{ParseError, ErrorRecovery};
use crate::lexer::{Token, ParserError};
use crate::ast::*;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Expr {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use std::str::FromStr;
    use lalrpop_util::{ParseError, ErrorRecovery};
    use crate::lexer::{Token, ParserError};
    use crate::ast::*;
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
        Variant10((usize, usize)),
        Variant11(::std::option::Option<(usize, usize)>),
        Variant12(MemberVariable),
        Variant13(::std::vec::Vec<MemberVariable>),
        Variant14(usize),
        Variant15(Vec<MemberVariable>),
        Variant16(Identifier),
        Variant17(LitKind),
        Variant18(Vec<ComplexType>),
        Variant19(Vec<Expr>),
        Variant20(::std::option::Option<MemberVariable>),
        Variant21(BinOpKind),
        Variant22(Vec<ProgramPart>),
        Variant23(ProgramPart),
        Variant24(::std::vec::Vec<ProgramPart>),
        Variant25(Spanned<BinOpKind>),
        Variant26(Stmt),
        Variant27(StructDefinition),
    }
    const __ACTION: &[i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 21, 22, 23, 24,
        // State 1
        0, 0, 0, 26, -45, 0, -45, 0, -45, 27, -45, -45, -45, -45, -45, -45, 0, 0, 28, -45, 0, 0, 0, 0, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -45, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, -46, 0, -46, 0, -46, 0, -46, 30, -46, -46, 31, -46, 0, 0, 0, -46, 0, 0, 0, 0, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -46, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 33, 0, 34, 0, -49, 0, 35, 0, 36, -49, 0, 37, 0, 0, 0, 38, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 21, 22, 23, 24,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 21, 22, 23, 24,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 21, 22, 23, 24,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 21, 22, 23, 24,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 21, 22, 23, 24,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, -57, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 21, 22, 23, 24,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 21, 22, 23, 24,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, -27, -27, 0, -27, 0, -27, -27, -27, -27, -27, -27, -27, -27, 0, 0, -27, -27, 0, 0, 0, 0, -27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -27, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, -29, 0, -29, 0, -29, 0, -29, -29, -29, -29, -29, -29, 0, 0, 0, -29, 0, 0, 0, 0, -29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -29, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, -40, 0, 0, 0, 0, -40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, -42, -42, 0, -42, 0, -42, -42, -42, -42, -42, -42, -42, -42, 0, 0, -42, -42, 0, 0, 0, 0, -42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -42, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, -41, -41, 0, -41, 0, -41, -41, -41, -41, -41, -41, -41, -41, 0, 0, -41, -41, 0, 0, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -41, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, -52, -52, 0, -52, 0, -52, -52, -52, -52, -52, -52, -52, -52, 0, 0, -52, -52, 0, 0, 0, 0, -52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -52, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, -51, -51, 0, -51, 0, -51, -51, -51, -51, -51, -51, -51, -51, 0, 0, -51, -51, 0, 0, 0, 0, -51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -51, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, -50, -50, 0, -50, 0, -50, -50, -50, -50, -50, -50, -50, -50, 0, 0, -50, -50, 0, 0, 0, 0, -50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -50, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, -54, -54, 0, -54, 0, -54, -54, -54, -54, -54, -54, -54, -54, 0, 0, -54, -54, 0, 0, 0, 0, -54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -54, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, -53, -53, 0, -53, 0, -53, -53, -53, -53, -53, -53, -53, -53, 0, 0, -53, -53, 0, 0, 0, 0, -53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -53, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, -44, -44, 0, -44, 0, -44, -44, -44, -44, -44, -44, -44, -44, 0, 0, -44, -44, 0, 0, 0, 0, -44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -44, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, -79, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -79, 0, 0, 0, 0, 0, 0, 0, 0, 0, -79, 0, 0, 0, 0, 0, -79, -79, -79, -79,
        // State 25
        0, 0, 0, 0, 0, 0, 0, -64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -64, 0, 0, 0, 0, 0, 0, 0, 0, 0, -64, 0, 0, 0, 0, 0, -64, -64, -64, -64,
        // State 26
        0, 0, 0, 0, 0, 0, 0, -62, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -62, 0, 0, 0, 0, 0, 0, 0, 0, 0, -62, 0, 0, 0, 0, 0, -62, -62, -62, -62,
        // State 27
        0, 0, 0, 0, 0, 0, 0, -63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -63, 0, 0, 0, 0, 0, 0, 0, 0, 0, -63, 0, 0, 0, 0, 0, -63, -63, -63, -63,
        // State 28
        0, 0, 0, 0, 0, 0, 0, -80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -80, 0, 0, 0, 0, 0, 0, 0, 0, 0, -80, 0, 0, 0, 0, 0, -80, -80, -80, -80,
        // State 29
        0, 0, 0, 0, 0, 0, 0, -65, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -65, 0, 0, 0, 0, 0, 0, 0, 0, 0, -65, 0, 0, 0, 0, 0, -65, -65, -65, -65,
        // State 30
        0, 0, 0, 0, 0, 0, 0, -66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -66, 0, 0, 0, 0, 0, 0, 0, 0, 0, -66, 0, 0, 0, 0, 0, -66, -66, -66, -66,
        // State 31
        0, 0, 0, 0, 0, 0, 0, -81, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -81, 0, 0, 0, 0, 0, 0, 0, 0, 0, -81, 0, 0, 0, 0, 0, -81, -81, -81, -81,
        // State 32
        0, 0, 0, 0, 0, 0, 0, -71, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -71, 0, 0, 0, 0, 0, 0, 0, 0, 0, -71, 0, 0, 0, 0, 0, -71, -71, -71, -71,
        // State 33
        0, 0, 0, 0, 0, 0, 0, -72, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -72, 0, 0, 0, 0, 0, 0, 0, 0, 0, -72, 0, 0, 0, 0, 0, -72, -72, -72, -72,
        // State 34
        0, 0, 0, 0, 0, 0, 0, -69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -69, 0, 0, 0, 0, 0, 0, 0, 0, 0, -69, 0, 0, 0, 0, 0, -69, -69, -69, -69,
        // State 35
        0, 0, 0, 0, 0, 0, 0, -67, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -67, 0, 0, 0, 0, 0, 0, 0, 0, 0, -67, 0, 0, 0, 0, 0, -67, -67, -67, -67,
        // State 36
        0, 0, 0, 0, 0, 0, 0, -68, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -68, 0, 0, 0, 0, 0, 0, 0, 0, 0, -68, 0, 0, 0, 0, 0, -68, -68, -68, -68,
        // State 37
        0, 0, 0, 0, 0, 0, 0, -70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -70, 0, 0, 0, 0, 0, 0, 0, 0, 0, -70, 0, 0, 0, 0, 0, -70, -70, -70, -70,
        // State 38
        0, 0, 0, 0, 0, 0, 0, -73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -73, 0, 0, 0, 0, 0, 0, 0, 0, 0, -73, 0, 0, 0, 0, 0, -73, -73, -73, -73,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, -26, -26, 0, -26, 0, -26, -26, -26, -26, -26, -26, -26, -26, 0, 0, -26, -26, 0, 0, 0, 0, -26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -26, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, -28, 0, -28, 0, -28, 0, -28, -28, -28, -28, -28, -28, 0, 0, 0, -28, 0, 0, 0, 0, -28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -28, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, -48, 0, 0, 0, 0, -48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, -47, 0, 0, 0, 0, -47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, -58, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, -43, -43, 0, -43, 0, -43, -43, -43, -43, -43, -43, -43, -43, 0, 0, -43, -43, 0, 0, 0, 0, -43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -43, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, -11, 0, 0, 0, 0, -11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, -12, 0, 0, 0, 0, -12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 57 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        -45,
        // State 2
        -46,
        // State 3
        -49,
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
        -93,
        // State 13
        -27,
        // State 14
        -29,
        // State 15
        -40,
        // State 16
        -42,
        // State 17
        -41,
        // State 18
        -52,
        // State 19
        -51,
        // State 20
        -50,
        // State 21
        -54,
        // State 22
        -53,
        // State 23
        -44,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
        // State 27
        0,
        // State 28
        0,
        // State 29
        0,
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
        -26,
        // State 41
        -28,
        // State 42
        -48,
        // State 43
        -47,
        // State 44
        0,
        // State 45
        -43,
        // State 46
        0,
        // State 47
        0,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            6 => 44,
            16 => 1,
            17 => 2,
            22 => match state {
                0 => 12,
                10 => 46,
                11 => 47,
                _ => 9,
            },
            23 => match state {
                5 => 40,
                _ => 13,
            },
            24 => match state {
                6 => 41,
                _ => 14,
            },
            25 => 3,
            26 => match state {
                7 => 42,
                8 => 43,
                _ => 15,
            },
            27 => 16,
            28 => 17,
            30 => 39,
            33 => 24,
            34 => 28,
            35 => 31,
            39 => 5,
            40 => 6,
            41 => 7,
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> Vec<::std::string::String> {
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
    pub struct __StateMachine<'input, 'err>
    where 'input: 'err
    {
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __phantom: ::std::marker::PhantomData<(&'input (), &'err ())>,
    }
    impl<'input, 'err> __state_machine::ParserDefinition for __StateMachine<'input, 'err>
    where 'input: 'err
    {
        type Location = usize;
        type Error = ParserError;
        type Token = Token<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = Expr;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
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
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 57 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, ::std::marker::PhantomData::<(&(), &())>)
        }

        fn expected_tokens(&self, state: i8) -> Vec<String> {
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
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut Vec<i8>,
            symbols: &mut Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.input,
                self.errs,
                action,
                start_location,
                states,
                symbols,
                ::std::marker::PhantomData::<(&(), &())>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
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
            Token::Semicolon if true => Some(21),
            Token::Lt if true => Some(22),
            Token::Le if true => Some(23),
            Token::Assign if true => Some(24),
            Token::Eq if true => Some(25),
            Token::Gt if true => Some(26),
            Token::Ge if true => Some(27),
            Token::OpenBracket if true => Some(28),
            Token::CloseBracket if true => Some(29),
            Token::Caret if true => Some(30),
            Token::Bool if true => Some(31),
            Token::Builtin if true => Some(32),
            Token::Const if true => Some(33),
            Token::Do if true => Some(34),
            Token::Else if true => Some(35),
            Token::Enum if true => Some(36),
            Token::False if true => Some(37),
            Token::Fn if true => Some(38),
            Token::If if true => Some(39),
            Token::Int if true => Some(40),
            Token::Let if true => Some(41),
            Token::Mod if true => Some(42),
            Token::Return if true => Some(43),
            Token::String if true => Some(44),
            Token::Struct if true => Some(45),
            Token::Trait if true => Some(46),
            Token::True if true => Some(47),
            Token::While if true => Some(48),
            Token::OpenBlock if true => Some(49),
            Token::Or if true => Some(50),
            Token::OrAssign if true => Some(51),
            Token::CloseBlock if true => Some(52),
            Token::Identifier(_) if true => Some(53),
            Token::IntLiteral(_) if true => Some(54),
            Token::StringLiteral(_) if true => Some(55),
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
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34 | 35 | 36 | 37 | 38 | 39 | 40 | 41 | 42 | 43 | 44 | 45 | 46 | 47 | 48 | 49 | 50 | 51 | 52 => __Symbol::Variant0(__token),
            53 | 54 | 55 => match __token {
                Token::Identifier(__tok0) | Token::IntLiteral(__tok0) | Token::StringLiteral(__tok0) if true => __Symbol::Variant1(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    fn __simulate_reduce<
        'input,
        'err,
    >(
        __reduce_index: i8,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> __state_machine::SimulatedReduce<__StateMachine<'input, 'err>>
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
                    states_to_pop: 1,
                    nonterminal_produced: 9,
                }
            }
            16 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 10,
                }
            }
            17 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
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
                    states_to_pop: 0,
                    nonterminal_produced: 12,
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
                    states_to_pop: 2,
                    nonterminal_produced: 13,
                }
            }
            22 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 13,
                }
            }
            23 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
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
                    states_to_pop: 3,
                    nonterminal_produced: 16,
                }
            }
            26 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 16,
                }
            }
            27 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 17,
                }
            }
            28 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 17,
                }
            }
            29 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            30 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 18,
                }
            }
            31 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 18,
                }
            }
            32 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            33 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 19,
                }
            }
            34 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 20,
                }
            }
            35 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 20,
                }
            }
            36 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 21,
                }
            }
            37 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 21,
                }
            }
            38 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 21,
                }
            }
            39 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 22,
                }
            }
            40 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
                }
            }
            41 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
                }
            }
            42 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 23,
                }
            }
            43 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
                }
            }
            44 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 24,
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
                    states_to_pop: 3,
                    nonterminal_produced: 26,
                }
            }
            47 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 26,
                }
            }
            48 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 26,
                }
            }
            49 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 27,
                }
            }
            50 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
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
                    nonterminal_produced: 28,
                }
            }
            53 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 28,
                }
            }
            54 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 29,
                }
            }
            55 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 29,
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
                    states_to_pop: 2,
                    nonterminal_produced: 30,
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
                    nonterminal_produced: 32,
                }
            }
            60 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
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
                    nonterminal_produced: 33,
                }
            }
            63 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 33,
                }
            }
            64 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 34,
                }
            }
            65 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 34,
                }
            }
            66 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 35,
                }
            }
            67 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 35,
                }
            }
            68 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 35,
                }
            }
            69 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 35,
                }
            }
            70 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 35,
                }
            }
            71 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 35,
                }
            }
            72 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 35,
                }
            }
            73 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 36,
                }
            }
            74 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 37,
                }
            }
            75 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 37,
                }
            }
            76 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 38,
                }
            }
            77 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 38,
                }
            }
            78 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 39,
                }
            }
            79 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 40,
                }
            }
            80 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 41,
                }
            }
            81 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 42,
                }
            }
            82 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 42,
                }
            }
            83 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 8,
                    nonterminal_produced: 43,
                }
            }
            84 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 43,
                }
            }
            85 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 7,
                    nonterminal_produced: 43,
                }
            }
            86 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 43,
                }
            }
            87 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 44,
                }
            }
            88 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 44,
                }
            }
            89 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 44,
                }
            }
            90 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 44,
                }
            }
            91 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 44,
                }
            }
            92 => __state_machine::SimulatedReduce::Accept,
            93 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 46,
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
            errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
            __tokens0: __TOKENS,
        ) -> Result<Expr, __lalrpop_util::ParseError<usize, Token<'input>, ParserError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            __state_machine::Parser::drive(
                __StateMachine {
                    input,
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
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __error_state: i8,
        __states: & [i8],
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
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> Option<Result<Expr,__lalrpop_util::ParseError<usize, Token<'input>, ParserError>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            1 => {
                __reduce1(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            2 => {
                __reduce2(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            3 => {
                __reduce3(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            4 => {
                __reduce4(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            5 => {
                __reduce5(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            6 => {
                __reduce6(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            7 => {
                __reduce7(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            8 => {
                __reduce8(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            9 => {
                __reduce9(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            10 => {
                __reduce10(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            11 => {
                __reduce11(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            12 => {
                __reduce12(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            13 => {
                __reduce13(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            14 => {
                __reduce14(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            15 => {
                __reduce15(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            16 => {
                __reduce16(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            17 => {
                __reduce17(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            18 => {
                __reduce18(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            19 => {
                __reduce19(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            20 => {
                __reduce20(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            21 => {
                __reduce21(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            22 => {
                __reduce22(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            23 => {
                __reduce23(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            24 => {
                __reduce24(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            25 => {
                __reduce25(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            26 => {
                __reduce26(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            27 => {
                __reduce27(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            28 => {
                __reduce28(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            29 => {
                __reduce29(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            30 => {
                __reduce30(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            31 => {
                __reduce31(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            32 => {
                __reduce32(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            33 => {
                __reduce33(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            34 => {
                __reduce34(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            35 => {
                __reduce35(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            36 => {
                __reduce36(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            37 => {
                __reduce37(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            38 => {
                __reduce38(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            39 => {
                __reduce39(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            40 => {
                __reduce40(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            41 => {
                __reduce41(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            42 => {
                __reduce42(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            43 => {
                __reduce43(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            44 => {
                __reduce44(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            45 => {
                __reduce45(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            46 => {
                __reduce46(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            47 => {
                __reduce47(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            48 => {
                __reduce48(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            49 => {
                __reduce49(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            50 => {
                __reduce50(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            51 => {
                __reduce51(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            52 => {
                __reduce52(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            53 => {
                __reduce53(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            54 => {
                __reduce54(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            55 => {
                __reduce55(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            56 => {
                __reduce56(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            57 => {
                __reduce57(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            58 => {
                __reduce58(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            59 => {
                __reduce59(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            60 => {
                __reduce60(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            61 => {
                __reduce61(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            62 => {
                __reduce62(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            63 => {
                __reduce63(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            64 => {
                __reduce64(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            65 => {
                __reduce65(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            66 => {
                __reduce66(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            67 => {
                __reduce67(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            68 => {
                __reduce68(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            69 => {
                __reduce69(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            70 => {
                __reduce70(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            71 => {
                __reduce71(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            72 => {
                __reduce72(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            73 => {
                __reduce73(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            74 => {
                __reduce74(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            75 => {
                __reduce75(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            76 => {
                __reduce76(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            77 => {
                __reduce77(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            78 => {
                __reduce78(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            79 => {
                __reduce79(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            80 => {
                __reduce80(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            81 => {
                __reduce81(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            82 => {
                __reduce82(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            83 => {
                __reduce83(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            84 => {
                __reduce84(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            85 => {
                __reduce85(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            86 => {
                __reduce86(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            87 => {
                __reduce87(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            88 => {
                __reduce88(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            89 => {
                __reduce89(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            90 => {
                __reduce90(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            91 => {
                __reduce91(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            92 => {
                // __Expr = Expr => ActionFn(1);
                let __sym0 = __pop_Variant6(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(input, errs, __sym0);
                return Some(Ok(__nt));
            }
            93 => {
                __reduce93(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
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
    fn __pop_Variant10<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (usize, usize), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant10(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant21<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BinOpKind, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant21(__v), __r) => (__l, __v, __r),
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
    fn __pop_Variant16<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Identifier, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant16(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant17<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, LitKind, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant17(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant12<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, MemberVariable, usize)
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
    ) -> (usize, ProgramPart, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant23(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant25<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Spanned<BinOpKind>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant25(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant26<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Stmt, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant26(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant27<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, StructDefinition, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant27(__v), __r) => (__l, __v, __r),
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
    fn __pop_Variant18<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<ComplexType>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant18(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant19<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Expr>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant19(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant15<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<MemberVariable>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant15(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<ProgramPart>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant22(__v), __r) => (__l, __v, __r),
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
    fn __pop_Variant14<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant14(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant11<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<(usize, usize)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant11(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant20<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<MemberVariable>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant20(__v), __r) => (__l, __v, __r),
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
    fn __pop_Variant13<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<MemberVariable>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant13(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant24<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<ProgramPart>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant24(__v), __r) => (__l, __v, __r),
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
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // "builtin"? = "builtin" => ActionFn(62);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action62::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 0)
    }
    pub(crate) fn __reduce1<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // "builtin"? =  => ActionFn(63);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action63::<>(input, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 0)
    }
    pub(crate) fn __reduce2<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ("+" <ComplexType1>) = "+", ComplexType1 => ActionFn(78);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action78::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 1)
    }
    pub(crate) fn __reduce3<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ("+" <ComplexType1>)* =  => ActionFn(76);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action76::<>(input, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (0, 2)
    }
    pub(crate) fn __reduce4<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ("+" <ComplexType1>)* = ("+" <ComplexType1>)+ => ActionFn(77);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action77::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 2)
    }
    pub(crate) fn __reduce5<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ("+" <ComplexType1>)+ = "+", ComplexType1 => ActionFn(87);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action87::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce6<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ("+" <ComplexType1>)+ = ("+" <ComplexType1>)+, "+", ComplexType1 => ActionFn(88);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action88::<>(input, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 3)
    }
    pub(crate) fn __reduce7<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ("," <Expr>) = ",", Expr => ActionFn(75);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action75::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 4)
    }
    pub(crate) fn __reduce8<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ("," <Expr>)* =  => ActionFn(73);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action73::<>(input, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 5)
    }
    pub(crate) fn __reduce9<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ("," <Expr>)* = ("," <Expr>)+ => ActionFn(74);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action74::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce10<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ("," <Expr>)+ = ",", Expr => ActionFn(91);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action91::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 6)
    }
    pub(crate) fn __reduce11<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ("," <Expr>)+ = ("," <Expr>)+, ",", Expr => ActionFn(92);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action92::<>(input, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 6)
    }
    pub(crate) fn __reduce12<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (":" <Type>) = ":", Type => ActionFn(61);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action61::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 7)
    }
    pub(crate) fn __reduce13<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (":" <Type>)? = ":", Type => ActionFn(95);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action95::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 8)
    }
    pub(crate) fn __reduce14<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (":" <Type>)? =  => ActionFn(60);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action60::<>(input, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (0, 8)
    }
    pub(crate) fn __reduce15<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<@L> ";" <@R>) = ";" => ActionFn(125);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action125::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce16<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<@L> ";" <@R>)? = ";" => ActionFn(150);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action150::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce17<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<@L> ";" <@R>)? =  => ActionFn(54);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action54::<>(input, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (0, 10)
    }
    pub(crate) fn __reduce18<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<MemberVariable> ",") = MemberVariable, "," => ActionFn(70);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action70::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (2, 11)
    }
    pub(crate) fn __reduce19<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<MemberVariable> ",")* =  => ActionFn(68);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action68::<>(input, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (0, 12)
    }
    pub(crate) fn __reduce20<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<MemberVariable> ",")* = (<MemberVariable> ",")+ => ActionFn(69);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action69::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce21<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<MemberVariable> ",")+ = MemberVariable, "," => ActionFn(153);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action153::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (2, 13)
    }
    pub(crate) fn __reduce22<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<MemberVariable> ",")+ = (<MemberVariable> ",")+, MemberVariable, "," => ActionFn(154);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant12(__symbols);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action154::<>(input, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (3, 13)
    }
    pub(crate) fn __reduce23<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // @L =  => ActionFn(57);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action57::<>(input, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (0, 14)
    }
    pub(crate) fn __reduce24<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // @R =  => ActionFn(56);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action56::<>(input, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (0, 15)
    }
    pub(crate) fn __reduce25<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // BinOpExpr<Op4, Expr0> = BinOpExpr<Op4, Expr0>, Spanned<Op4>, Expr0 => ActionFn(126);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant25(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action126::<>(input, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce26<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // BinOpExpr<Op4, Expr0> = Expr0 => ActionFn(49);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action49::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce27<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // BinOpExpr<Op5, Expr4> = BinOpExpr<Op5, Expr4>, Spanned<Op5>, Expr4 => ActionFn(127);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant25(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action127::<>(input, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 17)
    }
    pub(crate) fn __reduce28<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // BinOpExpr<Op5, Expr4> = Expr4 => ActionFn(51);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action51::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce29<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Comma<MemberVariable> = MemberVariable => ActionFn(157);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action157::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce30<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Comma<MemberVariable> =  => ActionFn(158);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action158::<>(input, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (0, 18)
    }
    pub(crate) fn __reduce31<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Comma<MemberVariable> = (<MemberVariable> ",")+, MemberVariable => ActionFn(159);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant12(__symbols);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action159::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (2, 18)
    }
    pub(crate) fn __reduce32<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Comma<MemberVariable> = (<MemberVariable> ",")+ => ActionFn(160);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action160::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce33<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ComplexType = Many1<"+", ComplexType1> => ActionFn(128);
        let __sym0 = __pop_Variant18(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action128::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce34<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ComplexType0 = Identifier => ActionFn(129);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action129::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce35<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ComplexType0 = "(", ComplexType, ")" => ActionFn(130);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action130::<>(input, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 20)
    }
    pub(crate) fn __reduce36<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ComplexType1 = "^", ComplexType0 => ActionFn(131);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action131::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 21)
    }
    pub(crate) fn __reduce37<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ComplexType1 = "!", ComplexType0 => ActionFn(132);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action132::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 21)
    }
    pub(crate) fn __reduce38<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ComplexType1 = ComplexType0 => ActionFn(43);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action43::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce39<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr = Expr6 => ActionFn(9);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action9::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 22)
    }
    pub(crate) fn __reduce40<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr0 = Lit => ActionFn(133);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action133::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 23)
    }
    pub(crate) fn __reduce41<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr0 = Identifier => ActionFn(134);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action134::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 23)
    }
    pub(crate) fn __reduce42<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr0 = "(", Many1<",", Expr>, ")" => ActionFn(135);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant19(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action135::<>(input, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 23)
    }
    pub(crate) fn __reduce43<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr0 = error => ActionFn(136);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action136::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 23)
    }
    pub(crate) fn __reduce44<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr4 = BinOpExpr<Op4, Expr0> => ActionFn(23);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action23::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce45<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr5 = BinOpExpr<Op5, Expr4> => ActionFn(20);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action20::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce46<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr6 = Expr5, "=", Expr6 => ActionFn(137);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action137::<>(input, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 26)
    }
    pub(crate) fn __reduce47<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr6 = Expr5, Spanned<OpAssign>, Expr6 => ActionFn(138);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant25(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action138::<>(input, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 26)
    }
    pub(crate) fn __reduce48<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr6 = Expr5 => ActionFn(12);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action12::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 26)
    }
    pub(crate) fn __reduce49<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Identifier = TokenIdentifier => ActionFn(139);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action139::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce50<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Lit = "true" => ActionFn(31);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action31::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 28)
    }
    pub(crate) fn __reduce51<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Lit = "false" => ActionFn(32);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action32::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 28)
    }
    pub(crate) fn __reduce52<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Lit = TokenString => ActionFn(33);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action33::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 28)
    }
    pub(crate) fn __reduce53<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Lit = TokenInt => ActionFn(140);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action140::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 28)
    }
    pub(crate) fn __reduce54<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Many1<"+", ComplexType1> = ComplexType1 => ActionFn(89);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action89::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 29)
    }
    pub(crate) fn __reduce55<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Many1<"+", ComplexType1> = ComplexType1, ("+" <ComplexType1>)+ => ActionFn(90);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action90::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (2, 29)
    }
    pub(crate) fn __reduce56<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Many1<",", Expr> = Expr => ActionFn(93);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action93::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 30)
    }
    pub(crate) fn __reduce57<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Many1<",", Expr> = Expr, ("," <Expr>)+ => ActionFn(94);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action94::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (2, 30)
    }
    pub(crate) fn __reduce58<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // MemberVariable = Identifier, ":", Type => ActionFn(6);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action6::<>(input, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (3, 31)
    }
    pub(crate) fn __reduce59<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // MemberVariable? = MemberVariable => ActionFn(66);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action66::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 32)
    }
    pub(crate) fn __reduce60<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // MemberVariable? =  => ActionFn(67);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action67::<>(input, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (0, 32)
    }
    pub(crate) fn __reduce61<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op4 = "*" => ActionFn(24);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action24::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 33)
    }
    pub(crate) fn __reduce62<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op4 = "/" => ActionFn(25);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action25::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 33)
    }
    pub(crate) fn __reduce63<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op4 = "%" => ActionFn(26);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 33)
    }
    pub(crate) fn __reduce64<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op5 = "+" => ActionFn(21);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action21::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 34)
    }
    pub(crate) fn __reduce65<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op5 = "-" => ActionFn(22);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 34)
    }
    pub(crate) fn __reduce66<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // OpAssign = "+=" => ActionFn(13);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action13::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 35)
    }
    pub(crate) fn __reduce67<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // OpAssign = "-=" => ActionFn(14);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action14::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 35)
    }
    pub(crate) fn __reduce68<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // OpAssign = "*=" => ActionFn(15);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action15::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 35)
    }
    pub(crate) fn __reduce69<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // OpAssign = "/=" => ActionFn(16);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action16::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 35)
    }
    pub(crate) fn __reduce70<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // OpAssign = "%=" => ActionFn(17);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 35)
    }
    pub(crate) fn __reduce71<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // OpAssign = "&=" => ActionFn(18);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action18::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 35)
    }
    pub(crate) fn __reduce72<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // OpAssign = "|=" => ActionFn(19);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 35)
    }
    pub(crate) fn __reduce73<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Program = ProgramPart+ => ActionFn(2);
        let __sym0 = __pop_Variant24(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 36)
    }
    pub(crate) fn __reduce74<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ProgramPart = "mod", Identifier, "{", Program, "}" => ActionFn(3);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant22(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant16(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action3::<>(input, errs, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (5, 37)
    }
    pub(crate) fn __reduce75<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ProgramPart = StructDefinition => ActionFn(4);
        let __sym0 = __pop_Variant27(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 37)
    }
    pub(crate) fn __reduce76<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ProgramPart+ = ProgramPart => ActionFn(64);
        let __sym0 = __pop_Variant23(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action64::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant24(__nt), __end));
        (1, 38)
    }
    pub(crate) fn __reduce77<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ProgramPart+ = ProgramPart+, ProgramPart => ActionFn(65);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant23(__symbols);
        let __sym0 = __pop_Variant24(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action65::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant24(__nt), __end));
        (2, 38)
    }
    pub(crate) fn __reduce78<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Spanned<Op4> = Op4 => ActionFn(141);
        let __sym0 = __pop_Variant21(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action141::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant25(__nt), __end));
        (1, 39)
    }
    pub(crate) fn __reduce79<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Spanned<Op5> = Op5 => ActionFn(142);
        let __sym0 = __pop_Variant21(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action142::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant25(__nt), __end));
        (1, 40)
    }
    pub(crate) fn __reduce80<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Spanned<OpAssign> = OpAssign => ActionFn(143);
        let __sym0 = __pop_Variant21(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action143::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant25(__nt), __end));
        (1, 41)
    }
    pub(crate) fn __reduce81<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Stmt = Expr, ";" => ActionFn(151);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action151::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant26(__nt), __end));
        (2, 42)
    }
    pub(crate) fn __reduce82<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Stmt = Expr => ActionFn(152);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action152::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant26(__nt), __end));
        (1, 42)
    }
    pub(crate) fn __reduce83<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // StructDefinition = "builtin", "struct", Identifier, ":", Type, "{", Comma<MemberVariable>, "}" => ActionFn(96);
        assert!(__symbols.len() >= 8);
        let __sym7 = __pop_Variant0(__symbols);
        let __sym6 = __pop_Variant15(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant8(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant16(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym7.2.clone();
        let __nt = super::__action96::<>(input, errs, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7);
        __symbols.push((__start, __Symbol::Variant27(__nt), __end));
        (8, 43)
    }
    pub(crate) fn __reduce84<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // StructDefinition = "builtin", "struct", Identifier, "{", Comma<MemberVariable>, "}" => ActionFn(97);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant15(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant16(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action97::<>(input, errs, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant27(__nt), __end));
        (6, 43)
    }
    pub(crate) fn __reduce85<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // StructDefinition = "struct", Identifier, ":", Type, "{", Comma<MemberVariable>, "}" => ActionFn(98);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant15(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant8(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant16(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action98::<>(input, errs, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant27(__nt), __end));
        (7, 43)
    }
    pub(crate) fn __reduce86<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // StructDefinition = "struct", Identifier, "{", Comma<MemberVariable>, "}" => ActionFn(99);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant15(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant16(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action99::<>(input, errs, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant27(__nt), __end));
        (5, 43)
    }
    pub(crate) fn __reduce87<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Type = "bool" => ActionFn(145);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action145::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 44)
    }
    pub(crate) fn __reduce88<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Type = "string" => ActionFn(146);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action146::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 44)
    }
    pub(crate) fn __reduce89<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Type = "int" => ActionFn(147);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action147::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 44)
    }
    pub(crate) fn __reduce90<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Type = "(", ")" => ActionFn(148);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action148::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 44)
    }
    pub(crate) fn __reduce91<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Type = ComplexType => ActionFn(149);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action149::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 44)
    }
    pub(crate) fn __reduce93<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // __Program = Program => ActionFn(0);
        let __sym0 = __pop_Variant22(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 46)
    }
}
pub use self::__parse__Expr::ExprParser;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Program {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use std::str::FromStr;
    use lalrpop_util::{ParseError, ErrorRecovery};
    use crate::lexer::{Token, ParserError};
    use crate::ast::*;
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
        Variant10((usize, usize)),
        Variant11(::std::option::Option<(usize, usize)>),
        Variant12(MemberVariable),
        Variant13(::std::vec::Vec<MemberVariable>),
        Variant14(usize),
        Variant15(Vec<MemberVariable>),
        Variant16(Identifier),
        Variant17(LitKind),
        Variant18(Vec<ComplexType>),
        Variant19(Vec<Expr>),
        Variant20(::std::option::Option<MemberVariable>),
        Variant21(BinOpKind),
        Variant22(Vec<ProgramPart>),
        Variant23(ProgramPart),
        Variant24(::std::vec::Vec<ProgramPart>),
        Variant25(Spanned<BinOpKind>),
        Variant26(Stmt),
        Variant27(StructDefinition),
    }
    const __ACTION: &[i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 4, 0, 0, 0, 0, 0, 0, -74, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        12, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 37, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 0, 39, 0, 0, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -31, 28, 0, 0, 0,
        // State 8
        12, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 37, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 0, 39, 0, 0, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -31, 28, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, -55, 0, 0, 16, 0, -55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -55, 0, 0, -55, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0,
        // State 12
        12, 0, 0, 0, 0, 0, 0, 18, 49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -33, 28, 0, 0, 0,
        // State 15
        12, 0, 0, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -31, 28, 0, 0, 0,
        // State 17
        12, 0, 0, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0,
        // State 18
        12, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 37, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 0, 39, 0, 0, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -31, 28, 0, 0, 0,
        // State 20
        12, 0, 0, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -77, 0, 0, 0, 0, 0, 0, 0, 0, 0, -77, 0, 0, -77, 0, 0, 0, 0, 0, 0, -77, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -76, 0, 0, 0, 0, 0, 0, 0, 0, 0, -76, 0, 0, -76, 0, 0, 0, 0, 0, 0, -76, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -78, 0, 0, 0, 0, 0, 0, 0, 0, 0, -78, 0, 0, -78, 0, 0, 0, 0, 0, 0, -78, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, -50, 0, 0, -50, 0, -50, 0, 0, 0, 0, 0, 0, -50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -50, 0, 0, -50, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 45, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -92, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -92, 0, 0, -92, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, -39, 0, 0, -39, 0, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -39, 0, 0, -39, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, -35, 0, 0, -35, 0, -35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -35, 0, 0, -35, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, -34, 0, 0, 0, 0, -34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -34, 0, 0, -34, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -88, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -88, 0, 0, -88, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -90, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -90, 0, 0, -90, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -89, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -89, 0, 0, -89, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 52, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -30, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 54, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -75, 0, 0, 0, 0, 0, 0, 0, 0, 0, -75, 0, 0, -75, 0, 0, 0, 0, 0, 0, -75, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 0, 0, -56, 0, 0, 21, 0, -56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -56, 0, 0, -56, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, -38, 0, 0, -38, 0, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -38, 0, 0, -38, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -91, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -91, 0, 0, -91, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, -37, 0, 0, -37, 0, -37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -37, 0, 0, -37, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -32, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -87, 0, 0, 0, 0, 0, 0, 0, 0, 0, -87, 0, 0, -87, 0, 0, 0, 0, 0, 0, -87, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -22, -22, 0, 0, 0,
        // State 53
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -85, 0, 0, 0, 0, 0, 0, 0, 0, 0, -85, 0, 0, -85, 0, 0, 0, 0, 0, 0, -85, 0, 0, 0, 0,
        // State 54
        0, 0, 0, 0, 0, 0, 0, 0, -6, 0, 0, -6, 0, -6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -6, 0, 0, -6, 0, 0, 0, 0,
        // State 55
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 62, 0, 0, 0, 0,
        // State 56
        0, 0, 0, 0, 0, 0, 0, 0, -36, 0, 0, -36, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -36, 0, 0, -36, 0, 0, 0, 0,
        // State 57
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -23, -23, 0, 0, 0,
        // State 58
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -59, 0, 0, 0, 0,
        // State 59
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 63, 0, 0, 0, 0,
        // State 60
        0, 0, 0, 0, 0, 0, 0, 0, -7, 0, 0, -7, 0, -7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -7, 0, 0, -7, 0, 0, 0, 0,
        // State 61
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -86, 0, 0, 0, 0, 0, 0, 0, 0, 0, -86, 0, 0, -86, 0, 0, 0, 0, 0, 0, -86, 0, 0, 0, 0,
        // State 62
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -84, 0, 0, 0, 0, 0, 0, 0, 0, 0, -84, 0, 0, -84, 0, 0, 0, 0, 0, 0, -84, 0, 0, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 57 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        -74,
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
        -94,
        // State 22
        -77,
        // State 23
        -76,
        // State 24
        0,
        // State 25
        -78,
        // State 26
        0,
        // State 27
        0,
        // State 28
        0,
        // State 29
        0,
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
        -75,
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
        -87,
        // State 52
        0,
        // State 53
        -85,
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
        -86,
        // State 62
        -84,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            3 => 45,
            13 => 14,
            18 => match state {
                9 => 43,
                16 => 55,
                19 => 59,
                _ => 39,
            },
            19 => match state {
                12 | 17 => 47,
                _ => 31,
            },
            20 => match state {
                11 => 46,
                13 => 49,
                _ => 32,
            },
            21 => match state {
                15 => 54,
                20 => 60,
                _ => 10,
            },
            27 => match state {
                2 => 26,
                3 => 28,
                4 => 29,
                7 | 9 | 14 | 16 | 19 => 40,
                _ => 33,
            },
            29 => 34,
            31 => match state {
                14 => 50,
                _ => 41,
            },
            36 => match state {
                5 => 30,
                _ => 21,
            },
            37 => match state {
                1 => 25,
                _ => 22,
            },
            38 => 1,
            43 => 23,
            44 => match state {
                8 => 42,
                18 => 58,
                _ => 35,
            },
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> Vec<::std::string::String> {
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
    pub struct __StateMachine<'input, 'err>
    where 'input: 'err
    {
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __phantom: ::std::marker::PhantomData<(&'input (), &'err ())>,
    }
    impl<'input, 'err> __state_machine::ParserDefinition for __StateMachine<'input, 'err>
    where 'input: 'err
    {
        type Location = usize;
        type Error = ParserError;
        type Token = Token<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = Vec<ProgramPart>;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
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
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 57 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, ::std::marker::PhantomData::<(&(), &())>)
        }

        fn expected_tokens(&self, state: i8) -> Vec<String> {
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
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut Vec<i8>,
            symbols: &mut Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.input,
                self.errs,
                action,
                start_location,
                states,
                symbols,
                ::std::marker::PhantomData::<(&(), &())>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
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
            Token::Semicolon if true => Some(21),
            Token::Lt if true => Some(22),
            Token::Le if true => Some(23),
            Token::Assign if true => Some(24),
            Token::Eq if true => Some(25),
            Token::Gt if true => Some(26),
            Token::Ge if true => Some(27),
            Token::OpenBracket if true => Some(28),
            Token::CloseBracket if true => Some(29),
            Token::Caret if true => Some(30),
            Token::Bool if true => Some(31),
            Token::Builtin if true => Some(32),
            Token::Const if true => Some(33),
            Token::Do if true => Some(34),
            Token::Else if true => Some(35),
            Token::Enum if true => Some(36),
            Token::False if true => Some(37),
            Token::Fn if true => Some(38),
            Token::If if true => Some(39),
            Token::Int if true => Some(40),
            Token::Let if true => Some(41),
            Token::Mod if true => Some(42),
            Token::Return if true => Some(43),
            Token::String if true => Some(44),
            Token::Struct if true => Some(45),
            Token::Trait if true => Some(46),
            Token::True if true => Some(47),
            Token::While if true => Some(48),
            Token::OpenBlock if true => Some(49),
            Token::Or if true => Some(50),
            Token::OrAssign if true => Some(51),
            Token::CloseBlock if true => Some(52),
            Token::Identifier(_) if true => Some(53),
            Token::IntLiteral(_) if true => Some(54),
            Token::StringLiteral(_) if true => Some(55),
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
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34 | 35 | 36 | 37 | 38 | 39 | 40 | 41 | 42 | 43 | 44 | 45 | 46 | 47 | 48 | 49 | 50 | 51 | 52 => __Symbol::Variant0(__token),
            53 | 54 | 55 => match __token {
                Token::Identifier(__tok0) | Token::IntLiteral(__tok0) | Token::StringLiteral(__tok0) if true => __Symbol::Variant1(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    fn __simulate_reduce<
        'input,
        'err,
    >(
        __reduce_index: i8,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> __state_machine::SimulatedReduce<__StateMachine<'input, 'err>>
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
                    states_to_pop: 1,
                    nonterminal_produced: 9,
                }
            }
            16 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 10,
                }
            }
            17 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
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
                    states_to_pop: 0,
                    nonterminal_produced: 12,
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
                    states_to_pop: 2,
                    nonterminal_produced: 13,
                }
            }
            22 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 13,
                }
            }
            23 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
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
                    states_to_pop: 3,
                    nonterminal_produced: 16,
                }
            }
            26 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 16,
                }
            }
            27 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 17,
                }
            }
            28 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 17,
                }
            }
            29 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            30 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 18,
                }
            }
            31 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 18,
                }
            }
            32 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            33 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 19,
                }
            }
            34 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 20,
                }
            }
            35 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 20,
                }
            }
            36 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 21,
                }
            }
            37 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 21,
                }
            }
            38 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 21,
                }
            }
            39 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 22,
                }
            }
            40 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
                }
            }
            41 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
                }
            }
            42 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 23,
                }
            }
            43 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
                }
            }
            44 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 24,
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
                    states_to_pop: 3,
                    nonterminal_produced: 26,
                }
            }
            47 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 26,
                }
            }
            48 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 26,
                }
            }
            49 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 27,
                }
            }
            50 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
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
                    nonterminal_produced: 28,
                }
            }
            53 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 28,
                }
            }
            54 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 29,
                }
            }
            55 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 29,
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
                    states_to_pop: 2,
                    nonterminal_produced: 30,
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
                    nonterminal_produced: 32,
                }
            }
            60 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
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
                    nonterminal_produced: 33,
                }
            }
            63 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 33,
                }
            }
            64 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 34,
                }
            }
            65 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 34,
                }
            }
            66 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 35,
                }
            }
            67 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 35,
                }
            }
            68 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 35,
                }
            }
            69 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 35,
                }
            }
            70 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 35,
                }
            }
            71 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 35,
                }
            }
            72 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 35,
                }
            }
            73 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 36,
                }
            }
            74 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 37,
                }
            }
            75 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 37,
                }
            }
            76 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 38,
                }
            }
            77 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 38,
                }
            }
            78 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 39,
                }
            }
            79 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 40,
                }
            }
            80 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 41,
                }
            }
            81 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 42,
                }
            }
            82 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 42,
                }
            }
            83 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 8,
                    nonterminal_produced: 43,
                }
            }
            84 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 43,
                }
            }
            85 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 7,
                    nonterminal_produced: 43,
                }
            }
            86 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 43,
                }
            }
            87 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 44,
                }
            }
            88 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 44,
                }
            }
            89 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 44,
                }
            }
            90 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 44,
                }
            }
            91 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 44,
                }
            }
            92 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 45,
                }
            }
            93 => __state_machine::SimulatedReduce::Accept,
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
            errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
            __tokens0: __TOKENS,
        ) -> Result<Vec<ProgramPart>, __lalrpop_util::ParseError<usize, Token<'input>, ParserError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            __state_machine::Parser::drive(
                __StateMachine {
                    input,
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
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __error_state: i8,
        __states: & [i8],
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
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> Option<Result<Vec<ProgramPart>,__lalrpop_util::ParseError<usize, Token<'input>, ParserError>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            1 => {
                __reduce1(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            2 => {
                __reduce2(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            3 => {
                __reduce3(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            4 => {
                __reduce4(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            5 => {
                __reduce5(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            6 => {
                __reduce6(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            7 => {
                __reduce7(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            8 => {
                __reduce8(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            9 => {
                __reduce9(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            10 => {
                __reduce10(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            11 => {
                __reduce11(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            12 => {
                __reduce12(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            13 => {
                __reduce13(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            14 => {
                __reduce14(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            15 => {
                __reduce15(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            16 => {
                __reduce16(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            17 => {
                __reduce17(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            18 => {
                __reduce18(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            19 => {
                __reduce19(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            20 => {
                __reduce20(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            21 => {
                __reduce21(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            22 => {
                __reduce22(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            23 => {
                __reduce23(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            24 => {
                __reduce24(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            25 => {
                __reduce25(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            26 => {
                __reduce26(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            27 => {
                __reduce27(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            28 => {
                __reduce28(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            29 => {
                __reduce29(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            30 => {
                __reduce30(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            31 => {
                __reduce31(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            32 => {
                __reduce32(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            33 => {
                __reduce33(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            34 => {
                __reduce34(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            35 => {
                __reduce35(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            36 => {
                __reduce36(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            37 => {
                __reduce37(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            38 => {
                __reduce38(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            39 => {
                __reduce39(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            40 => {
                __reduce40(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            41 => {
                __reduce41(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            42 => {
                __reduce42(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            43 => {
                __reduce43(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            44 => {
                __reduce44(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            45 => {
                __reduce45(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            46 => {
                __reduce46(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            47 => {
                __reduce47(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            48 => {
                __reduce48(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            49 => {
                __reduce49(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            50 => {
                __reduce50(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            51 => {
                __reduce51(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            52 => {
                __reduce52(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            53 => {
                __reduce53(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            54 => {
                __reduce54(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            55 => {
                __reduce55(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            56 => {
                __reduce56(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            57 => {
                __reduce57(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            58 => {
                __reduce58(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            59 => {
                __reduce59(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            60 => {
                __reduce60(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            61 => {
                __reduce61(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            62 => {
                __reduce62(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            63 => {
                __reduce63(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            64 => {
                __reduce64(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            65 => {
                __reduce65(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            66 => {
                __reduce66(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            67 => {
                __reduce67(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            68 => {
                __reduce68(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            69 => {
                __reduce69(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            70 => {
                __reduce70(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            71 => {
                __reduce71(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            72 => {
                __reduce72(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            73 => {
                __reduce73(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            74 => {
                __reduce74(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            75 => {
                __reduce75(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            76 => {
                __reduce76(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            77 => {
                __reduce77(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            78 => {
                __reduce78(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            79 => {
                __reduce79(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            80 => {
                __reduce80(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            81 => {
                __reduce81(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            82 => {
                __reduce82(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            83 => {
                __reduce83(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            84 => {
                __reduce84(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            85 => {
                __reduce85(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            86 => {
                __reduce86(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            87 => {
                __reduce87(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            88 => {
                __reduce88(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            89 => {
                __reduce89(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            90 => {
                __reduce90(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            91 => {
                __reduce91(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            92 => {
                __reduce92(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            93 => {
                // __Program = Program => ActionFn(0);
                let __sym0 = __pop_Variant22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, errs, __sym0);
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
    fn __pop_Variant10<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (usize, usize), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant10(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant21<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BinOpKind, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant21(__v), __r) => (__l, __v, __r),
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
    fn __pop_Variant16<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Identifier, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant16(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant17<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, LitKind, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant17(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant12<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, MemberVariable, usize)
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
    ) -> (usize, ProgramPart, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant23(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant25<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Spanned<BinOpKind>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant25(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant26<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Stmt, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant26(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant27<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, StructDefinition, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant27(__v), __r) => (__l, __v, __r),
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
    fn __pop_Variant18<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<ComplexType>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant18(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant19<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Expr>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant19(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant15<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<MemberVariable>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant15(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<ProgramPart>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant22(__v), __r) => (__l, __v, __r),
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
    fn __pop_Variant14<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant14(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant11<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<(usize, usize)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant11(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant20<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<MemberVariable>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant20(__v), __r) => (__l, __v, __r),
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
    fn __pop_Variant13<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<MemberVariable>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant13(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant24<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<ProgramPart>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant24(__v), __r) => (__l, __v, __r),
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
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // "builtin"? = "builtin" => ActionFn(62);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action62::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 0)
    }
    pub(crate) fn __reduce1<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // "builtin"? =  => ActionFn(63);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action63::<>(input, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 0)
    }
    pub(crate) fn __reduce2<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ("+" <ComplexType1>) = "+", ComplexType1 => ActionFn(78);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action78::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 1)
    }
    pub(crate) fn __reduce3<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ("+" <ComplexType1>)* =  => ActionFn(76);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action76::<>(input, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (0, 2)
    }
    pub(crate) fn __reduce4<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ("+" <ComplexType1>)* = ("+" <ComplexType1>)+ => ActionFn(77);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action77::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 2)
    }
    pub(crate) fn __reduce5<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ("+" <ComplexType1>)+ = "+", ComplexType1 => ActionFn(87);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action87::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce6<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ("+" <ComplexType1>)+ = ("+" <ComplexType1>)+, "+", ComplexType1 => ActionFn(88);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action88::<>(input, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 3)
    }
    pub(crate) fn __reduce7<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ("," <Expr>) = ",", Expr => ActionFn(75);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action75::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 4)
    }
    pub(crate) fn __reduce8<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ("," <Expr>)* =  => ActionFn(73);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action73::<>(input, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 5)
    }
    pub(crate) fn __reduce9<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ("," <Expr>)* = ("," <Expr>)+ => ActionFn(74);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action74::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce10<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ("," <Expr>)+ = ",", Expr => ActionFn(91);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action91::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 6)
    }
    pub(crate) fn __reduce11<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ("," <Expr>)+ = ("," <Expr>)+, ",", Expr => ActionFn(92);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action92::<>(input, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 6)
    }
    pub(crate) fn __reduce12<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (":" <Type>) = ":", Type => ActionFn(61);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action61::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 7)
    }
    pub(crate) fn __reduce13<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (":" <Type>)? = ":", Type => ActionFn(95);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action95::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 8)
    }
    pub(crate) fn __reduce14<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (":" <Type>)? =  => ActionFn(60);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action60::<>(input, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (0, 8)
    }
    pub(crate) fn __reduce15<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<@L> ";" <@R>) = ";" => ActionFn(125);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action125::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce16<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<@L> ";" <@R>)? = ";" => ActionFn(150);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action150::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce17<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<@L> ";" <@R>)? =  => ActionFn(54);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action54::<>(input, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (0, 10)
    }
    pub(crate) fn __reduce18<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<MemberVariable> ",") = MemberVariable, "," => ActionFn(70);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action70::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (2, 11)
    }
    pub(crate) fn __reduce19<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<MemberVariable> ",")* =  => ActionFn(68);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action68::<>(input, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (0, 12)
    }
    pub(crate) fn __reduce20<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<MemberVariable> ",")* = (<MemberVariable> ",")+ => ActionFn(69);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action69::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce21<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<MemberVariable> ",")+ = MemberVariable, "," => ActionFn(153);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action153::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (2, 13)
    }
    pub(crate) fn __reduce22<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<MemberVariable> ",")+ = (<MemberVariable> ",")+, MemberVariable, "," => ActionFn(154);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant12(__symbols);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action154::<>(input, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (3, 13)
    }
    pub(crate) fn __reduce23<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // @L =  => ActionFn(57);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action57::<>(input, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (0, 14)
    }
    pub(crate) fn __reduce24<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // @R =  => ActionFn(56);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action56::<>(input, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (0, 15)
    }
    pub(crate) fn __reduce25<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // BinOpExpr<Op4, Expr0> = BinOpExpr<Op4, Expr0>, Spanned<Op4>, Expr0 => ActionFn(126);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant25(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action126::<>(input, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce26<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // BinOpExpr<Op4, Expr0> = Expr0 => ActionFn(49);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action49::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce27<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // BinOpExpr<Op5, Expr4> = BinOpExpr<Op5, Expr4>, Spanned<Op5>, Expr4 => ActionFn(127);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant25(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action127::<>(input, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 17)
    }
    pub(crate) fn __reduce28<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // BinOpExpr<Op5, Expr4> = Expr4 => ActionFn(51);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action51::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce29<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Comma<MemberVariable> = MemberVariable => ActionFn(157);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action157::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce30<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Comma<MemberVariable> =  => ActionFn(158);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action158::<>(input, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (0, 18)
    }
    pub(crate) fn __reduce31<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Comma<MemberVariable> = (<MemberVariable> ",")+, MemberVariable => ActionFn(159);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant12(__symbols);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action159::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (2, 18)
    }
    pub(crate) fn __reduce32<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Comma<MemberVariable> = (<MemberVariable> ",")+ => ActionFn(160);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action160::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce33<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ComplexType = Many1<"+", ComplexType1> => ActionFn(128);
        let __sym0 = __pop_Variant18(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action128::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce34<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ComplexType0 = Identifier => ActionFn(129);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action129::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce35<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ComplexType0 = "(", ComplexType, ")" => ActionFn(130);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action130::<>(input, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 20)
    }
    pub(crate) fn __reduce36<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ComplexType1 = "^", ComplexType0 => ActionFn(131);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action131::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 21)
    }
    pub(crate) fn __reduce37<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ComplexType1 = "!", ComplexType0 => ActionFn(132);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action132::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 21)
    }
    pub(crate) fn __reduce38<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ComplexType1 = ComplexType0 => ActionFn(43);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action43::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce39<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr = Expr6 => ActionFn(9);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action9::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 22)
    }
    pub(crate) fn __reduce40<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr0 = Lit => ActionFn(133);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action133::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 23)
    }
    pub(crate) fn __reduce41<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr0 = Identifier => ActionFn(134);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action134::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 23)
    }
    pub(crate) fn __reduce42<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr0 = "(", Many1<",", Expr>, ")" => ActionFn(135);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant19(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action135::<>(input, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 23)
    }
    pub(crate) fn __reduce43<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr0 = error => ActionFn(136);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action136::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 23)
    }
    pub(crate) fn __reduce44<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr4 = BinOpExpr<Op4, Expr0> => ActionFn(23);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action23::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce45<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr5 = BinOpExpr<Op5, Expr4> => ActionFn(20);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action20::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce46<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr6 = Expr5, "=", Expr6 => ActionFn(137);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action137::<>(input, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 26)
    }
    pub(crate) fn __reduce47<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr6 = Expr5, Spanned<OpAssign>, Expr6 => ActionFn(138);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant25(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action138::<>(input, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 26)
    }
    pub(crate) fn __reduce48<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr6 = Expr5 => ActionFn(12);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action12::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 26)
    }
    pub(crate) fn __reduce49<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Identifier = TokenIdentifier => ActionFn(139);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action139::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce50<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Lit = "true" => ActionFn(31);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action31::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 28)
    }
    pub(crate) fn __reduce51<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Lit = "false" => ActionFn(32);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action32::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 28)
    }
    pub(crate) fn __reduce52<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Lit = TokenString => ActionFn(33);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action33::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 28)
    }
    pub(crate) fn __reduce53<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Lit = TokenInt => ActionFn(140);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action140::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 28)
    }
    pub(crate) fn __reduce54<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Many1<"+", ComplexType1> = ComplexType1 => ActionFn(89);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action89::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 29)
    }
    pub(crate) fn __reduce55<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Many1<"+", ComplexType1> = ComplexType1, ("+" <ComplexType1>)+ => ActionFn(90);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action90::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (2, 29)
    }
    pub(crate) fn __reduce56<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Many1<",", Expr> = Expr => ActionFn(93);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action93::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 30)
    }
    pub(crate) fn __reduce57<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Many1<",", Expr> = Expr, ("," <Expr>)+ => ActionFn(94);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action94::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (2, 30)
    }
    pub(crate) fn __reduce58<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // MemberVariable = Identifier, ":", Type => ActionFn(6);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action6::<>(input, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (3, 31)
    }
    pub(crate) fn __reduce59<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // MemberVariable? = MemberVariable => ActionFn(66);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action66::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 32)
    }
    pub(crate) fn __reduce60<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // MemberVariable? =  => ActionFn(67);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action67::<>(input, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (0, 32)
    }
    pub(crate) fn __reduce61<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op4 = "*" => ActionFn(24);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action24::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 33)
    }
    pub(crate) fn __reduce62<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op4 = "/" => ActionFn(25);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action25::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 33)
    }
    pub(crate) fn __reduce63<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op4 = "%" => ActionFn(26);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 33)
    }
    pub(crate) fn __reduce64<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op5 = "+" => ActionFn(21);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action21::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 34)
    }
    pub(crate) fn __reduce65<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Op5 = "-" => ActionFn(22);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 34)
    }
    pub(crate) fn __reduce66<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // OpAssign = "+=" => ActionFn(13);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action13::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 35)
    }
    pub(crate) fn __reduce67<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // OpAssign = "-=" => ActionFn(14);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action14::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 35)
    }
    pub(crate) fn __reduce68<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // OpAssign = "*=" => ActionFn(15);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action15::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 35)
    }
    pub(crate) fn __reduce69<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // OpAssign = "/=" => ActionFn(16);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action16::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 35)
    }
    pub(crate) fn __reduce70<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // OpAssign = "%=" => ActionFn(17);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 35)
    }
    pub(crate) fn __reduce71<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // OpAssign = "&=" => ActionFn(18);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action18::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 35)
    }
    pub(crate) fn __reduce72<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // OpAssign = "|=" => ActionFn(19);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 35)
    }
    pub(crate) fn __reduce73<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Program = ProgramPart+ => ActionFn(2);
        let __sym0 = __pop_Variant24(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 36)
    }
    pub(crate) fn __reduce74<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ProgramPart = "mod", Identifier, "{", Program, "}" => ActionFn(3);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant22(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant16(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action3::<>(input, errs, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (5, 37)
    }
    pub(crate) fn __reduce75<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ProgramPart = StructDefinition => ActionFn(4);
        let __sym0 = __pop_Variant27(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 37)
    }
    pub(crate) fn __reduce76<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ProgramPart+ = ProgramPart => ActionFn(64);
        let __sym0 = __pop_Variant23(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action64::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant24(__nt), __end));
        (1, 38)
    }
    pub(crate) fn __reduce77<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ProgramPart+ = ProgramPart+, ProgramPart => ActionFn(65);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant23(__symbols);
        let __sym0 = __pop_Variant24(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action65::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant24(__nt), __end));
        (2, 38)
    }
    pub(crate) fn __reduce78<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Spanned<Op4> = Op4 => ActionFn(141);
        let __sym0 = __pop_Variant21(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action141::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant25(__nt), __end));
        (1, 39)
    }
    pub(crate) fn __reduce79<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Spanned<Op5> = Op5 => ActionFn(142);
        let __sym0 = __pop_Variant21(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action142::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant25(__nt), __end));
        (1, 40)
    }
    pub(crate) fn __reduce80<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Spanned<OpAssign> = OpAssign => ActionFn(143);
        let __sym0 = __pop_Variant21(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action143::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant25(__nt), __end));
        (1, 41)
    }
    pub(crate) fn __reduce81<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Stmt = Expr, ";" => ActionFn(151);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action151::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant26(__nt), __end));
        (2, 42)
    }
    pub(crate) fn __reduce82<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Stmt = Expr => ActionFn(152);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action152::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant26(__nt), __end));
        (1, 42)
    }
    pub(crate) fn __reduce83<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // StructDefinition = "builtin", "struct", Identifier, ":", Type, "{", Comma<MemberVariable>, "}" => ActionFn(96);
        assert!(__symbols.len() >= 8);
        let __sym7 = __pop_Variant0(__symbols);
        let __sym6 = __pop_Variant15(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant8(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant16(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym7.2.clone();
        let __nt = super::__action96::<>(input, errs, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7);
        __symbols.push((__start, __Symbol::Variant27(__nt), __end));
        (8, 43)
    }
    pub(crate) fn __reduce84<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // StructDefinition = "builtin", "struct", Identifier, "{", Comma<MemberVariable>, "}" => ActionFn(97);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant15(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant16(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action97::<>(input, errs, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant27(__nt), __end));
        (6, 43)
    }
    pub(crate) fn __reduce85<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // StructDefinition = "struct", Identifier, ":", Type, "{", Comma<MemberVariable>, "}" => ActionFn(98);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant15(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant8(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant16(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action98::<>(input, errs, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant27(__nt), __end));
        (7, 43)
    }
    pub(crate) fn __reduce86<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // StructDefinition = "struct", Identifier, "{", Comma<MemberVariable>, "}" => ActionFn(99);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant15(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant16(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action99::<>(input, errs, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant27(__nt), __end));
        (5, 43)
    }
    pub(crate) fn __reduce87<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Type = "bool" => ActionFn(145);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action145::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 44)
    }
    pub(crate) fn __reduce88<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Type = "string" => ActionFn(146);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action146::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 44)
    }
    pub(crate) fn __reduce89<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Type = "int" => ActionFn(147);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action147::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 44)
    }
    pub(crate) fn __reduce90<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Type = "(", ")" => ActionFn(148);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action148::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 44)
    }
    pub(crate) fn __reduce91<
        'input,
        'err,
    >(
        input: &'input str,
        errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Type = ComplexType => ActionFn(149);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action149::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 44)
    }
    pub(crate) fn __reduce92<
        'input,
        'err,
    >(
        input: &'input str,
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
        let __nt = super::__action1::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 45)
    }
}
pub use self::__parse__Program::ProgramParser;

#[allow(unused_variables)]
fn __action0<
    'input,
    'err,
>(
    input: &'input str,
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
fn __action6<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, name, _): (usize, Identifier, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, ty, _): (usize, Type, usize),
) -> MemberVariable
{
    MemberVariable{name, ty}
}

#[allow(unused_variables)]
fn __action7<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, l, _): (usize, usize, usize),
    (_, n, _): (usize, &'input str, usize),
    (_, r, _): (usize, usize, usize),
) -> Identifier
{
    Identifier::new(l, r, n.to_string())
}

#[allow(unused_variables)]
fn __action8<
    'input,
    'err,
>(
    input: &'input str,
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
fn __action9<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(unused_variables)]
fn __action10<
    'input,
    'err,
>(
    input: &'input str,
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
fn __action11<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, l, _): (usize, usize, usize),
    (_, lhs, _): (usize, Expr, usize),
    (_, op, _): (usize, Spanned<BinOpKind>, usize),
    (_, rhs, _): (usize, Expr, usize),
    (_, r, _): (usize, usize, usize),
) -> Expr
{
    Expr::new(l, r, ExprKind::AssignOp(op, bx(lhs), bx(rhs)))
}

#[allow(unused_variables)]
fn __action12<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(unused_variables)]
fn __action13<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> BinOpKind
{
    BinOpKind::Add
}

#[allow(unused_variables)]
fn __action14<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> BinOpKind
{
    BinOpKind::Sub
}

#[allow(unused_variables)]
fn __action15<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> BinOpKind
{
    BinOpKind::Mul
}

#[allow(unused_variables)]
fn __action16<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> BinOpKind
{
    BinOpKind::Div
}

#[allow(unused_variables)]
fn __action17<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> BinOpKind
{
    BinOpKind::Rem
}

#[allow(unused_variables)]
fn __action18<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> BinOpKind
{
    BinOpKind::Rem
}

#[allow(unused_variables)]
fn __action19<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> BinOpKind
{
    BinOpKind::Rem
}

#[allow(unused_variables)]
fn __action20<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(unused_variables)]
fn __action21<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> BinOpKind
{
    BinOpKind::Add
}

#[allow(unused_variables)]
fn __action22<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> BinOpKind
{
    BinOpKind::Sub
}

#[allow(unused_variables)]
fn __action23<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(unused_variables)]
fn __action24<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> BinOpKind
{
    BinOpKind::Mul
}

#[allow(unused_variables)]
fn __action25<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> BinOpKind
{
    BinOpKind::Div
}

#[allow(unused_variables)]
fn __action26<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> BinOpKind
{
    BinOpKind::Rem
}

#[allow(unused_variables)]
fn __action27<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, l, _): (usize, usize, usize),
    (_, lit, _): (usize, LitKind, usize),
    (_, r, _): (usize, usize, usize),
) -> Expr
{
    Expr::new(l, r, ExprKind::Lit(lit))
}

#[allow(unused_variables)]
fn __action28<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, l, _): (usize, usize, usize),
    (_, ident, _): (usize, Identifier, usize),
    (_, r, _): (usize, usize, usize),
) -> Expr
{
    Expr::new(l, r, ExprKind::Variable(ident))
}

#[allow(unused_variables)]
fn __action29<
    'input,
    'err,
>(
    input: &'input str,
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
fn __action30<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, l, _): (usize, usize, usize),
    (_, err, _): (usize, __lalrpop_util::ErrorRecovery<usize, Token<'input>, ParserError>, usize),
    (_, r, _): (usize, usize, usize),
) -> Expr
{
    { errs.push(err); Expr::new(l, r, ExprKind::Err) }
}

#[allow(unused_variables)]
fn __action31<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> LitKind
{
    LitKind::Bool(true)
}

#[allow(unused_variables)]
fn __action32<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> LitKind
{
    LitKind::Bool(false)
}

#[allow(unused_variables)]
fn __action33<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, s, _): (usize, &'input str, usize),
) -> LitKind
{
    LitKind::String(s.to_string())
}

#[allow(unused_variables)]
fn __action34<
    'input,
    'err,
>(
    input: &'input str,
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
fn __action35<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, r, _): (usize, usize, usize),
) -> Type
{
    Type::new(l, r, TypeKind::Bool)
}

#[allow(unused_variables)]
fn __action36<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, r, _): (usize, usize, usize),
) -> Type
{
    Type::new(l, r, TypeKind::String)
}

#[allow(unused_variables)]
fn __action37<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, r, _): (usize, usize, usize),
) -> Type
{
    Type::new(l, r, TypeKind::Int)
}

#[allow(unused_variables)]
fn __action38<
    'input,
    'err,
>(
    input: &'input str,
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
fn __action39<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, l, _): (usize, usize, usize),
    (_, ty, _): (usize, ComplexType, usize),
    (_, r, _): (usize, usize, usize),
) -> Type
{
    Type::new(l, r, TypeKind::Complex(ty))
}

#[allow(unused_variables)]
fn __action40<
    'input,
    'err,
>(
    input: &'input str,
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
fn __action41<
    'input,
    'err,
>(
    input: &'input str,
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
fn __action42<
    'input,
    'err,
>(
    input: &'input str,
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
fn __action43<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, ComplexType, usize),
) -> ComplexType
{
    __0
}

#[allow(unused_variables)]
fn __action44<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, l, _): (usize, usize, usize),
    (_, name, _): (usize, Identifier, usize),
    (_, r, _): (usize, usize, usize),
) -> ComplexType
{
    ComplexType::new(l, r, ComplexTypeKind::Base(name, ComplexReferent::Infer))
}

#[allow(unused_variables)]
fn __action45<
    'input,
    'err,
>(
    input: &'input str,
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
fn __action46<
    'input,
    'err,
>(
    input: &'input str,
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
fn __action47<
    'input,
    'err,
>(
    input: &'input str,
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
fn __action48<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, l, _): (usize, usize, usize),
    (_, lhs, _): (usize, Expr, usize),
    (_, op, _): (usize, Spanned<BinOpKind>, usize),
    (_, rhs, _): (usize, Expr, usize),
    (_, r, _): (usize, usize, usize),
) -> Expr
{
    Expr::new(l, r, ExprKind::BinOp(op, bx(lhs), bx(rhs)))
}

#[allow(unused_variables)]
fn __action49<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(unused_variables)]
fn __action50<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, l, _): (usize, usize, usize),
    (_, lhs, _): (usize, Expr, usize),
    (_, op, _): (usize, Spanned<BinOpKind>, usize),
    (_, rhs, _): (usize, Expr, usize),
    (_, r, _): (usize, usize, usize),
) -> Expr
{
    Expr::new(l, r, ExprKind::BinOp(op, bx(lhs), bx(rhs)))
}

#[allow(unused_variables)]
fn __action51<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(unused_variables)]
fn __action52<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, l, _): (usize, usize, usize),
    (_, val, _): (usize, BinOpKind, usize),
    (_, r, _): (usize, usize, usize),
) -> Spanned<BinOpKind>
{
    Spanned::new(l, r, val)
}

#[allow(unused_variables)]
fn __action53<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, (usize, usize), usize),
) -> ::std::option::Option<(usize, usize)>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action54<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<(usize, usize)>
{
    None
}

#[allow(unused_variables)]
fn __action55<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, usize, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, __1, _): (usize, usize, usize),
) -> (usize, usize)
{
    (__0, __1)
}

#[allow(unused_variables)]
fn __action56<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    __lookbehind.clone()
}

#[allow(unused_variables)]
fn __action57<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    __lookahead.clone()
}

#[allow(unused_variables)]
fn __action58<
    'input,
    'err,
>(
    input: &'input str,
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
fn __action59<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Type, usize),
) -> ::std::option::Option<Type>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action60<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<Type>
{
    None
}

#[allow(unused_variables)]
fn __action61<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, _, _): (usize, Token<'input>, usize),
    (_, __0, _): (usize, Type, usize),
) -> Type
{
    __0
}

#[allow(unused_variables)]
fn __action62<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> ::std::option::Option<Token<'input>>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action63<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<Token<'input>>
{
    None
}

#[allow(unused_variables)]
fn __action64<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, ProgramPart, usize),
) -> ::std::vec::Vec<ProgramPart>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action65<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, v, _): (usize, ::std::vec::Vec<ProgramPart>, usize),
    (_, e, _): (usize, ProgramPart, usize),
) -> ::std::vec::Vec<ProgramPart>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action66<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, MemberVariable, usize),
) -> ::std::option::Option<MemberVariable>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action67<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<MemberVariable>
{
    None
}

#[allow(unused_variables)]
fn __action68<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<MemberVariable>
{
    vec![]
}

#[allow(unused_variables)]
fn __action69<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, v, _): (usize, ::std::vec::Vec<MemberVariable>, usize),
) -> ::std::vec::Vec<MemberVariable>
{
    v
}

#[allow(unused_variables)]
fn __action70<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, MemberVariable, usize),
    (_, _, _): (usize, Token<'input>, usize),
) -> MemberVariable
{
    __0
}

#[allow(unused_variables)]
fn __action71<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, l, _): (usize, usize, usize),
    (_, val, _): (usize, BinOpKind, usize),
    (_, r, _): (usize, usize, usize),
) -> Spanned<BinOpKind>
{
    Spanned::new(l, r, val)
}

#[allow(unused_variables)]
fn __action72<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, l, _): (usize, usize, usize),
    (_, val, _): (usize, BinOpKind, usize),
    (_, r, _): (usize, usize, usize),
) -> Spanned<BinOpKind>
{
    Spanned::new(l, r, val)
}

#[allow(unused_variables)]
fn __action73<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Expr>
{
    vec![]
}

#[allow(unused_variables)]
fn __action74<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, v, _): (usize, ::std::vec::Vec<Expr>, usize),
) -> ::std::vec::Vec<Expr>
{
    v
}

#[allow(unused_variables)]
fn __action75<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, _, _): (usize, Token<'input>, usize),
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(unused_variables)]
fn __action76<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<ComplexType>
{
    vec![]
}

#[allow(unused_variables)]
fn __action77<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, v, _): (usize, ::std::vec::Vec<ComplexType>, usize),
) -> ::std::vec::Vec<ComplexType>
{
    v
}

#[allow(unused_variables)]
fn __action78<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, _, _): (usize, Token<'input>, usize),
    (_, __0, _): (usize, ComplexType, usize),
) -> ComplexType
{
    __0
}

#[allow(unused_variables)]
fn __action79<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, ComplexType, usize),
) -> ::std::vec::Vec<ComplexType>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action80<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, v, _): (usize, ::std::vec::Vec<ComplexType>, usize),
    (_, e, _): (usize, ComplexType, usize),
) -> ::std::vec::Vec<ComplexType>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action81<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, Expr, usize),
) -> ::std::vec::Vec<Expr>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action82<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, v, _): (usize, ::std::vec::Vec<Expr>, usize),
    (_, e, _): (usize, Expr, usize),
) -> ::std::vec::Vec<Expr>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action83<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, __0, _): (usize, MemberVariable, usize),
) -> ::std::vec::Vec<MemberVariable>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action84<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    (_, v, _): (usize, ::std::vec::Vec<MemberVariable>, usize),
    (_, e, _): (usize, MemberVariable, usize),
) -> ::std::vec::Vec<MemberVariable>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action85<
    'input,
    'err,
>(
    input: &'input str,
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
    let __temp0 = __action62(
        input,
        errs,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action5(
        input,
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
fn __action86<
    'input,
    'err,
>(
    input: &'input str,
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
    let __temp0 = __action63(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action5(
        input,
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
fn __action87<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, ComplexType, usize),
) -> ::std::vec::Vec<ComplexType>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action78(
        input,
        errs,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action79(
        input,
        errs,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action88<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, ::std::vec::Vec<ComplexType>, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, ComplexType, usize),
) -> ::std::vec::Vec<ComplexType>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action78(
        input,
        errs,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action80(
        input,
        errs,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action89<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, ComplexType, usize),
) -> Vec<ComplexType>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action76(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action46(
        input,
        errs,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action90<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, ComplexType, usize),
    __1: (usize, ::std::vec::Vec<ComplexType>, usize),
) -> Vec<ComplexType>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action77(
        input,
        errs,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action46(
        input,
        errs,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action91<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, Expr, usize),
) -> ::std::vec::Vec<Expr>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action75(
        input,
        errs,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action81(
        input,
        errs,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action92<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, ::std::vec::Vec<Expr>, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, Expr, usize),
) -> ::std::vec::Vec<Expr>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action75(
        input,
        errs,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action82(
        input,
        errs,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action93<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Expr, usize),
) -> Vec<Expr>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action73(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action47(
        input,
        errs,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action94<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Expr, usize),
    __1: (usize, ::std::vec::Vec<Expr>, usize),
) -> Vec<Expr>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action74(
        input,
        errs,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action47(
        input,
        errs,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action95<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, Type, usize),
) -> ::std::option::Option<Type>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action61(
        input,
        errs,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action59(
        input,
        errs,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action96<
    'input,
    'err,
>(
    input: &'input str,
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
    let __temp0 = __action95(
        input,
        errs,
        __3,
        __4,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action85(
        input,
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
fn __action97<
    'input,
    'err,
>(
    input: &'input str,
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
    let __temp0 = __action60(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action85(
        input,
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
fn __action98<
    'input,
    'err,
>(
    input: &'input str,
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
    let __temp0 = __action95(
        input,
        errs,
        __2,
        __3,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action86(
        input,
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
fn __action99<
    'input,
    'err,
>(
    input: &'input str,
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
    let __temp0 = __action60(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action86(
        input,
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
fn __action100<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, Token<'input>, usize),
) -> (usize, usize)
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action56(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action55(
        input,
        errs,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action101<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, Expr, usize),
    __2: (usize, Spanned<BinOpKind>, usize),
    __3: (usize, Expr, usize),
) -> Expr
{
    let __start0 = __3.2.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action56(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action48(
        input,
        errs,
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action102<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, Expr, usize),
    __2: (usize, Spanned<BinOpKind>, usize),
    __3: (usize, Expr, usize),
) -> Expr
{
    let __start0 = __3.2.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action56(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action50(
        input,
        errs,
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action103<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, Vec<ComplexType>, usize),
) -> ComplexType
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action56(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action40(
        input,
        errs,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action104<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, Identifier, usize),
) -> ComplexType
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action56(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action44(
        input,
        errs,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action105<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, ComplexType, usize),
    __3: (usize, Token<'input>, usize),
) -> ComplexType
{
    let __start0 = __3.2.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action56(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action45(
        input,
        errs,
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action106<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, ComplexType, usize),
) -> ComplexType
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action56(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action41(
        input,
        errs,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action107<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, ComplexType, usize),
) -> ComplexType
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action56(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action42(
        input,
        errs,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action108<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, LitKind, usize),
) -> Expr
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action56(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action27(
        input,
        errs,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action109<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, Identifier, usize),
) -> Expr
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action56(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action28(
        input,
        errs,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action110<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, Vec<Expr>, usize),
    __3: (usize, Token<'input>, usize),
) -> Expr
{
    let __start0 = __3.2.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action56(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action29(
        input,
        errs,
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action111<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, __lalrpop_util::ErrorRecovery<usize, Token<'input>, ParserError>, usize),
) -> Expr
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action56(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action30(
        input,
        errs,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action112<
    'input,
    'err,
>(
    input: &'input str,
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
    let __temp0 = __action56(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    let __temp1 = __action56(
        input,
        errs,
        &__start1,
        &__end1,
    );
    let __temp1 = (__start1, __temp1, __end1);
    __action10(
        input,
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
fn __action113<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, Expr, usize),
    __2: (usize, Spanned<BinOpKind>, usize),
    __3: (usize, Expr, usize),
) -> Expr
{
    let __start0 = __3.2.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action56(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action11(
        input,
        errs,
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action114<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, &'input str, usize),
) -> Identifier
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action56(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action7(
        input,
        errs,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action115<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, &'input str, usize),
) -> LitKind
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action56(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action34(
        input,
        errs,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action116<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, BinOpKind, usize),
) -> Spanned<BinOpKind>
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action56(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action72(
        input,
        errs,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action117<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, BinOpKind, usize),
) -> Spanned<BinOpKind>
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action56(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action71(
        input,
        errs,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action118<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, BinOpKind, usize),
) -> Spanned<BinOpKind>
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action56(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action52(
        input,
        errs,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action119<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, Expr, usize),
    __2: (usize, ::std::option::Option<(usize, usize)>, usize),
) -> Stmt
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action56(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action8(
        input,
        errs,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action120<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, Token<'input>, usize),
) -> Type
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action56(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action35(
        input,
        errs,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action121<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, Token<'input>, usize),
) -> Type
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action56(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action36(
        input,
        errs,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action122<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, Token<'input>, usize),
) -> Type
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action56(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action37(
        input,
        errs,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action123<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, Token<'input>, usize),
) -> Type
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action56(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action38(
        input,
        errs,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action124<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, usize, usize),
    __1: (usize, ComplexType, usize),
) -> Type
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action56(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action39(
        input,
        errs,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action125<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Token<'input>, usize),
) -> (usize, usize)
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action57(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action100(
        input,
        errs,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action126<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Expr, usize),
    __1: (usize, Spanned<BinOpKind>, usize),
    __2: (usize, Expr, usize),
) -> Expr
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action57(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action101(
        input,
        errs,
        __temp0,
        __0,
        __1,
        __2,
    )
}

#[allow(unused_variables)]
fn __action127<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Expr, usize),
    __1: (usize, Spanned<BinOpKind>, usize),
    __2: (usize, Expr, usize),
) -> Expr
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action57(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action102(
        input,
        errs,
        __temp0,
        __0,
        __1,
        __2,
    )
}

#[allow(unused_variables)]
fn __action128<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Vec<ComplexType>, usize),
) -> ComplexType
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action57(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action103(
        input,
        errs,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action129<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Identifier, usize),
) -> ComplexType
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action57(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action104(
        input,
        errs,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action130<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, ComplexType, usize),
    __2: (usize, Token<'input>, usize),
) -> ComplexType
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action57(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action105(
        input,
        errs,
        __temp0,
        __0,
        __1,
        __2,
    )
}

#[allow(unused_variables)]
fn __action131<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, ComplexType, usize),
) -> ComplexType
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action57(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action106(
        input,
        errs,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action132<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, ComplexType, usize),
) -> ComplexType
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action57(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action107(
        input,
        errs,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action133<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, LitKind, usize),
) -> Expr
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action57(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action108(
        input,
        errs,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action134<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Identifier, usize),
) -> Expr
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action57(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action109(
        input,
        errs,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action135<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, Vec<Expr>, usize),
    __2: (usize, Token<'input>, usize),
) -> Expr
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action57(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action110(
        input,
        errs,
        __temp0,
        __0,
        __1,
        __2,
    )
}

#[allow(unused_variables)]
fn __action136<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, __lalrpop_util::ErrorRecovery<usize, Token<'input>, ParserError>, usize),
) -> Expr
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action57(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action111(
        input,
        errs,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action137<
    'input,
    'err,
>(
    input: &'input str,
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
    let __temp0 = __action57(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    let __temp1 = __action57(
        input,
        errs,
        &__start1,
        &__end1,
    );
    let __temp1 = (__start1, __temp1, __end1);
    __action112(
        input,
        errs,
        __temp0,
        __0,
        __temp1,
        __1,
        __2,
    )
}

#[allow(unused_variables)]
fn __action138<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Expr, usize),
    __1: (usize, Spanned<BinOpKind>, usize),
    __2: (usize, Expr, usize),
) -> Expr
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action57(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action113(
        input,
        errs,
        __temp0,
        __0,
        __1,
        __2,
    )
}

#[allow(unused_variables)]
fn __action139<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, &'input str, usize),
) -> Identifier
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action57(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action114(
        input,
        errs,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action140<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, &'input str, usize),
) -> LitKind
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action57(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action115(
        input,
        errs,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action141<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, BinOpKind, usize),
) -> Spanned<BinOpKind>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action57(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action116(
        input,
        errs,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action142<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, BinOpKind, usize),
) -> Spanned<BinOpKind>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action57(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action117(
        input,
        errs,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action143<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, BinOpKind, usize),
) -> Spanned<BinOpKind>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action57(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action118(
        input,
        errs,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action144<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Expr, usize),
    __1: (usize, ::std::option::Option<(usize, usize)>, usize),
) -> Stmt
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action57(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action119(
        input,
        errs,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action145<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Token<'input>, usize),
) -> Type
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action57(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action120(
        input,
        errs,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action146<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Token<'input>, usize),
) -> Type
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action57(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action121(
        input,
        errs,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action147<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Token<'input>, usize),
) -> Type
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action57(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action122(
        input,
        errs,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action148<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, Token<'input>, usize),
) -> Type
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action57(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action123(
        input,
        errs,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action149<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, ComplexType, usize),
) -> Type
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action57(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action124(
        input,
        errs,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action150<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Token<'input>, usize),
) -> ::std::option::Option<(usize, usize)>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action125(
        input,
        errs,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action53(
        input,
        errs,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action151<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Expr, usize),
    __1: (usize, Token<'input>, usize),
) -> Stmt
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action150(
        input,
        errs,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action144(
        input,
        errs,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action152<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, Expr, usize),
) -> Stmt
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action54(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action144(
        input,
        errs,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action153<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, MemberVariable, usize),
    __1: (usize, Token<'input>, usize),
) -> ::std::vec::Vec<MemberVariable>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action70(
        input,
        errs,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action83(
        input,
        errs,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action154<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, ::std::vec::Vec<MemberVariable>, usize),
    __1: (usize, MemberVariable, usize),
    __2: (usize, Token<'input>, usize),
) -> ::std::vec::Vec<MemberVariable>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action70(
        input,
        errs,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action84(
        input,
        errs,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action155<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, ::std::option::Option<MemberVariable>, usize),
) -> Vec<MemberVariable>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action68(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action58(
        input,
        errs,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action156<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, ::std::vec::Vec<MemberVariable>, usize),
    __1: (usize, ::std::option::Option<MemberVariable>, usize),
) -> Vec<MemberVariable>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action69(
        input,
        errs,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action58(
        input,
        errs,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action157<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, MemberVariable, usize),
) -> Vec<MemberVariable>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action66(
        input,
        errs,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action155(
        input,
        errs,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action158<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<MemberVariable>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action67(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action155(
        input,
        errs,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action159<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, ::std::vec::Vec<MemberVariable>, usize),
    __1: (usize, MemberVariable, usize),
) -> Vec<MemberVariable>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action66(
        input,
        errs,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action156(
        input,
        errs,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action160<
    'input,
    'err,
>(
    input: &'input str,
    errs: &'err mut Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    __0: (usize, ::std::vec::Vec<MemberVariable>, usize),
) -> Vec<MemberVariable>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action67(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action156(
        input,
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
