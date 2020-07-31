use hlcl_asm::function::*;
use hlcl_asm::selector::*;
use hlcl_asm::{Assembly, NameResolver};
use std::collections::VecDeque;
use std::slice::Iter;

use crate::token::*;
use hlcl_asm::function::commands::{Command, TagArgs};

#[cfg(test)]
mod tests;

pub struct FunctionAssembler<'asm> {
    asm_ctx: &'asm Assembly,
    func: &'asm Function,
    internal_buffer: VecDeque<McToken<'asm>>,
    internal_iter: Iter<'asm, Op>,
    state: AsmState,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AsmState {
    /// There have been no subcommands since the last terminal.
    ///
    /// This is the initial state.
    Bare,
    /// There has been at least one subcommand since the last terminal.
    Execute,
    /// We are currently buffering a terminal.
    Terminal,
}

impl Default for AsmState {
    fn default() -> Self {
        AsmState::Bare
    }
}

impl<'asm> Iterator for FunctionAssembler<'asm> {
    type Item = McToken<'asm>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.internal_buffer.is_empty() {
            match self.internal_iter.next() {
                Some(Op::NonTerminal(sub)) => self.buffer_sub_command(sub),
                Some(Op::Terminal(cmd)) => self.buffer_command(cmd),
                Some(Op::BinOp(op, lhs, rhs)) => self.buffer_binop(*op, lhs, rhs),
                Some(Op::Block(id)) => {
                    let name = self.func.blocks.get(id).expect("invalid block id");
                    self.transition(AsmState::Terminal);
                    self.push(McToken::Function);
                    self.push(McToken::NamespacedPath(name.as_str()));
                    self.transition(AsmState::Bare);
                    self.push(McToken::BeginBlock(name.path()));
                }
                Some(Op::EndBlock) => {
                    self.push(McToken::EndBlock);
                    self.transition(AsmState::Bare);
                }
                _ => return None,
            }
        }

        self.internal_buffer.pop_front()
    }
}

impl<'asm> FunctionAssembler<'asm> {
    pub fn emit_function<'a>(asm_ctx: &'a Assembly, func: &'a Function) -> FunctionAssembler<'a> {
        FunctionAssembler {
            asm_ctx,
            func,
            internal_buffer: VecDeque::with_capacity(16),
            internal_iter: func.code.iter(),
            state: AsmState::default(),
        }
    }

    #[inline(always)]
    fn push(&mut self, tok: McToken<'asm>) {
        self.internal_buffer.push_back(tok)
    }

    /// This manages buffering additional tokens based on context.
    fn transition(&mut self, state: AsmState) {
        use AsmState::*;

        if state == self.state {
            return; // nothing to do
        }

        match (self.state, state) {
            (Bare, Execute) => self.push(McToken::Execute),
            (Execute, Terminal) => self.push(McToken::Run),
            (Terminal, Bare) => self.push(McToken::EndLine),
            (Execute, Bare) => panic!("cannot move directly from execute context to bare context"),
            (Terminal, Execute) => {
                self.push(McToken::EndLine);
                self.push(McToken::Execute);
            }
            (Bare, Terminal) | (Execute, Execute) | (Bare, Bare) | (Terminal, Terminal) => {}
        }

        self.state = state;
    }

    fn buffer_command(&mut self, cmd: &'asm Command) {
        self.transition(AsmState::Terminal);
        match cmd {
            Command::Tag(target, args) => {
                self.push(McToken::Tag);
                self.push(McToken::Selector(self.resolve_target(target)));
                self.push(args.into());

                match args {
                    TagArgs::Add(tag) | TagArgs::Remove(tag) => {
                        self.push(McToken::Path(self.asm_ctx.names.resolve(tag).unwrap()))
                    }
                    TagArgs::List => {}
                }
            }
        }
        self.transition(AsmState::Bare);
    }

    fn buffer_binop(&mut self, op: ScoreOp, rhs: &'asm Operand, lhs: &'asm Operand) {
        self.transition(AsmState::Terminal);

        self.push(McToken::Scoreboard);
        self.push(McToken::Players);
        self.push(McToken::Operation);

        self.buffer_operand(rhs);
        self.push(McToken::BinOp(op));
        self.buffer_operand(lhs);

        self.transition(AsmState::Bare);
    }

    fn buffer_sub_command(&mut self, sub: &'asm ExecuteItem) {
        self.transition(AsmState::Execute);

        self.push(sub.into());
        match sub {
            ExecuteItem::Align(align) => {
                if align.all_false() {
                    panic!("empty align subcommand"); // TODO: Graceful errors
                }

                self.push(McToken::Swizzle(align));
            }
            ExecuteItem::Anchored(mode) => {
                self.push(mode.into())
            }
            ExecuteItem::As(target) | ExecuteItem::At(target) => {
                self.push(McToken::Selector(self.resolve_target(target)))
            }
            ExecuteItem::Facing(facing) => {
                match facing {
                    Facing::Pos(pos) => self.push(McToken::Pos(pos)),
                    Facing::Entity(target, mode) => {
                        self.push(McToken::Entity);
                        self.push(McToken::Selector(self.resolve_target(target)));
                        self.push(mode.into());
                    }
                }
            }
            ExecuteItem::In(dim) => {
                let dim = self.asm_ctx.resolve(dim).expect("invalid dimension id");
                self.push(McToken::NamespacedPath(dim.as_str()))
            },
            ExecuteItem::Positioned(TargetOr::Target(target)) | ExecuteItem::Rotated(TargetOr::Target(target)) => {
                self.push(McToken::As);
                self.push(McToken::Selector(self.resolve_target(target)))
            }
            ExecuteItem::Positioned(TargetOr::Other(pos)) => self.push(McToken::Pos(pos)),
            ExecuteItem::Rotated(TargetOr::Other(rot)) => self.push(McToken::Rotation(rot)),
            ExecuteItem::If(cond) | ExecuteItem::Unless(cond) => {
                self.buffer_cond(cond);
            }
            ExecuteItem::Store(mode, loc) => {
                self.push(mode.into());

                match loc {
                    StoreLocation::Score(op) => {
                        self.push(McToken::Score);
                        self.buffer_operand(op);
                    }
                    StoreLocation::Block => {}
                    StoreLocation::Bossbar => {}
                    StoreLocation::Entity => {}
                    StoreLocation::Storage => {}
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
            Condition::Block => {}
            Condition::Blocks => {}
            Condition::Data => {}
            Condition::Predicate => {}
        }
    }

    fn buffer_operand(&mut self, operand: &'asm Operand) {
        let (selector, score) = self.resolve_operand(operand);
        self.push(McToken::Selector(selector));
        self.push(McToken::Path(score));
    }

    fn resolve_operand(&self, operand: &'asm Operand) -> (&'asm Selector, &'asm str) {
        let pair = match operand {
            Operand::Register(val) => self
                .func
                .registers
                .get(val)
                .map(|(selector, score)| (selector, score)),
            Operand::Entity(target, score) => Some((self.resolve_target(target), score)),
        };

        pair.and_then(|(selector, score)| {
            self.asm_ctx
                .resolve(score)
                .map(|score| (selector, score))
        })
        .expect("invalid operand")
    }

    fn resolve_target(&self, target: &'asm Target) -> &'asm Selector {
        match target {
            Target::Selector(s) => s.as_ref(),
            Target::Argument(id) => self.func.entity_args.get(id).expect("invalid argument id"),
        }
    }
}
