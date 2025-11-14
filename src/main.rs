use clap::Parser;
use colored::Colorize;
use humansize::{BINARY, format_size, FormatSizeOptions};
use num_format::{Locale, ToFormattedString};
use regex::Regex;
use std::io::{self, BufRead};

/// Format numbers with thousand separators and colorful size descriptions
#[derive(Parser)]
#[command(name = "nn")]
#[command(version)]
#[command(about)]
#[command(
    long_about = "Format numbers with thousand separators and colorful size descriptions

Reads a number from the command line or stdin and outputs it formatted with
commas as thousand separators, along with a description of its size. Supports
integers, decimals, and scientific notation. Decimal numbers are rounded to 2
decimal places, with a \"(rounded)\" note when applicable.

Can also process arbitrary text with embedded numbers, formatting only the
numbers while preserving the rest of the text unchanged.

Use --bytes flag to also display the number in binary units (KiB, MiB, GiB, etc.)
using 1024-based prefixes.

EXAMPLES:
  nn 42                        # 42 (small)
  nn 5000                      # 5,000 (medium)
  nn 42.123456                 # 42.12 (rounded) (small)
  nn 1234567.89                # 1,234,567.89 (pretty big)
  nn 9876543210                # 9,876,543,210 (extremely big)
  nn 1.23e5                    # 123,000 (medium)
  nn -- -5000                  # -5,000 (medium) [use -- for negative numbers]
  nn 1048576 --bytes           # Also shows: 1 MiB
  echo \"The file is 1024 bytes\" | nn  # The file is 1,024 bytes
  echo \"42\" | nn              # Can also read from stdin"
)]
struct Cli {
    /// The number to format (reads from stdin if not provided)
    number: Option<String>,

    /// Display the number in binary units (KiB, MiB, GiB, etc.)
    #[arg(short, long)]
    bytes: bool,
}

fn get_size_description(number: f64) -> String {
    let abs_value = number.abs();

    // Special case for zero
    if abs_value == 0.0 {
        return "(zero)".bright_white().bold().to_string();
    }

    match abs_value as i64 {
        0..=999 => "(small)".bright_cyan().to_string(),
        1_000..=999_999 => "(medium)".bright_green().to_string(),
        1_000_000..=999_999_999 => "(pretty big)".bright_yellow().to_string(),
        _ => "(extremely big)".bright_red().bold().to_string(),
    }
}

fn format_number_with_separators(number: f64) -> String {
    let integer_part = number.trunc() as i64;
    let decimal_part = ((number.abs() - number.abs().trunc()) * 100.0).round() / 100.0;

    let formatted_int = integer_part.to_formatted_string(&Locale::en);

    if decimal_part > 0.0 {
        format!(
            "{}.{:02}",
            formatted_int,
            (decimal_part * 100.0).round() as u32
        )
    } else {
        formatted_int
    }
}

fn format_as_binary_units(number: f64) -> String {
    // For large values (>= 1 KiB), show one decimal place
    let options = FormatSizeOptions::from(BINARY).decimal_places(1);
    format_size(number as u64, options)
}

fn format_single_number(number: f64) -> String {
    let rounded = (number * 100.0).round() / 100.0;
    format_number_with_separators(rounded)
}

fn process_text_with_numbers(text: &str) -> String {
    // Regex to match numbers including decimals and scientific notation
    let re = Regex::new(r"-?\d+\.?\d*(?:[eE][+-]?\d+)?").unwrap();

    re.replace_all(text, |caps: &regex::Captures| {
        let num_str = &caps[0];
        if let Ok(number) = num_str.parse::<f64>() {
            format_single_number(number)
        } else {
            num_str.to_string()
        }
    })
    .to_string()
}

fn process_number(input: &str, show_bytes: bool) {
    let trimmed = input.trim();

    // Try to parse as a single number first
    match trimmed.parse::<f64>() {
        Ok(number) => {
            // Round to 2 decimal places
            let rounded = (number * 100.0).round() / 100.0;

            // Check if rounding occurred
            let was_rounded = (number - rounded).abs() > f64::EPSILON;

            let formatted = format_number_with_separators(rounded);
            let rounded_text = if was_rounded {
                " (rounded)".dimmed().to_string()
            } else {
                String::new()
            };

            println!(
                "{}{} {}",
                formatted,
                rounded_text,
                get_size_description(rounded)
            );

            if show_bytes {
                let binary_format = format_as_binary_units(rounded);
                println!("{}", binary_format.bright_magenta());
            }
        }
        Err(_) => {
            // Not a pure number, treat as text with embedded numbers
            let processed = process_text_with_numbers(trimmed);
            println!("{}", processed);
        }
    }
}

fn main() {
    let cli = Cli::parse();

    if let Some(number_arg) = cli.number {
        // Process number from command-line argument
        process_number(&number_arg, cli.bytes);
    } else {
        // Process number from stdin
        let stdin = io::stdin();
        let mut lines = stdin.lock().lines();

        if let Some(Ok(line)) = lines.next()
            && !line.trim().is_empty()
        {
            process_number(&line, cli.bytes);
        }
        // Empty input is allowed - just do nothing
    }
}
