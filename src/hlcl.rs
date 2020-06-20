// auto-generated: "lalrpop 0.19.0"
// sha256: 5e6b01ee5bb55d6e3841193827f7a77252db1b3de95973162298ab641c2b9c
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
        Variant3(ComplexType),
        Variant4(::std::vec::Vec<ComplexType>),
        Variant5(Expr),
        Variant6(::std::vec::Vec<Expr>),
        Variant7(Type),
        Variant8(::std::option::Option<Type>),
        Variant9((usize, usize)),
        Variant10(::std::option::Option<(usize, usize)>),
        Variant11(MemberVariable),
        Variant12(::std::vec::Vec<MemberVariable>),
        Variant13(usize),
        Variant14(Vec<MemberVariable>),
        Variant15(Identifier),
        Variant16(LitKind),
        Variant17(Vec<ComplexType>),
        Variant18(Vec<Expr>),
        Variant19(::std::option::Option<MemberVariable>),
        Variant20(Vec<ProgramPart>),
        Variant21(ProgramPart),
        Variant22(::std::vec::Vec<ProgramPart>),
        Variant23(Stmt),
        Variant24(StructDefinition),
    }
    const __ACTION: &[i8] = &[
        // State 0
        0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 12, 13, 14, 15,
        // State 1
        0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 12, 13, 14, 15,
        // State 2
        0, 0, 0, -46, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 12, 13, 14, 15,
        // State 4
        0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 12, 13, 14, 15,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, -34, 0, -34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, -36, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, -35, 0, -35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, -41, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, -40, 0, -40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, -39, 0, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, -43, 0, -43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, -42, 0, -42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, -38, 0, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, -47, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, -37, 0, -37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, -9, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, -10, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 35 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        0,
        // State 2
        0,
        // State 3
        0,
        // State 4
        0,
        // State 5
        -64,
        // State 6
        -34,
        // State 7
        -36,
        // State 8
        -35,
        // State 9
        -41,
        // State 10
        -40,
        // State 11
        -39,
        // State 12
        -43,
        // State 13
        -42,
        // State 14
        -38,
        // State 15
        0,
        // State 16
        0,
        // State 17
        -37,
        // State 18
        0,
        // State 19
        0,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            5 => 16,
            19 => match state {
                0 => 5,
                3 => 18,
                4 => 19,
                _ => 2,
            },
            20 => 6,
            21 => 7,
            22 => 8,
            24 => 15,
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> Vec<::std::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""!""###,
            r###""#""###,
            r###""(""###,
            r###"")""###,
            r###""+""###,
            r###"",""###,
            r###"".""###,
            r###"".""###,
            r###"":""###,
            r###"";""###,
            r###""[""###,
            r###""]""###,
            r###""^""###,
            r###""bool""###,
            r###""const""###,
            r###""do""###,
            r###""else""###,
            r###""enum""###,
            r###""false""###,
            r###""fn""###,
            r###""if""###,
            r###""int""###,
            r###""let""###,
            r###""return""###,
            r###""string""###,
            r###""struct""###,
            r###""trait""###,
            r###""true""###,
            r###""while""###,
            r###""{""###,
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
            __action(state, 35 - 1)
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
            Token::Hash if true => Some(1),
            Token::OpenParen if true => Some(2),
            Token::CloseParen if true => Some(3),
            Token::Plus if true => Some(4),
            Token::Comma if true => Some(5),
            Token::Dot if true => Some(6),
            Token::Dot if true => Some(7),
            Token::Colon if true => Some(8),
            Token::Semicolon if true => Some(9),
            Token::OpenBracket if true => Some(10),
            Token::CloseBracket if true => Some(11),
            Token::Caret if true => Some(12),
            Token::Bool if true => Some(13),
            Token::Const if true => Some(14),
            Token::Do if true => Some(15),
            Token::Else if true => Some(16),
            Token::Enum if true => Some(17),
            Token::False if true => Some(18),
            Token::Fn if true => Some(19),
            Token::If if true => Some(20),
            Token::Int if true => Some(21),
            Token::Let if true => Some(22),
            Token::Return if true => Some(23),
            Token::String if true => Some(24),
            Token::Struct if true => Some(25),
            Token::Trait if true => Some(26),
            Token::True if true => Some(27),
            Token::While if true => Some(28),
            Token::OpenBlock if true => Some(29),
            Token::CloseBlock if true => Some(30),
            Token::Identifier(_) if true => Some(31),
            Token::IntLiteral(_) if true => Some(32),
            Token::StringLiteral(_) if true => Some(33),
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
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 => __Symbol::Variant0(__token),
            31 | 32 | 33 => match __token {
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
                    states_to_pop: 2,
                    nonterminal_produced: 0,
                }
            }
            1 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 1,
                }
            }
            2 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 1,
                }
            }
            3 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 2,
                }
            }
            4 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
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
                    states_to_pop: 0,
                    nonterminal_produced: 4,
                }
            }
            7 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            8 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 5,
                }
            }
            9 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
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
                    states_to_pop: 2,
                    nonterminal_produced: 7,
                }
            }
            12 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 7,
                }
            }
            13 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 8,
                }
            }
            14 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 9,
                }
            }
            15 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 9,
                }
            }
            16 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 10,
                }
            }
            17 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 11,
                }
            }
            18 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 11,
                }
            }
            19 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 12,
                }
            }
            20 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 12,
                }
            }
            21 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 13,
                }
            }
            22 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 14,
                }
            }
            23 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
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
                    states_to_pop: 2,
                    nonterminal_produced: 15,
                }
            }
            26 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
                }
            }
            27 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 16,
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
                    states_to_pop: 3,
                    nonterminal_produced: 17,
                }
            }
            30 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
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
                    states_to_pop: 1,
                    nonterminal_produced: 20,
                }
            }
            36 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 20,
                }
            }
            37 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 20,
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
                    nonterminal_produced: 22,
                }
            }
            41 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 22,
                }
            }
            42 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 22,
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
                    states_to_pop: 2,
                    nonterminal_produced: 23,
                }
            }
            45 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 24,
                }
            }
            46 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 24,
                }
            }
            47 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 25,
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
                    states_to_pop: 0,
                    nonterminal_produced: 26,
                }
            }
            50 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 27,
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
                    states_to_pop: 2,
                    nonterminal_produced: 29,
                }
            }
            54 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 30,
                }
            }
            55 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 30,
                }
            }
            56 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 7,
                    nonterminal_produced: 31,
                }
            }
            57 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 31,
                }
            }
            58 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 32,
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
                    states_to_pop: 1,
                    nonterminal_produced: 32,
                }
            }
            61 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 32,
                }
            }
            62 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 32,
                }
            }
            63 => __state_machine::SimulatedReduce::Accept,
            64 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 34,
                }
            }
            65 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 35,
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
                // __Expr = Expr => ActionFn(2);
                let __sym0 = __pop_Variant5(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(input, errs, __sym0);
                return Some(Ok(__nt));
            }
            64 => {
                __reduce64(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            65 => {
                __reduce65(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
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
    fn __pop_Variant9<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (usize, usize), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant9(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ComplexType, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expr, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant15<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Identifier, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant15(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant16<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, LitKind, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant16(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant11<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, MemberVariable, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant11(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant21<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ProgramPart, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant21(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Stmt, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant23(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant24<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, StructDefinition, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant24(__v), __r) => (__l, __v, __r),
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
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Type, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant7(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant17<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<ComplexType>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant17(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant18<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Expr>, usize)
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
    ) -> (usize, Vec<MemberVariable>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant14(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant20<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<ProgramPart>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant20(__v), __r) => (__l, __v, __r),
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
    fn __pop_Variant13<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant13(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<(usize, usize)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant10(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant19<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<MemberVariable>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant19(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Type>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant8(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<ComplexType>, usize)
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
    ) -> (usize, ::std::vec::Vec<Expr>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant6(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant12<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<MemberVariable>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant12(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<ProgramPart>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant22(__v), __r) => (__l, __v, __r),
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
        // ("+" <ComplexType1>) = "+", ComplexType1 => ActionFn(52);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action52::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 0)
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
        // ("+" <ComplexType1>)* =  => ActionFn(50);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action50::<>(input, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (0, 1)
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
        // ("+" <ComplexType1>)* = ("+" <ComplexType1>)+ => ActionFn(51);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action51::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 1)
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
        // ("+" <ComplexType1>)+ = "+", ComplexType1 => ActionFn(59);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action59::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 2)
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
        // ("+" <ComplexType1>)+ = ("+" <ComplexType1>)+, "+", ComplexType1 => ActionFn(60);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action60::<>(input, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 2)
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
        // ("," <Expr>) = ",", Expr => ActionFn(49);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action49::<>(input, errs, __sym0, __sym1);
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
        // ("," <Expr>)* =  => ActionFn(47);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action47::<>(input, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 4)
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
        // ("," <Expr>)* = ("," <Expr>)+ => ActionFn(48);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action48::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 4)
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
        // ("," <Expr>)+ = ",", Expr => ActionFn(63);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action63::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 5)
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
        // ("," <Expr>)+ = ("," <Expr>)+, ",", Expr => ActionFn(64);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action64::<>(input, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 5)
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
        // (":" <Type>) = ":", Type => ActionFn(39);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action39::<>(input, errs, __sym0, __sym1);
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
        // (":" <Type>)? = ":", Type => ActionFn(67);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action67::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 7)
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
        // (":" <Type>)? =  => ActionFn(38);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action38::<>(input, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 7)
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
        // (<@L> ";" <@R>) = ";" => ActionFn(88);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action88::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 8)
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
        // (<@L> ";" <@R>)? = ";" => ActionFn(106);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action106::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 9)
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
        // (<@L> ";" <@R>)? =  => ActionFn(32);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action32::<>(input, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (0, 9)
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
        // (<MemberVariable> ",") = MemberVariable, "," => ActionFn(46);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action46::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (2, 10)
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
        // (<MemberVariable> ",")* =  => ActionFn(44);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action44::<>(input, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (0, 11)
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
        // (<MemberVariable> ",")* = (<MemberVariable> ",")+ => ActionFn(45);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action45::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 11)
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
        // (<MemberVariable> ",")+ = MemberVariable, "," => ActionFn(109);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action109::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (2, 12)
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
        // (<MemberVariable> ",")+ = (<MemberVariable> ",")+, MemberVariable, "," => ActionFn(110);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action110::<>(input, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (3, 12)
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
        // @L =  => ActionFn(35);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action35::<>(input, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (0, 13)
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
        // @R =  => ActionFn(34);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action34::<>(input, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (0, 14)
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
        // Comma<MemberVariable> = MemberVariable => ActionFn(113);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action113::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 15)
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
        // Comma<MemberVariable> =  => ActionFn(114);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action114::<>(input, errs, &__start, &__end);
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
        // Comma<MemberVariable> = (<MemberVariable> ",")+, MemberVariable => ActionFn(115);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action115::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (2, 15)
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
        // Comma<MemberVariable> = (<MemberVariable> ",")+ => ActionFn(116);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action116::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 15)
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
        // ComplexType = Many1<"+", ComplexType1> => ActionFn(89);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action89::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 16)
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
        // ComplexType0 = Identifier => ActionFn(90);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action90::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
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
        // ComplexType0 = "(", ComplexType, ")" => ActionFn(91);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action91::<>(input, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 17)
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
        // ComplexType1 = "^", ComplexType0 => ActionFn(92);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action92::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 18)
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
        // ComplexType1 = "!", ComplexType0 => ActionFn(93);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action93::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
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
        // ComplexType1 = ComplexType0 => ActionFn(26);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
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
        // Expr = Expr0 => ActionFn(9);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action9::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
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
        // Expr0 = Lit => ActionFn(94);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action94::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
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
        // Expr0 = Identifier => ActionFn(95);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action95::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 20)
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
        // Expr0 = "(", Many1<",", Expr>, ")" => ActionFn(96);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant18(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action96::<>(input, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 20)
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
        // Expr0 = error => ActionFn(97);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action97::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 20)
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
        // Identifier = TokenIdentifier => ActionFn(98);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action98::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
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
        // Lit = "true" => ActionFn(14);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action14::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
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
        // Lit = "false" => ActionFn(15);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action15::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 22)
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
        // Lit = TokenString => ActionFn(16);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action16::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 22)
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
        // Lit = TokenInt => ActionFn(99);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action99::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 22)
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
        // Many1<"+", ComplexType1> = ComplexType1 => ActionFn(61);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action61::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
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
        // Many1<"+", ComplexType1> = ComplexType1, ("+" <ComplexType1>)+ => ActionFn(62);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action62::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (2, 23)
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
        // Many1<",", Expr> = Expr => ActionFn(65);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action65::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 24)
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
        // Many1<",", Expr> = Expr, ("," <Expr>)+ => ActionFn(66);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action66::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (2, 24)
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
        // MemberVariable = Identifier, ":", Type => ActionFn(6);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action6::<>(input, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (3, 25)
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
        // MemberVariable? = MemberVariable => ActionFn(42);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action42::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
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
        // MemberVariable? =  => ActionFn(43);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action43::<>(input, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (0, 26)
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
        // Program = ProgramPart+ => ActionFn(3);
        let __sym0 = __pop_Variant22(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 27)
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
        // ProgramPart = StructDefinition => ActionFn(4);
        let __sym0 = __pop_Variant24(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
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
        // ProgramPart+ = ProgramPart => ActionFn(40);
        let __sym0 = __pop_Variant21(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action40::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 29)
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
        // ProgramPart+ = ProgramPart+, ProgramPart => ActionFn(41);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant21(__symbols);
        let __sym0 = __pop_Variant22(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action41::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (2, 29)
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
        // Stmt = Expr, ";" => ActionFn(107);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action107::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (2, 30)
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
        // Stmt = Expr => ActionFn(108);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action108::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 30)
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
        // StructDefinition = "struct", Identifier, ":", Type, "{", Comma<MemberVariable>, "}" => ActionFn(68);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant14(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant7(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant15(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action68::<>(input, errs, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant24(__nt), __end));
        (7, 31)
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
        // StructDefinition = "struct", Identifier, "{", Comma<MemberVariable>, "}" => ActionFn(69);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant14(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant15(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action69::<>(input, errs, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant24(__nt), __end));
        (5, 31)
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
        // Type = "bool" => ActionFn(101);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action101::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 32)
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
        // Type = "string" => ActionFn(102);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action102::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
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
        // Type = "int" => ActionFn(103);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action103::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 32)
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
        // Type = "(", ")" => ActionFn(104);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action104::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 32)
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
        // Type = ComplexType => ActionFn(105);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action105::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 32)
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
        // __Program = Program => ActionFn(0);
        let __sym0 = __pop_Variant20(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
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
        // __Stmt = Stmt => ActionFn(1);
        let __sym0 = __pop_Variant23(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 35)
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
        Variant3(ComplexType),
        Variant4(::std::vec::Vec<ComplexType>),
        Variant5(Expr),
        Variant6(::std::vec::Vec<Expr>),
        Variant7(Type),
        Variant8(::std::option::Option<Type>),
        Variant9((usize, usize)),
        Variant10(::std::option::Option<(usize, usize)>),
        Variant11(MemberVariable),
        Variant12(::std::vec::Vec<MemberVariable>),
        Variant13(usize),
        Variant14(Vec<MemberVariable>),
        Variant15(Identifier),
        Variant16(LitKind),
        Variant17(Vec<ComplexType>),
        Variant18(Vec<Expr>),
        Variant19(::std::option::Option<MemberVariable>),
        Variant20(Vec<ProgramPart>),
        Variant21(ProgramPart),
        Variant22(::std::vec::Vec<ProgramPart>),
        Variant23(Stmt),
        Variant24(StructDefinition),
    }
    const __ACTION: &[i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 21, 0, 0, 0,
        // State 3
        7, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 27, 0, 0, 0, 0, 0, 0, 0, 28, 0, 0, 29, 0, 0, 0, 0, 0, 0, 21, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -25, 21, 0, 0, 0,
        // State 5
        0, 0, 0, -44, 11, -44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -44, -44, 0, 0, 0, 0,
        // State 6
        0, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 21, 0, 0, 0,
        // State 7
        7, 0, 13, 36, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 21, 0, 0, 0,
        // State 8
        0, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 21, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -27, 21, 0, 0, 0,
        // State 10
        7, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 21, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -25, 21, 0, 0, 0,
        // State 12
        7, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 21, 0, 0, 0,
        // State 13
        7, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 27, 0, 0, 0, 0, 0, 0, 0, 28, 0, 0, 29, 0, 0, 0, 0, 0, 0, 21, 0, 0, 0,
        // State 14
        7, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 21, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -53, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -52, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -54, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, -39, -39, -39, 0, 0, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -39, -39, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, -63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -63, -63, 0, 0, 0, 0,
        // State 22
        0, 0, 0, -33, -33, -33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -33, -33, 0, 0, 0, 0,
        // State 23
        0, 0, 0, -29, -29, -29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -29, -29, 0, 0, 0, 0,
        // State 24
        0, 0, 0, -28, 0, -28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -28, -28, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, -59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -59, -59, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, -61, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -61, -61, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, -60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -60, -60, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -24, 0, 0, 0, 0,
        // State 32
        0, 0, 0, -45, 15, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -45, -45, 0, 0, 0, 0,
        // State 33
        0, 0, 0, -32, -32, -32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -32, -32, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, -62, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -62, -62, 0, 0, 0, 0,
        // State 36
        0, 0, 0, -31, -31, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -31, -31, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -26, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -58, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -20, -20, 0, 0, 0,
        // State 40
        0, 0, 0, -4, -4, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, -4, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 47, 0, 0, 0, 0,
        // State 42
        0, 0, 0, -30, -30, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -30, -30, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -21, -21, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, -48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -48, 0, 0, 0, 0,
        // State 45
        0, 0, 0, -5, -5, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -57, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 35 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        -51,
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
        -65,
        // State 16
        -53,
        // State 17
        -52,
        // State 18
        -54,
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
        -58,
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
        -57,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            2 => 32,
            12 => 9,
            15 => match state {
                11 => 41,
                _ => 29,
            },
            16 => match state {
                7 | 12 => 34,
                _ => 21,
            },
            17 => match state {
                6 => 33,
                8 => 36,
                _ => 22,
            },
            18 => match state {
                10 => 40,
                14 => 45,
                _ => 5,
            },
            21 => match state {
                2 => 19,
                4 | 9 | 11 => 30,
                _ => 23,
            },
            23 => 24,
            25 => match state {
                9 => 37,
                _ => 31,
            },
            27 => 15,
            28 => match state {
                1 => 18,
                _ => 16,
            },
            29 => 1,
            31 => 17,
            32 => match state {
                13 => 44,
                _ => 25,
            },
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> Vec<::std::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""!""###,
            r###""#""###,
            r###""(""###,
            r###"")""###,
            r###""+""###,
            r###"",""###,
            r###"".""###,
            r###"".""###,
            r###"":""###,
            r###"";""###,
            r###""[""###,
            r###""]""###,
            r###""^""###,
            r###""bool""###,
            r###""const""###,
            r###""do""###,
            r###""else""###,
            r###""enum""###,
            r###""false""###,
            r###""fn""###,
            r###""if""###,
            r###""int""###,
            r###""let""###,
            r###""return""###,
            r###""string""###,
            r###""struct""###,
            r###""trait""###,
            r###""true""###,
            r###""while""###,
            r###""{""###,
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
            __action(state, 35 - 1)
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
            Token::Hash if true => Some(1),
            Token::OpenParen if true => Some(2),
            Token::CloseParen if true => Some(3),
            Token::Plus if true => Some(4),
            Token::Comma if true => Some(5),
            Token::Dot if true => Some(6),
            Token::Dot if true => Some(7),
            Token::Colon if true => Some(8),
            Token::Semicolon if true => Some(9),
            Token::OpenBracket if true => Some(10),
            Token::CloseBracket if true => Some(11),
            Token::Caret if true => Some(12),
            Token::Bool if true => Some(13),
            Token::Const if true => Some(14),
            Token::Do if true => Some(15),
            Token::Else if true => Some(16),
            Token::Enum if true => Some(17),
            Token::False if true => Some(18),
            Token::Fn if true => Some(19),
            Token::If if true => Some(20),
            Token::Int if true => Some(21),
            Token::Let if true => Some(22),
            Token::Return if true => Some(23),
            Token::String if true => Some(24),
            Token::Struct if true => Some(25),
            Token::Trait if true => Some(26),
            Token::True if true => Some(27),
            Token::While if true => Some(28),
            Token::OpenBlock if true => Some(29),
            Token::CloseBlock if true => Some(30),
            Token::Identifier(_) if true => Some(31),
            Token::IntLiteral(_) if true => Some(32),
            Token::StringLiteral(_) if true => Some(33),
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
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 => __Symbol::Variant0(__token),
            31 | 32 | 33 => match __token {
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
                    states_to_pop: 2,
                    nonterminal_produced: 0,
                }
            }
            1 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 1,
                }
            }
            2 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 1,
                }
            }
            3 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 2,
                }
            }
            4 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
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
                    states_to_pop: 0,
                    nonterminal_produced: 4,
                }
            }
            7 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            8 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 5,
                }
            }
            9 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
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
                    states_to_pop: 2,
                    nonterminal_produced: 7,
                }
            }
            12 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 7,
                }
            }
            13 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 8,
                }
            }
            14 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 9,
                }
            }
            15 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 9,
                }
            }
            16 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 10,
                }
            }
            17 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 11,
                }
            }
            18 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 11,
                }
            }
            19 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 12,
                }
            }
            20 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 12,
                }
            }
            21 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 13,
                }
            }
            22 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 14,
                }
            }
            23 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
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
                    states_to_pop: 2,
                    nonterminal_produced: 15,
                }
            }
            26 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
                }
            }
            27 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 16,
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
                    states_to_pop: 3,
                    nonterminal_produced: 17,
                }
            }
            30 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
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
                    states_to_pop: 1,
                    nonterminal_produced: 20,
                }
            }
            36 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 20,
                }
            }
            37 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 20,
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
                    nonterminal_produced: 22,
                }
            }
            41 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 22,
                }
            }
            42 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 22,
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
                    states_to_pop: 2,
                    nonterminal_produced: 23,
                }
            }
            45 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 24,
                }
            }
            46 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 24,
                }
            }
            47 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 25,
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
                    states_to_pop: 0,
                    nonterminal_produced: 26,
                }
            }
            50 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 27,
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
                    states_to_pop: 2,
                    nonterminal_produced: 29,
                }
            }
            54 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 30,
                }
            }
            55 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 30,
                }
            }
            56 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 7,
                    nonterminal_produced: 31,
                }
            }
            57 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 31,
                }
            }
            58 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 32,
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
                    states_to_pop: 1,
                    nonterminal_produced: 32,
                }
            }
            61 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 32,
                }
            }
            62 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 32,
                }
            }
            63 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 33,
                }
            }
            64 => __state_machine::SimulatedReduce::Accept,
            65 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 35,
                }
            }
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
                // __Program = Program => ActionFn(0);
                let __sym0 = __pop_Variant20(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, errs, __sym0);
                return Some(Ok(__nt));
            }
            65 => {
                __reduce65(input, errs, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
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
    fn __pop_Variant9<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (usize, usize), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant9(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ComplexType, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expr, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant15<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Identifier, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant15(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant16<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, LitKind, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant16(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant11<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, MemberVariable, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant11(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant21<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ProgramPart, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant21(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Stmt, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant23(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant24<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, StructDefinition, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant24(__v), __r) => (__l, __v, __r),
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
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Type, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant7(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant17<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<ComplexType>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant17(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant18<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Expr>, usize)
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
    ) -> (usize, Vec<MemberVariable>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant14(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant20<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<ProgramPart>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant20(__v), __r) => (__l, __v, __r),
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
    fn __pop_Variant13<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant13(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<(usize, usize)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant10(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant19<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<MemberVariable>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant19(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Type>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant8(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<ComplexType>, usize)
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
    ) -> (usize, ::std::vec::Vec<Expr>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant6(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant12<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<MemberVariable>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant12(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<ProgramPart>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant22(__v), __r) => (__l, __v, __r),
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
        // ("+" <ComplexType1>) = "+", ComplexType1 => ActionFn(52);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action52::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 0)
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
        // ("+" <ComplexType1>)* =  => ActionFn(50);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action50::<>(input, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (0, 1)
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
        // ("+" <ComplexType1>)* = ("+" <ComplexType1>)+ => ActionFn(51);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action51::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 1)
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
        // ("+" <ComplexType1>)+ = "+", ComplexType1 => ActionFn(59);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action59::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 2)
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
        // ("+" <ComplexType1>)+ = ("+" <ComplexType1>)+, "+", ComplexType1 => ActionFn(60);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action60::<>(input, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 2)
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
        // ("," <Expr>) = ",", Expr => ActionFn(49);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action49::<>(input, errs, __sym0, __sym1);
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
        // ("," <Expr>)* =  => ActionFn(47);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action47::<>(input, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 4)
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
        // ("," <Expr>)* = ("," <Expr>)+ => ActionFn(48);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action48::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 4)
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
        // ("," <Expr>)+ = ",", Expr => ActionFn(63);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action63::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 5)
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
        // ("," <Expr>)+ = ("," <Expr>)+, ",", Expr => ActionFn(64);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action64::<>(input, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 5)
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
        // (":" <Type>) = ":", Type => ActionFn(39);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action39::<>(input, errs, __sym0, __sym1);
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
        // (":" <Type>)? = ":", Type => ActionFn(67);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action67::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 7)
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
        // (":" <Type>)? =  => ActionFn(38);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action38::<>(input, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 7)
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
        // (<@L> ";" <@R>) = ";" => ActionFn(88);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action88::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 8)
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
        // (<@L> ";" <@R>)? = ";" => ActionFn(106);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action106::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 9)
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
        // (<@L> ";" <@R>)? =  => ActionFn(32);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action32::<>(input, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (0, 9)
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
        // (<MemberVariable> ",") = MemberVariable, "," => ActionFn(46);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action46::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (2, 10)
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
        // (<MemberVariable> ",")* =  => ActionFn(44);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action44::<>(input, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (0, 11)
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
        // (<MemberVariable> ",")* = (<MemberVariable> ",")+ => ActionFn(45);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action45::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 11)
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
        // (<MemberVariable> ",")+ = MemberVariable, "," => ActionFn(109);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action109::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (2, 12)
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
        // (<MemberVariable> ",")+ = (<MemberVariable> ",")+, MemberVariable, "," => ActionFn(110);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action110::<>(input, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (3, 12)
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
        // @L =  => ActionFn(35);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action35::<>(input, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (0, 13)
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
        // @R =  => ActionFn(34);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action34::<>(input, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (0, 14)
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
        // Comma<MemberVariable> = MemberVariable => ActionFn(113);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action113::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 15)
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
        // Comma<MemberVariable> =  => ActionFn(114);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action114::<>(input, errs, &__start, &__end);
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
        // Comma<MemberVariable> = (<MemberVariable> ",")+, MemberVariable => ActionFn(115);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action115::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (2, 15)
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
        // Comma<MemberVariable> = (<MemberVariable> ",")+ => ActionFn(116);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action116::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 15)
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
        // ComplexType = Many1<"+", ComplexType1> => ActionFn(89);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action89::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 16)
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
        // ComplexType0 = Identifier => ActionFn(90);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action90::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
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
        // ComplexType0 = "(", ComplexType, ")" => ActionFn(91);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action91::<>(input, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 17)
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
        // ComplexType1 = "^", ComplexType0 => ActionFn(92);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action92::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 18)
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
        // ComplexType1 = "!", ComplexType0 => ActionFn(93);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action93::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
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
        // ComplexType1 = ComplexType0 => ActionFn(26);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
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
        // Expr = Expr0 => ActionFn(9);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action9::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
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
        // Expr0 = Lit => ActionFn(94);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action94::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
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
        // Expr0 = Identifier => ActionFn(95);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action95::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 20)
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
        // Expr0 = "(", Many1<",", Expr>, ")" => ActionFn(96);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant18(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action96::<>(input, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 20)
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
        // Expr0 = error => ActionFn(97);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action97::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 20)
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
        // Identifier = TokenIdentifier => ActionFn(98);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action98::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
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
        // Lit = "true" => ActionFn(14);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action14::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
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
        // Lit = "false" => ActionFn(15);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action15::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 22)
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
        // Lit = TokenString => ActionFn(16);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action16::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 22)
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
        // Lit = TokenInt => ActionFn(99);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action99::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 22)
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
        // Many1<"+", ComplexType1> = ComplexType1 => ActionFn(61);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action61::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
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
        // Many1<"+", ComplexType1> = ComplexType1, ("+" <ComplexType1>)+ => ActionFn(62);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action62::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (2, 23)
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
        // Many1<",", Expr> = Expr => ActionFn(65);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action65::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 24)
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
        // Many1<",", Expr> = Expr, ("," <Expr>)+ => ActionFn(66);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action66::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (2, 24)
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
        // MemberVariable = Identifier, ":", Type => ActionFn(6);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action6::<>(input, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (3, 25)
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
        // MemberVariable? = MemberVariable => ActionFn(42);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action42::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
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
        // MemberVariable? =  => ActionFn(43);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action43::<>(input, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (0, 26)
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
        // Program = ProgramPart+ => ActionFn(3);
        let __sym0 = __pop_Variant22(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 27)
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
        // ProgramPart = StructDefinition => ActionFn(4);
        let __sym0 = __pop_Variant24(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
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
        // ProgramPart+ = ProgramPart => ActionFn(40);
        let __sym0 = __pop_Variant21(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action40::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 29)
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
        // ProgramPart+ = ProgramPart+, ProgramPart => ActionFn(41);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant21(__symbols);
        let __sym0 = __pop_Variant22(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action41::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (2, 29)
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
        // Stmt = Expr, ";" => ActionFn(107);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action107::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (2, 30)
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
        // Stmt = Expr => ActionFn(108);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action108::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 30)
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
        // StructDefinition = "struct", Identifier, ":", Type, "{", Comma<MemberVariable>, "}" => ActionFn(68);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant14(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant7(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant15(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action68::<>(input, errs, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant24(__nt), __end));
        (7, 31)
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
        // StructDefinition = "struct", Identifier, "{", Comma<MemberVariable>, "}" => ActionFn(69);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant14(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant15(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action69::<>(input, errs, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant24(__nt), __end));
        (5, 31)
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
        // Type = "bool" => ActionFn(101);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action101::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 32)
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
        // Type = "string" => ActionFn(102);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action102::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
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
        // Type = "int" => ActionFn(103);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action103::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 32)
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
        // Type = "(", ")" => ActionFn(104);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action104::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 32)
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
        // Type = ComplexType => ActionFn(105);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action105::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 32)
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
        // __Expr = Expr => ActionFn(2);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 33)
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
        // __Stmt = Stmt => ActionFn(1);
        let __sym0 = __pop_Variant23(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 35)
    }
}
pub use self::__parse__Program::ProgramParser;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Stmt {
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
        Variant3(ComplexType),
        Variant4(::std::vec::Vec<ComplexType>),
        Variant5(Expr),
        Variant6(::std::vec::Vec<Expr>),
        Variant7(Type),
        Variant8(::std::option::Option<Type>),
        Variant9((usize, usize)),
        Variant10(::std::option::Option<(usize, usize)>),
        Variant11(MemberVariable),
        Variant12(::std::vec::Vec<MemberVariable>),
        Variant13(usize),
        Variant14(Vec<MemberVariable>),
        Variant15(Identifier),
        Variant16(LitKind),
        Variant17(Vec<ComplexType>),
        Variant18(Vec<Expr>),
        Variant19(::std::option::Option<MemberVariable>),
        Variant20(Vec<ProgramPart>),
        Variant21(ProgramPart),
        Variant22(::std::vec::Vec<ProgramPart>),
        Variant23(Stmt),
        Variant24(StructDefinition),
    }
    const __ACTION: &[i8] = &[
        // State 0
        0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 13, 14, 15, 16,
        // State 1
        0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 13, 14, 15, 16,
        // State 2
        0, 0, 0, -46, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 13, 14, 15, 16,
        // State 4
        0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 13, 14, 15, 16,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, -34, 0, -34, 0, 0, 0, -34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, -36, 0, -36, 0, 0, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, -35, 0, -35, 0, 0, 0, -35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, -41, 0, -41, 0, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, -40, 0, -40, 0, 0, 0, -40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, -39, 0, -39, 0, 0, 0, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, -43, 0, -43, 0, 0, 0, -43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, -42, 0, -42, 0, 0, 0, -42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, -38, 0, -38, 0, 0, 0, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, -47, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, -37, 0, -37, 0, 0, 0, -37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, -9, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, -10, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 35 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        0,
        // State 2
        0,
        // State 3
        0,
        // State 4
        0,
        // State 5
        -56,
        // State 6
        -34,
        // State 7
        -36,
        // State 8
        -35,
        // State 9
        -66,
        // State 10
        -41,
        // State 11
        -40,
        // State 12
        -39,
        // State 13
        -43,
        // State 14
        -42,
        // State 15
        -38,
        // State 16
        -55,
        // State 17
        0,
        // State 18
        0,
        // State 19
        -37,
        // State 20
        0,
        // State 21
        0,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            5 => 18,
            19 => match state {
                0 => 5,
                3 => 20,
                4 => 21,
                _ => 2,
            },
            20 => 6,
            21 => 7,
            22 => 8,
            24 => 17,
            30 => 9,
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> Vec<::std::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""!""###,
            r###""#""###,
            r###""(""###,
            r###"")""###,
            r###""+""###,
            r###"",""###,
            r###"".""###,
            r###"".""###,
            r###"":""###,
            r###"";""###,
            r###""[""###,
            r###""]""###,
            r###""^""###,
            r###""bool""###,
            r###""const""###,
            r###""do""###,
            r###""else""###,
            r###""enum""###,
            r###""false""###,
            r###""fn""###,
            r###""if""###,
            r###""int""###,
            r###""let""###,
            r###""return""###,
            r###""string""###,
            r###""struct""###,
            r###""trait""###,
            r###""true""###,
            r###""while""###,
            r###""{""###,
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
        type Success = Stmt;
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
            __action(state, 35 - 1)
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
            Token::Hash if true => Some(1),
            Token::OpenParen if true => Some(2),
            Token::CloseParen if true => Some(3),
            Token::Plus if true => Some(4),
            Token::Comma if true => Some(5),
            Token::Dot if true => Some(6),
            Token::Dot if true => Some(7),
            Token::Colon if true => Some(8),
            Token::Semicolon if true => Some(9),
            Token::OpenBracket if true => Some(10),
            Token::CloseBracket if true => Some(11),
            Token::Caret if true => Some(12),
            Token::Bool if true => Some(13),
            Token::Const if true => Some(14),
            Token::Do if true => Some(15),
            Token::Else if true => Some(16),
            Token::Enum if true => Some(17),
            Token::False if true => Some(18),
            Token::Fn if true => Some(19),
            Token::If if true => Some(20),
            Token::Int if true => Some(21),
            Token::Let if true => Some(22),
            Token::Return if true => Some(23),
            Token::String if true => Some(24),
            Token::Struct if true => Some(25),
            Token::Trait if true => Some(26),
            Token::True if true => Some(27),
            Token::While if true => Some(28),
            Token::OpenBlock if true => Some(29),
            Token::CloseBlock if true => Some(30),
            Token::Identifier(_) if true => Some(31),
            Token::IntLiteral(_) if true => Some(32),
            Token::StringLiteral(_) if true => Some(33),
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
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 => __Symbol::Variant0(__token),
            31 | 32 | 33 => match __token {
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
                    states_to_pop: 2,
                    nonterminal_produced: 0,
                }
            }
            1 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 1,
                }
            }
            2 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 1,
                }
            }
            3 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 2,
                }
            }
            4 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
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
                    states_to_pop: 0,
                    nonterminal_produced: 4,
                }
            }
            7 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            8 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 5,
                }
            }
            9 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
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
                    states_to_pop: 2,
                    nonterminal_produced: 7,
                }
            }
            12 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 7,
                }
            }
            13 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 8,
                }
            }
            14 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 9,
                }
            }
            15 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 9,
                }
            }
            16 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 10,
                }
            }
            17 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 11,
                }
            }
            18 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 11,
                }
            }
            19 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 12,
                }
            }
            20 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 12,
                }
            }
            21 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 13,
                }
            }
            22 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 14,
                }
            }
            23 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
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
                    states_to_pop: 2,
                    nonterminal_produced: 15,
                }
            }
            26 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
                }
            }
            27 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 16,
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
                    states_to_pop: 3,
                    nonterminal_produced: 17,
                }
            }
            30 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
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
                    states_to_pop: 1,
                    nonterminal_produced: 20,
                }
            }
            36 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 20,
                }
            }
            37 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 20,
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
                    nonterminal_produced: 22,
                }
            }
            41 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 22,
                }
            }
            42 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 22,
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
                    states_to_pop: 2,
                    nonterminal_produced: 23,
                }
            }
            45 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 24,
                }
            }
            46 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 24,
                }
            }
            47 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 25,
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
                    states_to_pop: 0,
                    nonterminal_produced: 26,
                }
            }
            50 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 27,
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
                    states_to_pop: 2,
                    nonterminal_produced: 29,
                }
            }
            54 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 30,
                }
            }
            55 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 30,
                }
            }
            56 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 7,
                    nonterminal_produced: 31,
                }
            }
            57 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 31,
                }
            }
            58 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 32,
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
                    states_to_pop: 1,
                    nonterminal_produced: 32,
                }
            }
            61 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 32,
                }
            }
            62 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 32,
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
            65 => __state_machine::SimulatedReduce::Accept,
            _ => panic!("invalid reduction index {}", __reduce_index)
        }
    }
    pub struct StmtParser {
        _priv: (),
    }

    impl StmtParser {
        pub fn new() -> StmtParser {
            StmtParser {
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
        ) -> Result<Stmt, __lalrpop_util::ParseError<usize, Token<'input>, ParserError>>
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
    ) -> Option<Result<Stmt,__lalrpop_util::ParseError<usize, Token<'input>, ParserError>>>
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
                // __Stmt = Stmt => ActionFn(1);
                let __sym0 = __pop_Variant23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(input, errs, __sym0);
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
    fn __pop_Variant9<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (usize, usize), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant9(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ComplexType, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expr, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant15<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Identifier, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant15(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant16<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, LitKind, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant16(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant11<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, MemberVariable, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant11(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant21<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ProgramPart, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant21(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Stmt, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant23(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant24<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, StructDefinition, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant24(__v), __r) => (__l, __v, __r),
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
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Type, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant7(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant17<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<ComplexType>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant17(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant18<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Expr>, usize)
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
    ) -> (usize, Vec<MemberVariable>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant14(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant20<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<ProgramPart>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant20(__v), __r) => (__l, __v, __r),
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
    fn __pop_Variant13<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant13(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<(usize, usize)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant10(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant19<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<MemberVariable>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant19(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Type>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant8(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<ComplexType>, usize)
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
    ) -> (usize, ::std::vec::Vec<Expr>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant6(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant12<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<MemberVariable>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant12(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<ProgramPart>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant22(__v), __r) => (__l, __v, __r),
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
        // ("+" <ComplexType1>) = "+", ComplexType1 => ActionFn(52);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action52::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 0)
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
        // ("+" <ComplexType1>)* =  => ActionFn(50);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action50::<>(input, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (0, 1)
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
        // ("+" <ComplexType1>)* = ("+" <ComplexType1>)+ => ActionFn(51);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action51::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 1)
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
        // ("+" <ComplexType1>)+ = "+", ComplexType1 => ActionFn(59);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action59::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 2)
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
        // ("+" <ComplexType1>)+ = ("+" <ComplexType1>)+, "+", ComplexType1 => ActionFn(60);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action60::<>(input, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 2)
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
        // ("," <Expr>) = ",", Expr => ActionFn(49);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action49::<>(input, errs, __sym0, __sym1);
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
        // ("," <Expr>)* =  => ActionFn(47);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action47::<>(input, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 4)
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
        // ("," <Expr>)* = ("," <Expr>)+ => ActionFn(48);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action48::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 4)
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
        // ("," <Expr>)+ = ",", Expr => ActionFn(63);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action63::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 5)
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
        // ("," <Expr>)+ = ("," <Expr>)+, ",", Expr => ActionFn(64);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action64::<>(input, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 5)
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
        // (":" <Type>) = ":", Type => ActionFn(39);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action39::<>(input, errs, __sym0, __sym1);
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
        // (":" <Type>)? = ":", Type => ActionFn(67);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action67::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 7)
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
        // (":" <Type>)? =  => ActionFn(38);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action38::<>(input, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 7)
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
        // (<@L> ";" <@R>) = ";" => ActionFn(88);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action88::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 8)
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
        // (<@L> ";" <@R>)? = ";" => ActionFn(106);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action106::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 9)
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
        // (<@L> ";" <@R>)? =  => ActionFn(32);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action32::<>(input, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (0, 9)
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
        // (<MemberVariable> ",") = MemberVariable, "," => ActionFn(46);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action46::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (2, 10)
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
        // (<MemberVariable> ",")* =  => ActionFn(44);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action44::<>(input, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (0, 11)
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
        // (<MemberVariable> ",")* = (<MemberVariable> ",")+ => ActionFn(45);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action45::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 11)
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
        // (<MemberVariable> ",")+ = MemberVariable, "," => ActionFn(109);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action109::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (2, 12)
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
        // (<MemberVariable> ",")+ = (<MemberVariable> ",")+, MemberVariable, "," => ActionFn(110);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action110::<>(input, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (3, 12)
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
        // @L =  => ActionFn(35);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action35::<>(input, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (0, 13)
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
        // @R =  => ActionFn(34);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action34::<>(input, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (0, 14)
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
        // Comma<MemberVariable> = MemberVariable => ActionFn(113);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action113::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 15)
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
        // Comma<MemberVariable> =  => ActionFn(114);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action114::<>(input, errs, &__start, &__end);
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
        // Comma<MemberVariable> = (<MemberVariable> ",")+, MemberVariable => ActionFn(115);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action115::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (2, 15)
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
        // Comma<MemberVariable> = (<MemberVariable> ",")+ => ActionFn(116);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action116::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 15)
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
        // ComplexType = Many1<"+", ComplexType1> => ActionFn(89);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action89::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 16)
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
        // ComplexType0 = Identifier => ActionFn(90);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action90::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
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
        // ComplexType0 = "(", ComplexType, ")" => ActionFn(91);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action91::<>(input, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 17)
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
        // ComplexType1 = "^", ComplexType0 => ActionFn(92);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action92::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 18)
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
        // ComplexType1 = "!", ComplexType0 => ActionFn(93);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action93::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
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
        // ComplexType1 = ComplexType0 => ActionFn(26);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
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
        // Expr = Expr0 => ActionFn(9);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action9::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
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
        // Expr0 = Lit => ActionFn(94);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action94::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
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
        // Expr0 = Identifier => ActionFn(95);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action95::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 20)
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
        // Expr0 = "(", Many1<",", Expr>, ")" => ActionFn(96);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant18(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action96::<>(input, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 20)
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
        // Expr0 = error => ActionFn(97);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action97::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 20)
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
        // Identifier = TokenIdentifier => ActionFn(98);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action98::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
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
        // Lit = "true" => ActionFn(14);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action14::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
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
        // Lit = "false" => ActionFn(15);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action15::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 22)
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
        // Lit = TokenString => ActionFn(16);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action16::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 22)
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
        // Lit = TokenInt => ActionFn(99);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action99::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 22)
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
        // Many1<"+", ComplexType1> = ComplexType1 => ActionFn(61);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action61::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
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
        // Many1<"+", ComplexType1> = ComplexType1, ("+" <ComplexType1>)+ => ActionFn(62);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action62::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (2, 23)
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
        // Many1<",", Expr> = Expr => ActionFn(65);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action65::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 24)
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
        // Many1<",", Expr> = Expr, ("," <Expr>)+ => ActionFn(66);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action66::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (2, 24)
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
        // MemberVariable = Identifier, ":", Type => ActionFn(6);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action6::<>(input, errs, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (3, 25)
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
        // MemberVariable? = MemberVariable => ActionFn(42);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action42::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
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
        // MemberVariable? =  => ActionFn(43);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action43::<>(input, errs, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (0, 26)
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
        // Program = ProgramPart+ => ActionFn(3);
        let __sym0 = __pop_Variant22(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 27)
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
        // ProgramPart = StructDefinition => ActionFn(4);
        let __sym0 = __pop_Variant24(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
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
        // ProgramPart+ = ProgramPart => ActionFn(40);
        let __sym0 = __pop_Variant21(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action40::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 29)
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
        // ProgramPart+ = ProgramPart+, ProgramPart => ActionFn(41);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant21(__symbols);
        let __sym0 = __pop_Variant22(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action41::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (2, 29)
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
        // Stmt = Expr, ";" => ActionFn(107);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action107::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (2, 30)
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
        // Stmt = Expr => ActionFn(108);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action108::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 30)
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
        // StructDefinition = "struct", Identifier, ":", Type, "{", Comma<MemberVariable>, "}" => ActionFn(68);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant14(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant7(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant15(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action68::<>(input, errs, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant24(__nt), __end));
        (7, 31)
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
        // StructDefinition = "struct", Identifier, "{", Comma<MemberVariable>, "}" => ActionFn(69);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant14(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant15(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action69::<>(input, errs, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant24(__nt), __end));
        (5, 31)
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
        // Type = "bool" => ActionFn(101);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action101::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 32)
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
        // Type = "string" => ActionFn(102);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action102::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
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
        // Type = "int" => ActionFn(103);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action103::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 32)
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
        // Type = "(", ")" => ActionFn(104);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action104::<>(input, errs, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 32)
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
        // Type = ComplexType => ActionFn(105);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action105::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 32)
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
        // __Expr = Expr => ActionFn(2);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
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
        // __Program = Program => ActionFn(0);
        let __sym0 = __pop_Variant20(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(input, errs, __sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 34)
    }
}
pub use self::__parse__Stmt::StmtParser;

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
    (_, __0, _): (usize, Stmt, usize),
) -> Stmt
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
    (_, __0, _): (usize, Expr, usize),
) -> Expr
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
    (_, __0, _): (usize, ::std::vec::Vec<ProgramPart>, usize),
) -> Vec<ProgramPart>
{
    __0
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
    (_, _, _): (usize, Token<'input>, usize),
    (_, name, _): (usize, Identifier, usize),
    (_, bounds, _): (usize, ::std::option::Option<Type>, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, members, _): (usize, Vec<MemberVariable>, usize),
    (_, _, _): (usize, Token<'input>, usize),
) -> StructDefinition
{
    StructDefinition{name, bounds, members}
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
    Identifier{span: Span{r, l}, val: n.to_string()}
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

        Stmt{span: Span{l,r}, val}
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
    (_, l, _): (usize, usize, usize),
    (_, lit, _): (usize, LitKind, usize),
    (_, r, _): (usize, usize, usize),
) -> Expr
{
    Expr{span: Span{l,r}, val: ExprKind::Lit(lit)}
}

#[allow(unused_variables)]
fn __action11<
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
    Expr{span: Span{l,r}, val: ExprKind::Variable(ident)}
}

#[allow(unused_variables)]
fn __action12<
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
            Expr{span: Span{l,r}, val: ExprKind::Tuple(exprs)}
        }
    }
}

#[allow(unused_variables)]
fn __action13<
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
    { errs.push(err); Expr{span: Span{l,r}, val: ExprKind::Err} }
}

#[allow(unused_variables)]
fn __action14<
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
fn __action15<
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
fn __action16<
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
fn __action17<
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
fn __action18<
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
    Type{span: Span{l,r}, val: TypeKind::Bool}
}

#[allow(unused_variables)]
fn __action19<
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
    Type{span: Span{l,r}, val: TypeKind::String}
}

#[allow(unused_variables)]
fn __action20<
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
    Type{span: Span{l,r}, val: TypeKind::Int}
}

#[allow(unused_variables)]
fn __action21<
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
    Type{span: Span{l,r}, val: TypeKind::Unit}
}

#[allow(unused_variables)]
fn __action22<
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
    Type{span: Span{l,r}, val: TypeKind::Complex(ty)}
}

#[allow(unused_variables)]
fn __action23<
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
            ComplexType{span: Span{l,r}, val: ComplexTypeKind::Compound(ty)}
        }
    }
}

