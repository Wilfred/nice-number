use clap::Parser;
use colored::Colorize;
use num_format::{Locale, ToFormattedString};
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

EXAMPLES:
  nn 42                        # 42 (small)
  nn 5000                      # 5,000 (medium)
  nn 42.123456                 # 42.12 (rounded) (small)
  nn 1234567.89                # 1,234,567.89 (pretty big)
  nn 9876543210                # 9,876,543,210 (extremely big)
  nn 1.23e5                    # 123,000 (medium)
  nn -- -5000                  # -5,000 (medium) [use -- for negative numbers]
  echo \"42\" | nn              # Can also read from stdin"
)]
struct Cli {
    /// The number to format (reads from stdin if not provided)
    number: Option<String>,
}

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
        format!(
            "{}.{:02}",
            formatted_int,
            (decimal_part * 100.0).round() as u32
        )
    } else {
        formatted_int
    }
}

fn process_number(input: &str) {
    match input.trim().parse::<f64>() {
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
        }
        Err(_) => {
            eprintln!("Error: Please enter a valid number");
            std::process::exit(1);
        }
    }
}

fn main() {
    let cli = Cli::parse();

    if let Some(number_arg) = cli.number {
        // Process number from command-line argument
        process_number(&number_arg);
    } else {
        // Process number from stdin
        let stdin = io::stdin();
        let mut lines = stdin.lock().lines();

        if let Some(Ok(line)) = lines.next() {
            process_number(&line);
        } else {
            eprintln!("Error: Please enter a valid number");
            std::process::exit(1);
        }
    }
}
