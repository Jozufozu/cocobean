#[derive(Debug, Copy, Clone)]
pub struct AstId(u32);

impl AstId {
    pub const PROJECT: AstId = AstId(0);
    pub const DUMMY: AstId = AstId(u32::MAX);
}
