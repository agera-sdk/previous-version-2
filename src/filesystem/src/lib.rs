#![feature(io_error_more)]
#![allow(unused_assignments)]

use std::{path::{Path}};
use lazy_regex::{regex_find, regex_replace, regex_is_match};

mod errors;
pub use errors::FileError;

use rialight_temporal::{Local};
use rialight_util::{encode_uri, decode_uri};

#[doc(hidden)]
pub static mut __APP_DIRECTORY: Option<String> = None;
#[doc(hidden)]
pub static mut __APP_STORAGE_DIRECTORY: Option<String> = None;

/// Represents a path to a file or directory.
///
/// # Constructing a `File` object
///
/// `File` can be constructed either with a path or an
/// URL. The following URL schemes are supported:
/// - `file:`
/// - `app:` file in the application installation directory
/// - `app-storage:` file in the application private directory
///
/// The `File` constructor performs implicit normalization of the
/// given path argument.
///
#[derive(Clone, Eq, PartialEq)]
pub struct File {
    m_path: String,
    m_scheme: FileScheme,
}

#[derive(Clone, Eq, PartialEq)]
enum FileScheme {
    File,
    App,
    AppStorage,
}

macro_rules! browser_behavior {
    {$s:expr} => {
        #[cfg(all(target_family = "wasm", target_os = "unknown"))]
        { $s }
    };
    {$s:item} => {
        #[cfg(all(target_family = "wasm", target_os = "unknown"))]
        $s
    };
}

macro_rules! host_os_behavior {
    {$s:expr} => {
        #[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
        { $s }
    };
    {$s:item} => {
        #[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
        $s
    };
}

impl FileScheme {
    pub fn prefix(&self) -> &str {
        match self {
            FileScheme::File => "file:",
            FileScheme::App => "app:",
            FileScheme::AppStorage => "app-storage:",
        }
    }
}

fn std_io_error_to_file_error(arg: std::io::Error) -> FileError {
    match arg.kind() {
        std::io::ErrorKind::NotFound => FileError::NotFound,
        std::io::ErrorKind::PermissionDenied => FileError::PermissionDenied,
        std::io::ErrorKind::StorageFull => FileError::StorageFull,
        std::io::ErrorKind::FileTooLarge => FileError::FileTooLarge,
        std::io::ErrorKind::NotADirectory => FileError::NotADirectory,
        std::io::ErrorKind::InvalidFilename => FileError::InvalidFilename,
        _ => FileError::UnassignedError,
    }
}

impl File {
    /// Constructs a new `File` object.
    pub fn new(url_or_path: impl AsRef<str>) -> Self {
        let url_or_path = url_or_path.as_ref().to_owned();
        let mut path = String::from("");
        let mut scheme: FileScheme = FileScheme::File;

        if url_or_path.starts_with("file:") {
            path = decode_uri(url_or_path[5..].to_owned());
        } else if url_or_path.starts_with("app:") {
            path = decode_uri(url_or_path[4..].to_owned());
            scheme = FileScheme::App;
        } else if url_or_path.starts_with("app-storage:") {
            path = decode_uri(url_or_path[12..].to_owned());
            scheme = FileScheme::AppStorage;
        } else {
            path = url_or_path.to_owned();
        }

        if scheme == FileScheme::File {
            #[cfg(target_os = "windows")]
            {
                path = regex_replace!(r"^[\\/]+", path.as_ref(), |_| "").into_owned();
            }
            #[cfg(not(target_os = "windows"))]
            {
                path = regex_replace!(r"^[\\/]+", path.as_ref(), |_| "/").into_owned();
            }
        } else {
            path = regex_replace!(r"^[\\/]+", path.as_ref(), |_| "/").into_owned();
        }

        if scheme == FileScheme::File {
            #[cfg(not(target_os = "windows"))]
            if !path.starts_with("/") {
                path = "/".to_owned() + &path;
            }
        } else if !path.starts_with("/") {
            path = "/".to_owned() + &path;
        }
        path = rialight_util::path::resolve(path.as_ref(), "");
        File { m_scheme: scheme, m_path: path }
    }

    /// The URL for this file path.
    pub fn url(&self) -> String {
        let path = encode_uri(self.m_path.replace("\\", "/"));
        self.m_scheme.prefix().to_owned() + "//" + &regex_replace!(r"^[\\/]+", path.as_ref(), |_| "").into_owned()
    }

