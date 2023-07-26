//! Provides literals for various collections.
//!
//! # Example
//! 
//! ```
//! use rialight::prelude::*;
//! 
//! let m: Map<K, V> = map! { k => v };
//! let s: Set<T> = set! [v1, v2];
//! ```

/// Initialises any map type from a list of key-value pairs in curly brackets.
///
/// ## Example
///
/// ```
/// use rialight::prelude::*;
/// take_my_map(map!{
///     "a" => "foo",
///     "b" => "bar",
/// });
/// ```
///
/// ## Rest
/// 
/// Rest is not supported yet. If you need it, just use `FromIterator`.
///
pub macro map {
    ($($key:expr => $value:expr,)+) => {
        {
            ::std::iter::FromIterator::from_iter([$(($key, $value)),+])
        }
    },
    ($($key:expr => $value:expr),*) => {
        {
            ::std::iter::FromIterator::from_iter([$(($key, $value)),+])
        }
    }
}

/// Creates a `HashMap` object from a list of key-value pairs in curly brackets.
///
/// ## Example
///
/// ```
/// use rialight::util::collection_literals::hash_map;
/// let map = hash_map!{
///     "a" => "foo",
///     "b" => "bar",
/// };
/// assert_eq!(map["a"], "foo");
/// assert_eq!(map["b"], "bar");
/// ```
///
/// ## Rest
/// 
/// Rest is not supported yet. If you need it, just use `FromIterator`.
///
pub macro hash_map {
    ($($key:expr => $value:expr,)+) => {
        {
            ::std::collections::HashMap::<_, _>::from_iter([$(($key, $value)),+])
        }
    },
    ($($key:expr => $value:expr),*) => {
        {
            ::std::collections::HashMap::<_, _>::from_iter([$(($key, $value)),+])
        }
    }
}

/// Creates a `BTreeMap` object from a list of key-value pairs in curly brackets.
///
/// ## Example
///
/// ```
/// use rialight::util::collection_literals::btree_map;
/// let map = btree_map!{
///     "a" => "foo",
///     "b" => "bar",
/// };
/// assert_eq!(map["a"], "foo");
/// assert_eq!(map["b"], "bar");
/// ```
/// 
/// ## Rest
/// 
/// Rest is not supported yet. If you need it, just use `FromIterator`.
///
pub macro btree_map {
    ($($key:expr => $value:expr,)+) => {
        {
            ::std::collections::BTreeMap::<_, _>::from_iter([$(($key, $value)),+])
        }
    },
    ($($key:expr => $value:expr),*) => {
        {
            ::std::collections::BTreeMap::<_, _>::from_iter([$(($key, $value)),+])
        }
    }
}

/// Initialises any set type from a list of values in brackets.
///
/// ## Example
///
/// ```
/// use rialight::prelude::*;
/// take_my_set(set!["foo"]);
/// ```
///
/// ## Rest
///
/// Rest is not supported yet. If you need it, just use `FromIterator`.
///
pub macro set {
    ($($value:expr,)+) => [
        {
            ::std::iter::FromIterator::from_iter([$($value),+])
        }
    ],
    ($($value:expr),*) => [
        {
            ::std::iter::FromIterator::from_iter([$($value),+])
        }
    ]
}

/// Creates a `HashSet` object from a list of values in brackets.
///
/// ## Example
///
/// ```
/// use rialight::util::collection_literals::hash_set;
/// assert!(hash_set!["foo"].contains("foo"));
/// ```
///
/// ## Rest
/// 
/// Rest is not supported yet. If you need it, just use `FromIterator`.
///
pub macro hash_set {
    ($($value:expr,)+) => [
        {
            ::std::collections::HashSet::<_>::from_iter([$($value),+])
        }
    ],
    ($($value:expr),*) => [
        {
            ::std::collections::HashSet::<_>::from_iter([$($value),+])
        }
    ]
}

/// Creates a `BTreeSet` object from a list of values in brackets.
///
/// ## Example
///
/// ```
/// use rialight::util::collection_literals::btree_set;
/// assert!(btree_set!{"foo"}.contains("foo"));
/// ```
///
/// ## Rest
/// 
/// Rest is not supported yet. If you need it, just use `FromIterator`.
///
pub macro btree_set {
    ($($value:expr,)+) => [
        {
            ::std::collections::BTreeSet::<_>::from_iter([$($value),+])
        }
    ],
    ($($value:expr),*) => [
        {
            ::std::collections::BTreeSet::<_>::from_iter([$($value),+])
        }
    ]
}

#[cfg(test)]
mod test {
    use super::{
        map, set, hash_map, hash_set, btree_map, btree_set,
    };
    use std::collections::{HashMap, HashSet};

    #[test]
    fn map_literal() {
        let map: HashMap<&'static str, &'static str> = map!{
            "a" => "foo",
            "b" => "bar",
        };
        assert_eq!(map["a"], "foo");
        assert_eq!(map["b"], "bar");

        let map = hash_map!{"a" => "foo", "b" => "bar"};
        assert_eq!(map["a"], "foo");
        assert_eq!(map["b"], "bar");

        let map = btree_map!{"a" => "foo", "b" => "bar"};
        assert_eq!(map["a"], "foo");
        assert_eq!(map["b"], "bar");
    }

    #[test]
    fn set_literal() {
        let set: HashSet<&'static str> = set!["foo"];
        assert!(set.contains("foo"));

        assert!(hash_set!["foo"].contains("foo"));
        assert!(btree_set!["foo"].contains("foo"));
    }
}