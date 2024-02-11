#![warn(clippy::all, clippy::pedantic)]
use dirs::config_dir;
use std::fs::read_to_string;

fn main() {
    let config = config_dir();
    if config.is_none() {
        println!("The config directory does not exist or is not accessible.");
        return;
    }
    let mut config = config.unwrap();
    config.push("quoted");
    config.push("config");
    config.set_extension("json");
    let table_raw =
        read_to_string(config).expect("Quoted's config file does not exist or is not accessible.");
    let quotes: Vec<(String, String)> = serde_json::from_str(&table_raw).unwrap();
    let rng = rand::random::<usize>();
    let (quote, author) = &quotes[rng & quotes.len()];
    let mut quote_lines: Vec<String> = Vec::new();
    let mut quote_line = String::from("|");
    for quote_word in quote.split(' ') {
        if format!("{} {quote_word}", &quote_line).len() > 50 || quote_word.contains('\n') {
            quote_lines.push(quote_line.clone());
            quote_line = String::from("|");
        }
        if !quote_word.contains('\n') {
            quote_line = format!("{} {quote_word}", &quote_line);
        }
    }
    quote_lines.push(quote_line);
    let mut author_lines: Vec<String> = Vec::new();
    let mut author_line = String::from(" -");
    for author_word in author.split(' ') {
        if format!("{} {author_word}", &author_line).len() > 50 || author_word.contains('\n') {
            author_lines.push(author_line.clone());
            author_line = String::from("  ");
        }
        if !author_word.contains('\n') {
            author_line = format!("{} {author_word}", &author_line);
        }
    }
    author_lines.push(author_line);
    for line in quote_lines {
        println!("{line}");
    }
    for line in author_lines {
        println!("{line}");
    }
}
