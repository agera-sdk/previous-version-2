/*!
Utilities for strings.
 */

use super::{collections::*, reg_exp::*};

/// The `StringIncognitoFormat` trait allows formatting string parameters
/// of arbitrary name that is computed at runtime.
///
/// The implementation for `&str` accepts parameters in curly brackets form:
/// 
/// ```plain
/// {param_name}     # parameter to replace
/// {"unparsed"}     # uninterpreted sequence
/// ```
///
/// # Example
/// 
/// ```
/// use rialight::prelude::*;
/// let user_string = "some user string: {id}";
/// assert_eq!("x", user_string.incognito_format(map!{"id".into() => "x".into()}));
/// 
/// // if a string contains curly brackets, they must be escaped.
/// let escaped = r#"{"{"}"#;
/// ```
///
pub trait StringIncognitoFormat {
    fn incognito_format(&self, arguments: Map<String, String>) -> String;
}