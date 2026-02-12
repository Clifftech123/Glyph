use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct FileStats {
    pub file_path: String,
    pub size_bytes: u64,
    pub lines: usize,
    pub words: usize,
    pub characters: usize,
    pub reading_time_secs: f64,
    pub uppercase: usize,
    pub lowercase: usize,
    pub digits: usize,
    pub punctuation: usize,
    pub whitespace: usize,
    pub longest_line_number: usize,
    pub longest_line_content: String,
    pub shortest_line_number: usize,
    pub shortest_line_content: String,
    pub top_words: Vec<WordFrequency>,
    pub emoji_count: usize,
}

#[derive(Debug, Serialize)]
pub struct WordFrequency {
    pub word: String,
    pub count: usize,
}
