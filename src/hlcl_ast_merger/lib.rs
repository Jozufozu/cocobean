use hlcl_span::kw;
use smallvec::smallvec;

use hlcl_ast::{
    Block, Branch, BranchVariant, Class, Expr, FnParam, Item, ItemKind, Mod, Program, Stmt, Struct,
    StructField, Type,
};
use hlcl_ast::id::AstId;
use hlcl_ast::mut_visit::{self, MutVisitor};
use hlcl_project::{Modules, Path as ProjectPath, PathMap, Project};
use hlcl_span::lasso::Spur;

pub struct AstFinalizer<'a> {
    project: &'a mut Project,
    modules: PathMap<Result<Program, String>>,
    next_id: AstId,
    current_path: ProjectPath,
    discovered: Vec<ProjectPath>,
}

impl<'a> AstFinalizer<'a> {
    pub fn unify(main: Modules, project: &'a mut Project) -> Program {
        let (mut main, modules) = main.into();
        let current_path = smallvec![*kw::PACK];
        let mut unifier = AstFinalizer {
            project,
            modules,
            next_id: 0.into(),
            current_path,
            discovered: Vec::new(),
        };

        unifier.visit_program(&mut main);

        main
    }

    fn discover(&mut self, path: &ProjectPath) -> Option<Program> {
        if let Some(result) = self.modules.remove(path) {
            match result {
                Ok(source) => {
                    return Some(source);
                }
                Err(msg) => println!("{}", msg)
            }
        }

        None
    }

    fn next_id(&mut self) -> AstId {
        let out = self.next_id;
        self.next_id = out + 1.into();
        out
    }

    fn push_mod_path(&mut self, name: Spur) {
        self.current_path.push(name);
    }

    fn pop_mod_path(&mut self) {
        self.current_path.pop();
    }
}

impl<'a> MutVisitor for AstFinalizer<'a> {
    fn visit_program(&mut self, program: &mut Program) {
        program.id = self.next_id();
        mut_visit::walk_program(self, program);
    }

    fn visit_item(&mut self, item: &mut Item) {
        item.id = self.next_id();

        if let ItemKind::Mod(Mod { items, inline }) = &mut item.kind {
            self.push_mod_path(item.name.spur);

            if !(*inline) {
                let path = self.current_path.clone();
                if let Some(program) = self.discover(&path) {
                    *items = program.items
                }
            }

            for item in items {
                self.visit_item(item);
            }

            self.pop_mod_path();
        }
    }

    fn visit_mod(&mut self, module: &mut Mod) {
        unimplemented!()
    }

    fn visit_struct(&mut self, struc: &mut Struct) {
        unimplemented!()
    }

    fn visit_struct_field(&mut self, field: &mut StructField) {
        unimplemented!()
    }

    fn visit_class(&mut self, class: &mut Class) {
        unimplemented!()
    }

    fn visit_branch(&mut self, branch: &mut Branch) {
        unimplemented!()
    }

    fn visit_branch_variant(&mut self, variant: &mut BranchVariant) {
        unimplemented!()
    }

    fn visit_type(&mut self, ty: &mut Type) {
        unimplemented!()
    }

    fn visit_block(&mut self, block: &mut Block) {
        unimplemented!()
    }

    fn visit_expr(&mut self, expr: &mut Expr) {
        unimplemented!()
    }

    fn visit_stmt(&mut self, stmt: &mut Stmt) {
        stmt.id = self.next_id();
    }

    fn visit_fn_param(&mut self, param: &mut FnParam) {
        unimplemented!()
    }
}
