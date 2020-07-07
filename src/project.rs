use crate::ast::{Path, Program};
use lasso::{RodeoResolver, Spur, ThreadedRodeo, Key};
use std::hash::{Hash, BuildHasher};

pub trait InternResolver<K: Key = Spur> {
    fn resolve(&self, key: &K) -> &str;
}

impl<K: Key> InternResolver<K> for RodeoResolver<K> {
    #[inline]
    fn resolve(&self, key: &K) -> &str {
        self.resolve(key)
    }
}

impl<K, S> InternResolver<K> for ThreadedRodeo<K, S>
    where
        K: Key + Hash,
        S: BuildHasher + Clone
{
    #[inline]
    fn resolve(&self, key: &K) -> &str {
        self.resolve(key)
    }
}

#[derive(Debug)]
pub struct Project {
    names: RodeoResolver,
    main_module: (Path, SourceFile),
    loaded_files: Vec<(Path, SourceFile)>,
    stored_errs: Vec<(Path, ())>,
}

impl Project {
    pub fn new(
        names: RodeoResolver,
        main_module: (Path, SourceFile),
        loaded_files: Vec<(Path, SourceFile)>,
        stored_errs: Vec<(Path, ())>,
    ) -> Self {
        Project {
            names,
            main_module,
            loaded_files,
            stored_errs,
        }
    }
}

#[derive(Debug)]
pub struct SourceFile {
    program: Program,
    source: String,
    file_name: String,
}

impl SourceFile {
    pub fn new(program: Program, source: String, file_name: String) -> Self {
        SourceFile {
            program,
            source,
            file_name,
        }
    }
}
