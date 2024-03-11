use std::{fs, io};

fn longest_line_by_bytes(text: &String) {
    let mut longest_line: String = String::from("");
    let mut max_len: u32 = 0;
    let mut curr_len: u32;
    for l in text.lines() {
        curr_len = l.len() as u32;
        if curr_len > max_len {
            max_len = curr_len;
            longest_line = l.to_string();
        }
    }
    println!("{} {}", max_len, longest_line);
}

fn longest_line_by_chars(text: &String) {
    let mut longest_line: String = String::from("");
    let mut max_len: u32 = 0;
    let mut curr_len: u32;
    for l in text.lines() {
        curr_len = l.chars().count() as u32;
        if curr_len > max_len {
            max_len = curr_len;
            longest_line = l.to_string();
        }
    }
    println!("{} {}", max_len, longest_line);
}

fn main() -> Result<(), io::Error> {
    let input = fs::read_to_string("src/text.txt")?;
    longest_line_by_bytes(&input);
    longest_line_by_chars(&input);
    Ok(())
}
