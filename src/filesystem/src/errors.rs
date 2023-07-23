#[derive(Clone, Debug)]
pub enum FileError {
    NotFound,
    PermissionDenied,
    StorageFull,
    FileTooLarge,
    NotADirectory,
    NotAFile,
    /// Caused by invalid or too large file name.
    InvalidFilename,
    /// Error of unassigned category.
    UnassignedError,
}