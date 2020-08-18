use std::collections::HashMap;

use smallvec::SmallVec;

use coco_ast::Program;
use coco_span::sourcemap::SourceMap;
use coco_span::lasso::{RodeoResolver, Spur};

/// This is a simpler structure for making reasoning about project files more closely related to the ast
pub type Path = SmallVec<[Spur; 2]>;

pub type PathMap<T> = HashMap<Path, T>;

pub type Resolver = RodeoResolver<Spur>;

pub struct Modules {
    pub main: (Program, Vec<String>),
    pub modules: PathMap<(Result<Program, String>, Vec<String>)>,
}

impl Modules {
    pub fn new(main: (Program, Vec<String>), modules: PathMap<(Result<Program, String>, Vec<String>)>) -> Self {
        Modules { main, modules }
    }
}

#[derive(Debug)]
pub struct Project {
    pub names: Resolver,
    pub sources: SourceMap,
}

impl Project {
    pub fn new(names: Resolver, sources: SourceMap) -> Self {
        Project { names, sources }
    }
}
