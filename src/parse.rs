use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use crossbeam::crossbeam_channel;
use fxhash::FxBuildHasher;
use lalrpop_util::ErrorRecovery;
use lalrpop_util::ParseError;
use lasso::{Spur, ThreadedRodeo};
use walkdir::{DirEntry, WalkDir};

pub use hlcl::ProgramParser;
pub use lexer::Lexer;

use crate::ast::{Identifier, Path as ASTPath, Program};
use crate::parse::err::ParserError;
use crate::parse::lexer::Token;
use crate::span::Span;

#[allow(clippy::all)]
#[cfg_attr(rustfmt, rustfmt_skip)]
pub mod hlcl;
pub mod err;
pub mod lexer;

pub const ROOT_FILE: &'static str = "main.hlcl";
pub const EXTENSION: &'static str = ".hlcl";

pub type Interner = ThreadedRodeo<Spur, FxBuildHasher>;

type ParserResult<'input> = (
    Vec<ErrorRecovery<usize, Token<'input>, ParserError>>,
    Result<Program, ParseError<usize, Token<'input>, ParserError>>,
);

#[derive(Debug)]
pub struct Project {
    interner: Interner,
    main_module: SourceFile,
    loaded_files: Vec<(ASTPath, SourceFile)>,
    stored_errs: Vec<(ASTPath, ())>,
}

#[derive(Debug)]
pub struct SourceFile {
    program: Program,
    source: String,
    file_name: String,
}

#[derive(Debug)]
pub struct ParsingSession {
    interner: Interner,
    project_path: PathBuf,
}

impl ParsingSession {
    pub fn new<P: AsRef<Path>>(project_path: P) -> Self {
        let mut project_path = project_path.as_ref().to_path_buf();

        if project_path.is_file() {
            if let Some(parent) = project_path.parent() {
                project_path = parent.to_path_buf();
            }
        }

        ParsingSession {
            interner: Interner::with_hasher(FxBuildHasher::default()),
            project_path,
        }
    }

    pub fn parse_project(self) -> Result<Project, ()> {
        let ParsingSession {
            interner,
            project_path,
        } = self;

        let main_file = project_path.join(ROOT_FILE);

        if !main_file.is_file() {
            return Err(());
        }

        let workers = rayon::ThreadPoolBuilder::new().build().unwrap();

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
                    let result = File::open(path.path())
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
                                .map(|program| SourceFile {
                                    program,
                                    source,
                                    file_name: name.to_string(),
                                })
                        });

                    sender.send((ast_path, result)).unwrap();
                })
            }
        }

        let mut main_module = None;
        let mut modules = Vec::new();
        let mut stored_errs = Vec::new();

        for (module_path, result) in prc.iter() {
            if main_module.is_none()
                && module_path.items.len() == 1
                && "main" == interner.resolve(&module_path.items[0].val)
            {
                main_module = Some(result?);
            } else {
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

        if let Some(main_module) = main_module {
            Ok(Project {
                interner,
                main_module,
                loaded_files: modules,
                stored_errs,
            })
        } else {
            Err(())
        }
    }

    fn dir_to_module_path(path: &DirEntry, interner: &Interner) -> ASTPath {
        ASTPath {
            items: path
                .path()
                .ancestors()
                .into_iter()
                .take(path.depth())
                .map(|part| interner.get_or_intern(part.to_string_lossy()))
                .map(|val| Identifier {
                    span: Span::DUMMY,
                    val,
                })
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect(),
        }
    }

    fn parse_string<'input>(interner: &Interner, source: &'input String) -> ParserResult<'input> {
        let mut errs = Vec::new();

        let program =
            ProgramParser::new().parse(&source, &interner, &mut errs, Lexer::new(&source));

        (errs, program)
    }
}
