# Glyph

A fast, beautiful CLI tool for analyzing text files — built in Rust.

Pass it a file (or a whole directory), and Glyph breaks down everything about the text: word counts, line counts, character frequencies, reading time, and more. Think of it as `wc` on steroids.

## Features

- **Basic Stats** — Word count, line count, character count, byte size
- **Reading Time** — Estimated reading time based on average reading speed
- **Frequency Analysis** — Top N most frequent words in the file
- **Character Breakdown** — Uppercase, lowercase, digits, punctuation, whitespace, Unicode/emoji counts
- **Longest/Shortest Lines** — Spot outliers in your text
- **Average Word Length** — Measure text complexity at a glance
- **Multi-File Support** — Pass a directory and analyze every file in it
- **Summary Table** — Side-by-side comparison when analyzing multiple files
- **Colorized Output** — Beautiful terminal output with colored stats
- **JSON Export** — Output stats as JSON for scripting and pipelines (`--json`)
- **Search Mode** — Count occurrences of a specific word or pattern (`--search <pattern>`)
- **Top Emojis** — Detect and rank emojis found in text

## Installation

```bash
# Clone the repo
git clone https://github.com/Clifftech123/Glyph.git
cd Glyph

# Build in release mode
cargo build --release

# Run it
cargo run -- path/to/file.txt
```

## Usage

```bash
# Analyze a single file
glyph README.md

# Analyze all files in a directory
glyph src/

# Output as JSON
glyph notes.txt --json

# Search for a word/pattern
glyph book.txt --search "rust"

# Show top 20 most frequent words (default is 10)
glyph essay.txt --top 20
```

### Example Output

```
 Glyph — File Analysis
 ─────────────────────────────────────
  File:           notes.txt
  Size:           2.4 KB
  Lines:          48
  Words:          412
  Characters:     2,461
  Reading Time:   ~2 min

 Top 5 Words:
  1. the        — 32
  2. and        — 18
  3. rust       — 14
  4. file       — 11
  5. function   — 9

 Character Breakdown:
  Uppercase:    84
  Lowercase:    1,892
  Digits:       23
  Punctuation:  117
  Whitespace:   345
 ─────────────────────────────────────
```



## Dependencies

| Crate       | Purpose                        |
| ----------- | ------------------------------ |
| `clap`      | CLI argument parsing           |
| `colored`   | Colorized terminal output      |
| `serde`     | Serialization for JSON export  |
| `serde_json`| JSON output formatting         |
| `walkdir`   | Recursive directory traversal  |

