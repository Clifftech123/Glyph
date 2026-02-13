use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "glyph",
    version,
    about = "A fast, beautiful CLI tool for analyzing text files"
)]
pub struct Cli {
    /// Path to a file or directory to analyze
    pub path: String,

    /// Output results as JSON
    #[arg(long)]
    pub json: bool,

    /// Search for a specific word or pattern
    #[arg(long)]
    pub search: Option<String>,

    /// Number of top frequent words to display (default: 10)
    #[arg(long, default_value = "10")]
    pub top: usize,
}
