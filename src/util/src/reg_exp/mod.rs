//! Work with regular expressions.
//!
//! Regular expressions are patterns used to match character
//! combinations in strings. The syntax is modeled after Perl.
//!
//! # Syntax
//! 
//! The syntax is modeled after Perl. [Consult here for more information on the syntax.][syntax]
//! 
//! # Creating a regular expression
//!
//! There are two ways of constructing a regular expression object:
//!
//! - Using a `reg_exp!` literal, which consists of a pattern and optional flags, as follows:
//! ```
//! use rialight::prelude::*;
//! let my_reg_exp = reg_exp!(r"pattern");
//! let my_reg_exp = reg_exp!(r"pattern");
//! ```
//!   `reg_exp!` literals compile the regular expression only once.
//! - Or calling the `RegExp::new` constructor:
//! ```
//! use rialight::prelude::*;
//! let my_reg_exp = RegExp::new(r"pattern").unwrap();
//! ```
//! 
//! Flags, such as `i`, can be passed as suffix when using the `reg_exp!` literal:
//! 
//! ```
//! let _ = reg_exp!(r"pattern"i);
//! ```
//! 
//! # Replacement
//!
//! Most commonly, macros such as `reg_exp_replace_all!` can be used to replace occurrences:
//! 
//! ```
//! let text = "Foo fuu";
//! let text = reg_exp_replace_all!(
//!     r#"\bf(?P<suffix>\w+)"#i,
//!     text,
//!     |_, suffix| format!("F<{}>", suffix),
//! );
//! assert_eq!(text, "F<oo> F<uu>");
//! ```

pub mod syntax;

pub use lazy_regex::{
    regex as reg_exp,
    regex::Replacer as RegExpReplacer,

    regex_captures as reg_exp_captures,
    regex_find as reg_exp_find,
    regex_is_match as reg_exp_is_match,
    regex_replace as reg_exp_replace,
    regex_replace_all as reg_exp_replace_all,

    Regex as RegExp,
    regex::Error as RegExpError,
    Captures as RegExpCaptures,
};