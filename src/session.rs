use lasso::{Rodeo, Spur};
use std::path::Path;
use std::fs::File;
use std::io::Read;
use crate::parse::{Lexer, ProgramParser};
use fxhash::FxBuildHasher;

pub type Interner = Rodeo<Spur, FxBuildHasher>;

pub struct Session {
    interner: Interner,
}

impl Session {
    pub fn new() -> Self {
        Session {
            interner: Rodeo::with_hasher(FxBuildHasher::default()),
        }
    }

    pub fn parse<P: AsRef<Path>>(&mut self, main: P) {
        let main = main.as_ref();
        let mut file = File::open(main).unwrap();
        let mut source = String::with_capacity(1024);

        file.read_to_string(&mut source);

        let mut errs = Vec::new();
        let mut mods = Vec::new();

        let program = ProgramParser::new().parse(&source, &mut self.interner, &mut mods, &mut errs, Lexer::new(&source));

        println!("{:?}", mods);
        println!("{:?}", program);

        if let Some(folder) = main.parent() {
            for ident in mods.into_iter() {
                let name = self.interner.resolve(&ident.val);

                let module = folder.join(format!("{}.hlcl", name));

                self.parse_sub(module);
            }
        }
    }

    fn parse_sub<P: AsRef<Path>>(&mut self, main: P) {
        let module = main.as_ref();
        let mut file = File::open(module).unwrap();
        let mut source = String::with_capacity(1024);

        file.read_to_string(&mut source);

        let mut errs = Vec::new();
        let mut mods = Vec::new();

        let program = ProgramParser::new().parse(&source, &mut self.interner, &mut mods, &mut errs, Lexer::new(&source));

        println!("{:?}", mods);
        println!("{:?}", program);

        if let (Some(stem), Some(folder)) = (module.file_stem(), module.parent()) {
            let folder = folder.join(stem);
            println!("{:?}", folder);
            for ident in mods.into_iter() {
                let name = self.interner.resolve(&ident.val);

                let module = folder.join(format!("{}.hlcl", name));

                self.parse_sub(module);
            }
        }
    }
}