use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use crossbeam::crossbeam_channel;
use fxhash::FxBuildHasher;
use lalrpop_util::ErrorRecovery;
use lalrpop_util::ParseError;
use lasso::{Spur, ThreadedRodeo};
use rayon::{ThreadPool, ThreadPoolBuilder};
use smallvec::smallvec;
use walkdir::{DirEntry, WalkDir};

pub use hlcl::ProgramParser;
use hlcl_ast::Program;
use hlcl_project::{Modules, Path as ASTPath, PathMap, Project};
use hlcl_span::SourceFile;
pub use lexer::Lexer;

use crate::err::ParserError;
use crate::lexer::Token;

#[allow(clippy::all)]
#[cfg_attr(rustfmt, rustfmt_skip)]
pub mod hlcl;
pub mod err;
pub mod lexer;
pub mod util;

pub const PRIMITIVES: &[&str] = &["int", "bool", "string", "pos"];
pub const MAIN: &str = "main";
pub const ROOT_FILE: &str = "main.hlcl";
pub const EXTENSION: &str = ".hlcl";

pub type Interner = ThreadedRodeo<Spur, FxBuildHasher>;

type ParserResult<'input> = (
    Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    Result<Program, ParseError<usize, Token<'input>, ParserError>>,
);

#[derive(Debug)]
pub struct ParsingSession {
    interner: Interner,
    project_name: String,
    project_path: PathBuf,
    workers: ThreadPool,
}

#[derive(Debug)]
pub enum ProjectStructureErr {
    NameErr(NameErr),
    MissingMain,
    MustBeDir,
}

impl From<NameErr> for ProjectStructureErr {
    #[inline(always)]
    fn from(err: NameErr) -> Self {
        ProjectStructureErr::NameErr(err)
    }
}

#[derive(Debug)]
pub enum NameErr {
    TooShort,
    InvalidCharacter,
    MustStartWithAlphabetic,
}

impl ParsingSession {
    pub fn new<P: AsRef<Path>, S: AsRef<str>>(
        project_path: P,
        project_name: S,
    ) -> Result<Self, ProjectStructureErr> {
        Self::validate_name(project_name.as_ref())?;

        let project_path = project_path.as_ref().to_path_buf();

        if project_path.is_file() {
            return Err(ProjectStructureErr::MustBeDir);
        }

        if !project_path.join(ROOT_FILE).is_file() {
            return Err(ProjectStructureErr::MissingMain);
        }

        let interner = ThreadedRodeo::with_hasher(FxBuildHasher::default());

        for str in PRIMITIVES {
            interner.get_or_intern(str);
        }

        Ok(ParsingSession {
            interner,
            project_name: project_name.as_ref().to_string(),
            project_path,
            workers: ThreadPoolBuilder::new().num_threads(8).build().unwrap(),
        })
    }

    pub fn parse_project(self) -> Result<(Modules, Project), ()> {
        let ParsingSession {
            interner,
            project_name,
            project_path,
            workers,
        } = self;

        let project_name = interner.get_or_intern(project_name);
        let main = interner.get_or_intern(MAIN);

        let mut found = 0;
        let (psend, prc) = crossbeam_channel::unbounded();

        for path in WalkDir::new(&project_path)
            .follow_links(true)
            .into_iter()
            .filter_map(|f| f.ok())
        {
            let name = path.file_name().to_string_lossy();

            if name.ends_with(EXTENSION) {
                found += 1;
                let sender = psend.clone();
                let interner = &interner;

                path.metadata()
                    .map(|met| met.len())
                    .workers
                    .install(move || {
                        let ast_path = Self::dir_to_module_path(&path, interner);
                        let result = ParsingSession::process_file(path.path(), name, interner);

                        sender.send((found, ast_path, result)).unwrap();
                    })
            }
        }

        let mut main_module = None;
        let mut modules = PathMap::new();
        let mut sources = PathMap::new();

        for (idx, mut module_path, result) in prc.iter() {
            if main_module.is_none() && module_path.len() == 1 && main == module_path[0] {
                let (source, program) = result?;
                main_module = Some(program?);

                sources.insert(smallvec![project_name], source);
            } else {
                match result {
                    Ok((source, result)) => {
                        module_path.insert(0, project_name);

                        modules.insert(module_path.clone(), result);
                        sources.insert(module_path, source);
                    }
                    Err(()) => {}
                }
            }

            found -= 1;
            if found == 0 {
                break;
            }
        }

        main_module
            .map(|main_module| {
                (
                    Modules::new(main_module, modules),
                    Project::new(interner.into_resolver(), project_name, sources),
                )
            })
            .ok_or(())
    }

    fn process_file<S, P>(
        path: P,
        name: S,
        interner: &Interner,
    ) -> Result<(SourceFile, Result<Program, ()>), ()>
    where
        S: AsRef<str>,
        P: AsRef<Path>,
    {
        File::open(path.as_ref())
            .map_err(|_| ())
            .and_then(|mut file| {
                let mut source = String::with_capacity(1024);

                file.read_to_string(&mut source)
                    .map(|_| source)
                    .map_err(|_| ())
            })
            .map(|source| {
                let (soft_errs, result) = Self::parse_string(interner, &source);

                for _err in soft_errs.into_iter() {
                    // TODO: Process and store errors to be displayed later. Send them to the main thread separately?
                }

                let program = result.map_err(|_| ()); // TODO: Process this error too
                (SourceFile::new(source, name.as_ref().to_string()), program)
            })
    }

    fn validate_name(name: &str) -> Result<(), NameErr> {
        let mut chars = name.chars();
        if let Some(first) = &chars.next() {
            if !first.is_ascii_alphabetic() {
                return Err(NameErr::MustStartWithAlphabetic);
            }
        }

        if !chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_') {
            Err(NameErr::InvalidCharacter)
        } else {
            Ok(())
        }
    }

    fn dir_to_module_path(path: &DirEntry, interner: &Interner) -> ASTPath {
        let mut rev: ASTPath = path
            .path()
            .ancestors()
            .into_iter()
            .take(path.depth())
            .map(|part| interner.get_or_intern(part.file_stem().unwrap().to_string_lossy()))
            .collect();
        rev.reverse();
        rev
    }

    fn parse_string<'input>(interner: &Interner, source: &'input String) -> ParserResult<'input> {
        let mut errs = Vec::new();

        let program =
            ProgramParser::new().parse(&source, &interner, &mut errs, Lexer::new(&source));

        (errs, program)
    }
}
