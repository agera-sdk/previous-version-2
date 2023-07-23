//! Module for formatting strings. It allows replacing strings
//! in the form `"$x $y $z"`; more information is documented at the `apply()` function

use std::any::Any;
use std::collections::HashMap;
use lazy_regex::{regex, regex_replace_all};

/// Maps parameters to their arguments for formatting strings.
pub type Map = HashMap<String, Box<dyn Any>>;

/// Creates a `template_string::Map` object from a list of key-value pairs.
///
/// ## Example
///
/// ```
/// use rialight::util::template_string;
/// let map = template_string::map!{
///     "a" => "foo",
///     "b" => "bar",
/// };
/// assert_eq!(*map[&"a".to_owned()].downcast_ref::<&'static str>().unwrap(), "foo");
/// assert_eq!(*map[&"b".to_owned()].downcast_ref::<&'static str>().unwrap(), "bar");
/// assert!(map.get(&"c".to_owned()).is_none());
/// ```
pub macro map {
    ($($key:expr => $value:expr,)+) => {
        {
            #[allow(unused_mut)]
            let mut r_map = ::std::collections::HashMap::<String, Box<dyn ::std::any::Any>>::new();
            $(
                let _ = r_map.insert($key.to_string(), Box::new($value));
            )*
            r_map
        }
    },
    ($($key:expr => $value:expr),*) => {
        {
            #[allow(unused_mut)]
            let mut r_map = ::std::collections::HashMap::<String, Box<dyn ::std::any::Any>>::new();
            $(
                let _ = r_map.insert($key.to_string(), Box::new($value));
            )*
            r_map
        }
    }
}

/// Applies arguments to a paramaterized string. The function supports an optional
/// processor function to manipulate given arguments.
///
/// If no processor function is specified, the arguments are forced to be either
/// of the type `String` or `&'static str`.
///
/// If a parameter is missing from the arguments map, it is replaced by `"None"`.
///
/// # Syntax
/// The following sequences are affected by the function:
/// - `$parametername` (cannot contain hyphens and underscores)
/// - `$<parameter-name>` (can contain hyphens and underscores)
/// - `$$` (translates to a single dollar sign)
/// # Example
/// ```
/// // forces arguments to either `String` or `&'static str`.
/// let applied = template_string::apply("$<foo-qux>", &template_string::map! { "foo-qux" => "Fq" }, None);
/// assert_eq!(applied, "Fq".to_owned());
///
/// // uses a processor.
/// let applied = template_string::apply("$<foo-qux>", &template_string::map! { "foo-qux" => "Fq" }, Some(|v| (*v.downcast_ref::<&'static str>().unwrap()).to_owned()));
/// assert_eq!(applied, "Fq".to_owned());
///
/// // lacking parameter.
/// let applied = template_string::apply("$<foo-qux>", &template_string::map! {}, None);
/// assert_eq!(applied, "None".to_owned());
/// ```
pub fn apply(parameterized: impl AsRef<str>, arguments: &Map, process: Option<fn(arg: &Box<dyn Any>) -> String>) -> String {
    let process: fn(arg: &Box<dyn Any>) -> String = match process {
        Some(f) => f,
        None => |v| {
            if let Some(r) = v.downcast_ref::<String>() {
                return r.clone();
            }
            (*v.downcast_ref::<&'static str>().unwrap_or(&"None")).to_owned()
        },
    };
    regex!(r"\$(\$|[A-Za-z0-9]+|<[A-Za-z0-9\-_]+>)").replace_all(parameterized.as_ref(), |s: &regex::Captures<'_>| {
        let s = s.get(0).unwrap().as_str();
        if s == "$$" {
            "$".to_owned()
        } else {
            let k = regex_replace_all!(r"[$<>]", &s.to_string(), |_| "").into_owned().to_owned();
            let v = arguments.get(&k);
            if let Some(v) = v { process(v) } else { "None".to_owned() }
        }
    }).as_ref().to_string()
}