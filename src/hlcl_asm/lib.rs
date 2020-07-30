#[macro_use]
extern crate derive_more;

use crate::coord::Pos;
use crate::function::{Function, Op, SubCommand, AnchorMode, Target, Align};
use crate::selector::{Selector, RangeArg};
use std::collections::HashMap;
use std::borrow::Cow;
use std::slice::Iter;
use std::collections::VecDeque;
use hlcl_helpers::resource_name::ResourceName;
use hlcl_helpers::id_map::*;

pub mod coord;
pub mod function;
pub mod selector;

macro_rules! identifiers {
    (
    $($other:ident),* ;
    $($name:ident : $id:ident),* ;
    $($sp_name:ident : $sp:ident -> $sp_map:ident),* ;
    ) => {
        $(id!($other))*
        $(id!($id))*
        $(id!($sp))*

        #[derive(Debug)]
        pub struct Names {
            $(
            pub $name: IdMap<$id, String>,
            )*
            $(
            pub $sp_name: IdMap<$sp, $sp_map>,
            )*
        }
    };
}

macro_rules! id {
    ($name:ident) => {
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
        pub struct $name(::std::num::NonZeroUsize);

        impl ::hlcl_helpers::id_map::Index for $name {
            fn to_usize(&self) -> usize {
                self.0.get()
            }

            fn from_usize(u: usize) -> Self {
                $name(::std::num::NonZeroUsize::new(u).unwrap())
            }
        }
    }
}

identifiers!(
    FnId,
    Register,
    BlockId;
    scores: Score,
    teams: Team,
    names: Name,
    tags: Tag,
    storages: Storage;
    dims: Dimension -> ResourceName;
);

pub struct Assembly {
    namespace: Sting,
    functions: HashMap<FnId, Function>,
    names: Names,
}

impl Assembly {
    pub fn resolve_score(&self, score: &Score) -> Option<&str> {
        self.names.scores.get(score).map(|name| name.as_ref())
    }
}