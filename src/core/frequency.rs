use std::collections::HashMap;

use crate::models::{CharBreakdown, WordFrequency};

pub fn word_frequencies(content: &str, top_n: usize) -> Vec<WordFrequency> {
    let mut map: HashMap<String, usize> = HashMap::new();

    // Count the most frequent words in the text.
    // Words are lowercased and stripped of punctuation (except apostrophes)
    // so "Hello", "hello", and "hello!" all count as the same word.
    for word in content.split_whitespace() {
        let cleaned: String = word
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '\'')
            .collect::<String>()
            .to_lowercase();

        if !cleaned.is_empty() {
            *map.entry(cleaned).or_insert(0) += 1;
        }
    }

    let mut sorted: Vec<(String, usize)> = map.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    sorted.truncate(top_n);
    sorted
        .into_iter()
        .map(|(word, count)| WordFrequency { word, count })
        .collect()
}

// Count how many uppercase, lowercase, digit, punctuation, whitespace,
// and emoji characters are in the text.
pub fn char_breakdown(content: &str) -> (CharBreakdown, usize) {
    let mut uppercase = 0;
    let mut lowercase = 0;
    let mut digits = 0;
    let mut punctuation = 0;
    let mut whitespace = 0;
    let mut emoji = 0;
    for ch in content.chars() {
        if ch.is_uppercase() {
            uppercase += 1;
        } else if ch.is_lowercase() {
            lowercase += 1;
        } else if ch.is_ascii_digit() {
            digits += 1;
        } else if ch.is_whitespace() {
            whitespace += 1;
        } else if ch.is_ascii_punctuation() {
            punctuation += 1;
        } else if is_emoji(ch) {
            emoji += 1;
        }
    }
    let breakdown = CharBreakdown {
        uppercase,
        lowercase,
        digits,
        punctuation,
        whitespace,
    };
    (breakdown, emoji)
}

/// Check if a character falls within common emoji Unicode ranges.
fn is_emoji(ch: char) -> bool {
    let code = ch as u32;
    matches!(
        code,
        0x1F600..=0x1F64F       // Emoticons
            | 0x1F300..=0x1F5FF // Misc symbols & pictographs
            | 0x1F680..=0x1F6FF // Transport & map
            | 0x1F1E0..=0x1F1FF // Regional flags
            | 0x2600..=0x26FF   // Misc symbols
            | 0x2700..=0x27BF   // Dingbats
            | 0xFE00..=0xFE0F   // Variation selectors
            | 0x1F900..=0x1F9FF // Supplemental symbols
            | 0x1FA00..=0x1FA6F // Chess symbols
            | 0x1FA70..=0x1FAFF // Symbols extended-A
            | 0x200D            // Zero-width joiner
    )
}
