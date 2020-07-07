use super::*;

pub trait MutVisitor<'ast>: Sized {
    fn visit_ident(&mut self, _ident: &'ast mut Identifier) {}

    fn visit_path(&mut self, path: &'ast mut Path, id: AstId) {
        walk_path(self, path, id)
    }

    fn visit_program(&mut self, program: &'ast mut Program) {
        walk_program(self, program)
    }

    fn visit_item(&mut self, item: &'ast mut Item) {
        walk_item(self, item)
    }

    fn visit_mod(&mut self, module: &'ast mut Mod) {
        walk_mod(self, module)
    }

    fn visit_struct(&mut self, struc: &'ast mut Struct) {
        walk_struct(self, struc)
    }

    fn visit_struct_field(&mut self, field: &'ast mut StructField) {
        walk_struct_field(self, field)
    }

    fn visit_class(&mut self, class: &'ast mut Class) {
        walk_class(self, class)
    }

    fn visit_branch(&mut self, branch: &'ast mut Branch) {
        walk_branch(self, branch)
    }

    fn visit_branch_variant(&mut self, variant: &'ast mut BranchVariant) {
        walk_branch_variant(self, variant)
    }

    fn visit_type(&mut self, ty: &'ast mut Type) {
        walk_type(self, ty)
    }

    fn visit_block(&mut self, block: &'ast mut Block) {
        walk_block(self, block)
    }

    fn visit_expr(&mut self, expr: &'ast mut Expr) {
        walk_expr(self, expr)
    }

    fn visit_stmt(&mut self, stmt: &'ast mut Stmt) {
        walk_stmt(self, stmt)
    }

    fn visit_fn_param(&mut self, param: &'ast mut FnParam) {
        walk_fn_param(self, param)
    }
}

#[inline]
pub fn visit_vec<'ast, T, F>(elems: &'ast mut Vec<T>, mut visit_elem: F)
    where
        F: FnMut(&'ast mut T),
{
    for elem in elems {
        visit_elem(elem);
    }
}

pub fn walk_path<'ast, T: MutVisitor<'ast>>(visitor: &mut T, Path { items }: &'ast mut Path, _id: AstId) {
    visit_vec(items, |elem| visitor.visit_ident(elem))
}

pub fn walk_program<'ast, T: MutVisitor<'ast>>(visitor: &mut T, Program { items }: &'ast mut Program) {
    visit_vec(items, |elem| visitor.visit_item(elem))
}

pub fn walk_item<'ast, T: MutVisitor<'ast>>(
    visitor: &mut T,
    Item {
        name,
        vis: _,
        kind,
        span: _,
        id: _,
    }: &'ast mut Item,
) {
    visitor.visit_ident(name);

    match kind {
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

pub fn walk_mod<'ast, T: MutVisitor<'ast>>(visitor: &mut T, Mod { items, inline: _ }: &'ast mut Mod) {
    visit_vec(items, |elem| visitor.visit_item(elem))
}

pub fn walk_struct<'ast, T: MutVisitor<'ast>>(visitor: &mut T, Struct { members }: &'ast mut Struct) {
    visit_vec(members, |elem| visitor.visit_struct_field(elem))
}

pub fn walk_struct_field<'ast, T: MutVisitor<'ast>>(
    visitor: &mut T,
    StructField {
        name,
        ty,
        vis: _,
        default,
    }: &'ast mut StructField,
) {
    visitor.visit_ident(name);
    visitor.visit_type(ty);

    if let Some(expr) = default {
        visitor.visit_expr(expr)
    }
}

pub fn walk_class<'ast, T: MutVisitor<'ast>>(
    visitor: &mut T,
    Class {
        builtin: _,
        bounds,
        members,
    }: &'ast mut Class,
) {
    if let ClassBounds::Ty(ty) = bounds {
        visitor.visit_type(ty);
    }

    visit_vec(members, |elem| visitor.visit_struct_field(elem))
}

pub fn walk_branch<'ast, T: MutVisitor<'ast>>(
    visitor: &mut T,
    Branch { bounds, variants }: &'ast mut Branch,
) {
    if let ClassBounds::Ty(ty) = bounds {
        visitor.visit_type(ty);
    }

    visit_vec(variants, |elem| visitor.visit_branch_variant(elem))
}

pub fn walk_branch_variant<'ast, T: MutVisitor<'ast>>(
    visitor: &mut T,
    BranchVariant {
        span: _,
        name,
        members,
        id: _
    }: &'ast mut BranchVariant,
) {
    visitor.visit_ident(name);
    visit_vec(members, |elem| visitor.visit_struct_field(elem))
}

pub fn walk_type<'ast, T: MutVisitor<'ast>>(visitor: &mut T, Type { span: _, kind, id }: &'ast mut Type) {
    match kind {
        TypeKind::Tuple(items) => visit_vec(items, |elem| visitor.visit_type(elem)),
        TypeKind::And(items) => visit_vec(items, |TypeRef { name, id}| visitor.visit_path(name, *id)),
        TypeKind::Named(name) => visitor.visit_path(name, *id),

        TypeKind::Never => {}
        TypeKind::Unit => {}
        TypeKind::Infer => {}
        TypeKind::Err => {}
    }
}

pub fn walk_block<'ast, T: MutVisitor<'ast>>(
    visitor: &mut T,
    Block {
        span: _,
        stmts,
        id: _,
    }: &'ast mut Block,
) {
    visit_vec(stmts, |elem| visitor.visit_stmt(elem));
}

pub fn walk_expr<'ast, T: MutVisitor<'ast>>(visitor: &mut T, Expr { span: _, kind, id }: &'ast mut Expr) {
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

pub fn walk_stmt<'ast, T: MutVisitor<'ast>>(
    visitor: &mut T,
    Stmt {
        span: _,
        kind,
        id: _,
    }: &'ast mut Stmt,
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

pub fn walk_fn_param<'ast, T: MutVisitor<'ast>>(visitor: &mut T, FnParam { name, ty }: &'ast mut FnParam) {
    visitor.visit_ident(name);
    visitor.visit_type(ty);
}
