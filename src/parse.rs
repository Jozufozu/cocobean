use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use crossbeam::crossbeam_channel;
use fxhash::FxBuildHasher;
use lalrpop_util::ErrorRecovery;
use lalrpop_util::ParseError;
use lasso::{Spur, ThreadedRodeo};
use rayon::{ThreadPool, ThreadPoolBuilder};
use walkdir::{DirEntry, WalkDir};

pub use hlcl::ProgramParser;
pub use lexer::Lexer;

use crate::ast::{Identifier, Path as ASTPath, Program};
use crate::parse::err::ParserError;
use crate::parse::lexer::Token;
use crate::project::{Project, SourceFile};
use crate::span::Span;

#[allow(clippy::all)]
#[cfg_attr(rustfmt, rustfmt_skip)]
pub mod hlcl;
pub mod err;
pub mod lexer;

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
    MustStartWithAlphabetic
}

impl ParsingSession {
    pub fn new<P: AsRef<Path>>(project_path: P, project_name: String) -> Result<Self, ProjectStructureErr> {
        Self::validate_name(&project_name)?;

        let project_path = project_path.as_ref().to_path_buf();

        if project_path.is_file() {
            return Err(ProjectStructureErr::MustBeDir)
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
            project_name,
            project_path,
            workers: ThreadPoolBuilder::new().build().unwrap(),
        })
    }

    pub fn parse_project(self) -> Result<Project, ()> {
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

                workers.install(|| {
                    let ast_path = Self::dir_to_module_path(&path, interner);
                    let result = ParsingSession::process_file(path.path(), name, interner);

                    sender.send((ast_path, result)).unwrap();
                })
            }
        }

        let mut main_module = None;
        let mut modules = Vec::new();
        let mut stored_errs = Vec::new();

        for (mut module_path, result) in prc.iter() {
            if main_module.is_none()
                && module_path.items.len() == 1
                && main == module_path.items[0].val
            {
                module_path.items[0].val = project_name;
                main_module = Some((module_path, result?));
            } else {
                module_path.items.insert(0, Identifier::dummy(project_name));

                match result {
                    Ok(file) => modules.push((module_path, file)),
                    Err(err) => stored_errs.push((module_path, err)),
                }
            }

            found -= 1;
            if found == 0 {
                break;
            }
        }

        main_module
            .map(|main_module| {
                Project::new(
                    interner.into_resolver(),
                    main_module,
                    modules,
                    stored_errs,
                )
            })
            .ok_or(())
    }

    fn process_file<S: AsRef<str>, P: AsRef<Path>>(path: P, name: S, interner: &Interner) -> Result<SourceFile, ()> {
        File::open(path.as_ref())
            .map_err(|_| ())
            .and_then(|mut file| {
                let mut source = String::with_capacity(1024);

                file.read_to_string(&mut source)
                    .map(|_| source)
                    .map_err(|_| ())
            })
            .and_then(|source| {
                let (soft_errs, result) = Self::parse_string(interner, &source);

                for _err in soft_errs.into_iter() {
                    // TODO: Process and store errors to be displayed later. Send them to the main thread separately?
                }

                result
                    .map_err(|_| ()) // TODO: Process this error too
                    .map(|program| SourceFile::new(program, source, name.as_ref().to_string()))
            })
    }

    fn validate_name(name: &String) -> Result<(), NameErr> {
        let mut chars = name.chars();
        if let Some(first) = &chars.next() {
            if !first.is_ascii_alphabetic() {
                return Err(NameErr::MustStartWithAlphabetic)
            }
        }

        if !chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_') {
            Err(NameErr::InvalidCharacter)
        } else {
            Ok(())
        }
    }

    fn dir_to_module_path(path: &DirEntry, interner: &Interner) -> ASTPath {
        path.path()
            .ancestors()
            .into_iter()
            .take(path.depth())
            .map(|part| interner.get_or_intern(part.file_stem().unwrap().to_string_lossy()))
            .map(|val| Identifier {
                span: Span::DUMMY,
                val,
            })
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect()
    }

    fn parse_string<'input>(interner: &Interner, source: &'input String) -> ParserResult<'input> {
        let mut errs = Vec::new();

        let program =
            ProgramParser::new().parse(&source, &interner, &mut errs, Lexer::new(&source));

        (errs, program)
    }
}
