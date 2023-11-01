use std::{fs, io};

fn main() -> Result<(), io::Error> {
    let abbreviations = vec![
        ("pentru", "pt"),
        ("pentru", "ptr"),
        ("domnul", "dl"),
        ("doamna", "dna"),
        ("You all ought not to have", "Y'ought'n't've"),
    ];
    let text = fs::read_to_string("src/text.txt")?;
    let mut replaced: bool;
    for word in text.split(" ") {
        replaced = false;
        for (long_form, short_form) in &abbreviations {
            if word == short_form as &str {
                print!("{} ", long_form);
                replaced = true;
                break;
            }
        }
        if !replaced {
            print!("{} ", word);
        }
    }
    Ok(())
}
