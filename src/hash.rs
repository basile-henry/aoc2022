#[allow(dead_code)]
pub(crate) type DefaultHasherBuilder = core::hash::BuildHasherDefault<rustc_hash::FxHasher>;

#[allow(dead_code)]
pub(crate) type HashSet<K, A> = hashbrown::HashSet<K, DefaultHasherBuilder, A>;

#[allow(dead_code)]
pub(crate) type HashMap<K, V, A> = hashbrown::HashMap<K, V, DefaultHasherBuilder, A>;

#[macro_export]
macro_rules! hash_set {
    ($alloc:expr) => {{
        let s = $crate::hash::DefaultHasherBuilder::default();
        hashbrown::HashSet::with_hasher_in(s, $alloc)
    }};

    // Build a hash_map from an iterator
    ($alloc:expr, $iter:expr) => {{
        let mut set = hash_set!($alloc);
        $iter.collect_into(&mut set);
        set
    }};
}

#[macro_export]
macro_rules! hash_map {
    ($alloc:expr) => {{
        let s = $crate::hash::DefaultHasherBuilder::default();
        hashbrown::HashMap::with_hasher_in(s, $alloc)
    }};
}
