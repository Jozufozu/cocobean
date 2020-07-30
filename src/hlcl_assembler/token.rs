use hlcl_asm::coord::*;
use hlcl_asm::function::commands::TagArgs;
use hlcl_asm::function::*;
use hlcl_asm::selector::*;
use hlcl_helpers::static_assert_size;
use std::borrow::Cow;
use std::num::NonZeroU16;

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

    Function,

    Feet,
    Eyes,

    Tag,
    Add,
    List,
    Remove,
}

// We want to keep this small
static_assert_size!(McToken, 24);

impl<'asm> McToken<'asm> {
    pub fn to_str(&self) -> Cow<'asm, str> {
        match *self {
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

            McToken::BeginBlock(_) | McToken::EndBlock | McToken::EndLine => Cow::Borrowed(""),
        }
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

#[cfg(test)]
mod tests {
    use crate::token::McToken;

    #[test]
    fn test_string() {
        assert_eq!("feet", McToken::Feet.to_str());
    }
}
