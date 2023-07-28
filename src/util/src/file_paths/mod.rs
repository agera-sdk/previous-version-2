/*!
Work with generic file paths.

# Absolute Paths

This module only considers an _absolute path_ to be a path
that starts with a path separator.

This module does not handle Windows operating system absolute paths.
In the Windows operating system, absolute paths may either start with a drive letter followed by
a colon or an UNC path prefix (`\\`). This module by default does not
handle Windows absolute paths. If you need this, use the [`rialight_util::file_paths::os_based`]
module instead, which has functions that accept an additional parameter whose type is an enumeration of
a special operating system to handle determined at runtime.

This allows this module to be used for purposes other than
working with files in an operating system.

# Example

```
use rialight_util::file_paths;

assert_eq!("a", file_paths::resolve("a/b", ".."));
assert_eq!("a", file_paths::resolve_one("a/b/.."));
assert_eq!("a/b/c/d/e", file_paths::resolve_n(["a/b", "c/d", "e/f", ".."]));
assert_eq!("../../c/d", file_paths::relative("/a/b", "/c/d"));
```
*/

use super::{reg_exp, reg_exp::*};

static PATH_SEPARATOR: StaticRegExp = static_reg_exp!(r"[/\\]");
static STARTS_WITH_PATH_SEPARATOR: StaticRegExp = static_reg_exp!(r"^[/\\]");

pub mod os_based;

/**
Finds the relative path from `from_path` and `to_path`.

# Behavior:

- If the paths refer to the same path, this function returns
  an empty string.
- The function ensures that both paths are absolute and resolves
  any `..` and `.` portions inside.

# Exceptions

Panics if given paths are not absolute, that is, if they do not start
with a path separator.

# Example

```
use rialight_util::file_paths;
assert_eq!("", file_paths::relative("/a/b", "/a/b"));
assert_eq!("c", file_paths::relative("/a/b", "/a/b/c"));
assert_eq!("../../c/d", file_paths::relative("/a/b", "/c/d"));
assert_eq!("../c", file_paths::relative("/a/b", "/a/c"));
```
*/
pub fn relative(from_path: &str, to_path: &str) -> String {
    if ![from_path.to_owned(), to_path.to_owned()].iter().all(|path| STARTS_WITH_PATH_SEPARATOR.is_match(path)) {
        panic!("file_paths::relative() requires absolute paths as arguments");
    }

    let mut r = Vec::<String>::new();

    let mut from_parts: Vec<String> = PATH_SEPARATOR.split(resolve_one(from_path).as_ref()).map(|s| s.to_owned()).collect();
    let mut to_parts: Vec<String> = PATH_SEPARATOR.split(resolve_one(to_path).as_ref()).map(|s| s.to_owned()).collect();

    let mut common_indices = Vec::<usize>::new();

    for i in 0..usize::max(from_parts.len(), to_parts.len()) {
        if i >= from_parts.len() || i >= to_parts.len() {
            break;
        }
        if from_parts[i] == to_parts[i] {
            common_indices.push(i);
        }
    }
    for i in common_indices.iter().rev() {
        let j = common_indices[*i];
        from_parts.remove(j);
        to_parts.remove(j);
    }
    for _i in 0..from_parts.len() {
        r.push("..".to_owned());
    }
    for s in to_parts {
        r.push(s.clone());
    }

    let r = r.join("/");
    let r = r.trim();
    if r.is_empty() { "".to_owned() } else { r.to_owned() }
}

/// Resolves multiple paths.
///
/// Behavior:
/// - If no paths are provided, this method returns an empty string.
/// - Eliminates the portions `..` and `.`.
/// - If a path starts with a path separator, any subsequent paths are resolved relative to that path.
/// - All path separators that are backslashes (`\`) are replaced by forward ones (`/`).
/// - If any path starts with a path separator, this function returns an absolute path.
/// - Any empty portion and trailing path separators, such as in `a/b/` and `a//b` are eliminated.
/// ```
/// use rialight_util::file_paths;
/// assert_eq!("", file_paths::resolve_n([]));
/// assert_eq!("a", file_paths::resolve_n(["a/b/.."]));
/// assert_eq!("a", file_paths::resolve_n(["a/b", ".."]));
/// assert_eq!("/bar", file_paths::resolve_n(["/foo", "/bar"]));
/// ```
pub fn resolve_n<'a, T: IntoIterator<Item = &'a str>>(paths: T) -> String {
    let paths = paths.into_iter().collect::<Vec<&'a str>>();
    if paths.len() == 0 {
        return "".to_owned();
    }
    if paths.len() == 1 {
        return resolve_one(paths[0].as_ref());
    }
    let initial_path = resolve(paths[0].as_ref(), paths[1].as_ref());
    paths[2..].iter().fold(initial_path, |a, b| resolve(a.as_ref(), b.as_ref()))
}

