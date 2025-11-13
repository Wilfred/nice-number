use std::io::{self, BufRead};
use num_format::{Locale, ToFormattedString};
use colored::Colorize;
use clap::Parser;

/// Format numbers with thousand separators and colorful size descriptions
#[derive(Parser)]
#[command(name = "nn")]
#[command(version)]
#[command(about)]
#[command(long_about = "Format numbers with thousand separators and colorful size descriptions

Reads a number from stdin and outputs it formatted with commas as thousand
separators, along with a description of its size. Decimal numbers
are rounded to 2 decimal places, with a \"(rounded)\" note when applicable.

EXAMPLES:
  echo \"42\" | nn              # 42 (small)
  echo \"5000\" | nn            # 5,000 (medium)
  echo \"42.123456\" | nn       # 42.12 (rounded) (small)
  echo \"1234567.89\" | nn      # 1,234,567.89 (pretty big)
  echo \"9876543210\" | nn      # 9,876,543,210 (extremely big)")]
struct Cli {}

fn get_size_description(number: f64) -> String {
    let abs_value = number.abs();

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
        format!("{}.{:02}", formatted_int, (decimal_part * 100.0).round() as u32)
    } else {
        formatted_int
    }
}

fn main() {
    let _cli = Cli::parse();

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(line)) = lines.next() {
        match line.trim().parse::<f64>() {
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

                println!("{}{} {}", formatted, rounded_text, get_size_description(rounded));
            }
            Err(_) => {
                eprintln!("Error: Please enter a valid number");
                std::process::exit(1);
            }
        }
    } else {
        eprintln!("Error: Please enter a valid number");
        std::process::exit(1);
    }
}
