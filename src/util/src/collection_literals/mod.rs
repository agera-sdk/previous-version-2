//! Provides literals for various collections.

/// Initialises any map type from a list of key-value pairs in curly brackets.
///
/// ## Example
///
/// ```
/// use rialight::util::collection_literals::map;
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
            #[allow(unused_mut)]
            let mut r_map = ::std::vec::Vec::new();
            $(
                let _ = r_map.push(($key, $value));
            )*
            ::std::iter::FromIterator::from_iter(r_map)
        }
    },
    ($($key:expr => $value:expr),*) => {
        {
            #[allow(unused_mut)]
            let mut r_map = ::std::vec::Vec::new();
            $(
                let _ = r_map.push(($key, $value));
            )*
            ::std::iter::FromIterator::from_iter(r_map)
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
            #[allow(unused_mut)]
            let mut r_map = ::std::collections::HashMap::new();
            $(
                let _ = r_map.insert($key, $value);
            )*
            r_map
        }
    },
    ($($key:expr => $value:expr),*) => {
        {
            #[allow(unused_mut)]
            let mut r_map = ::std::collections::HashMap::new();
            $(
                let _ = r_map.insert($key, $value);
            )*
            r_map
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
            #[allow(unused_mut)]
            let mut r_map = ::std::collections::BTreeMap::new();
            $(
                let _ = r_map.insert($key, $value);
            )*
            r_map
        }
    },
    ($($key:expr => $value:expr),*) => {
        {
            #[allow(unused_mut)]
            let mut r_map = ::std::collections::BTreeMap::new();
            $(
                let _ = r_map.insert($key, $value);
            )*
            r_map
        }
    }
}

/// Initialises any set type from a list of values in brackets.
///
/// ## Example
///
/// ```
/// use rialight::util::collection_literals::set;
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
            #[allow(unused_mut)]
            let mut r_set = ::std::vec::Vec::new();
            $(
                let _ = r_set.push($value);
            )*
            ::std::iter::FromIterator::from_iter(r_set)
        }
    ],
    ($($value:expr),*) => [
        {
            #[allow(unused_mut)]
            let mut r_set = ::std::vec::Vec::new();
            $(
                let _ = r_set.push($value);
            )*
            ::std::iter::FromIterator::from_iter(r_set)
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
            #[allow(unused_mut)]
            let mut r_set = ::std::collections::HashSet::new();
            $(
                let _ = r_set.insert($value);
            )*
            r_set
        }
    ],
    ($($value:expr),*) => [
        {
            #[allow(unused_mut)]
            let mut r_set = ::std::collections::HashSet::new();
            $(
                let _ = r_set.insert($value);
            )*
            r_set
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
            #[allow(unused_mut)]
            let mut r_set = ::std::collections::BTreeSet::new();
            $(
                let _ = r_set.insert($value);
            )*
            r_set
        }
    ],
    ($($value:expr),*) => [
        {
            #[allow(unused_mut)]
            let mut r_set = ::std::collections::BTreeSet::new();
            $(
                let _ = r_set.insert($value);
            )*
            r_set
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