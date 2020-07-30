use crate::translate::FunctionAssembler;
use crate::CommandWriter;
use hlcl_asm::function::commands::{Command, TagArgs};
use hlcl_asm::function::{ExecuteItem, Function, Op, Target};
use hlcl_asm::selector::Selector;
use hlcl_asm::{Assembly, Names};
use hlcl_helpers::id_map::IdMap;
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
        Op::Block(id),
        Op::Terminal(Command::Tag(Target::executor(), TagArgs::List)),
    ];

    let mut fns = IdMap::new();

    let id = fns.insert(func);

    let asm = Assembly::new(fns, Names::new());

    let func = asm.get_fn(&id).unwrap();

    let assembler = FunctionAssembler::emit_function(&asm, func);

    let writer: CommandWriter = assembler.collect();

    println!("{:#?}", writer)
}
