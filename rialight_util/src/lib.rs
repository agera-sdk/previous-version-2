#![feature(decl_macro)]

mod uri;
pub use uri::{encode_uri, decode_uri, encode_uri_component, decode_uri_component};

pub mod path;
pub mod template_string;

pub mod lazy_static;
pub mod flags;
pub mod regex;

/// Creates a `HashMap` object from a list of key-value pairs.
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

/// Creates a `BTreeMap` object from a list of key-value pairs.
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

/// Creates a `HashSet` object from a list of values.
///
/// ## Example
///
/// ```
/// use rialight::util::hashset;
/// assert!(hashset!{"foo"}.contains("foo"));
/// ```
pub macro hashset {
    ($($value:expr,)+) => {
        {
            #[allow(unused_mut)]
            let mut r_set = ::std::collections::HashSet::new();
            $(
                let _ = r_set.insert($value);
            )*
            r_set
        }
    },
    ($($value:expr),*) => {
        {
            #[allow(unused_mut)]
            let mut r_set = ::std::collections::HashSet::new();
            $(
                let _ = r_set.insert($value);
            )*
            r_set
        }
    }
}

/// Creates a `BTreeSet` object from a list of values.
///
/// ## Example
///
/// ```
/// use rialight::util::btreeset;
/// assert!(btreeset!{"foo"}.contains("foo"));
/// ```
pub macro btreeset {
    ($($value:expr,)+) => {
        {
            #[allow(unused_mut)]
            let mut r_set = ::std::collections::BTreeSet::new();
            $(
                let _ = r_set.insert($value);
            )*
            r_set
        }
    },
    ($($value:expr),*) => {
        {
            #[allow(unused_mut)]
            let mut r_set = ::std::collections::BTreeSet::new();
            $(
                let _ = r_set.insert($value);
            )*
            r_set
        }
    }
}

#[cfg(test)]
mod test {
    use super::{
        template_string,
        hashmap, hashset, btreemap, btreeset, lazy_static::lazy_static,
    };
    use std::collections::HashMap;

    #[test]
    fn lazy_static_utility() {
        lazy_static! {
            static ref HASHMAP: HashMap<i64, i64> = hashmap! {
                0 => 64,
                1 => 128,
            };
        }
        assert_eq!(HASHMAP[&0], 64);
    }

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
        assert!(hashset!{"foo"}.contains("foo"));
        assert!(btreeset!{"foo"}.contains("foo"));
    }

    #[test]
    fn template_string() {
        let map = template_string::map!{
            "a" => "foo",
            "b" => "bar",
        };
        assert_eq!(*map[&"a".to_owned()].downcast_ref::<&'static str>().unwrap(), "foo");
        assert_eq!(*map[&"b".to_owned()].downcast_ref::<&'static str>().unwrap(), "bar");
        assert!(map.get(&"c".to_owned()).is_none());

        let applied = template_string::apply("$<foo-qux>", &template_string::map! { "foo-qux" => "Fq" }, None);
        assert_eq!(applied, "Fq".to_owned());

        let applied = template_string::apply("$<foo-qux>", &template_string::map! { "foo-qux" => "Fq" }, Some(|v| (*v.downcast_ref::<&'static str>().unwrap()).to_owned()));
        assert_eq!(applied, "Fq".to_owned());

        let applied = template_string::apply("$<foo-qux>", &template_string::map! {}, None);
        assert_eq!(applied, "None".to_owned());
    }
}