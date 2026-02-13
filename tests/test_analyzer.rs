use std::path::Path;

// Note: These tests use the library directly.
// Run with: cargo test

#[test]
fn test_analyze_simple_file() {
    let path = Path::new("tests/fixtures/simple.txt");
    let stats = Glyph::core::analyzer::analyze(path, 5).unwrap();

    assert!(stats.words > 0);
    assert!(stats.lines > 0);
    assert!(stats.characters > 0);
    assert!(stats.size_bytes > 0);
    assert!(stats.reading_time_secs > 0.0);
}

#[test]
fn test_analyze_returns_top_words() {
    let path = Path::new("tests/fixtures/simple.txt");
    let stats = Glyph::core::analyzer::analyze(path, 3).unwrap();

    assert!(stats.top_words.len() <= 3);
    // Words should be sorted by frequency (highest first)
    for window in stats.top_words.windows(2) {
        assert!(window[0].count >= window[1].count);
    }
}

#[test]
fn test_analyze_empty_file() {
    let path = Path::new("tests/fixtures/empty.txt");
    let stats = Glyph::core::analyzer::analyze(path, 5).unwrap();

    assert_eq!(stats.words, 0);
    assert_eq!(stats.characters, 0);
    assert!(stats.top_words.is_empty());
}

#[test]
fn test_analyze_unicode_file() {
    let path = Path::new("tests/fixtures/unicode.txt");
    let stats = Glyph::core::analyzer::analyze(path, 5).unwrap();

    assert!(stats.emoji_count > 0);
    assert!(stats.characters > 0);
}

#[test]
fn test_analyze_nonexistent_file() {
    let path = Path::new("tests/fixtures/nonexistent.txt");
    let result = Glyph::core::analyzer::analyze(path, 5);

    assert!(result.is_err());
}
