#[macro_use]
extern crate derive_more;

use std::fmt;
use std::fmt::{Display, Formatter};
use std::iter::FromIterator;

use itertools::*;

use hlcl_helpers::InternResolver;
use hlcl_helpers::static_assert_size;
use hlcl_span::*;
use hlcl_span::lasso::Spur;

use crate::id::AstId;

pub mod id;
pub mod mut_visit;
pub mod visit;

macro_rules! new_spanned_id {
    ($struc:ident, $field:ident, $kind:path) => {
        impl $struc {
            #[inline]
            pub fn new(l: usize, r: usize, $field: $kind) -> Self {
                Self {
                    span: Span::new(l, r),
                    $field,
                    id: AstId::DUMMY,
                }
            }
        }
    };
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub span: Span,
    pub spur: Spur,
}

impl Identifier {
    pub fn new(l: usize, r: usize, spur: Spur) -> Self {
        Identifier {
            span: Span::new(l, r),
            spur,
        }
    }

    pub fn to_string<'r, R: InternResolver<Spur>>(&self, names: &'r R) -> &'r str {
        names.resolve(&self.spur)
    }
}

impl PartialEq for Identifier {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.spur == other.spur
    }

    #[inline]
    fn ne(&self, other: &Self) -> bool {
        self.spur != other.spur
    }
}

impl Eq for Identifier {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Path {
    pub items: Vec<Identifier>,
}

impl Path {
    pub fn to_string<R: InternResolver<Spur>>(&self, names: &R) -> String {
        self.items
            .iter()
            .map(|ident| ident.to_string(names))
            .join("::")
    }
}

impl FromIterator<Identifier> for Path {
    fn from_iter<T: IntoIterator<Item = Identifier>>(iter: T) -> Self {
        Path {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Visibility {
    Pub(Span),
    Private,
}

#[derive(Debug)]
pub struct Program {
    pub items: Vec<Item>,
    pub id: AstId,
}

#[derive(Debug)]
pub struct Item {
    pub name: Identifier,
    pub vis: Visibility,
    pub kind: ItemKind,
    pub span: Span,
    pub id: AstId,
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
    pub id: AstId,
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

#[derive(Debug)]
pub struct Type {
    pub span: Span,
    pub kind: TypeKind,
    pub id: AstId,
}

new_spanned_id!(Type, kind, TypeKind);

#[derive(Debug)]
pub struct TypeRef {
    pub name: Path,
    pub id: AstId,
}

#[derive(Debug)]
pub enum TypeKind {
    Unit,
    Never,
    Tuple(Vec<Type>),
    And(Vec<TypeRef>),
    Named(Path),

    Infer,

    Err,
}

#[derive(Debug)]
pub struct Block {
    pub span: Span,
    pub stmts: Vec<Stmt>,
    pub id: AstId,
}

new_spanned_id!(Block, stmts, Vec<Stmt>);

#[derive(Debug)]
pub struct Disambiguator {
    pub span: Span,
    pub name: Path,
    pub id: AstId,
}

new_spanned_id!(Disambiguator, name, Path);

#[derive(Debug)]
pub struct Expr {
    pub span: Span,
    pub kind: ExprKind,
    pub id: AstId,
}

static_assert_size!(Expr, 80);
new_spanned_id!(Expr, kind, ExprKind);

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
    FieldAccess(Box<Expr>, Option<Box<Disambiguator>>, Identifier),
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

#[derive(Debug)]
pub struct Stmt {
    pub span: Span,
    pub kind: StmtKind,
    pub id: AstId,
}

new_spanned_id!(Stmt, kind, StmtKind);

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