    /// The full path in the host operating system representation.
    pub fn native_path(&self) -> String {
        self.m_path.clone()
    }

    /// Finds the relative path from the `File` object to another `File` object.
    /// If they point to the same path, this function returns a single dot (`"."`).
    ///
    /// # Example
    ///
    /// ```
    /// let path = File::new("app://foo/bar").relative_path(&File::new("app://qux/foo"));
    /// assert_eq!(path.as_str(), "../../qux/foo");
    /// ```
    pub fn relative_path(&self, another: &File) -> String {
        rialight_util::path::relative(self.native_path().as_ref(), another.native_path().as_ref())
    }

    /// Resolves relative path.
    ///
    /// # Example
    ///
    /// ```
    /// let file = File::new("app://foo/bar").resolve_path("zxc/../abc");
    /// assert_eq!(file.url().as_str(), "app://foo/bar/abc");
    /// ```
    pub fn resolve_path(&self, arg: impl AsRef<str>) -> Self {
        let r = rialight_util::path::resolve(&self.m_path.clone(), arg.as_ref());
        File {
            m_scheme: self.m_scheme.clone(),
            m_path: r,
        }
    }

    /// The last portion of this path.
    /// # Example
    /// ```
    /// assert_eq!(File::new("app://foo.txt").name(), "foo.txt".to_owned());
    /// ```
    pub fn name(&self) -> String {
        if !regex_is_match!(r"[\\/]", self.native_path().as_ref()) {
            return self.native_path();
        }
        regex_find!(r"[\\/].+$", self.native_path().as_ref()).unwrap_or("/")[1..].to_owned()
    }

    /// The last portion of this path, excluding the given extension.
    /// # Example
    /// ```
    /// assert_eq!(File::new("app://foo.txt").name_without_extension(".txt"), "foo".to_owned());
    /// ```
    pub fn name_without_extension(&self, extension: impl AsRef<str>) -> String {
        let s = self.name();
        let extension = extension.as_ref();
        if s.ends_with(extension) { s[..(s.len() - extension.len())].to_owned() } else {s}
    }

    /// The filename extension. This includes the dot.
    /// # Example
    /// ```
    /// assert_eq!(File::new("app://foo.txt").extension(), ".txt".to_owned());
    /// ```
    pub fn extension(&self) -> String {
        regex_find!(r"[\\.].+$", self.native_path().as_ref()).unwrap_or("").to_owned()
    }

    /// The directory that contains the file or directory referenced by the `File` object.
    ///
    /// This property is identical to the return value of `resolve_path("..")`
    /// except that the parent of a root directory is `None`.
    pub fn parent(&self) -> Option<File> {
        let r = self.resolve_path("..");
        let p = r.native_path();
        if p.len() == 0
        || p == ".".to_owned()
        || p == "/".to_owned()
        || p == "\\".to_owned()
        {
            None
        } else {
            Some(r)
        }
    }

    /// Returns a reference to the application installation directory.
    /// This is equivalent to `(File::new"app://"")`.
    pub fn application_directory() -> Self {
        File { m_scheme: FileScheme::App, m_path: String::from("") }
    }

    /// Returns a reference to the application private directory.
    /// This is equivalent to the URL `File::new("app-storage://")`.
    pub fn application_storage_directory() -> Self {
        File { m_scheme: FileScheme::AppStorage, m_path: String::from("") }
    }

    /// The user downloads directory.
    pub fn downloads_directory() -> Option<File> {
        if let Some(r) = dirs::download_dir() {
            if let Some(r) = r.to_str() {
                return Some(File::new(r));
            }
        }
        None
    }

    /// The user documents directory.
    pub fn documents_directory() -> Option<File> {
        if let Some(r) = dirs::document_dir() {
            if let Some(r) = r.to_str() {
                return Some(File::new(r));
            }
        }
        None
    }

    /// The executable directory.
    pub fn executable_directory() -> Option<File> {
        if let Some(r) = dirs::executable_dir() {
            if let Some(r) = r.to_str() {
                return Some(File::new(r));
            }
        }
        None
    }

    // The user home directory.
    pub fn user_directory() -> Option<File> {
        if let Some(r) = dirs::home_dir() {
            if let Some(r) = r.to_str() {
                return Some(File::new(r));
            }
        }
        None
    }

    /// The user pictures directory.
    pub fn pictures_directory() -> Option<File> {
        if let Some(r) = dirs::picture_dir() {
            if let Some(r) = r.to_str() {
                return Some(File::new(r));
            }
        }
        None
    }

