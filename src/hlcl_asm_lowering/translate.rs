use hlcl_asm::selector::*;
use hlcl_asm::coord::*;
use hlcl_asm::function::*;
use hlcl_asm::{Assembly};
use std::collections::VecDeque;
use std::slice::Iter;

use crate::token::*;
use hlcl_asm::function::commands::Command;

pub struct FunctionPrinter<'asm> {
    asm_ctx: &'asm Assembly,
    func: &'asm Function,
    internal_buffer: VecDeque<McToken<'asm>>,
    internal_iter: Iter<'asm, Op>,
    /// Whether or not the translator is in an `execute` context.
    bare_op: bool,
}

impl<'asm> Iterator for FunctionPrinter<'asm> {
    type Item = McToken<'asm>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.internal_buffer.is_empty() {
            match self.internal_iter.next() {
                Some(Op::SubCommand(sub)) => self.buffer_sub_command(sub),
                Some(Op::Command(cmd)) => self.buffer_command(cmd),
                Some(Op::BinOp(op, lhs, rhs)) => self.buffer_binop(*op, lhs, rhs),
                Some(Op::Block(id)) => self.buffer_terminal(|| {
                    self.push(McToken::Function);
                    let name = self.func.blocks.get(id).expect("invalid block id");

                    self.push(McToken::NamespacedPath(name.as_str()));
                    self.push(McToken::BeginBlock(name.path()));
                }),
                _ => return None,
            }
        }

        self.internal_buffer.pop_front()
    }
}

impl<'asm> FunctionPrinter<'asm> {
    pub fn emit_function<'a>(asm_ctx: &'a Assembly, func: &'a Function) {
        let mut printer = FunctionPrinter {
            asm_ctx,
            func,
            internal_buffer: VecDeque::with_capacity(16),
            internal_iter: func.code.iter(),
            bare_op: true,
        };
    }

    #[inline(always)]
    fn push(&mut self, tok: McToken<'asm>) {
        self.internal_buffer.push_back(tok)
    }

    #[inline]
    fn buffer_terminal<F>(&mut self, action: F)
    where
        F: FnOnce()
    {
        if !self.bare_op {
            self.push(McToken::Run);
            self.bare_op = true;
        }

        action();

        self.push(McToken::EndLine);
    }

    fn buffer_command(&mut self, cmd: &'asm Command) {
        self.buffer_terminal(|| {
            match cmd {
                Command::Tag(target, args) => {
                    self.resolve_target(target);
                }
            }
        });
    }

    fn buffer_binop(&mut self, op: ScoreOp, rhs: &'asm Operand, lhs: &'asm Operand) {
        self.buffer_terminal(|| {

        });
    }

    fn buffer_sub_command(&mut self, sub: &'asm SubCommand) {
        if self.bare_op {
            self.push(McToken::Execute);
            self.bare_op = false;
        }

        match sub {
            SubCommand::Align(align) => {
                if align.all_false() {
                    panic!("empty align subcommand"); // TODO: Graceful errors
                }

                self.push(McToken::Align);
                self.push(McToken::Swizzle(align));
            }
            SubCommand::Anchored(mode) => {
                self.push(McToken::Anchored);
                self.push(mode.into())
            }
            SubCommand::As(target) => {
                self.push(McToken::As);
                self.push(McToken::Selector(self.resolve_target(target)))
            }
            SubCommand::At(target) => {
                self.push(McToken::At);
                self.push(McToken::Selector(self.resolve_target(target)))
            }
            SubCommand::Facing(facing) => {
                self.push(McToken::Facing);
                match facing {
                    Facing::Pos(pos) => self.push(McToken::Pos(pos)),
                    Facing::Entity(target, mode) => {
                        self.push(McToken::Entity);
                        self.push(McToken::Selector(self.resolve_target(target)));
                        self.push(mode.into());
                    }
                }
            }
            SubCommand::In(_) => unimplemented!(),
            SubCommand::Positioned(loc) => {
                self.push(McToken::Positioned);
                match loc {
                    TargetOr::Other(pos) => self.push(McToken::Pos(pos)),
                    TargetOr::Target(target) => {
                        self.push(McToken::As);
                        self.push(McToken::Selector(self.resolve_target(target)))
                    },
                }
            }
            SubCommand::Rotated(rot) => {
                self.push(McToken::Positioned);
                match rot {
                    TargetOr::Other(pos) => self.push(McToken::Rotation(pos)),
                    TargetOr::Target(target) => {
                        self.push(McToken::As);
                        self.push(McToken::Selector(self.resolve_target(target)))
                    },
                }
            }
            SubCommand::If(cond) => {
                self.push(McToken::If);
                self.buffer_cond(cond)
            }
            SubCommand::Unless(cond) => {
                self.push(McToken::Unless);
                self.buffer_cond(cond);
            }

            SubCommand::Store(mode, loc) => {
                self.push(McToken::Store);
                self.push(mode.into());

                match loc {
                    StoreLocation::Score(op) => {
                        self.push(McToken::Score);
                        self.buffer_operand(op);
                    },
                    StoreLocation::Block => {},
                    StoreLocation::Bossbar => {},
                    StoreLocation::Entity => {},
                    StoreLocation::Storage => {},
                }
            }
        }
    }

    fn buffer_cond(&mut self, cond: &'asm Condition) {
        match cond {
            Condition::Entity(target) => {
                self.push(McToken::Entity);
                self.push(McToken::Selector(self.resolve_target(target)))
            }
            Condition::Score(score_cond) => {
                self.push(McToken::Score);

                match score_cond {
                    ScoreCondition::Compare(op, lhs, rhs) => {
                        self.buffer_operand(lhs);
                        self.push(op.into());
                        self.buffer_operand(rhs);
                    }
                    ScoreCondition::Match(lhs, range) => {
                        self.buffer_operand(lhs);
                        self.push(McToken::Matches);
                        self.push(McToken::IntRange(range));
                    }
                }
            }
            Condition::Block => {},
            Condition::Blocks => {},
            Condition::Data => {},
            Condition::Predicate => {},
        }
    }

    fn buffer_operand(&mut self, operand: &'asm Operand) {
        let (selector, score) = self.resolve_operand(operand);
        self.push(McToken::Selector(selector));
        self.push(McToken::Path(score));
    }

    fn resolve_operand(&self, operand: &'asm Operand) -> (&'asm Selector, &'asm str) {
        let pair = match operand {
            Operand::Register(val) => {
                self.func.registers.get(val).map(|(selector, score)| (selector, score))
            },
            Operand::Entity(target, score) => {
                self.resolve_target(target).map(|selector| (selector, score))
            },
        };

        pair.and_then(|(selector, score)| self.asm_ctx.resolve_score(score).map(|score| (selector, score))).expect("invalid operand")
    }

    fn resolve_target(&self, target: &'asm Target) -> &'asm Selector {
        match target {
            Target::Selector(s) => s.as_ref(),
            Target::Argument(id) => self.func.entity_args.get(id).expect("invalid argument id"),
        }
    }
}
