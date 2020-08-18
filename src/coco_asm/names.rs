use std::hash::Hash;

use coco_helpers::id_map::Index;
use coco_helpers::resource_name::ResourceName;

use crate::selector::Selector;

pub trait NameResolver<K: Index, V: ?Sized> {
    fn resolve(&self, key: &K) -> Option<&V>;
}

pub trait NameInterner<K, V> {
    fn insert(&mut self, value: V) -> K;
}

macro_rules! identifiers {
    (
    $($other:ident),* ;
    $($name:ident : $id:ident),* ;
    $($sp_name:ident : $sp:ident -> $sp_map:ty),* ;
    ) => {
        $(id!{$other})*
        $(id!{$id})*
        $(id!{$sp})*

        #[derive(Debug)]
        pub struct Names {
            $(
            $name: ::indexmap::set::IndexSet<String>,
            )*
            $(
            $sp_name: ::indexmap::set::IndexSet<$sp_map>,
            )*
        }

        impl Default for Names {
            #[inline]
            fn default() -> Self {
                Names {
                    $(
                    $name: ::indexmap::set::IndexSet::<String>::new(),
                    )*
                    $(
                    $sp_name: ::indexmap::set::IndexSet::<$sp_map>::new(),
                    )*
                }
            }
        }

        impl Names {
            #[inline]
            pub fn new() -> Names {
                Names {
                    $(
                    $name: ::indexmap::set::IndexSet::<String>::new(),
                    )*
                    $(
                    $sp_name: ::indexmap::set::IndexSet::<$sp_map>::new(),
                    )*
                }
            }
        }

        $(
        impl NameResolver<$id, str> for Names {
            #[inline(always)]
            fn resolve(&self, key: &$id) -> Option<&str> {
                self.$name.get_index(key.to_usize()).map(|s| s.as_str())
            }
        }

        impl NameInterner<$id, String> for Names {
            #[inline(always)]
            fn insert(&mut self, value: String) -> $id {
                $id::from_usize(self.$name.insert_full(value).0)
            }
        }
        )*
        $(
        impl NameResolver<$sp, $sp_map> for Names {
            #[inline(always)]
            fn resolve(&self, key: &$sp) -> Option<&$sp_map> {
                self.$sp_name.get_index(key.to_usize())
            }
        }

        impl NameInterner<$sp, $sp_map> for Names {
            #[inline(always)]
            fn insert(&mut self, value: $sp_map) -> $sp {
                $sp::from_usize(self.$sp_name.insert_full(value).0)
            }
        }
        )*
    };
}

macro_rules! id {
    ($name:ident) => {
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
        pub struct $name(::std::num::NonZeroUsize);

        impl ::coco_helpers::id_map::Index for $name {
            fn to_usize(&self) -> usize {
                self.0.get() - 1
            }

            fn from_usize(u: usize) -> Self {
                $name(::std::num::NonZeroUsize::new(u + 1).unwrap())
            }
        }
    };
}

identifiers!(
    SelectorId,
    Register,
    FnId;
    scores: Score,
    teams: Team,
    names: Name,
    tags: Tag;
    storages: Storage -> ResourceName,
    blocks: BlockId -> ResourceName,
    dims: Dimension -> ResourceName;
);
