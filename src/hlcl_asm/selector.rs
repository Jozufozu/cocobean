use crate::{Name, Score, Tag, Team};
use smallvec::SmallVec;
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::ops::Range;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum TargetKind {
    AllPlayers,
    NearestPlayer,
    RandomPlayer,
    AllEntities,
    Executor,
}

impl TargetKind {
    pub fn selector_char(self) -> char {
        match self {
            TargetKind::AllPlayers => 'a',
            TargetKind::NearestPlayer => 'p',
            TargetKind::RandomPlayer => 'r',
            TargetKind::AllEntities => 'e',
            TargetKind::Executor => 's',
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Selector {
    pub target: TargetKind,
    pub pos: PosSelector,
    pub scores: Option<HashMap<Score, RangeArg<i32>>>,
    pub team: Option<TeamSelector>,
    pub limit: Option<u32>,
    pub sort: Option<SortingMode>,
    pub level: Option<RangeArg<u32>>,
    pub gamemode: Option<(ArgumentMode, Gamemode)>,
    pub pitch: Option<RangeArg<i8>>,
    pub yaw: Option<RangeArg<i16>>,
    pub name: Option<Name>,
    pub tags: SmallVec<[(ArgumentMode, Tag); 1]>,
    // TODO: NBT, advancements, and predicates
}

impl Selector {
    pub fn executor() -> Self {
        Selector::new(TargetKind::Executor)
    }
    pub fn new(target: TargetKind) -> Self {
        Selector {
            target,
            pos: Default::default(),
            scores: None,
            team: None,
            limit: None,
            sort: None,
            level: None,
            gamemode: None,
            pitch: None,
            yaw: None,
            name: None,
            tags: Default::default(),
        }
    }
}

impl Display for Selector {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Selector {
            target,
            pos,
            scores,
            team,
            limit,
            sort,
            level,
            gamemode,
            pitch,
            yaw,
            name,
            tags,
        } = self;

        let mut out = String::with_capacity(8);

        out.push('@');
        out.push(target.selector_char());

        write!(f, "{}", out)
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct PosSelector {
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub z: Option<f64>,
    pub kind: PosKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PosKind {
    Volume(Option<f64>, Option<f64>, Option<f64>),
    Distance(RangeArg<f64>),
    None,
}

impl Default for PosKind {
    fn default() -> Self {
        PosKind::None
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TeamSelector {
    pub team: Option<Team>,
    pub mode: ArgumentMode,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ArgumentMode {
    Is,
    Not,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SortingMode {
    Nearest,
    Furthest,
    Random,
    Arbitrary,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Gamemode {
    Spectator,
    Adventure,
    Creative,
    Survival,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RangeArg<Idx> {
    Single(Idx),
    Range(Range<Idx>),
}

impl<Idx> Display for RangeArg<Idx>
where
    Idx: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            RangeArg::Single(i) => i.fmt(f),
            RangeArg::Range(range) => {
                range.start.fmt(f)?;
                write!(f, "..")?;
                range.end.fmt(f)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::selector::RangeArg;

    #[test]
    fn range_arg_display() {
        let single = RangeArg::Single(20);
        let range = RangeArg::Range(20..40);

        assert_eq!("20", single.to_string());
        assert_eq!("20..40", range.to_string());
    }
}