    /// The user videos directory.
    pub fn videos_directory() -> Option<File> {
        if let Some(r) = dirs::video_dir() {
            if let Some(r) = r.to_str() {
                return Some(File::new(r));
            }
        }
        None
    }

    /// The application working directory. This is used primarily for command-line applications.
    pub fn working_directory() -> Option<File> {
        if let Ok(r) = std::env::current_dir() {
            if let Some(r) = r.to_str() {
                return Some(File::new(r));
            }
        }
        None
    }

    /// If file reference uses a scheme other than `file:`, returns resolution
    /// from the scheme native directory to `file.native_path()`;
    /// otherwise simply returns `file.native_path()`.
    /// This is used for host operating system behaviors only.
    fn internal_native_path(&self) -> String {
        if self.m_scheme == FileScheme::App {
            let l = File::new(unsafe {__APP_DIRECTORY.clone()}.unwrap_or("".to_owned()));
            let r = regex_replace!(r"^[\\/]", self.native_path().as_ref(), |_| "").to_owned().to_string();
            l.resolve_path(r).native_path().clone()
        } else if self.m_scheme == FileScheme::AppStorage {
            let l = File::new(unsafe {__APP_STORAGE_DIRECTORY.clone()}.unwrap_or("".to_owned()));
            let r = regex_replace!(r"^[\\/]", self.native_path().as_ref(), |_| "").to_owned().to_string();
            l.resolve_path(r).native_path().clone()
        } else {
            self.native_path().clone()
        }
    }

    /// Determines whether the referenced path exists.
    pub fn exists(&self) -> bool {
        host_os_behavior! {{
            return Path::new(&self.internal_native_path()).exists();
        }}
        browser_behavior! {{
            return false;
        }}
    }

    /// Determines whether the referenced path is a directory.
    pub fn is_directory(&self) -> bool {
        host_os_behavior! {{
            return Path::new(&self.internal_native_path()).is_dir();
        }}
        browser_behavior! {{
            return false;
        }}
    }

    /// Determines whether the referenced path is a file.
    pub fn is_file(&self) -> bool {
        host_os_behavior! {{
            return Path::new(&self.internal_native_path()).is_file();
        }}
        browser_behavior! {{
            return false;
        }}
    }

    /// Determines whether the referenced path is a symbolic link.
    pub fn is_symbolic_link(&self) -> bool {
        host_os_behavior! {{
            return Path::new(&self.internal_native_path()).is_symlink();
        }}
        browser_behavior! {{
            return false;
        }}
    }

    /// Returns a canonicalization of the `File` path.
    pub fn canonicalize(&self) -> File {
        host_os_behavior! {{
            if let Ok(r) = Path::new(&self.internal_native_path()).canonicalize() {
                if let Some(r) = r.to_str() {
                    return File { m_scheme: FileScheme::File, m_path: r.to_owned() };
                }
            }
            return self.clone();
        }}
        browser_behavior! {{
            return self.clone();
        }}
    }

    /// Returns a canonicalization of the `File` path.
    pub async fn canonicalize_async(&self) -> File {
        host_os_behavior! {{
            if let Ok(r) = tokio::fs::canonicalize(&self.internal_native_path()).await {
                if let Some(r) = r.to_str() {
                    return File { m_scheme: FileScheme::File, m_path: r.to_owned() };
                }
            }
            return self.clone();
        }}
        browser_behavior! {{
            return self.clone();
        }}
    }

    /// Copies the file at the location specified by the `File` object to the location specified by the `new_location` parameter.
    /// 
    /// This method will overwrite the contents of `new_location`.
    pub fn copy_to(&self, new_location: &File) -> Result<(), FileError> {
        host_os_behavior! {{
            return match std::fs::copy(&self.internal_native_path(), new_location.internal_native_path()) {
                Err(e) => Err(std_io_error_to_file_error(e)),
                Ok(_) => Ok(()),
            };
        }}
        browser_behavior! {{
            return Err(FileError::UnassignedError);
        }}
    }

    /// Asynchronously copies the file at the location specified by the `File` object to the location specified by the `new_location` parameter.
    /// 
    /// This method will overwrite the contents of `new_location`.
    pub async fn copy_to_async(&self, new_location: &File) -> Result<(), FileError> {
        host_os_behavior! {{
            return match tokio::fs::copy(&self.internal_native_path(), new_location.internal_native_path()).await {
                Err(e) => Err(std_io_error_to_file_error(e)),
                Ok(_) => Ok(()),
            };
        }}
        browser_behavior! {{
            return Err(FileError::UnassignedError);
        }}
    }

