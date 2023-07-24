//! Provides literals for certain `std` collections, including `HashMap`.

/// Creates a `HashMap` object from a list of key-value pairs in curly brackets.
///
/// ## Example
///
/// ```
/// use rialight::util::hashmap;
/// let map = hashmap!{
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
pub macro hashmap {
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
/// use rialight::util::btreemap;
/// let map = btreemap!{
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
pub macro btreemap {
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

/// Creates a `HashSet` object from a list of values in brackets.
///
/// ## Example
///
/// ```
/// use rialight::util::hashset;
/// assert!(hashset!["foo"].contains("foo"));
/// ```
///
/// ## Rest
/// 
/// Rest is not supported yet. If you need it, just use `FromIterator`.
///
pub macro hashset {
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
/// use rialight::util::btreeset;
/// assert!(btreeset!{"foo"}.contains("foo"));
/// ```
///
/// ## Rest
/// 
/// Rest is not supported yet. If you need it, just use `FromIterator`.
///
pub macro btreeset {
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
        hashmap, hashset, btreemap, btreeset,
    };

    #[test]
    fn map_literal() {
        let map = hashmap!{"a" => "foo", "b" => "bar"};
        assert_eq!(map["a"], "foo");
        assert_eq!(map["b"], "bar");
        let map = btreemap!{"a" => "foo", "b" => "bar"};
        assert_eq!(map["a"], "foo");
        assert_eq!(map["b"], "bar");
    }

    #[test]
    fn set_literal() {
        assert!(hashset!["foo"].contains("foo"));
        assert!(btreeset!["foo"].contains("foo"));
    }
}