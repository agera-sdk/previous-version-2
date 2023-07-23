use std::{sync::{Arc, Weak}};

/// Represents either a bitmap, shape, container, movie clip, user interface component or
/// three-dimensional object.
///
/// # User Interface Components
///
/// User interface components cannot be direct children of a three-dimensional
/// container. To insert user interface components in a three-dimensional container,
/// they need to be inserted into a two-dimensional container,
/// which can then be inserted into the target three-dimensional container.
///
pub struct DisplayObject {
}

enum DisplayObjectDepth1 {
}