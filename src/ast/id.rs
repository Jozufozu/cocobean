#[derive(Debug, Copy, Clone)]
pub struct Id(u32);

impl Id {
    pub const PROJECT: Id = Id(0);
    pub const DUMMY: Id = Id(u32::MAX);
}
