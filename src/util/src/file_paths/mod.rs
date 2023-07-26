/*!
Work with file paths in a cross-platform way.

# Example

```
use rialight::util::file_paths;

assert_eq!("a", file_paths::resolve("a/b", ".."));
assert_eq!("a", file_paths::resolve_one("a/b/.."));
assert_eq!("a", file_paths::resolve_n(["a/b", "c/d", "e/f", ".."]));
assert_eq!("../../c/d", file_paths::relative("/a/b", "/c/d"));
```
*/

use super::reg_exp::*;

static PATH_SEPARATOR: StaticRegExp = static_reg_exp!(r"[/\\]");
static STARTS_WITH_PATH_SEPARATOR: StaticRegExp = static_reg_exp!(r"^[/\\]");

/// Finds relative path between `from_path` and `to_path`.
/// 
/// Behavior:
///
/// - If the paths refer to the same path, this function returns
/// the string `.`.
/// 
/// # Exceptions
///
/// Panics if given paths are not absolute, that is, if they do not start
/// with a path separator.
///
pub fn relative(from_path: &str, to_path: &str) -> String {
    if ![from_path.to_owned(), to_path.to_owned()].iter().all(|path| STARTS_WITH_PATH_SEPARATOR.is_match(path)) {
        panic!("file_paths::relative() requires absolute paths as arguments");
    }

    let mut r = Vec::<String>::new();

    let mut from_parts: Vec<String> = PATH_SEPARATOR.split(resolve_one(from_path).as_ref()).map(|s| s.to_owned()).collect();
    let mut to_parts: Vec<String> = PATH_SEPARATOR.split(resolve_one(to_path).as_ref()).map(|s| s.to_owned()).collect();

    let mut common_indices = Vec::<usize>::new();

    for i in 0..from_parts.len() {
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
    if r.is_empty() { ".".to_owned() } else { r.to_owned() }
}

pub fn resolve_n<'a, T: IntoIterator<Item = &'a str>>(paths: T) -> String {
    let paths = paths.into_iter().collect::<Vec<&'a str>>();
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
/// - If any of the paths starts with a path separator, it is kept.
/// - Any empty portion and trailing path separators, such as in `a/b/` and `a//b` are eliminated.
/// ```
/// assert_eq!("/a/b", file_paths::resolve("/c", "/a/b"));
/// assert_eq!("a/b", file_paths::resolve_one("a/b/"));
/// assert_eq!("a/b", file_paths::resolve_one("a//b"));
/// ```
pub fn resolve(path1: &str, path2: &str) -> String {
    let starts_with_slash = STARTS_WITH_PATH_SEPARATOR.is_match(path1);
    let mut r: String;
    if STARTS_WITH_PATH_SEPARATOR.is_match(path2) {
        r = resolve_one_without_starting_sep(path2);
    } else {
        let path1_resolved = resolve_one_without_starting_sep(path1);
        if path2.is_empty() {
            r = path1_resolved;
        }
        else {
            let paths_combination = path1_resolved + "/" + path2;
            r = resolve_one_without_starting_sep(paths_combination.as_ref());
        }
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
/// - If the path starts with a path separator, it is kept.
/// - Any empty portion and trailing path separators, such as in `a/b/` and `a//b` are eliminated.
/// ```
/// assert_eq!("a/b", file_paths::resolve_one("a/b/"));
/// assert_eq!("a/b", file_paths::resolve_one("a//b"));
/// ```
pub fn resolve_one(path: &str) -> String {
    let starts_with_slash = STARTS_WITH_PATH_SEPARATOR.is_match(path);
    let r = resolve_one_without_starting_sep(path);
    return if starts_with_slash { "/".to_owned() + &r } else { r };
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
        assert_eq!(".", relative("/a/b", "/a/b"));
        assert_eq!("c", relative("/a/b", "/a/b/c"));
        assert_eq!("../../c/d", relative("/a/b", "/c/d"));
        assert_eq!("../c", relative("/a/b", "/a/c"));
    }
}