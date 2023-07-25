//! Working with JSON serialization.

use super::{Deserialize, Serialize};

pub use serde_json::{
    Error,
    Value
};
pub type Result<T> = std::result::Result<T, Error>;

/// Deserializes a JSON string into a value.
pub fn deserialize<'a, T>(string: &'a str) -> Result<T>
    where T: ?Sized + Deserialize<'a>
{
    serde_json::from_str(string)
}

/// Deserializes a JSON string into a value.
pub fn serialize<'a, T>(value: &T) -> Result<String>
    where T: ?Sized + Serialize
{
    serde_json::to_string(value)
}