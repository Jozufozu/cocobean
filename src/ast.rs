use std::fmt::{Display, Formatter};
use std::fmt;
use string_interner::{Sym, Symbol};

#[inline]
pub(crate) fn bx<T>(val: T) -> Box<T> {
    Box::new(val)
}

#[inline]
pub(crate) fn box_opt<T>(val: Option<T>) -> Option<Box<T>> {
    val.map(|v| Box::new(v))
}

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

impl<T> Spanned<T> {
    #[inline]
    pub fn new(l: usize, r: usize, val: T) -> Self {
        Spanned{ span: Span{ l, r }, val }
    }
}

impl<T: Display> Display for Spanned<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.val.fmt(f)
    }
}

pub type Identifier = Spanned<Sym>;

#[derive(Debug)]
pub struct Path {
    pub items: Vec<Identifier>,
}

#[derive(Debug)]
pub enum ProgramPart {
    Module(Identifier, Vec<ProgramPart>),
    StructDefinition(StructDefinition),

    Err,
}

#[derive(Debug)]
pub struct StructDefinition {
    pub name: Identifier,
    pub builtin: bool,
    pub bounds: Option<Type>,
    pub members: Vec<MemberVariable>,
}

#[derive(Debug)]
pub struct MemberVariable {
    pub name: Identifier,
    pub ty: Type
}

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
    Base(Path, ComplexReferent),
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
    BinOp(BinOp, Box<Expr>, Box<Expr>),
    UnOp(UnOp, Box<Expr>),
    Assign(Span, Box<Expr>, Box<Expr>),
    AssignOp(BinOp, Box<Expr>, Box<Expr>),
    Call(),
    MethodCall(),
    Tuple(Vec<Expr>),
    Loop(),
    If(),

    Err
}

impl Display for ExprKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ExprKind::Lit(l) => write!(f, "{}", l),
            ExprKind::Variable(var) => write!(f, "ID[{}]", var.val.to_usize()),
            ExprKind::BinOp(op, lhs, rhs) => write!(f, "({} {} {})", lhs, op, rhs),
            ExprKind::UnOp(op, arg) => write!(f, "({}{})", op, arg),
            ExprKind::Assign(_, lhs, rhs) => write!(f, "({} = {})", lhs, rhs),
            ExprKind::AssignOp(op, lhs, rhs) => write!(f, "({} {}= {})", lhs, op, rhs),
            ExprKind::Tuple(v) => {
                f.write_str("(")?;
                let mut iter = v.iter();
                write!(f, "{}", iter.next().unwrap())?;
                for x in iter {
                    write!(f, ", {}", x)?;
                }
                f.write_str(")")
            },
            ExprKind::Err => f.write_str("ERR"),
            _ => f.write_str("uhhh"),
        }
    }
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

impl Display for LitKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            LitKind::Int(i) => write!(f, "{}", i),
            LitKind::Bool(b) => write!(f, "{}", b),
            LitKind::String(s) => write!(f, "\"{}\"", s),
            LitKind::Err => write!(f, "`err`"),
        }
    }
}

pub type BinOp = Spanned<BinOpKind>;

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
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