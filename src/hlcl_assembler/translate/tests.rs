use std::convert::TryFrom;

use hlcl_asm::{Assembly, NameInterner};
use hlcl_asm::function::{ExecuteItem, Function, Op, Target};
use hlcl_asm::function::commands::{Command, TagArgs};
use hlcl_asm::selector;
use hlcl_asm::selector::Selector;
use hlcl_helpers::resource_name::ResourceName;

use crate::translate::FunctionAssembler;

#[test]
fn works() {
    let executor = selector::Selector::executor();
    let mut func = Function::new();

    let block = func.insert(ResourceName::try_from("test:test/test1".to_string()).unwrap());

    let executorid = func.selectors.insert(Selector::executor());

    func.code = vec![
        Op::NonTerminal(ExecuteItem::As(Target::Selector(executorid))),
        Op::Terminal(Command::Tag(Target::Selector(executorid), TagArgs::List)),
        Op::NonTerminal(ExecuteItem::As(Target::Selector(executorid))),
        Op::Block(block),
        Op::Terminal(Command::Tag(Target::Selector(executorid), TagArgs::List)),
        Op::EndBlock,
        Op::Terminal(Command::Tag(Target::Selector(executorid), TagArgs::List)),
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
            Selector(&executor),
            Run,
            Tag,
            Selector(&executor),
            List,
            EndLine,
            Execute,
            As,
            Selector(&executor),
            Run,
            Function,
            NamespacedPath("test:test/test1"),
            EndLine,
            BeginBlock("test/test1"),
            Tag,
            Selector(&executor),
            List,
            EndLine,
            EndBlock,
            Tag,
            Selector(&executor),
            List,
            EndLine,
        ]
    });
}
