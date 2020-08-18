use super::*;

pub trait MutVisitor: Sized {
    fn visit_ident(&mut self, _ident: &mut Identifier) {}

    fn visit_path(&mut self, path: &mut Path, id: AstId) {
        walk_path(self, path, id)
    }

    fn visit_program(&mut self, program: &mut Program) {
        walk_mod(self, &mut program.module)
    }

    fn visit_item(&mut self, item: &mut Item) {
        walk_item(self, item)
    }

    fn visit_use_tree(&mut self, tree: &mut UseTree) {
        walk_use_tree(self, tree);
    }

    fn visit_mod(&mut self, module: &mut Mod) {
        walk_mod(self, module)
    }

    fn visit_struct(&mut self, struc: &mut Struct) {
        walk_struct(self, struc)
    }

    fn visit_struct_field(&mut self, field: &mut StructField) {
        walk_struct_field(self, field)
    }

    fn visit_class(&mut self, class: &mut Class) {
        walk_class(self, class)
    }

    fn visit_branch(&mut self, branch: &mut Branch) {
        walk_branch(self, branch)
    }

    fn visit_branch_variant(&mut self, variant: &mut BranchVariant) {
        walk_branch_variant(self, variant)
    }

    fn visit_type(&mut self, ty: &mut Type) {
        walk_type(self, ty)
    }

    fn visit_block(&mut self, block: &mut Block) {
        walk_block(self, block)
    }

    fn visit_expr(&mut self, expr: &mut Expr) {
        walk_expr(self, expr)
    }

    fn visit_stmt(&mut self, stmt: &mut Stmt) {
        walk_stmt(self, stmt)
    }

    fn visit_fn_param(&mut self, param: &mut FnParam) {
        walk_fn_param(self, param)
    }
}

#[inline]
pub fn visit_vec<T, F>(elems: &mut Vec<T>, mut visit_elem: F)
where
    F: FnMut(&mut T),
{
    for elem in elems {
        visit_elem(elem);
    }
}

pub fn walk_path<T: MutVisitor>(visitor: &mut T, Path { items }: &mut Path, _id: AstId) {
    visit_vec(items, |elem| visitor.visit_ident(elem))
}

pub fn walk_item<T: MutVisitor>(
    visitor: &mut T,
    Item {
        name,
        vis: _,
        kind,
        span: _,
        id: _,
    }: &mut Item,
) {
    visitor.visit_ident(name);

    match kind {
        ItemKind::Use(tree) => visitor.visit_use_tree(tree),
        ItemKind::Mod(item) => visitor.visit_mod(item),
        ItemKind::Struct(item) => visitor.visit_struct(item),
        ItemKind::Class(item) => visitor.visit_class(item),
        ItemKind::Branch(item) => visitor.visit_branch(item),
        ItemKind::Fn(FnSig { params, ret }, block) => {
            visit_vec(params, |elem| visitor.visit_fn_param(elem));

            if let FnReturn::Ty(ty) = ret {
                visitor.visit_type(ty);
            }

            if let Some(block) = block {
                visitor.visit_block(block);
            }
        }
        ItemKind::Err => {}
    }
}

pub fn walk_use_tree<T: MutVisitor>(visitor: &mut T, UseTree { span: _, path, kind, id }: &mut UseTree) {
    visitor.visit_path(path, *id);
    match kind {
        UseTreeKind::Rebind(ident) => visitor.visit_ident(ident),
        UseTreeKind::Tree(items) => visit_vec(items, |tree| visitor.visit_use_tree(tree)),
        _ => (),
    }
}

pub fn walk_mod<T: MutVisitor>(visitor: &mut T, Mod { items, inline: _ }: &mut Mod) {
    visit_vec(items, |elem| visitor.visit_item(elem))
}

pub fn walk_struct<T: MutVisitor>(visitor: &mut T, Struct { members }: &mut Struct) {
    visit_vec(members, |elem| visitor.visit_struct_field(elem))
}

pub fn walk_struct_field<T: MutVisitor>(
    visitor: &mut T,
    StructField { name, ty, vis: _ }: &mut StructField,
) {
    visitor.visit_ident(name);
    visitor.visit_type(ty);
}

pub fn walk_class<T: MutVisitor>(
    visitor: &mut T,
    Class {
        builtin: _,
        bounds,
        members,
    }: &mut Class,
) {
    if let ClassBounds::Ty(ty) = bounds {
        visitor.visit_type(ty);
    }

    visit_vec(members, |elem| visitor.visit_struct_field(elem))
}

