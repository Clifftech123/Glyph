use std::fs;
use std::path::Path;

use crate::error::{GlyphError, Result};

pub struct SearchResult {
    pub file_path: String,
    pub pattern: String,
    pub occurrences: usize,
}

/// Count how many times `pattern` appears in the file (case-insensitive).
pub fn search_file(path: &Path, pattern: &str) -> Result<SearchResult> {
    let content = fs::read_to_string(path)
        .map_err(|_| GlyphError::InvalidUtf8(path.display().to_string()))?;

    let lower_content = content.to_lowercase();
    let lower_pattern = pattern.to_lowercase();
    let occurrences = lower_content.matches(&lower_pattern).count();

    Ok(SearchResult {
        file_path: path.display().to_string(),
        pattern: pattern.to_string(),
        occurrences,
    })
}
