#[macro_use]
extern crate derive_more;

use crate::coord::Pos;
use crate::function::Function;
pub use crate::names::*;
use crate::selector::RangeArg;
use hlcl_helpers::id_map::*;
use std::collections::HashMap;

pub mod coord;
pub mod function;
pub mod selector;
pub mod names;

#[derive(Debug)]
pub struct Assembly {
    functions: IdMap<FnId, Function>,
    pub names: Names,
}

impl Assembly {
    pub fn new() -> Self {
        Assembly {
            functions: IdMap::new(),
            names: Names::new()
        }
    }

    pub fn get_fn(&self, id: &FnId) -> Option<&Function> {
        self.functions.get(id)
    }

    pub fn insert_fn(&mut self, func: Function) -> FnId {
        self.functions.insert(func)
    }
}
