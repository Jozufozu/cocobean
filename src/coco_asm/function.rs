use coco_helpers::id_map::{IdMap, Index};

use crate::*;
use crate::coord::Rotation;
use crate::function::commands::Command;
use crate::selector::Selector;

pub mod commands;

#[derive(Debug)]
pub struct Function {
    pub entity_args: HashMap<u8, Selector>,
    pub registers: IdMap<Register, (Selector, Score)>,
    pub selectors: IdMap<SelectorId, Selector>,
    pub names: Names,
    pub code: Vec<Op>,
}

impl Default for Function {
    fn default() -> Self {
        Self::new()
    }
}

impl Function {
    #[inline(always)]
    pub fn new() -> Self {
        Function {
            entity_args: HashMap::new(),
            registers: IdMap::new(),
            selectors: IdMap::new(),
            names: Names::default(),
            code: Vec::new(),
        }
    }
}

impl<K: Index, V: ?Sized> NameResolver<K, V> for Function
where
    Names: NameResolver<K, V>,
{
    #[inline(always)]
    fn resolve(&self, key: &K) -> Option<&V> {
        self.names.resolve(key)
    }
}

impl<K: Index, V> NameInterner<K, V> for Function
where
    Names: NameInterner<K, V>,
{
    #[inline(always)]
    fn insert(&mut self, value: V) -> K {
        self.names.insert(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    NonTerminal(ExecuteItem),
    /// Calls a function.
    Call(FnId),

    BinOp(ScoreOp, Operand, Operand),

    Terminal(Command),

    /// Begins a block
    Block(BlockId),
    /// Ends a block
    EndBlock,
    /// Jumps backwards to the start of a block
    Loop(BlockId),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Operand {
    Register(Register),
    Entity(Target, Score),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Target {
    /// An arbitrary entity selector.
    Selector(SelectorId),
    /// A compiler generated entity selector used to pass entities as arguments to functions.
    Argument(u8),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ScoreOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Min,
    Max,
    Swap,
    Assign,
}

/// Non-terminal operations that modify the execution of other operations.
///
/// Sub-commands are built up like a stack, and once a terminal operation is encountered,
/// the stack is cleared. Store operations are saved in their own stack.
///
/// The exception is blocks. When a block is encountered after a series of sub-commands,
/// the sub-commands will be applied to everything in the block. Store operations preceding a
/// block are applied only to the last terminal in the block.
#[derive(Debug, Clone, PartialEq)]
pub enum ExecuteItem {
    Align(Align),
    Anchored(AnchorMode),
    As(Target),
    At(Target),
    Facing(Facing),
    In(Dimension),
    Positioned(TargetOr<Pos>),
    Rotated(TargetOr<Rotation>),

    If(Condition),
    Unless(Condition),

    Store(StoreMode, StoreLocation),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum StoreMode {
    Result,
    Success,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StoreLocation {
    Block,
    Bossbar,
    Entity,
    Score(Operand),
    Storage,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AnchorMode {
    Feet,
    Eyes,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TargetOr<T> {
    Other(T),
    Target(Target),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Facing {
    Pos(Pos),
    Entity(Target, AnchorMode),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Condition {
    Block,
    Blocks,
    Data,
    Entity(Target),
    Predicate,
    Score(ScoreCondition),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ScoreCondition {
    Compare(ScoreCmp, Operand, Operand),
    Match(Operand, RangeArg<i32>),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ScoreCmp {
    Lt,
    Le,
    Eq,
    Ge,
    Gt,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Align {
    pub x: bool,
    pub y: bool,
    pub z: bool,
}

impl Align {
    pub const XYZ: Self = Align {
        x: true,
        y: true,
        z: true,
    };

    pub fn x(&mut self, b: bool) -> &mut Self {
        self.x = b;
        self
    }

    pub fn y(&mut self, b: bool) -> &mut Self {
        self.y = b;
        self
    }

    pub fn z(&mut self, b: bool) -> &mut Self {
        self.z = b;
        self
    }

    pub fn all_false(&self) -> bool {
        !self.x && !self.y && !self.z
    }

    pub fn str(&self) -> &'static str {
        match (self.x, self.y, self.z) {
            (false, false, false) => "",
            (true, false, false) => "x",
            (false, true, false) => "y",
            (true, true, false) => "xy",
            (false, false, true) => "z",
            (true, false, true) => "xz",
            (false, true, true) => "yz",
            (true, true, true) => "xyz",
        }
    }
}
