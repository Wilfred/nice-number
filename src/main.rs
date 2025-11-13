use std::io::{self, BufRead};
use num_format::{Locale, ToFormattedString};
use colored::Colorize;

fn get_size_description(number: f64) -> String {
    let abs_value = number.abs();

    match abs_value as i64 {
        0..=999 => "(tiny)".bright_cyan().to_string(),
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
    }
}