    /// Creates the specified directory and any necessary parent directories.
    /// If the directory already exists, no action is taken.
    pub fn create_directory(&self) -> Result<(), FileError> {
        host_os_behavior! {{
            return match std::fs::create_dir_all(&self.internal_native_path()) {
                Err(e) => Err(std_io_error_to_file_error(e)),
                Ok(_) => Ok(()),
            };
        }}
        browser_behavior! {{
            return Err(FileError::UnassignedError);
        }}
    }

    /// Asynchronously creates the specified directory and any necessary parent directories.
    /// If the directory already exists, no action is taken.
    pub async fn create_directory_async(&self) -> Result<(), FileError> {
        host_os_behavior! {{
            return match tokio::fs::create_dir_all(&self.internal_native_path()).await {
                Err(e) => Err(std_io_error_to_file_error(e)),
                Ok(_) => Ok(()),
            };
        }}
        browser_behavior! {{
            return Err(FileError::UnassignedError);
        }}
    }

    /// Read file contents as bytes.
    pub fn read_bytes(&self) -> Result<Vec<u8>, FileError> {
        host_os_behavior! {{
            return match std::fs::read(&self.internal_native_path()) {
                Err(e) => Err(std_io_error_to_file_error(e)),
                Ok(r) => Ok(r),
            };
        }}
        browser_behavior! {{
            return Err(FileError::UnassignedError);
        }}
    }

    /// Asynchronously read file contents as bytes.
    pub async fn read_bytes_async(&self) -> Result<Vec<u8>, FileError> {
        host_os_behavior! {{
            return match tokio::fs::read(&self.internal_native_path()).await {
                Err(e) => Err(std_io_error_to_file_error(e)),
                Ok(r) => Ok(r),
            };
        }}
        browser_behavior! {{
            return Err(FileError::UnassignedError);
        }}
    }

    /// Read file contents as a UTF-8 string.
    pub fn read_utf8(&self) -> Result<String, FileError> {
        host_os_behavior! {{
            return match std::fs::read_to_string(&self.internal_native_path()) {
                Err(e) => Err(std_io_error_to_file_error(e)),
                Ok(r) => Ok(r),
            };
        }}
        browser_behavior! {{
            return Err(FileError::UnassignedError);
        }}
    }

    /// Asynchronously read file contents as a UTF-8 string.
    pub async fn read_utf8_async(&self) -> Result<String, FileError> {
        host_os_behavior! {{
            return match tokio::fs::read_to_string(&self.internal_native_path()).await {
                Err(e) => Err(std_io_error_to_file_error(e)),
                Ok(r) => Ok(r),
            };
        }}
        browser_behavior! {{
            return Err(FileError::UnassignedError);
        }}
    }

    /// Returns a vector of `File` objects corresponding to files and directories
    /// in the directory represented by the `File` object.
    pub fn get_directory_listing(&self) -> Result<Vec<File>, FileError> {
        host_os_behavior! {{
            let mut r = Vec::<File>::new();
            let read_dir_iter = match std::fs::read_dir(self.internal_native_path()) {
                Err(e) => {
                    return Err(std_io_error_to_file_error(e));
                },
                Ok(r) => r,
            };
            for entry in read_dir_iter {
                let entry = match entry {
                    Err(e) => {
                        return Err(std_io_error_to_file_error(e));
                    },
                    Ok(r) => r,
                };
                let path = entry.path();
                let path = path.to_str();
                if let Some(path) = path {
                    r.push(File::new(path));
                }
            }
            return Ok(r.clone());
        }}
        browser_behavior! {{
            return Err(FileError::UnassignedError);
        }}
    }

    /// Deletes empty directory.
    pub fn delete_empty_directory(&self) -> Result<(), FileError> {
        host_os_behavior! {{
            return match std::fs::remove_dir(self.internal_native_path()) {
                Ok(_) => Ok(()),
                Err(e) => Err(std_io_error_to_file_error(e)),
            };
        }}
        browser_behavior! {{
            return Err(FileError::UnassignedError);
        }}
    }

