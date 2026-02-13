use std::fs;
use std::path::Path;

use crate::core::frequency;
use crate::error::{GlyphError, Result};
use crate::models::{FileStats, LineInfo};

/// Average adult reading speed in words per minute
const WORDS_PER_MINUTE: f64 = 200.0;

/// Analyze a file and produce complete statistics.
///
/// `top_n` controls how many top-frequency words to include.
pub fn analyze(path: &Path, top_n: usize) -> Result<FileStats> {
    let content = fs::read_to_string(path)
        .map_err(|_| GlyphError::InvalidUtf8(path.display().to_string()))?;

    let metadata = fs::metadata(path)?;
    let size_bytes = metadata.len();

    // Line analysis
    let lines: Vec<&str> = content.lines().collect();
    let line_count = lines.len();

    // Word & character counts
    let word_count = content.split_whitespace().count();
    let char_count = content.chars().count();

    // Reading time estimate
    let reading_time_secs = (word_count as f64 / WORDS_PER_MINUTE) * 60.0;

    // Character breakdown & emoji count
    let (char_breakdown, emoji_count) = frequency::char_breakdown(&content);

    // Top frequent words
    let top_words = frequency::word_frequencies(&content, top_n);

    // Longest line (by character count)
    let longest_line = lines
        .iter()
        .enumerate()
        .max_by_key(|(_, line)| line.len())
        .map(|(i, line)| LineInfo {
            line_number: i + 1,
            content: line.to_string(),
        })
        .unwrap_or(LineInfo {
            line_number: 0,
            content: String::new(),
        });

    // Shortest non-empty line
    let shortest_line = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| !line.is_empty())
        .min_by_key(|(_, line)| line.len())
        .map(|(i, line)| LineInfo {
            line_number: i + 1,
            content: line.to_string(),
        })
        .unwrap_or(LineInfo {
            line_number: 0,
            content: String::new(),
        });

    Ok(FileStats {
        file_path: path.display().to_string(),
        size_bytes,
        lines: line_count,
        words: word_count,
        characters: char_count,
        reading_time_secs,
        char_breakdown,
        longest_line,
        shortest_line,
        top_words,
        emoji_count,
    })
}
