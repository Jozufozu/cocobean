#[macro_use]
extern crate derive_more;

use crate::coord::Pos;
use crate::function::{Align, AnchorMode, ExecuteItem, Function, Op, Target};
use crate::selector::{RangeArg, Selector};
use hlcl_helpers::id_map::*;
use hlcl_helpers::resource_name::ResourceName;
use std::borrow::Cow;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::slice::Iter;

pub mod coord;
pub mod function;
pub mod selector;

macro_rules! identifiers {
    (
    $($other:ident),* ;
    $($name:ident : $id:ident),* ;
    $($sp_name:ident : $sp:ident -> $sp_map:ident),* ;
    ) => {
        $(id!{$other})*
        $(id!{$id})*
        $(id!{$sp})*

        #[derive(Debug)]
        pub struct Names {
            $(
            pub $name: IdMap<$id, String>,
            )*
            $(
            pub $sp_name: IdMap<$sp, $sp_map>,
            )*
        }

        impl Names {
            pub fn new() -> Names {
                Names {
                    $(
                    $name: IdMap::new(),
                    )*
                    $(
                    $sp_name: IdMap::new(),
                    )*
                }
            }
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
    };
}

identifiers!(
    FnId,
    Register,
    BlockId;
    scores: Score,
    teams: Team,
    names: Name,
    tags: Tag;
    storages: Storage -> ResourceName,
    dims: Dimension -> ResourceName;
);

#[derive(Debug)]
pub struct Assembly {
    functions: IdMap<FnId, Function>,
    names: Names,
}

impl Assembly {
    pub fn resolve_score(&self, score: &Score) -> Option<&str> {
        self.names.scores.get(score).map(|name| name.as_ref())
    }

    pub fn resolve_tag(&self, score: &Tag) -> Option<&str> {
        self.names.tags.get(score).map(|name| name.as_ref())
    }

    pub fn new(functions: IdMap<FnId, Function>, names: Names) -> Self {
        Assembly { functions, names }
    }

    pub fn get_fn(&self, id: &FnId) -> Option<&Function> {
        self.functions.get(id)
    }
}
