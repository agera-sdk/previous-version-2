//! Module with functions for manipulating file paths.

use lazy_regex::{lazy_regex, Lazy, Regex};

static PATH_SEPARATOR: Lazy<Regex> = lazy_regex!(r"[/\\]");
static STARTS_WITH_PATH_SEPARATOR: Lazy<Regex> = lazy_regex!(r"^[/\\]");

pub fn relative(from_path: &str, to_path: &str) -> String {
    let mut r = Vec::<String>::new();

    let mut from_parts: Vec<String> = PATH_SEPARATOR.split(resolve_single_path(from_path).as_ref()).map(|s| s.to_owned()).collect();
    let mut to_parts: Vec<String> = PATH_SEPARATOR.split(resolve_single_path(to_path).as_ref()).map(|s| s.to_owned()).collect();

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

fn resolve_single_path(path: &str) -> String {
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

pub fn resolve(path1: &str, path2: &str) -> String {
    let starts_with_slash = path1.starts_with("/");
    #[allow(unused_assignments)]
    let mut r = "".to_owned();
    if STARTS_WITH_PATH_SEPARATOR.is_match(path2) {
        r = resolve_single_path(path2);
    } else {
        let path1_resolved = resolve_single_path(path1);
        if path2.is_empty() {
            r = path1_resolved;
        }
        else {
            let paths_combination = path1_resolved + "/" + path2;
            r = resolve_single_path(paths_combination.as_ref());
        }
    }
    if starts_with_slash {
        r = "/".to_owned() + &r;
    }
    r
}