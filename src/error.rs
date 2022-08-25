use std::path::PathBuf;

/// Archive error.
#[derive(thiserror::Error, Debug)]
// NONEXHAUSTIVE new formats could add new error types
#[non_exhaustive]
pub enum Error {
    /// I/O error.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Unsupported archive file type.
    #[error("unsupported archive file type: {0}")]
    UnsupportedArchiveType(PathBuf),
}

// used internally
pub type Result<T> = std::result::Result<T, Error>;
