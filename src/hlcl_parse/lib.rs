use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use crossbeam::crossbeam_channel;
use lalrpop_util::ErrorRecovery;
use lalrpop_util::ParseError;
use rayon::{ThreadPool, ThreadPoolBuilder};
use walkdir::{DirEntry, WalkDir};

pub use hlcl::ProgramParser;
use hlcl_ast::Program;
use hlcl_project::{Modules, Path as ASTPath, PathMap, Project};
use hlcl_span::Span;
use hlcl_span::sourcemap::{SourceMap, SourceFile};
use hlcl_span::kw::{self, Interner};
pub use lexer::Lexer;

use crate::err::ParserError;
use crate::lexer::Token;

#[allow(clippy::all)]
#[cfg_attr(rustfmt, rustfmt_skip)]
pub mod hlcl;
pub mod err;
pub mod lexer;
pub mod util;

pub const ROOT_FILE: &str = "main.hlcl";
pub const EXTENSION: &str = ".hlcl";

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

        let interner = kw::create_interner();

        Ok(ParsingSession {
            interner,
            project_name: project_name.as_ref().to_string(),
            project_path,
            workers: ThreadPoolBuilder::new().num_threads(8).build().unwrap(),
        })
    }

    pub fn parse_project(self) -> Result<(Modules, Project), ()> {
        let ParsingSession {
            mut interner,
            project_name,
            project_path,
            workers,
        } = self;

        let mut found = 0;
        let (psend, prc) = crossbeam_channel::unbounded();
        let mut total_len = 0;

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

                let length =
                    path.metadata()
                        .map(|met| met.len())
                        .expect("could not get the length of a file") as usize;

                let span = Span::new(total_len, total_len + length);

                workers.install(|| {
                    let ast_path = Self::dir_to_module_path(&path, interner);
                    let result = ParsingSession::process_file(path.path(), name, span, interner);

                    sender.send((ast_path, result)).unwrap();
                });

                total_len += length;
            }
        }

        let mut main_module = None;
        let mut modules = PathMap::new();
        let mut the_source = SourceMap::with_capacity(total_len.next_power_of_two());

        for (mut module_path, result) in prc.iter() {
            match result {
                Ok((source, program)) => {
                    if  main_module.is_none()
                        && module_path.len() == 1
                        && *kw::MAIN == module_path[0]
                    {
                        main_module = Some(program.expect("malformed main file"));
                    } else {
                        module_path.insert(0, *kw::PACK);
                        modules.insert(module_path.clone(), program);
                    }

                    the_source.insert_source(source);
                }
                Err(e) => println!("{:}", e),
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
                    Project::new(interner.into_resolver(), the_source),
                )
            })
            .ok_or(())
    }

    fn process_file<S, P>(
        path: P,
        name: S,
        file_span: Span,
        interner: &Interner,
    ) -> Result<(SourceFile, Result<Program, String>), std::io::Error>
    where
        S: AsRef<str>,
        P: AsRef<Path>,
    {
        File::open(path.as_ref())
            .and_then(|mut file| {
                let mut source = String::with_capacity(file_span.len() as usize);

                file.read_to_string(&mut source).map(|_| source)
            })
            .map(|source| {
                let (soft_errs, result) = Self::parse_string(file_span.lo_idx(), interner, &source);

                for _err in soft_errs.into_iter() {
                    // TODO: Process and store errors to be displayed later. Send them to the main thread separately?
                }

                let program = result.map_err(|e| format!("{}", e)); // TODO: Process this error too
                (
                    SourceFile::new(source, name.as_ref().to_string(), file_span),
                    program,
                )
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
            .take(path.depth())
            .map(|part| interner.get_or_intern(part.file_stem().unwrap().to_string_lossy()))
            .collect();
        rev.reverse();
        rev
    }

    fn parse_string<'input>(
        start_pos: usize,
        interner: &Interner,
        source: &'input str,
    ) -> ParserResult<'input> {
        let mut errs = Vec::new();

        let program = ProgramParser::new().parse(
            &source,
            &interner,
            &mut errs,
            Lexer::new(&source, start_pos),
        );

        (errs, program)
    }
}
