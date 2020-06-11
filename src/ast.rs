
#[derive(Debug, Copy, Clone)]
pub struct Span {
    pub l: usize,
    pub r: usize,
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

#[derive(Debug)]
pub struct Identifier {
    pub span: Span,
    pub name: String
}

#[derive(Debug)]
pub struct Type {
    pub span: Span,
    pub kind: TypeKind,
}

#[derive(Debug)]
pub enum TypeKind {
    Int,
    String,
    Bool,
    Unit,
    Complex(ComplexType)
}

#[derive(Debug)]
pub struct ComplexType {
    pub span: Span,
    pub kind: ComplexTypeKind
}

#[derive(Debug)]
pub enum ComplexTypeKind {
    Base(Identifier, ComplexReferent),
    Compound(Vec<ComplexType>),
    Not(Box<ComplexType>),
    Above(Box<ComplexType>),
}

#[derive(Debug, Copy, Clone)]
pub enum ComplexReferent {
    Infer,
    Entity,
    Struct,
    Trait
}