use fxhash::FxBuildHasher;
use lasso::{Key, ThreadedRodeo, Spur, Capacity};

pub type Interner = ThreadedRodeo<Spur, FxBuildHasher>;

lazy_static::lazy_static! {
    pub static ref MAIN: Spur = Spur::try_from_usize(0).unwrap();
    pub static ref PACK: Spur = Spur::try_from_usize(1).unwrap();
    pub static ref INT: Spur = Spur::try_from_usize(2).unwrap();
    pub static ref BOOL: Spur = Spur::try_from_usize(3).unwrap();
    pub static ref STRING: Spur = Spur::try_from_usize(4).unwrap();
    pub static ref POS: Spur = Spur::try_from_usize(5).unwrap();
}

pub fn create_interner() -> Interner {
    let rodeo = Interner::with_capacity_and_hasher(Capacity::for_strings(256), FxBuildHasher::default());
    for kw in ["main", "pack", "int", "bool", "string", "pos"].iter() {
        rodeo.get_or_intern_static(kw);
    }

    rodeo
}

#[cfg(test)]
mod tests {
    use crate::kw::{self, create_interner};

    #[test]
    fn statics_work() {
        let rodeo = create_interner();

        assert_eq!(rodeo.get("main"), Some(*kw::MAIN));
        assert_eq!(rodeo.get("pack"), Some(*kw::PACK));
        assert_eq!(rodeo.get("int"), Some(*kw::INT));
        assert_eq!(rodeo.get("bool"), Some(*kw::BOOL));
        assert_eq!(rodeo.get("string"), Some(*kw::STRING));
        assert_eq!(rodeo.get("pos"), Some(*kw::POS));
    }
}