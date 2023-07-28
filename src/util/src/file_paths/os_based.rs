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

pub use super::{  
    change_extension,
    change_last_extension,
    has_extension,
    has_extensions,
    base_name,
    base_name_without_ext,
};

use super::reg_exp::*;

static STARTS_WITH_WINDOWS_PATH_PREFIX: StaticRegExp = static_reg_exp!(r#"(?x)
    ^ (
        (\\\\)       | # UNC prefix
        ([A-Za-z]\:)   # drive prefix
    )
"#);

static UNC_PREFIX: &'static str = r"\\";

/// Resolves `path2` relative to `path1`. This methodd
/// has the same behavior from [`rialight_util::file_paths::resolve`],
/// except that if given a `manipulation` that is not `Default`,
/// it detects Windows absolute paths.
pub fn resolve(path1: &str, path2: &str, manipulation: OsPathManipulation) -> String {
    match manipulation {
        OsPathManipulation::Default => {
            super::resolve(path1, path2)
        },
        OsPathManipulation::Windows => {
            let paths = [path1, path2].map(|p| p.to_owned());
            let prefixed: Vec<String> = paths.iter().filter(|path| STARTS_WITH_WINDOWS_PATH_PREFIX.is_match(path)).map(|s| s.clone()).collect();
            if prefixed.is_empty() {
                return super::resolve(path1, path2);
            }
            let prefix = STARTS_WITH_WINDOWS_PATH_PREFIX.find(prefixed.last().unwrap().as_ref()).map(|m| m.as_str().to_owned()).unwrap();
            let paths: Vec<String> = paths.iter().map(|path| STARTS_WITH_WINDOWS_PATH_PREFIX.replace(path.as_ref(), |_: &RegExpCaptures| "/").into_owned()).collect();
            let r = super::resolve(&paths[0], &paths[1]);
            if prefix == UNC_PREFIX {
                return UNC_PREFIX.to_owned() + &r[1..];
            }
            prefix + &r
        },
    }
}

/// Resolves multiple paths with the same behavior from
/// [`rialight_util::file_paths::os_based::resolve`].
pub fn resolve_n<'a, T: IntoIterator<Item = &'a str>>(paths: T, manipulation: OsPathManipulation) -> String {
    let paths = paths.into_iter().collect::<Vec<&'a str>>();
    if paths.len() == 0 {
        return "".to_owned();
    }
    if paths.len() == 1 {
        return resolve(paths[0].as_ref(), "", manipulation);
    }
    let initial_path = resolve(paths[0].as_ref(), paths[1].as_ref(), manipulation);
    paths[2..].iter().fold(initial_path, |a, b| resolve(a.as_ref(), b.as_ref(), manipulation))
}

/// Resolves a single path with the same behavior from
/// [`rialight_util::file_paths::os_based::resolve_n`].
pub fn resolve_one(path: &str, manipulation: OsPathManipulation) -> String {
    resolve_n([path], manipulation)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(r"\\Whack/a/Box", resolve_n(["foo", r"\\Whack////a//Box", "..", "Box"], OsPathManipulation::Windows));
        assert_eq!("C:/a", resolve("C:/", "a", OsPathManipulation::Windows));
        assert_eq!("D:/", resolve("C:/", "D:/", OsPathManipulation::Windows));
        assert_eq!("D:/a", resolve_one("D:/a", OsPathManipulation::Windows));
        assert_eq!("C:/a/f/b", resolve("a", "C:/a///f//b", OsPathManipulation::Windows));
    }
}