/// Resolves `path2` relative to `path1`.
///
/// Behavior:
/// - Eliminates the portions `..` and `.`.
/// - If `path2` starts with a path separator, this function returns a resolution of solely `path2`.
/// - All path separators that are backslashes (`\`) are replaced by forward ones (`/`).
/// - If any path starts with a path separator, this function returns an absolute path.
/// - Any empty portion and trailing path separators, such as in `a/b/` and `a//b` are eliminated.
/// ```
/// use rialight_util::file_paths;
/// assert_eq!("/a/b", file_paths::resolve("/c", "/a/b"));
/// assert_eq!("a/b", file_paths::resolve_one("a/b/"));
/// assert_eq!("a/b", file_paths::resolve_one("a//b"));
/// ```
pub fn resolve(path1: &str, path2: &str) -> String {
    if STARTS_WITH_PATH_SEPARATOR.is_match(path2) {
        return resolve_one(path2);
    }
    let starts_with_slash = STARTS_WITH_PATH_SEPARATOR.is_match(path1);
    let mut r: String;
    let path1_resolved = resolve_one_without_starting_sep(path1);
    if path2.is_empty() {
        r = path1_resolved;
    }
    else {
        let paths_combination = path1_resolved + "/" + path2;
        r = resolve_one_without_starting_sep(paths_combination.as_ref());
    }
    if starts_with_slash {
        r = "/".to_owned() + &r;
    }
    r
}

/// Resolves a single path.
///
/// Behavior:
/// - Eliminates the portions `..` and `.`.
/// - All path separators that are backslashes (`\`) are replaced by forward ones (`/`).
/// - If the path starts with a path separator, an absolute path is returned.
/// - Any empty portion and trailing path separators, such as in `a/b/` and `a//b` are eliminated.
/// ```
/// use rialight_util::file_paths;
/// assert_eq!("a/b", file_paths::resolve_one("a/b/"));
/// assert_eq!("a/b", file_paths::resolve_one("a//b"));
/// ```
pub fn resolve_one(path: &str) -> String {
    let starts_with_slash = STARTS_WITH_PATH_SEPARATOR.is_match(path);
    let r = resolve_one_without_starting_sep(path);
    if starts_with_slash { "/".to_owned() + &r } else { r }
}

fn resolve_one_without_starting_sep(path: &str) -> String {
    let mut r = Vec::<String>::new();
    for p in PATH_SEPARATOR.split(path) {
        if p == "." {
            continue;
        } else if p == ".." {
            if !r.is_empty() {
                r.remove(r.len() - 1);
            }
        } else if !p.is_empty() {
            r.push(p.to_owned());
        }
    }
    r.join("/")
}

/// Changes the extension of a path and returns a new string.
/// This method adds any lacking dot (`.`) prefix automatically to the
/// `extension` argument.
///
/// This method allows multiple dots per extension. If that is not
/// desired, use [`change_last_extension`].
///
/// # Example
/// 
/// ```
/// use rialight_util::file_paths;
/// assert_eq!("a.y", file_paths::change_extension("a.x", ".y"));
/// assert_eq!("a.z", file_paths::change_extension("a.x.y", ".z"));
/// assert_eq!("a.z.w", file_paths::change_extension("a.x.y", ".z.w"));
/// ```
///
pub fn change_extension(path: &str, extension: &str) -> String {
    let extension = (if extension.starts_with(".") { "" } else { "." }).to_owned() + extension;
    if reg_exp_find!(r"(\.[^\.]+)+$", path).is_none() {
        return path.to_owned() + &extension;
    }
    reg_exp_replace!(r"(\.[^\.]+)+$", path, |_, _| &extension).into_owned()
}

