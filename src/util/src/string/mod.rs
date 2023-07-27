/*!
Utilities for strings, including formatting incognitos.
 */

use super::{
    collections::*,
    collection_literals::map,
    reg_exp::*,
};

/// The `StringIncognitoFormat` trait allows formatting string parameters
/// of arbitrary name that is computed at runtime.
///
/// The implementation for `&str` accepts parameters in curly brackets form:
/// 
/// ```plain
/// {param_name}     # parameter to replace
/// {"escaped"}      # escaped sequence
/// ```
///
/// Description of each syntax:
///
/// - Whitespace is allowed around the parameter name or escaped form, such as
/// `{ "foo" }` versus `{"foo"}`.
/// - `{param_name}` expands to either an argument given in the map (whose key string is `param_name`) or
/// the string `None` if not present. The parameter name may contain any of the following characters:
/// ``plain
/// A-Z a-z 0-9 . - _ $
/// ````
/// - `{"escaped"}` expands to the string `escaped`. It is often
/// used for escaping the curly brackets.
///
/// # Example
/// 
/// ```
/// use rialight::prelude::*;
/// let user_string = "some user string: {id}";
/// assert_eq!("some user string: x", user_string.incognito_format(map!{"id".into() => "x".into()}));
/// 
/// // if a string contains curly brackets, they must be escaped.
/// let escaped = r#"{"{"}"#;
/// ```
///
pub trait StringIncognitoFormat {
    fn incognito_format(&self, arguments: Map<String, String>) -> String;
}

impl StringIncognitoFormat for &str {
    fn incognito_format(&self, arguments: Map<String, String>) -> String {
        reg_exp_replace_all!(
            r#"(?x)
            \{\s*(
                ([a-zA-Z_0-9\-\.\$]+)   | # parameter
                ("([^\u{22}])*")          # escaped
            )\s*\}
            "#,
            self.to_owned().as_ref(),
            |_, s: &str, _, _, _| {
                if s.starts_with('"') {
                    return s[1..s.len() - 1].to_owned().clone();
                }
                arguments.get(s).map_or("None".to_owned(), |v| v.clone())
            }
        ).into_owned()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn incognito() {
        let user_string = "some user string: {id}";
        assert_eq!("some user string: x", user_string.incognito_format(map!{"id".into() => "x".into()}));
        let user_string = r#"some user string: {"id"}"#;
        assert_eq!("some user string: id", user_string.incognito_format(map!{"id".into() => "x".into()}));
        let user_string = "some user string: {id}";
        assert_eq!("some user string: None", user_string.incognito_format(map!{}));
    }
}