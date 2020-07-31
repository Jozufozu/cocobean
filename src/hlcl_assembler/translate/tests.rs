use crate::translate::FunctionAssembler;
use hlcl_asm::function::commands::{Command, TagArgs};
use hlcl_asm::function::{ExecuteItem, Function, Op, Target};
use hlcl_asm::Assembly;
use hlcl_asm::selector;
use hlcl_helpers::resource_name::ResourceName;
use std::convert::TryFrom;

#[test]
fn works() {
    let mut func = Function::new();

    let id = func
        .blocks
        .insert(ResourceName::try_from("test:test/test1".to_string()).unwrap());

    func.code = vec![
        Op::NonTerminal(ExecuteItem::As(Target::executor())),
        Op::Terminal(Command::Tag(Target::executor(), TagArgs::List)),
        Op::NonTerminal(ExecuteItem::As(Target::executor())),
        Op::Block(id),
        Op::Terminal(Command::Tag(Target::executor(), TagArgs::List)),
        Op::EndBlock,
        Op::Terminal(Command::Tag(Target::executor(), TagArgs::List)),
    ];

    let mut asm = Assembly::new();
    let id = asm.insert_fn(func);

    let func = asm.get_fn(&id).unwrap();

    let assembler = FunctionAssembler::emit_function(&asm, func);

    let output: Vec<_> = assembler.collect();

    //println!("{:#?}", output);

    assert_eq!(output, {
        use crate::token::McToken::*;
        vec![
            Execute,
            As,
            Selector(&selector::Selector::executor()),
            Run,
            Tag,
            Selector(&selector::Selector::executor()),
            List,
            EndLine,
            Execute,
            As,
            Selector(&selector::Selector::executor()),
            Run,
            Function,
            NamespacedPath("test:test/test1"),
            EndLine,
            BeginBlock("test/test1"),
            Tag,
            Selector(&selector::Selector::executor()),
            List,
            EndLine,
            EndBlock,
            Tag,
            Selector(&selector::Selector::executor()),
            List,
            EndLine,
        ]
    });
}
