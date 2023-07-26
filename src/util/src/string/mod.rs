/*!
Utilities for strings.
 */

use super::{collections::*, reg_exp::*};

/// The `StringApply` trait allows formatting string parameters
/// of arbitrary name that is computed dynamically.
///
/// The implementation for `&str` accepts parameters in curly brackets form:
/// 
/// ```plain
/// {param_name}     # parameter to replace
/// {"unparsed"}`    # uninterpreted sequence
/// ```
///
/// # Example
/// 
/// ```
/// use rialight::prelude::*;
/// let user_string = "some user string: {id}";
/// assert_eq!("x", user_string.apply(map!{"id".into() => "x".into()}));
/// ```
///
pub trait StringApply {
    fn apply(&self, arguments: Map<String, String>) -> String;
}