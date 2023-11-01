use std::time::Instant;
use std::{fs, io};
use thiserror::Error;

#[derive(Error, Debug)]
enum EncodingError {
    #[error("{0} is not ASCII")]
    NonASCII(char),
    #[error("Read/Write error")]
    BadInputFile(#[from] io::Error),
}

fn rot13_encode() -> Result<(), EncodingError> {
    let text: String = fs::read_to_string("input.txt")?;
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
    fs::write("output.txt", output)?;
    Ok(())
}

fn main() {
    let start = Instant::now();
    match rot13_encode() {
        Ok(()) => {}
        Err(e) => println!("{:?}", e),
    }
    println!("{:?}", start.elapsed());
}
