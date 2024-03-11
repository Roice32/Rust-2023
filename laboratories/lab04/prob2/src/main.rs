use std::{fs, io};
use thiserror::Error;

#[derive(Error, Debug)]
enum EncodingError {
    #[error("{0} is not ASCII")]
    NonASCII(char),
    #[error("Read error")]
    BadInputFile(#[from] io::Error),
}

fn rot13_encode(text: &String) -> Result<(), EncodingError> {
    let mut output: String = String::from("");
    let mut res_c: char;
    for c in text.chars() {
        if c.len_utf8() != 1 {
            return Err(EncodingError::NonASCII(c));
        }
        res_c = c;
        if c >= 'a' && c <= 'm' || c >= 'A' && c <= 'M' {
            res_c = (c as u8 + 13) as char;
        }
        if c >= 'n' && c <= 'z' || c >= 'N' && c <= 'Z' {
            res_c = (c as u8 - 13) as char;
        }
        output.push(res_c);
    }
    println!("{}", output);
    Ok(())
}

fn main() -> Result<(), EncodingError> {
    let input: String = fs::read_to_string("src/text.txt")?;
    rot13_encode(&input)?;
    Ok(())
}
