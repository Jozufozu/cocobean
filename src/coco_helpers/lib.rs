use std::hash::{BuildHasher, Hash};

use coco_span::lasso::{Key, Rodeo, RodeoResolver, Spur, ThreadedRodeo};

pub mod id_map;
pub mod resource_name;

#[macro_export]
macro_rules! static_assert_size {
    ($ty:ty, $size:expr) => {
        const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
    };
}

pub trait InternResolver<K: Key = Spur> {
    fn resolve(&self, key: &K) -> &str;
}

impl<K: Key> InternResolver<K> for RodeoResolver<K> {
    #[inline]
    fn resolve(&self, key: &K) -> &str {
        self.resolve(key)
    }
}

impl<K, S> InternResolver<K> for ThreadedRodeo<K, S>
where
    K: Key + Hash,
    S: BuildHasher + Clone,
{
    #[inline]
    fn resolve(&self, key: &K) -> &str {
        self.resolve(key)
    }
}

impl<K, S> InternResolver<K> for Rodeo<K, S>
where
    K: Key + Hash,
    S: BuildHasher + Clone,
{
    #[inline]
    fn resolve(&self, key: &K) -> &str {
        self.resolve(key)
    }
}
