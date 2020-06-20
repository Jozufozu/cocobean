
#[derive(Debug, Copy, Clone)]
pub struct Span {
    pub l: usize,
    pub r: usize,
}

#[derive(Debug, Clone)]
pub struct Spanned<T> {
    pub span: Span,
    pub val: T,
}

#[derive(Debug)]
pub enum ProgramPart {
    StructDefinition(StructDefinition)
}

#[derive(Debug)]
pub struct StructDefinition {
    pub name: Identifier,
    pub bounds: Option<Type>,
    pub members: Vec<MemberVariable>,
}

#[derive(Debug)]
pub struct MemberVariable {
    pub name: Identifier,
    pub ty: Type
}

pub type Identifier = Spanned<String>;

pub type Type = Spanned<TypeKind>;

#[derive(Debug)]
pub enum TypeKind {
    Int,
    String,
    Bool,
    Unit,
    Complex(ComplexType)
}

pub type ComplexType = Spanned<ComplexTypeKind>;

#[derive(Debug)]
pub enum ComplexTypeKind {
    Base(Identifier, ComplexReferent),
    Compound(Vec<ComplexType>),
    Not(Box<ComplexType>),
    Above(Box<ComplexType>),
}

#[derive(Debug, Clone)]
pub enum ComplexReferent {
    Infer,
    Entity,
    Struct,
    Trait
}

pub type Expr = Spanned<ExprKind>;

#[derive(Debug, Clone)]
pub enum ExprKind {
    Lit(LitKind),
    Variable(Identifier),
    BinOp(),
    UnOp(),
    Call(),
    MethodCall(),
    Tuple(Vec<Expr>),
    Loop(),
    If(),

    Err
}

pub type Stmt = Spanned<StmtKind>;

#[derive(Debug, Clone)]
pub enum StmtKind {
    Expr(Box<Expr>),
    Semi(Box<Expr>, Span)
}

#[derive(Debug, Clone)]
pub struct Block {
    span: Span,
    stmts: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub enum LitKind {
    Int(i32),
    Bool(bool),
    String(String),

    Err
}