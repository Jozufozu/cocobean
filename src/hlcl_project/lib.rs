use std::collections::HashMap;

use smallvec::SmallVec;

use hlcl_ast::Program;
use hlcl_span::sourcemap::SourceMap;
use hlcl_span::lasso::{RodeoResolver, Spur};

/// This is a simpler structure for making reasoning about project files more closely related to the ast
pub type Path = SmallVec<[Spur; 2]>;

pub type PathMap<T> = HashMap<Path, T>;

pub type Resolver = RodeoResolver<Spur>;

pub struct Modules {
    main: Program,
    modules: PathMap<Result<Program, String>>,
}

impl Modules {
    pub fn new(main: Program, modules: PathMap<Result<Program, String>>) -> Self {
        Modules { main, modules }
    }
}

impl From<Modules> for (Program, PathMap<Result<Program, String>>) {
    #[inline]
    fn from(Modules { main, modules }: Modules) -> Self {
        (main, modules)
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
