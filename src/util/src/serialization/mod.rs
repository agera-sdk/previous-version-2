//! Work with serialization.
//! 
//! # Typed serialization
//!
//! Here is a simple example serializing and deserializing a typed structure in JSON:
//!
//! ```rust
//! use rialight::prelude::*;
//! 
//! #[derive(Serialize, Deserialize, Debug)]
//! struct Point {
//!     x: i32,
//!     y: i32,
//! }
//! 
//! fn main() {
//!     let point = Point { x: 1, y: 2 };
//! 
//!     // Convert the Point to a JSON string.
//!     let serialized = json::serialize(&point).unwrap();
//! 
//!     // Prints serialized = {"x":1,"y":2}
//!     println!("serialized = {}", serialized);
//! 
//!     // Convert the JSON string back to a Point.
//!     let deserialized: Point = json::deserialize(&serialized).unwrap();
//! 
//!     // Prints deserialized = Point { x: 1, y: 2 }
//!     println!("deserialized = {:?}", deserialized);
//! }
//! ```
//! 
//! # Untyped JSON
//! 
//! The `json::deserialize` and `json::serialize` functions support the
//! [`rialight::util::serialization::json::Value`] structure, which represents
//! an untyped value.
//!
//! ```rust
//! enum Value {
//!     Null,
//!     Bool(bool),
//!     Number(Number),
//!     String(String),
//!     Array(Vec<Value>),
//!     Object(Map<String, Value>),
//! }
//! ```
//!
//! An untyped JSON value can be constructed with the `json!` macro.
//! 
//! ```
//! use rialight::prelude::*;
//! 
//! fn main() {
//!     let _ = json!({
//!         "x": "y"
//!     });
//! }
//! ```

pub use serde::{Deserialize, Serialize, Deserializer, Serializer};

pub mod json;

/// Work with generic deserialization.
pub mod generic_deserialization {
    pub use serde::de::*;
}

/// Work with generic serialization.
pub mod generic_serialization {
    pub use serde::ser::*;
}