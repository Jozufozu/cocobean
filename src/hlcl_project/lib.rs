use std::collections::HashMap;

use lasso::{RodeoResolver, Spur};
use smallvec::SmallVec;

use hlcl_ast::Program;
use hlcl_span::SourceFile;

pub type Path = SmallVec<[Spur; 2]>;

pub type PathMap<T> = HashMap<Path, T>;

pub struct Modules {
    main: Program,
    modules: PathMap<Result<Program, ()>>,
}

impl Modules {
    pub fn new(main: Program, modules: PathMap<Result<Program, ()>>) -> Self {
        Modules { main, modules }
    }
}

impl From<Modules> for (Program, PathMap<Result<Program, ()>>) {
    #[inline]
    fn from(Modules { main, modules }: Modules) -> Self {
        (main, modules)
    }
}

#[derive(Debug)]
pub struct Project {
    pub names: RodeoResolver,
    pub project_name: Spur,
    sources: PathMap<SourceFile>,
}

impl Project {
    pub fn new(names: RodeoResolver, project_name: Spur, sources: PathMap<SourceFile>) -> Self {
        Project {
            names,
            project_name,
            sources,
        }
    }
}
