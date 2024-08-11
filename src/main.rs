use std::fs;
use std::io::{self, IsTerminal};

fn main() {
    let argument = std::env::args().nth(1).unwrap_or_default();

    match argument.as_str() {
        "-c" => {
            match find_content() {
                Some(content) => {
                    println!("Bytes in content: {}", content.len());
                }
                None => {
                    eprintln!("Error: Invalid or No inputprovided. Please provide a file path or pipe content to stdin.");
                }
            }
        },
        "-l" => {
            match find_content() {
                Some(content) => {
                    println!("Lines in content: {}", content.lines().count());
                }
                None => {
                    eprintln!("Error: Invalid or No inputprovided. Please provide a file path or pipe content to stdin.");
                }
            }
        },
        "-w" => {
            match find_content() {
                Some(content) => {
                    println!("Words in content: {}", word_count(content));
                }
                None => {
                    eprintln!("Error: Invalid or No inputprovided. Please provide a file path or pipe content to stdin.");
                }
            }
        },
        _ => {
            match find_content() {
                Some(content) => {
                    print!("{}   ", content.len());
                    println!("{}", content.lines().count());
                }
                None => {
                    eprintln!("Error: Invalid or No inputprovided. Please provide a file path or pipe content to stdin.");
                }
            }
        },
    }
}

// Function to compute word count
fn word_count(content: String) -> u64 {
    let mut word_count: u64 = 0;
    for line in content.lines() {
        let words: Vec<&str> = line.split(' ').collect();
        let filtered_words: Vec<&str> = words.into_iter().filter(|word| word.len() > 0).collect();
        word_count+= filtered_words.len() as u64
    }
    word_count
    
}

// Function to find the content from a file or piped input
fn find_content() -> Option<String> {
    let stdin = io::stdin();

    // If input is piped, read from stdin
    if !stdin.is_terminal() {
        return io::read_to_string(stdin).ok();
    }

    // Otherwise, read from the provided file path
    if let Some(path) = std::env::args().nth(2) {
        return read_content(&path);
    } else if let Some(path) = std::env::args().nth(1) {
        return read_content(&path);
    }

    // Fallback: Return None if no file path or piped input is provided
    None
}

// Function to read content from a file
fn read_content(path: &str) -> Option<String> {
    fs::read_to_string(path).ok().filter(|content| !content.is_empty())
}
