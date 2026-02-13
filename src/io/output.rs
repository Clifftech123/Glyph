use colored::Colorize;
use std::path::Path;

use crate::core::search::SearchResult;
use crate::models::FileStats;

/// Print a full analysis report with colors to the terminal.
pub fn print_stats(stats: &FileStats) {
    let separator = "─".repeat(45);

    println!();
    println!("{}", " Glyph — File Analysis".bold().cyan());
    println!(" {}", separator.dimmed());
    println!("  {:<18}{}", "File:".bold(), stats.file_path);
    println!("  {:<18}{}", "Size:".bold(), format_bytes(stats.size_bytes));
    println!("  {:<18}{}", "Lines:".bold(), format_number(stats.lines));
    println!("  {:<18}{}", "Words:".bold(), format_number(stats.words));
    println!(
        "  {:<18}{}",
        "Characters:".bold(),
        format_number(stats.characters)
    );
    println!(
        "  {:<18}{}",
        "Reading Time:".bold(),
        format_reading_time(stats.reading_time_secs)
    );

    if stats.emoji_count > 0 {
        println!("  {:<18}{}", "Emojis:".bold(), stats.emoji_count);
    }

    // Top words section
    if !stats.top_words.is_empty() {
        println!();
        println!(" {}", "Top Words:".bold().yellow());
        for (i, wf) in stats.top_words.iter().enumerate() {
            println!(
                "  {}. {:<15} {} {}",
                (i + 1).to_string().bold(),
                wf.word.green(),
                "—".dimmed(),
                wf.count
            );
        }
    }

    // Character breakdown section
    println!();
    println!(" {}", "Character Breakdown:".bold().yellow());
    println!(
        "  {:<18}{}",
        "Uppercase:".bold(),
        format_number(stats.char_breakdown.uppercase)
    );
    println!(
        "  {:<18}{}",
        "Lowercase:".bold(),
        format_number(stats.char_breakdown.lowercase)
    );
    println!(
        "  {:<18}{}",
        "Digits:".bold(),
        format_number(stats.char_breakdown.digits)
    );
    println!(
        "  {:<18}{}",
        "Punctuation:".bold(),
        format_number(stats.char_breakdown.punctuation)
    );
    println!(
        "  {:<18}{}",
        "Whitespace:".bold(),
        format_number(stats.char_breakdown.whitespace)
    );

    // Line info section
    println!();
    println!(" {}", "Line Info:".bold().yellow());
    println!(
        "  {:<18}Line {} — \"{}\"",
        "Longest:".bold(),
        stats.longest_line.line_number,
        truncate(&stats.longest_line.content, 50)
    );
    println!(
        "  {:<18}Line {} — \"{}\"",
        "Shortest:".bold(),
        stats.shortest_line.line_number,
        truncate(&stats.shortest_line.content, 50)
    );

    println!(" {}", separator.dimmed());
    println!();
}

/// Print stats as pretty-printed JSON (for scripting/pipelines).
pub fn print_json(stats: &FileStats) {
    let json = serde_json::to_string_pretty(stats).unwrap();
    println!("{json}");
}

/// Print the result of a pattern search.
pub fn print_search_result(result: &SearchResult) {
    println!(
        "{} Found {} occurrence(s) of \"{}\" in {}",
        "=>".bold().cyan(),
        result.occurrences.to_string().bold().green(),
        result.pattern.yellow(),
        result.file_path
    );
}

/// Print a summary table when multiple files are analyzed.
pub fn print_summary(all_stats: &[FileStats]) {
    let total_files = all_stats.len();
    let total_lines: usize = all_stats.iter().map(|s| s.lines).sum();
    let total_words: usize = all_stats.iter().map(|s| s.words).sum();
    let total_chars: usize = all_stats.iter().map(|s| s.characters).sum();
    let total_bytes: u64 = all_stats.iter().map(|s| s.size_bytes).sum();

    let separator = "─".repeat(45);

    println!("{}", " Summary".bold().magenta());
    println!(" {}", separator.dimmed());
    println!("  {:<18}{}", "Files:".bold(), total_files);
    println!(
        "  {:<18}{}",
        "Total Size:".bold(),
        format_bytes(total_bytes)
    );
    println!(
        "  {:<18}{}",
        "Total Lines:".bold(),
        format_number(total_lines)
    );
    println!(
        "  {:<18}{}",
        "Total Words:".bold(),
        format_number(total_words)
    );
    println!(
        "  {:<18}{}",
        "Total Chars:".bold(),
        format_number(total_chars)
    );
    println!(" {}", separator.dimmed());
    println!();
}

// ── Helper functions ──────────────────────────────────────

/// Format bytes into human-readable form (B, KB, MB).
fn format_bytes(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{bytes} B")
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}

/// Format a number with comma separators (e.g. 1,234,567).
fn format_number(n: usize) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, ch) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(ch);
    }
    result.chars().rev().collect()
}

/// Format seconds into a human-readable reading time.
fn format_reading_time(secs: f64) -> String {
    if secs < 60.0 {
        format!("~{:.0} sec", secs)
    } else {
        format!("~{:.0} min", secs / 60.0)
    }
}

/// Truncate a string to `max` characters, adding "..." if truncated.
fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        let truncated: String = s.chars().take(max).collect();
        format!("{truncated}...")
    }
}
