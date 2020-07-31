
use hlcl_helpers::resource_name::ResourceName;

pub trait NameResolver<K, V: ?Sized> {
    fn resolve(&self, key: &K) -> Option<&V>;
}

pub trait NameInterner<K, V> {
    fn insert(&mut self, value: V) -> K;
}

macro_rules! identifiers {
    (
    $($other:ident),* ;
    $($name:ident : $id:ident),* ;
    $($sp_name:ident : $sp:ident -> $sp_map:ident),* ;
    ) => {
        $(id!{$other})*
        $(id!{$id})*
        $(id!{$sp})*

        #[derive(Debug)]
        pub struct Names {
            $(
            pub $name: ::hlcl_helpers::id_map::IdMap<$id, String>,
            )*
            $(
            pub $sp_name: ::hlcl_helpers::id_map::IdMap<$sp, $sp_map>,
            )*
        }

        impl Default for Names {
            #[inline]
            fn default() -> Self {
                Names {
                    $(
                    $name: ::hlcl_helpers::id_map::IdMap::<$id, String>::new(),
                    )*
                    $(
                    $sp_name: ::hlcl_helpers::id_map::IdMap::<$sp, $sp_map>::new(),
                    )*
                }
            }
        }

        impl Names {
            #[inline]
            pub fn new() -> Names {
                Names {
                    $(
                    $name: ::hlcl_helpers::id_map::IdMap::<$id, String>::new(),
                    )*
                    $(
                    $sp_name: ::hlcl_helpers::id_map::IdMap::<$sp, $sp_map>::new(),
                    )*
                }
            }
        }

        $(
        impl NameResolver<$id, str> for Names {
            #[inline(always)]
            fn resolve(&self, key: &$id) -> Option<&str> {
                self.$name.get(key).map(|s| s.as_str())
            }
        }

        impl NameInterner<$id, String> for Names {
            #[inline(always)]
            fn insert(&mut self, value: String) -> $id {
                self.$name.insert(value)
            }
        }
        )*
        $(
        impl NameResolver<$sp, $sp_map> for Names {
            #[inline(always)]
            fn resolve(&self, key: &$sp) -> Option<&$sp_map> {
                self.$sp_name.get(key)
            }
        }

        impl NameInterner<$sp, $sp_map> for Names {
            #[inline(always)]
            fn insert(&mut self, value: $sp_map) -> $sp {
                self.$sp_name.insert(value)
            }
        }
        )*
    };
}

macro_rules! id {
    ($name:ident) => {
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
        pub struct $name(::std::num::NonZeroUsize);

        impl ::hlcl_helpers::id_map::Index for $name {
            fn to_usize(&self) -> usize {
                self.0.get()
            }

            fn from_usize(u: usize) -> Self {
                $name(::std::num::NonZeroUsize::new(u).unwrap())
            }
        }
    };
}

identifiers!(
    FnId,
    Register,
    BlockId;
    scores: Score,
    teams: Team,
    names: Name,
    tags: Tag;
    storages: Storage -> ResourceName,
    dims: Dimension -> ResourceName;
);