#[allow(unused_variables)]
fn __action24<
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
    ComplexType{span: Span{l,r}, val: ComplexTypeKind::Above(Box::new(ty))}
}

#[allow(unused_variables)]
fn __action25<
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
    ComplexType{span: Span{l,r}, val: ComplexTypeKind::Not(Box::new(ty))}
}

#[allow(unused_variables)]
fn __action26<
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
fn __action27<
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
    ComplexType{span: Span{l,r}, val: ComplexTypeKind::Base(name, ComplexReferent::Infer)}
}

#[allow(unused_variables)]
fn __action28<
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
fn __action29<
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
fn __action30<
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
fn __action31<
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
fn __action32<
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
fn __action33<
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
fn __action34<
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
fn __action35<
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
fn __action36<
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
fn __action37<
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
fn __action38<
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
fn __action39<
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
fn __action40<
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
fn __action41<
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
fn __action42<
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
fn __action43<
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
fn __action44<
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
fn __action45<
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
fn __action46<
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
fn __action47<
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
fn __action48<
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
fn __action49<
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
fn __action50<
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
fn __action51<
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
fn __action52<
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
fn __action53<
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
fn __action54<
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
fn __action55<
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
fn __action56<
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
fn __action57<
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
fn __action58<
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
fn __action59<
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
    let __temp0 = __action52(
        input,
        errs,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action53(
        input,
        errs,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action60<
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
    let __temp0 = __action52(
        input,
        errs,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action54(
        input,
        errs,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action61<
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
    let __temp0 = __action50(
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
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action62<
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
    let __temp0 = __action51(
        input,
        errs,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action29(
        input,
        errs,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action63<
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
    let __temp0 = __action49(
        input,
        errs,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action55(
        input,
        errs,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action64<
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
    let __temp0 = __action49(
        input,
        errs,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action56(
        input,
        errs,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action65<
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
    let __temp0 = __action47(
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
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action66<
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
    let __temp0 = __action48(
        input,
        errs,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action30(
        input,
        errs,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action67<
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
    let __temp0 = __action39(
        input,
        errs,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action37(
        input,
        errs,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action68<
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
    let __temp0 = __action67(
        input,
        errs,
        __2,
        __3,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action5(
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
fn __action69<
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
    let __temp0 = __action38(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action5(
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
fn __action70<
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
    let __temp0 = __action34(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action33(
        input,
        errs,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action71<
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
    let __temp0 = __action34(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action23(
        input,
        errs,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action72<
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
    let __temp0 = __action34(
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
fn __action73<
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
    let __temp0 = __action34(
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
        __2,
        __3,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action74<
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
    let __temp0 = __action34(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action24(
        input,
        errs,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action75<
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
    let __temp0 = __action34(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action25(
        input,
        errs,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action76<
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
    let __temp0 = __action34(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action10(
        input,
        errs,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action77<
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
    let __temp0 = __action34(
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
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action78<
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
    let __temp0 = __action34(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action12(
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
fn __action79<
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
    let __temp0 = __action34(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action13(
        input,
        errs,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action80<
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
    let __temp0 = __action34(
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
fn __action81<
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
    let __temp0 = __action34(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action17(
        input,
        errs,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action82<
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
    let __temp0 = __action34(
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
fn __action83<
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
    let __temp0 = __action34(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action18(
        input,
        errs,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action84<
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
    let __temp0 = __action34(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action19(
        input,
        errs,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action85<
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
    let __temp0 = __action34(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action20(
        input,
        errs,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action86<
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
    let __temp0 = __action34(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action21(
        input,
        errs,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action87<
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
    let __temp0 = __action34(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action22(
        input,
        errs,
        __0,
        __1,
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
    __0: (usize, Token<'input>, usize),
) -> (usize, usize)
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action35(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action70(
        input,
        errs,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action89<
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
    let __temp0 = __action35(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action71(
        input,
        errs,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action90<
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
    let __temp0 = __action35(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action72(
        input,
        errs,
        __temp0,
        __0,
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
    __1: (usize, ComplexType, usize),
    __2: (usize, Token<'input>, usize),
) -> ComplexType
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action35(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action73(
        input,
        errs,
        __temp0,
        __0,
        __1,
        __2,
    )
}

#[allow(unused_variables)]
fn __action92<
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
    let __temp0 = __action35(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action74(
        input,
        errs,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action93<
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
    let __temp0 = __action35(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action75(
        input,
        errs,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action94<
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
    let __temp0 = __action35(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action76(
        input,
        errs,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action95<
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
    let __temp0 = __action35(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action77(
        input,
        errs,
        __temp0,
        __0,
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
    __1: (usize, Vec<Expr>, usize),
    __2: (usize, Token<'input>, usize),
) -> Expr
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action35(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action78(
        input,
        errs,
        __temp0,
        __0,
        __1,
        __2,
    )
}

#[allow(unused_variables)]
fn __action97<
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
    let __temp0 = __action35(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action79(
        input,
        errs,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action98<
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
    let __temp0 = __action35(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action80(
        input,
        errs,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action99<
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
    let __temp0 = __action35(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action81(
        input,
        errs,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action100<
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
    let __temp0 = __action35(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action82(
        input,
        errs,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action101<
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
    let __temp0 = __action35(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action83(
        input,
        errs,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action102<
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
    let __temp0 = __action35(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action84(
        input,
        errs,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action103<
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
    let __temp0 = __action35(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action85(
        input,
        errs,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action104<
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
    let __temp0 = __action35(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action86(
        input,
        errs,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action105<
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
    let __temp0 = __action35(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action87(
        input,
        errs,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action106<
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
    let __temp0 = __action88(
        input,
        errs,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action31(
        input,
        errs,
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
    __0: (usize, Expr, usize),
    __1: (usize, Token<'input>, usize),
) -> Stmt
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action106(
        input,
        errs,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action100(
        input,
        errs,
        __0,
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
    __0: (usize, Expr, usize),
) -> Stmt
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action32(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action100(
        input,
        errs,
        __0,
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
    __0: (usize, MemberVariable, usize),
    __1: (usize, Token<'input>, usize),
) -> ::std::vec::Vec<MemberVariable>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action46(
        input,
        errs,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action57(
        input,
        errs,
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
    __0: (usize, ::std::vec::Vec<MemberVariable>, usize),
    __1: (usize, MemberVariable, usize),
    __2: (usize, Token<'input>, usize),
) -> ::std::vec::Vec<MemberVariable>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action46(
        input,
        errs,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action58(
        input,
        errs,
        __0,
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
    __0: (usize, ::std::option::Option<MemberVariable>, usize),
) -> Vec<MemberVariable>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action44(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action36(
        input,
        errs,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action112<
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
    let __temp0 = __action45(
        input,
        errs,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action36(
        input,
        errs,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action113<
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
    let __temp0 = __action42(
        input,
        errs,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action111(
        input,
        errs,
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
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<MemberVariable>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action43(
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
    )
}

#[allow(unused_variables)]
fn __action115<
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
    let __temp0 = __action42(
        input,
        errs,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action112(
        input,
        errs,
        __0,
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
    __0: (usize, ::std::vec::Vec<MemberVariable>, usize),
) -> Vec<MemberVariable>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action43(
        input,
        errs,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action112(
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