    /// Asynchronously deletes empty directory.
    pub async fn delete_empty_directory_async(&self) -> Result<(), FileError> {
        host_os_behavior! {{
            return match tokio::fs::remove_dir(self.internal_native_path()).await {
                Ok(_) => Ok(()),
                Err(e) => Err(std_io_error_to_file_error(e)),
            };
        }}
        browser_behavior! {{
            return Err(FileError::UnassignedError);
        }}
    }

    /// Deletes directory after deleting all its contents.
    pub fn delete_all_directory(&self) -> Result<(), FileError> {
        host_os_behavior! {{
            return match std::fs::remove_dir_all(self.internal_native_path()) {
                Ok(_) => Ok(()),
                Err(e) => Err(std_io_error_to_file_error(e)),
            };
        }}
        browser_behavior! {{
            return Err(FileError::UnassignedError);
        }}
    }

    /// Asynchronously deletes directory after deleting all its contents.
    pub async fn delete_all_directory_async(&self) -> Result<(), FileError> {
        host_os_behavior! {{
            return match tokio::fs::remove_dir_all(self.internal_native_path()).await {
                Ok(_) => Ok(()),
                Err(e) => Err(std_io_error_to_file_error(e)),
            };
        }}
        browser_behavior! {{
            return Err(FileError::UnassignedError);
        }}
    }

    /// Deletes file.
    pub fn delete_file(&self) -> Result<(), FileError> {
        host_os_behavior! {{
            if self.is_directory() {
                return Err(FileError::NotAFile);
            }
            return match std::fs::remove_file(self.internal_native_path()) {
                Err(e) => Err(std_io_error_to_file_error(e)),
                Ok(_) => Ok(()),
            };
        }}
        browser_behavior! {{
            return Err(FileError::UnassignedError);
        }}
    }

    /// Asynchronously deletes file.
    pub async fn delete_file_async(&self) -> Result<(), FileError> {
        host_os_behavior! {{
            if self.is_directory() {
                return Err(FileError::NotAFile);
            }
            return match tokio::fs::remove_file(self.internal_native_path()).await {
                Err(e) => Err(std_io_error_to_file_error(e)),
                Ok(_) => Ok(()),
            };
        }}
        browser_behavior! {{
            return Err(FileError::UnassignedError);
        }}
    }

    /// Move a file or directory to another path specified by the _to_ parameter,
    /// replacing the original file if _to_ already exists.
    pub fn move_to(&self, to: &File) -> Result<(), FileError> {
        host_os_behavior! {{
            return match std::fs::rename(self.internal_native_path(), to.internal_native_path()) {
                Err(e) => Err(std_io_error_to_file_error(e)),
                Ok(_) => Ok(()),
            };
        }}
        browser_behavior! {{
            return Err(FileError::UnassignedError);
        }}
    }

    /// Asynchronously move a file or directory to another path specified by the _to_ parameter,
    /// replacing the original file if _to_ already exists.
    pub async fn move_to_async(&self, to: &File) -> Result<(), FileError> {
        host_os_behavior! {{
            return match tokio::fs::rename(self.internal_native_path(), to.internal_native_path()).await {
                Err(e) => Err(std_io_error_to_file_error(e)),
                Ok(_) => Ok(()),
            };
        }}
        browser_behavior! {{
            return Err(FileError::UnassignedError);
        }}
    }

    /// Rename a file or directory to a new name specified by the _to_ parameter,
    /// replacing the original file if _to_ already exists.
    pub fn rename(&self, to: &File) -> Result<(), FileError> {
        self.move_to(to)
    }

    /// Asynchronously rename a file or directory to a new name specified by the _to_ parameter,
    /// replacing the original file if _to_ already exists.
    pub async fn rename_async(&self, to: &File) -> Result<(), FileError> {
        self.move_to_async(to).await
    }

    /// Writes bytes to a file.
    pub fn write<B: AsRef<[u8]>>(&self, b: B) -> Result<(), FileError> {
        host_os_behavior! {{
            return match std::fs::write(self.internal_native_path(), b.as_ref()) {
                Err(e) => Err(std_io_error_to_file_error(e)),
                Ok(_) => Ok(()),
            };
        }}
        browser_behavior! {{
            return Err(FileError::UnassignedError);
        }}
    }

    /// Asynchronously writes bytes to a file.
    pub async fn write_async<B: AsRef<[u8]>>(&self, b: B) -> Result<(), FileError> {
        host_os_behavior! {{
            return match tokio::fs::write(self.internal_native_path(), b.as_ref()).await {
                Err(e) => Err(std_io_error_to_file_error(e)),
                Ok(_) => Ok(()),
            };
        }}
        browser_behavior! {{
            return Err(FileError::UnassignedError);
        }}
    }

