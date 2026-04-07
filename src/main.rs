use std::{env, error::Error, fs};

use threadline::{DiffLine, diff};

const USAGE: &str = "Usage: threadline <old> <new>";

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    args.next(); // ignore app name
    let old_file_path = args.next().ok_or(USAGE)?;
    let new_file_path = args.next().ok_or(USAGE)?;
    if args.next().is_some() {
        eprintln!("{USAGE}");
        std::process::exit(1);
    }

    let old_file_content = fs::read_to_string(&old_file_path).unwrap_or_else(|e| {
        eprintln!("Error reading {old_file_path}: {e}");
        std::process::exit(1);
    });

    let new_file_content = fs::read_to_string(&new_file_path).unwrap_or_else(|e| {
        eprintln!("Error reading {new_file_path}: {e}");
        std::process::exit(1);
    });

    let old_lines: Vec<&str> = old_file_content.lines().collect();
    let new_lines: Vec<&str> = new_file_content.lines().collect();

    let diff_result = diff(&old_lines, &new_lines);

    for line in diff_result {
        match line {
            DiffLine::Added(line) => println!("\x1b[32m+ {line}\x1b[0m"),
            DiffLine::Removed(line) => println!("\x1b[31m- {line}\x1b[0m"),
            DiffLine::Unchanged(line) => println!("  {line}"),
        }
    }

    Ok(())
}
