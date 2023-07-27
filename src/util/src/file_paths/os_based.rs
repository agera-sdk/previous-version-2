/*!
Work with file paths in a cross-platform way.

This a parametric alternative for the [`rialight_util::file_paths`] module.
Some of the methods in this module require an additional parameter
that is a variant of an enumeration of a special operating system that has a special
handling of each path.

The [`rialight_util::file_paths`] module, compared to this module,
works with generic file paths that use any path separator, but does not
consider the right prefix for Windows absolute paths.
*/

/// Indicates which kind of manipulation to perform in a path.
/// For example, it is given as the third for argument for `relative`.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum OsPathManipulation {
    /// Indicates that the path is manipulated in a generic way,
    /// that is, the same behavior from the [`rialight_util::file_paths`] module.
    Default,
    /// Indicates that the path is manipulated compatibly with the Windows operating system.
    Windows,
}