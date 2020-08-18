use coco_ast::{visit, Program, Path, BranchVariant, Mod, TypeRef, Struct, Stmt, Item, Type, StructField, Identifier, Expr, Branch, Block, Class, FnParam, ItemKind};
use coco_ast::id::AstId;
use smallvec::SmallVec;
use coco_project::PathMap;

pub struct DefCollector<'ast> {
    root: &'ast Program

}

impl DefCollector<'_> {
    pub fn collect(root: &Program) {
        let resolver = DefCollector { root };


    }

    pub fn collect_module_defs(&mut self, module: &Mod) {
        for item in &module.items {
            match item.kind {
                ItemKind::Struct(_) => {},
                ItemKind::Class(_) => {},
                ItemKind::Branch(_) => {},
                ItemKind::Fn(_, _) => {},
                _ => (),
            }
        }
    }
}

impl<'ast> visit::Visitor<'ast> for DefCollector<'ast> {
    fn visit_ident(&mut self, _ident: &'ast Identifier) {
        unimplemented!()
    }

    fn visit_path(&mut self, path: &'ast Path, id: AstId) {
        unimplemented!()
    }

    fn visit_program(&mut self, program: &'ast Program) {
        self.collect_module_defs(&program.module);
        visit::walk_mod(self, &program.module);
    }

    fn visit_mod(&mut self, module: &'ast Mod) {
        self.collect_module_defs(module);
        visit::walk_mod(self, module);
    }

    fn visit_struct(&mut self, struc: &'ast Struct) {
        unimplemented!()
    }

    fn visit_struct_field(&mut self, field: &'ast StructField) {
        unimplemented!()
    }

    fn visit_class(&mut self, class: &'ast Class) {
        unimplemented!()
    }

    fn visit_branch(&mut self, branch: &'ast Branch) {
        unimplemented!()
    }

    fn visit_branch_variant(&mut self, variant: &'ast BranchVariant) {
        unimplemented!()
    }

    fn visit_type(&mut self, ty: &'ast Type) {
        unimplemented!()
    }

    fn visit_type_ref(&mut self, ty: &'ast TypeRef) {
        unimplemented!()
    }

    fn visit_block(&mut self, block: &'ast Block) {
        unimplemented!()
    }

    fn visit_expr(&mut self, expr: &'ast Expr) {
        unimplemented!()
    }

    fn visit_stmt(&mut self, stmt: &'ast Stmt) {
        unimplemented!()
    }

    fn visit_fn_param(&mut self, param: &'ast FnParam) {
        unimplemented!()
    }
}