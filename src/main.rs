mod cli;
mod core;
mod error;
mod io;
mod models;

use std::path::Path;
use std::process;

use clap::Parser;

use cli::Cli;
use core::{analyzer, search};
use io::{output, walker};

fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(&cli) {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

fn run(cli: &Cli) -> error::Result<()> {
    let path = Path::new(&cli.path);

    // Validate path exists
    if !path.exists() {
        return Err(error::GlyphError::PathNotFound(cli.path.clone()));
    }

    // Collect files (single file or directory scan)
    let files = if path.is_dir() {
        let found = walker::collect_files(path);
        if found.is_empty() {
            return Err(error::GlyphError::NoFilesFound);
        }
        found
    } else {
        vec![path.to_path_buf()]
    };

    // Search mode
    if let Some(ref pattern) = cli.search {
        for file in &files {
            match search::search_file(file, pattern) {
                Ok(result) => {
                    if cli.json {
                        let json = serde_json::json!({
                            "file": result.file_path,
                            "pattern": result.pattern,
                            "occurrences": result.occurrences,
                        });
                        println!("{}", serde_json::to_string_pretty(&json).unwrap());
                    } else {
                        output::print_search_result(&result);
                    }
                }
                Err(e) => eprintln!("Skipping '{}': {e}", file.display()),
            }
        }
        return Ok(());
    }

    // Analysis mode
    let mut all_stats = Vec::new();

    for file in &files {
        match analyzer::analyze(file, cli.top) {
            Ok(stats) => {
                if cli.json {
                    output::print_json(&stats);
                } else {
                    output::print_stats(&stats);
                }
                all_stats.push(stats);
            }
            Err(e) => eprintln!("Skipping '{}': {e}", file.display()),
        }
    }

    // Print summary for multi-file analysis
    if all_stats.len() > 1 && !cli.json {
        output::print_summary(&all_stats);
    }

    Ok(())
}
