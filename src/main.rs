use std::io::{self, BufRead};
use num_format::{Locale, ToFormattedString};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(line)) = lines.next() {
        match line.trim().parse::<i64>() {
            Ok(number) => {
                let formatted = number.to_formatted_string(&Locale::en);
                println!("{}", formatted);
            }
            Err(_) => {
                eprintln!("Error: Please enter a valid integer");
                std::process::exit(1);
            }
        }
    }
}
