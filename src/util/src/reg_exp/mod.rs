//! Work with regular expressions.
//!
//! Regular expressions are patterns used to match character
//! combinations in strings. It is highly similiar to
//! JavaScript regular expressions with additional features,
//! including comments and whitespace.
//!
//! # Creating a regular expression
//!
//! There are two ways of constructing a regular expression object:
//!
//! - Using a `reg_exp!` literal, which consists of a pattern and optional flags, as follows:
//! ```
//! use rialight::util::reg_exp::reg_exp;
//! let my_reg_exp = reg_exp!(r"pattern");
//! let my_reg_exp = reg_exp!(r"pattern", "flags");
//! ```
//!   `reg_exp!` literals compile the regular expression only once.
//! - Or calling the `RegExp::new` constructor:
//! ```
//! use rialight::util::reg_exp::RegExp;
//! let my_reg_exp = RegExp::new(r"pattern", "flags");
//! ```

use super::code_points::CodePointsReader;