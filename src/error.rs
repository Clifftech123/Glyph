use std::fmt;
use std::io;

#[derive(Debug)]
pub enum GlyphError {
    Io(io::Error),
    InvalidUtf8(String),
    PathNotFound(String),
    NoFilesFound,
}

impl fmt::Display for GlyphError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GlyphError::Io(err) => write!(f, "I/O error: {err}"),
            GlyphError::InvalidUtf8(path) => write!(f, "File is not valid UTF-8: {path}"),
            GlyphError::PathNotFound(path) => write!(f, "Path not found: {path}"),
            GlyphError::NoFilesFound => write!(f, "No readable text files found"),
        }
    }
}
impl std::error::Error for GlyphError {}

impl From<io::Error> for GlyphError {
    fn from(err: io::Error) -> Self {
        GlyphError::Io(err)
    }
}
pub type Result<T> = std::result::Result<T, GlyphError>;
