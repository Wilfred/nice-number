use std::io::{self, BufRead};
use num_format::{Locale, ToFormattedString};
use colored::Colorize;

fn get_size_description(number: i64) -> String {
    let abs_value = number.abs();

    match abs_value {
        0..=999 => "Tiny!".bright_cyan().to_string(),
        1_000..=999_999 => "Medium".bright_green().to_string(),
        1_000_000..=999_999_999 => "Pretty big!".bright_yellow().to_string(),
        _ => "EXTREMELY BIG!".bright_red().bold().to_string(),
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(line)) = lines.next() {
        match line.trim().parse::<i64>() {
            Ok(number) => {
                let formatted = number.to_formatted_string(&Locale::en);
                println!("{} {}", formatted, get_size_description(number));
            }
            Err(_) => {
                eprintln!("Error: Please enter a valid integer");
                std::process::exit(1);
            }
        }
    }
}
