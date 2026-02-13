use std::path::{Path, PathBuf};

use walkdir::WalkDir;

/// Known text file extensions that Glyph can analyze
const TEXT_EXTENSIONS: &[&str] = &[
    "txt",
    "md",
    "rs",
    "py",
    "js",
    "ts",
    "jsx",
    "tsx",
    "html",
    "css",
    "json",
    "toml",
    "yaml",
    "yml",
    "xml",
    "csv",
    "log",
    "sh",
    "bat",
    "c",
    "cpp",
    "h",
    "hpp",
    "java",
    "go",
    "rb",
    "php",
    "sql",
    "swift",
    "kt",
    "scala",
    "r",
    "lua",
    "pl",
    "ex",
    "exs",
    "vim",
    "conf",
    "cfg",
    "ini",
    "env",
    "dockerfile",
];

/// Recursively collect all text files in a directory.
pub fn collect_files(dir: &Path) -> Vec<PathBuf> {
    WalkDir::new(dir)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .filter(|entry| is_text_file(entry.path()))
        .map(|entry| entry.into_path())
        .collect()
}

/// Check if a file has a known text extension.
fn is_text_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| TEXT_EXTENSIONS.contains(&ext.to_lowercase().as_str()))
        .unwrap_or(false)
}