    /// Creation date.
    pub fn creation_date(&self) -> Result<rialight_temporal::DateTime<Local>, FileError> {
        host_os_behavior! {{
            match std::fs::metadata(self.internal_native_path()) {
                Ok(metadata) => match metadata.created() {
                    Ok(st) => Ok(rialight_temporal::DateTime::from(st)),
                    Err(e) => Err(std_io_error_to_file_error(e)),
                },
                Err(e) => Err(std_io_error_to_file_error(e)),
            }
        }}
        browser_behavior! {{
            return Err(FileError::UnassignedError);
        }}
    }

    /// Creation date.
    pub async fn creation_date_async(&self) -> Result<rialight_temporal::DateTime<Local>, FileError> {
        host_os_behavior! {{
            match tokio::fs::metadata(self.internal_native_path()).await {
                Ok(metadata) => match metadata.created() {
                    Ok(st) => Ok(rialight_temporal::DateTime::from(st)),
                    Err(e) => Err(std_io_error_to_file_error(e)),
                },
                Err(e) => Err(std_io_error_to_file_error(e)),
            }
        }}
        browser_behavior! {{
            return Err(FileError::UnassignedError);
        }}
    }

    /// Modification date.
    pub fn modification_date(&self) -> Result<rialight_temporal::DateTime<Local>, FileError> {
        host_os_behavior! {{
            match std::fs::metadata(self.internal_native_path()) {
                Ok(metadata) => match metadata.modified() {
                    Ok(st) => Ok(rialight_temporal::DateTime::from(st)),
                    Err(e) => Err(std_io_error_to_file_error(e)),
                },
                Err(e) => Err(std_io_error_to_file_error(e)),
            }
        }}
        browser_behavior! {{
            return Err(FileError::UnassignedError);
        }}
    }

    /// Modification date.
    pub async fn modification_date_async(&self) -> Result<rialight_temporal::DateTime<Local>, FileError> {
        host_os_behavior! {{
            match tokio::fs::metadata(self.internal_native_path()).await {
                Ok(metadata) => match metadata.modified() {
                    Ok(st) => Ok(rialight_temporal::DateTime::from(st)),
                    Err(e) => Err(std_io_error_to_file_error(e)),
                },
                Err(e) => Err(std_io_error_to_file_error(e)),
            }
        }}
        browser_behavior! {{
            return Err(FileError::UnassignedError);
        }}
    }

    /// Size of the file in bytes.
    pub fn size(&self) -> Result<i64, FileError> {
        host_os_behavior! {{
            return match std::fs::metadata(self.internal_native_path()) {
                Err(e) => Err(std_io_error_to_file_error(e)),
                Ok(metadata) => Ok(metadata.len() as i64),
            };
        }}
        browser_behavior! {{
            return Err(FileError::UnassignedError);
        }}
    }

    /// Size of the file in bytes.
    pub async fn size_async(&self) -> Result<i64, FileError> {
        host_os_behavior! {{
            return match tokio::fs::metadata(self.internal_native_path()).await {
                Err(e) => Err(std_io_error_to_file_error(e)),
                Ok(metadata) => Ok(metadata.len() as i64),
            };
        }}
        browser_behavior! {{
            return Err(FileError::UnassignedError);
        }}
    }
}

#[cfg(test)]
mod test {
    use super::File;

    #[test]
    fn path_creation() {
        // for now, the File constructor does not adapt to the
        // working directory.
        let file = File::new("foo");
        assert_eq!(file.native_path().as_str(), "foo");

        // file.name(), file.name_without_extension() and file.extension()
        assert_eq!(File::new("app://foo.txt").name(), "foo.txt".to_owned());
        assert_eq!(File::new("app://foo.txt").name_without_extension(".txt"), "foo".to_owned());
        assert_eq!(File::new("app://foo.txt").extension(), ".txt".to_owned());
    }

    #[test]
    fn path_relativity() {
        // file.relative_path()
        let path = File::new("app://foo/bar").relative_path(&File::new("app://qux/foo"));
        assert_eq!(path.as_str(), "../../qux/foo");

        // file.resolve_path()
        let file = File::new("app://foo/bar").resolve_path("zxc/../abc");
        assert_eq!(file.url().as_str(), "app://foo/bar/abc");
    }
}