pub fn walk_branch<T: MutVisitor>(visitor: &mut T, Branch { bounds, variants }: &mut Branch) {
    if let ClassBounds::Ty(ty) = bounds {
        visitor.visit_type(ty);
    }

    visit_vec(variants, |elem| visitor.visit_branch_variant(elem))
}

pub fn walk_branch_variant<T: MutVisitor>(
    visitor: &mut T,
    BranchVariant {
        span: _,
        name,
        members,
        id: _,
    }: &mut BranchVariant,
) {
    visitor.visit_ident(name);
    visit_vec(members, |elem| visitor.visit_struct_field(elem))
}

pub fn walk_type<T: MutVisitor>(visitor: &mut T, Type { span: _, kind, id }: &mut Type) {
    match kind {
        TypeKind::Tuple(items) => visit_vec(items, |elem| visitor.visit_type(elem)),
        TypeKind::And(items) => {
            visit_vec(items, |TypeRef { name, id }| visitor.visit_path(name, *id))
        }
        TypeKind::Named(name) => visitor.visit_path(name, *id),

        TypeKind::Never => {}
        TypeKind::Unit => {}
        TypeKind::Infer => {}
        TypeKind::Err => {}
    }
}

pub fn walk_block<T: MutVisitor>(
    visitor: &mut T,
    Block {
        span: _,
        stmts,
        id: _,
    }: &mut Block,
) {
    visit_vec(stmts, |elem| visitor.visit_stmt(elem));
}

pub fn walk_expr<T: MutVisitor>(visitor: &mut T, Expr { span: _, kind, id }: &mut Expr) {
    match kind {
        ExprKind::Lit(_) => {}
        ExprKind::Variable(name) => visitor.visit_ident(name),
        ExprKind::UnOp(_, expr) => visitor.visit_expr(expr),
        ExprKind::BinOp(_, lhs, rhs)
        | ExprKind::Assign(_, lhs, rhs)
        | ExprKind::AssignOp(_, lhs, rhs) => {
            visitor.visit_expr(lhs);
            visitor.visit_expr(rhs);
        }
        ExprKind::Is(_, expr, ty) => {
            visitor.visit_expr(expr);
            visitor.visit_type(ty);
        }
        ExprKind::Call(path, args) => {
            visitor.visit_path(path, *id);
            visit_vec(args, |elem| visitor.visit_expr(elem))
        }
        ExprKind::MethodCall() => {}
        ExprKind::FieldAccess(expr, qualifier, field) => {
            visitor.visit_expr(expr);
            if let Some(b) = qualifier {
                let Disambiguator { span: _, name, id } = b.as_mut();
                visitor.visit_path(name, *id);
            }
            visitor.visit_ident(field);
        }
        ExprKind::Tuple(items) => visit_vec(items, |elem| visitor.visit_expr(elem)),
        ExprKind::Block(block) | ExprKind::Loop(block) => visitor.visit_block(block),
        ExprKind::While(expr, block) => {
            visitor.visit_expr(expr);
            visitor.visit_block(block);
        }
        ExprKind::If(expr, block, els) => {
            visitor.visit_expr(expr);
            visitor.visit_block(block);
            if let Some(expr) = els {
                visitor.visit_expr(expr)
            }
        }
        ExprKind::Paren(expr) => visitor.visit_expr(expr),
        ExprKind::Err => {}
    }
}

pub fn walk_stmt<T: MutVisitor>(
    visitor: &mut T,
    Stmt {
        span: _,
        kind,
        id: _,
    }: &mut Stmt,
) {
    match kind {
        StmtKind::Item(item) => visitor.visit_item(item),
        StmtKind::Expr(expr) | StmtKind::Semi(expr) => visitor.visit_expr(expr),
        StmtKind::Let(ident, ty, expr) => {
            visitor.visit_ident(ident);
            if let Some(ty) = ty {
                visitor.visit_type(ty);
            }
            visitor.visit_expr(expr);
        }
        StmtKind::Ret(expr) | StmtKind::Break(expr) => {
            if let Some(expr) = expr {
                visitor.visit_expr(expr)
            }
        }
        StmtKind::Continue => {}
        StmtKind::Err => {}
    }
}

pub fn walk_fn_param<T: MutVisitor>(visitor: &mut T, FnParam { name, ty }: &mut FnParam) {
    visitor.visit_ident(name);
    visitor.visit_type(ty);
}
