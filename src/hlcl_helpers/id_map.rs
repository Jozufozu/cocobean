use std::num::NonZeroUsize;
use std::collections::HashMap;
use std::hash::{Hash, BuildHasher};
use std::collections::hash_map::RandomState;
use std::marker::PhantomData;

pub trait Index: Sized + Copy + Eq + Hash {
    fn to_usize(&self) -> usize;
    fn from_usize(u: usize) -> Self;
}

pub struct IdMap<K: Index, V, S = RandomState> {
    base: HashMap<K, V, S>,
    next: usize,
}

impl<K, V, S> IdMap<K, V, S> where K: Index, S: BuildHasher {
    #[inline]
    pub fn get(&self, key: &K) -> Option<&V> {
        self.base.get(key)
    }

    #[inline]
    pub fn insert(&mut self, value: V) -> K {
        let key = K::from_usize(self.next);
        self.base.insert(key, value);
        self.next += 1;
        key
    }
}

impl<K: Index, V> IdMap<K, V, RandomState> {
    #[inline]
    pub fn new() -> Self {
        Self {
            base: HashMap::new(),
            next: 1,
        }
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        IdMap {
            base: HashMap::with_capacity(capacity),
            next: 1,
        }
    }
}