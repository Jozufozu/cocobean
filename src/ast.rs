use std::fmt;
use std::fmt::{Display, Formatter};

use lasso::Spur;

use crate::span::*;

pub mod visit;

#[inline]
pub(crate) fn bx<T>(val: T) -> Box<T> {
    Box::new(val)
}

#[inline]
pub(crate) fn box_opt<T>(val: Option<T>) -> Option<Box<T>> {
    val.map(|v| Box::new(v))
}

pub type Identifier = Spanned<Spur>;

#[derive(Debug, Clone)]
pub struct Path {
    pub items: Vec<Identifier>,
}

#[derive(Debug, Copy, Clone)]
pub enum Visibility {
    Pub(Span),
    Private,
}

#[derive(Debug)]
pub struct Program {
    pub items: Vec<Item>,
}

#[derive(Debug)]
pub struct Item {
    pub name: Identifier,
    pub vis: Visibility,
    pub kind: ItemKind,
    pub span: Span,
}

#[derive(Debug)]
pub enum ItemKind {
    Mod(Mod),
    Struct(Struct),
    Class(Class),
    Branch(Branch),
    Fn(FnSig, Option<Block>),

    Err,
}

#[derive(Debug)]
pub struct Mod {
    pub items: Vec<Item>,
    pub inline: bool,
}

#[derive(Debug)]
pub struct Struct {
    pub members: Vec<StructField>,
}

#[derive(Debug)]
pub struct Class {
    pub builtin: Option<Span>,
    pub bounds: ClassBounds,
    pub members: Vec<StructField>,
}

#[derive(Debug)]
pub struct Branch {
    pub bounds: ClassBounds,
    pub variants: Vec<BranchVariant>,
}

#[derive(Debug)]
pub struct BranchVariant {
    pub span: Span,
    pub name: Identifier,
    pub members: Vec<StructField>,
}

#[derive(Debug)]
pub enum ClassBounds {
    Default,
    Ty(Type),
}

#[derive(Debug)]
pub struct StructField {
    pub name: Identifier,
    pub ty: Type,
    pub vis: Visibility,
    pub default: Option<Box<Expr>>,
}

#[derive(Debug)]
pub struct FnSig {
    pub params: Vec<FnParam>,
    pub ret: FnReturn,
}

#[derive(Debug)]
pub enum FnReturn {
    Default,
    Ty(Type),
}

#[derive(Debug)]
pub struct FnParam {
    pub name: Identifier,
    pub ty: Type,
}

pub type Type = Spanned<TypeKind>;

#[derive(Debug)]
pub enum TypeKind {
    Int,
    String,
    Bool,
    Unit,
    Tuple(Vec<Type>),
    And(Vec<Path>),
    Named(Path),

    Infer,

    Err,
}

pub type Block = Spanned<Vec<Stmt>>;

pub type Expr = Spanned<ExprKind>;

#[derive(Debug)]
pub enum ExprKind {
    Lit(Lit),
    Variable(Identifier),
    BinOp(BinOp, Box<Expr>, Box<Expr>),
    UnOp(UnOp, Box<Expr>),
    Assign(Span, Box<Expr>, Box<Expr>),
    AssignOp(BinOp, Box<Expr>, Box<Expr>),
    Is(IsOp, Box<Expr>, Box<Type>),
    Call(Path, Vec<Expr>),
    MethodCall(),
    FieldAccess(Box<Expr>, Option<Path>, Identifier),
    Tuple(Vec<Expr>),
    Block(Block),
    Loop(Block),
    While(Box<Expr>, Block),
    If(Box<Expr>, Block, Option<Box<Expr>>),
    Paren(Box<Expr>),

    Err,
}

pub type IsOp = Spanned<IsOpKind>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum IsOpKind {
    Is,
    NotIs,
}

pub type Stmt = Spanned<StmtKind>;

#[derive(Debug)]
pub enum StmtKind {
    Item(Box<Item>),
    Expr(Box<Expr>),
    Semi(Box<Expr>),
    Let(Identifier, Option<Box<Type>>, Box<Expr>),

    Ret(Option<Box<Expr>>),
    Break(Option<Box<Expr>>),
    Continue,

    Err,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Lit {
    Int(i32),
    Bool(bool),
    String(String),

    Err,
}

impl Display for Lit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Lit::Int(i) => write!(f, "{}", i),
            Lit::Bool(b) => write!(f, "{}", b),
            Lit::String(s) => write!(f, "\"{}\"", s),
            Lit::Err => write!(f, "`err`"),
        }
    }
}

pub type BinOp = Spanned<BinOpKind>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BinOpKind {
    /// The `+` operator (addition)
    Add,
    /// The `-` operator (subtraction)
    Sub,
    /// The `*` operator (multiplication)
    Mul,
    /// The `/` operator (division)
    Div,
    /// The `%` operator (modulus)
    Rem,
    /// The `&&` operator (logical and)
    And,
    /// The `||` operator (logical or)
    Or,
    /// The `==` operator (equality)
    Eq,
    /// The `<` operator (less than)
    Lt,
    /// The `<=` operator (less than or equal to)
    Le,
    /// The `!=` operator (not equal to)
    Ne,
    /// The `>=` operator (greater than or equal to)
    Ge,
    /// The `>` operator (greater than)
    Gt,
}

impl Display for BinOpKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            BinOpKind::Add => write!(f, "+"),
            BinOpKind::Sub => write!(f, "-"),
            BinOpKind::Mul => write!(f, "*"),
            BinOpKind::Div => write!(f, "/"),
            BinOpKind::Rem => write!(f, "%"),
            BinOpKind::And => write!(f, "&"),
            BinOpKind::Or => write!(f, "|"),
            BinOpKind::Eq => write!(f, "=="),
            BinOpKind::Lt => write!(f, "<"),
            BinOpKind::Le => write!(f, "<="),
            BinOpKind::Ne => write!(f, "!="),
            BinOpKind::Ge => write!(f, ">="),
            BinOpKind::Gt => write!(f, ">"),
        }
    }
}

pub type UnOp = Spanned<UnOpKind>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum UnOpKind {
    Neg,
    Not,
}

impl Display for UnOpKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            UnOpKind::Neg => write!(f, "-"),
            UnOpKind::Not => write!(f, "!"),
        }
    }
}
