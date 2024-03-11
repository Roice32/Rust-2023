use std::{fs, io};

fn main() -> Result<(), io::Error> {
    let hosts_info = fs::read_to_string("src/hosts.txt")?;
    let mut first_word: String = String::from("");
    let mut second_word: String = String::from("");
    let mut word_count: u8;
    for line in hosts_info.lines() {
        if line.chars().nth(0) == Some('#') {
            continue;
        }
        word_count = 0;
        for word in line.split(" ") {
            if word_count == 0 {
                first_word = String::from(word);
                word_count += 1;
                continue;
            }
            if word_count == 1 && word.len() != 0 {
                second_word = String::from(word);
                break;
            }
        }
        if first_word != "" {
            println!("{} => {}", second_word, first_word);
        }
    }
    Ok(())
}
