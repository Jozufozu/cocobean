use super::*;

macro_rules! walk_list {
    ($visitor: expr, $method: ident, $list: expr) => {
        for elem in $list {
            $visitor.$method(elem)
        }
    };
    ($visitor: expr, $method: ident, $list: expr, $($extra_args: expr),*) => {
        for elem in $list {
            $visitor.$method(elem, $($extra_args,)*)
        }
    }
}

pub trait Visitor<'ast>: Sized {
    fn visit_ident(&mut self, _ident: &'ast Identifier) {}

    fn visit_path(&mut self, path: &'ast Path, id: AstId) {
        walk_path(self, path, id)
    }

    fn visit_program(&mut self, program: &'ast Program) {
        walk_program(self, program)
    }

    fn visit_item(&mut self, item: &'ast Item) {
        walk_item(self, item)
    }

    fn visit_mod(&mut self, module: &'ast Mod) {
        walk_mod(self, module)
    }

    fn visit_struct(&mut self, struc: &'ast Struct) {
        walk_struct(self, struc)
    }

    fn visit_struct_field(&mut self, field: &'ast StructField) {
        walk_struct_field(self, field)
    }

    fn visit_class(&mut self, class: &'ast Class) {
        walk_class(self, class)
    }

    fn visit_branch(&mut self, branch: &'ast Branch) {
        walk_branch(self, branch)
    }

    fn visit_branch_variant(&mut self, variant: &'ast BranchVariant) {
        walk_branch_variant(self, variant)
    }

    fn visit_type(&mut self, ty: &'ast Type) {
        walk_type(self, ty)
    }

    fn visit_type_ref(&mut self, ty: &'ast TypeRef) {
        walk_type_ref(self, ty)
    }

    fn visit_block(&mut self, block: &'ast Block) {
        walk_block(self, block)
    }

    fn visit_expr(&mut self, expr: &'ast Expr) {
        walk_expr(self, expr)
    }

    fn visit_stmt(&mut self, stmt: &'ast Stmt) {
        walk_stmt(self, stmt)
    }

    fn visit_fn_param(&mut self, param: &'ast FnParam) {
        walk_fn_param(self, param)
    }
}

pub fn walk_path<'ast, T: Visitor<'ast>>(visitor: &mut T, Path { items }: &'ast Path, _id: AstId) {
    walk_list!(visitor, visit_ident, items)
}

pub fn walk_program<'ast, T: Visitor<'ast>>(
    visitor: &mut T,
    Program { items, id: _ }: &'ast Program,
) {
    walk_list!(visitor, visit_item, items)
}

pub fn walk_item<'ast, T: Visitor<'ast>>(
    visitor: &mut T,
    Item {
        name,
        vis: _,
        kind,
        span: _,
        id: _,
    }: &'ast Item,
) {
    visitor.visit_ident(name);

    match kind {
        ItemKind::Mod(ref item) => visitor.visit_mod(item),
        ItemKind::Struct(ref item) => visitor.visit_struct(item),
        ItemKind::Class(ref item) => visitor.visit_class(item),
        ItemKind::Branch(ref item) => visitor.visit_branch(item),
        ItemKind::Fn(FnSig { params, ret }, block) => {
            walk_list!(visitor, visit_fn_param, params);

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

pub fn walk_mod<'ast, T: Visitor<'ast>>(visitor: &mut T, Mod { items, inline: _ }: &'ast Mod) {
    walk_list!(visitor, visit_item, items)
}

pub fn walk_struct<'ast, T: Visitor<'ast>>(visitor: &mut T, Struct { members }: &'ast Struct) {
    walk_list!(visitor, visit_struct_field, members)
}

pub fn walk_struct_field<'ast, T: Visitor<'ast>>(
    visitor: &mut T,
    StructField { name, ty, vis: _ }: &'ast StructField,
) {
    visitor.visit_ident(name);
    visitor.visit_type(ty);
}

pub fn walk_class<'ast, T: Visitor<'ast>>(
    visitor: &mut T,
    Class {
        builtin: _,
        bounds,
        members,
    }: &'ast Class,
) {
    if let ClassBounds::Ty(ty) = bounds {
        visitor.visit_type(ty);
    }

    walk_list!(visitor, visit_struct_field, members)
}

pub fn walk_branch<'ast, T: Visitor<'ast>>(
    visitor: &mut T,
    Branch { bounds, variants }: &'ast Branch,
) {
    if let ClassBounds::Ty(ty) = bounds {
        visitor.visit_type(ty);
    }

    walk_list!(visitor, visit_branch_variant, variants)
}

pub fn walk_branch_variant<'ast, T: Visitor<'ast>>(
    visitor: &mut T,
    BranchVariant {
        span: _,
        name,
        members,
        id: _,
    }: &'ast BranchVariant,
) {
    visitor.visit_ident(name);
    walk_list!(visitor, visit_struct_field, members)
}

pub fn walk_type<'ast, T: Visitor<'ast>>(visitor: &mut T, Type { span: _, kind, id }: &'ast Type) {
    match &kind {
        TypeKind::Tuple(items) => walk_list!(visitor, visit_type, items),
        TypeKind::And(items) => walk_list!(visitor, visit_type_ref, items),
        TypeKind::Named(name) => visitor.visit_path(name, *id),

        TypeKind::Never => {}
        TypeKind::Unit => {}
        TypeKind::Infer => {}
        TypeKind::Err => {}
    }
}

pub fn walk_type_ref<'ast, T: Visitor<'ast>>(visitor: &mut T, TypeRef { name, id }: &'ast TypeRef) {
    visitor.visit_path(name, *id);
}

pub fn walk_block<'ast, T: Visitor<'ast>>(
    visitor: &mut T,
    Block {
        span: _,
        stmts,
        id: _,
    }: &'ast Block,
) {
    walk_list!(visitor, visit_stmt, stmts);
}

pub fn walk_expr<'ast, T: Visitor<'ast>>(visitor: &mut T, Expr { span: _, kind, id }: &'ast Expr) {
    match &kind {
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
            walk_list!(visitor, visit_expr, args)
        }
        ExprKind::MethodCall() => {}
        ExprKind::FieldAccess(expr, qualifier, field) => {
            visitor.visit_expr(expr);
            if let Some(Disambiguator { span: _, name, id }) =
                qualifier.as_ref().map(|b| b.as_ref())
            {
                visitor.visit_path(name, *id);
            }
            visitor.visit_ident(field);
        }
        ExprKind::Tuple(items) => walk_list!(visitor, visit_expr, items),
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

pub fn walk_stmt<'ast, T: Visitor<'ast>>(
    visitor: &mut T,
    Stmt {
        span: _,
        kind,
        id: _,
    }: &'ast Stmt,
) {
    match &kind {
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

pub fn walk_fn_param<'ast, T: Visitor<'ast>>(visitor: &mut T, FnParam { name, ty }: &'ast FnParam) {
    visitor.visit_ident(name);
    visitor.visit_type(ty);
}
