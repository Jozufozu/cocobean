use std::borrow::Cow;
use std::fmt::{self, Display, Formatter};

use coco_asm::coord::*;
use coco_asm::function::*;
use coco_asm::function::commands::TagArgs;
use coco_asm::selector::*;
use coco_helpers::static_assert_size;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum McToken<'asm> {
    BeginBlock(&'asm str),
    EndBlock,
    EndLine,

    Path(&'asm str),
    NamespacedPath(&'asm str),
    Selector(&'asm Selector),
    IntRange(&'asm RangeArg<i32>),
    FloatRange(&'asm RangeArg<f32>),
    Pos(&'asm Pos),
    Rotation(&'asm Rotation),
    Swizzle(&'asm Align),

    Execute,
    Align,
    Anchored,
    As,
    At,
    Facing,
    In,
    Positioned,
    Rotated,
    Store,
    Result,
    Success,
    If,
    Unless,
    Block,
    Bossbar,
    Entity,
    Score,
    Storage,
    Blocks,
    Data,
    Predicate,
    Matches,
    Run,

    Lt,
    Le,
    Eq,
    Ge,
    Gt,

    BinOp(ScoreOp),

    Function,

    Feet,
    Eyes,

    Tag,
    Add,
    List,
    Remove,

    Scoreboard,
    Objectives,
    Players,
    Operation,
}

// We want to keep this small
static_assert_size!(McToken, 24);

impl<'asm> McToken<'asm> {
    pub fn to_str(&self) -> Cow<'asm, str> {
        match *self {
            McToken::BeginBlock(_) | McToken::EndBlock | McToken::EndLine => Cow::Borrowed(""),

            McToken::Path(name) | McToken::NamespacedPath(name) => Cow::Borrowed(name),
            McToken::Selector(s) => Cow::Owned(s.to_string()),
            McToken::IntRange(range) => Cow::Owned(range.to_string()),
            McToken::FloatRange(range) => Cow::Owned(range.to_string()),
            McToken::Pos(p) => Cow::Owned(p.to_string()),
            McToken::Rotation(p) => Cow::Owned(p.to_string()),
            McToken::Swizzle(align) => Cow::Borrowed(align.str()),

            McToken::Execute => Cow::Borrowed("execute"),
            McToken::Align => Cow::Borrowed("align"),
            McToken::Anchored => Cow::Borrowed("anchored"),
            McToken::As => Cow::Borrowed("as"),
            McToken::At => Cow::Borrowed("at"),
            McToken::Facing => Cow::Borrowed("facing"),
            McToken::In => Cow::Borrowed("in"),
            McToken::Positioned => Cow::Borrowed("positioned"),
            McToken::Rotated => Cow::Borrowed("rotated"),
            McToken::Store => Cow::Borrowed("store"),
            McToken::Result => Cow::Borrowed("result"),
            McToken::Success => Cow::Borrowed("success"),
            McToken::If => Cow::Borrowed("if"),
            McToken::Unless => Cow::Borrowed("unless"),
            McToken::Block => Cow::Borrowed("block"),
            McToken::Bossbar => Cow::Borrowed("bossbar"),
            McToken::Entity => Cow::Borrowed("entity"),
            McToken::Score => Cow::Borrowed("score"),
            McToken::Storage => Cow::Borrowed("storage"),
            McToken::Blocks => Cow::Borrowed("blocks"),
            McToken::Data => Cow::Borrowed("data"),
            McToken::Predicate => Cow::Borrowed("predicate"),
            McToken::Matches => Cow::Borrowed("matches"),
            McToken::Run => Cow::Borrowed("run"),
            McToken::Lt => Cow::Borrowed("<"),
            McToken::Le => Cow::Borrowed("<="),
            McToken::Eq => Cow::Borrowed("="),
            McToken::Ge => Cow::Borrowed(">="),
            McToken::Gt => Cow::Borrowed(">"),
            McToken::Function => Cow::Borrowed("function"),
            McToken::Feet => Cow::Borrowed("feet"),
            McToken::Eyes => Cow::Borrowed("eyes"),
            McToken::Tag => Cow::Borrowed("tag"),
            McToken::Add => Cow::Borrowed("add"),
            McToken::List => Cow::Borrowed("list"),
            McToken::Remove => Cow::Borrowed("remove"),
            McToken::Scoreboard => Cow::Borrowed("scoreboard"),
            McToken::Objectives => Cow::Borrowed("objectives"),
            McToken::Players => Cow::Borrowed("players"),
            McToken::Operation => Cow::Borrowed("operation"),
            McToken::BinOp(ScoreOp::Add) => Cow::Borrowed("+="),
            McToken::BinOp(ScoreOp::Sub) => Cow::Borrowed("-="),
            McToken::BinOp(ScoreOp::Mul) => Cow::Borrowed("*="),
            McToken::BinOp(ScoreOp::Div) => Cow::Borrowed("/="),
            McToken::BinOp(ScoreOp::Mod) => Cow::Borrowed("%="),
            McToken::BinOp(ScoreOp::Min) => Cow::Borrowed("<"),
            McToken::BinOp(ScoreOp::Max) => Cow::Borrowed(">"),
            McToken::BinOp(ScoreOp::Swap) => Cow::Borrowed("><"),
            McToken::BinOp(ScoreOp::Assign) => Cow::Borrowed("="),
        }
    }
}

impl Display for McToken<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.to_str().as_ref())
    }
}

impl From<AnchorMode> for McToken<'_> {
    fn from(mode: AnchorMode) -> Self {
        match mode {
            AnchorMode::Feet => McToken::Feet,
            AnchorMode::Eyes => McToken::Eyes,
        }
    }
}

impl From<&AnchorMode> for McToken<'_> {
    fn from(mode: &AnchorMode) -> Self {
        match *mode {
            AnchorMode::Feet => McToken::Feet,
            AnchorMode::Eyes => McToken::Eyes,
        }
    }
}

impl From<&ScoreCmp> for McToken<'_> {
    fn from(mode: &ScoreCmp) -> Self {
        match *mode {
            ScoreCmp::Lt => McToken::Lt,
            ScoreCmp::Le => McToken::Le,
            ScoreCmp::Eq => McToken::Eq,
            ScoreCmp::Ge => McToken::Ge,
            ScoreCmp::Gt => McToken::Gt,
        }
    }
}

impl From<&StoreMode> for McToken<'_> {
    fn from(mode: &StoreMode) -> Self {
        match *mode {
            StoreMode::Result => McToken::Result,
            StoreMode::Success => McToken::Success,
        }
    }
}

impl From<&TagArgs> for McToken<'_> {
    fn from(mode: &TagArgs) -> Self {
        match *mode {
            TagArgs::Add(_) => McToken::Add,
            TagArgs::Remove(_) => McToken::Remove,
            TagArgs::List => McToken::List,
        }
    }
}

impl From<&ExecuteItem> for McToken<'_> {
    fn from(cond: &ExecuteItem) -> Self {
        match cond {
            ExecuteItem::Align(_) => McToken::Align,
            ExecuteItem::Anchored(_) => McToken::Anchored,
            ExecuteItem::As(_) => McToken::As,
            ExecuteItem::At(_) => McToken::At,
            ExecuteItem::Facing(_) => McToken::Facing,
            ExecuteItem::In(_) => McToken::In,
            ExecuteItem::Positioned(_) => McToken::Positioned,
            ExecuteItem::Rotated(_) => McToken::Rotated,
            ExecuteItem::If(_) => McToken::If,
            ExecuteItem::Unless(_) => McToken::Unless,
            ExecuteItem::Store(_, _) => McToken::Store,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::token::McToken;

    #[test]
    fn test_string() {
        assert_eq!("feet", McToken::Feet.to_str());
    }
}
