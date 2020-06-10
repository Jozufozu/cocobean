
#[derive(Debug, Copy, Clone)]
pub struct Span {
    pub l: usize,
    pub r: usize,
}

#[derive(Debug)]
pub enum ProgramPart {
    TypeDefinition(TypeDefinition)
}

#[derive(Debug)]
pub struct TypeDefinition {
    pub name: Identifier,
    pub bounds: Type,
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
    ObjectType(ObjectType)
}

#[derive(Debug)]
pub struct ObjectType {
    pub span: Span,
    pub kind: ObjectTypeKind
}

#[derive(Debug)]
pub enum ObjectTypeKind {
    Not(Box<ObjectType>),
    Parents(Box<ObjectType>),
    Base(Identifier),
    Compound(Vec<ObjectType>)
}