/// Changes only the last extension of a path and returns a new string.
/// This method adds any lacking dot (`.`) prefix automatically to the
/// `extension` argument.
///
/// # Exceptions
///
/// Panics if the extension contains more than one dot.
///
pub fn change_last_extension(path: &str, extension: &str) -> String {
    let extension = (if extension.starts_with(".") { "" } else { "." }).to_owned() + extension;
    if extension[1..].find('.').is_some() {
        panic!("The argument to file_paths::change_last_extension() must only contain one extension; got {}", extension);
    }
    if reg_exp_find!(r"(\..+)$", path).is_none() {
        return path.to_owned() + &extension;
    }
    reg_exp_replace!(r"(\..+)$", path, |_, _| &extension).into_owned()
}

/// Adds prefix dot to extension if missing.
fn extension_arg(extension: &str) -> String {
    (if extension.starts_with(".") { "" } else { "." }).to_owned() + extension
}

/// Checks if a file path has a specific extension.
/// This method adds any lacking dot (`.`) prefix automatically to the
/// `extension` argument.
pub fn has_extension(path: &str, extension: &str) -> bool {
    let extension = (if extension.starts_with(".") { "" } else { "." }).to_owned() + extension;
    path.ends_with(&extension_arg(&extension))
}

/// Checks if a file path has any of multiple specific extensions.
/// This method adds any lacking dot (`.`) prefix automatically to each
/// extension argument.
pub fn has_extensions<'a, T: IntoIterator<Item = &'a str>>(path: &str, extensions: T) -> bool {
    extensions.into_iter().any(|ext| has_extension(path, ext))
}

/// Returns the base name of a file path.
///
/// # Example
/// 
/// ```
/// use rialight_util::file_paths;
/// assert_eq!("qux.html", file_paths::base_name("foo/qux.html"));
/// ```
pub fn base_name(path: &str) -> String {
    resolve_one(path).split('/').last().map_or("", |s| s).to_owned()
}

/// Returns the base name of a file path, removing any of the specified extensions.
/// This method adds any lacking dot (`.`) prefix automatically to each
/// extension argument.
///
/// # Example
/// 
/// ```
/// use rialight_util::file_paths;
/// assert_eq!("qux", file_paths::base_name_without_ext("foo/qux.html", [".html"]));
/// ```
pub fn base_name_without_ext<'a, T>(path: &str, extensions: T) -> String
    where T: IntoIterator<Item = &'a str>
{
    let extensions = extensions.into_iter().map(|s| extension_arg(s)).collect::<Vec<String>>();
    resolve_one(path).split('/').last().map_or("".to_owned(), |base| {
        reg_exp_replace!(r"(\.[^\.]+)+$", base, |_, prev_ext: &str| {
            (if extensions.iter().any(|ext| ext == &prev_ext) { "" } else { prev_ext }).to_owned()
        }).into_owned()
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!("a", resolve_n(["a/b/.."]));
        assert_eq!("a", resolve_n(["a", "b", ".."]));
        assert_eq!("/a/b", resolve("/c", "/a/b"));
        assert_eq!("a", resolve("a/b", ".."));
        assert_eq!("a/b", resolve_one("a/b/"));
        assert_eq!("a/b", resolve_one("a//b"));
        assert_eq!("", relative("/a/b", "/a/b"));
        assert_eq!("c", relative("/a/b", "/a/b/c"));
        assert_eq!("../../c/d", relative("/a/b", "/c/d"));
        assert_eq!("../c", relative("/a/b", "/a/c"));
        assert!(has_extensions("a.x", [".x", ".y"]));
        assert_eq!("a.y", change_extension("a.x", ".y"));
        assert_eq!("a.z", change_extension("a.x.y", ".z"));
        assert_eq!("a.z.w", change_extension("a.x.y", ".z.w"));

        assert_eq!("qux.html", base_name("foo/qux.html"));
        assert_eq!("qux", base_name_without_ext("foo/qux.html", [".html"]));
    }
}