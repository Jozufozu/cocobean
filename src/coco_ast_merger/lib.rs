use smallvec::smallvec;

use coco_ast::{Block, Branch, BranchVariant, Class, Expr, FnParam, Item, ItemKind, Mod, Program, Stmt, Struct, StructField, Type, UseTree, Path};
use coco_ast::id::AstId;
use coco_ast::mut_visit::{self, MutVisitor};
use coco_project::{Modules, Path as ProjectPath, PathMap, Project};
use coco_span::kw;
use coco_span::lasso::Spur;
use radix_trie::Trie;

pub struct AstFinalizer<'a> {
    project: &'a mut Project,
    modules: PathMap<(Result<Program, String>, Vec<String>)>,
    next_id: AstId,
    current_path: ProjectPath,
    discovered: Vec<ProjectPath>,
}

impl<'a> AstFinalizer<'a> {
    pub fn unify(Modules { main, modules }: Modules, project: &'a mut Project) -> Program {
        let (mut main, errs) = main;
        let current_path = smallvec![*kw::PACK];
        let mut unifier = AstFinalizer {
            project,
            modules,
            next_id: 0.into(),
            current_path,
            discovered: Vec::new(),
        };

        for err in errs {
            println!("{}", err);
        }

        unifier.visit_program(&mut main);

        main
    }

    fn discover(&mut self, path: &ProjectPath) -> Option<Program> {
        if let Some((result, errs)) = self.modules.remove(path) {
            for err in errs {
                println!("{}", err);
            }
            match result {
                Ok(source) => {
                    return Some(source);
                }
                Err(msg) => println!("{}", msg),
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
        mut_visit::walk_mod(self, &mut program.module);
    }

    fn visit_item(&mut self, item: &mut Item) {
        item.id = self.next_id();

        if let ItemKind::Mod(module) = &mut item.kind {
            self.push_mod_path(item.name.spur);

            if !module.inline {
                let path = self.current_path.clone();
                if let Some(program) = self.discover(&path) {
                    module.items = program.module.items
                }
            }

            for item in &mut module.items {
                self.visit_item(item);
            }

            self.pop_mod_path();
            return;
        }

        mut_visit::walk_item(self, item);
    }

    fn visit_use_tree(&mut self, tree: &mut UseTree) {
        tree.id = self.next_id();
        mut_visit::walk_use_tree(self, tree);
    }

    fn visit_branch_variant(&mut self, variant: &mut BranchVariant) {
        variant.id = self.next_id();
        mut_visit::walk_branch_variant(self, variant);
    }

    fn visit_type(&mut self, ty: &mut Type) {
        ty.id = self.next_id();
        mut_visit::walk_type(self, ty);
    }

    fn visit_block(&mut self, block: &mut Block) {
        block.id = self.next_id();
        mut_visit::walk_block(self, block);
    }

    fn visit_expr(&mut self, expr: &mut Expr) {
        expr.id = self.next_id();
        mut_visit::walk_expr(self, expr);
    }

    fn visit_stmt(&mut self, stmt: &mut Stmt) {
        stmt.id = self.next_id();
        mut_visit::walk_stmt(self, stmt);
    }
}
