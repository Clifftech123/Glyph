use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct FileStats {
    pub file_path: String,

    pub size_bytes: u64,

    pub lines: usize,

    pub words: usize,

    pub characters: usize,

    pub reading_time_secs: f64,

    pub char_breakdown: CharBreakdown,

    pub longest_line: LineInfo,

    pub shortest_line: LineInfo,

    pub top_words: Vec<WordFrequency>,

    pub emoji_count: usize,
}

#[derive(Debug, Serialize)]
pub struct CharBreakdown {
    pub uppercase: usize,
    pub lowercase: usize,
    pub digits: usize,
    pub punctuation: usize,
    pub whitespace: usize,
}

#[derive(Debug, Serialize)]
pub struct WordFrequency {
    pub word: String,
    pub count: usize,
}

#[derive(Debug, Serialize)]
pub struct LineInfo {
    /// 1-based line number
    pub line_number: usize,
    /// The content of the line
    pub content: String,
}
