use std::collections::HashMap;

use hlcl_helpers::id_map::Index;

use crate::coord::Pos;
use crate::function::Function;
pub use crate::names::*;
use crate::selector::RangeArg;

pub mod coord;
pub mod function;
pub mod names;
pub mod selector;

#[derive(Debug)]
pub struct Assembly {
    functions: Vec<Function>,
}

impl Assembly {
    pub fn new() -> Self {
        Assembly {
            functions: Vec::new(),
        }
    }

    pub fn get_fn(&self, id: &FnId) -> Option<&Function> {
        self.functions.get(id.to_usize() - 1)
    }

    pub fn insert_fn(&mut self, func: Function) -> FnId {
        self.functions.push(func);
        FnId::from_usize(self.functions.len())
    }
}
