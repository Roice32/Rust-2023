enum CharError {
    NotASCII,
    NotDigit,
    NotHexDigit,
    NotLetter,
    NotPrintable,
}

fn is_lowercase(c: char) -> bool {
    return c >= 'a' && c <= 'z';
}

fn is_uppercase(c: char) -> bool {
    return c >= 'A' && c <= 'Z';
}

fn is_letter(c: char) -> bool {
    return is_uppercase(c) || is_lowercase(c);
}

fn is_digit(c: char) -> bool {
    return c >= '0' && c <= '9';
}

fn is_hex_digit(c: char) -> bool {
    return is_digit(c) || is_lowercase(c) && c <= 'f' || is_uppercase(c) && c <= 'F';
}

fn is_printable(c: char) -> bool {
    return c >= 32 as char && c <= 126 as char;
}

fn is_ascii(c: char) -> bool {
    return c <= 127 as char;
}

fn to_uppercase(c: char) -> Result<char, CharError> {
    if !is_letter(c) {
        return Err(CharError::NotLetter);
    }
    if is_lowercase(c) {
        return Ok((c as u8 - 'a' as u8 + 'A' as u8) as char);
    } else {
        return Ok(c);
    }
}

fn to_lowercase(c: char) -> Result<char, CharError> {
    if !is_letter(c) {
        return Err(CharError::NotLetter);
    }
    if is_uppercase(c) {
        return Ok((c as u8 - 'A' as u8 + 'a' as u8) as char);
    } else {
        return Ok(c);
    }
}

fn print_char(c: char) -> Result<(), CharError> {
    if !is_printable(c) {
        return Err(CharError::NotPrintable);
    }
    println!("{c}");
    Ok(())
}

fn char_to_number(c: char) -> Result<u32, CharError> {
    if !is_ascii(c) {
        return Err(CharError::NotASCII);
    }
    if !is_digit(c) {
        return Err(CharError::NotDigit);
    }
    return Ok(c as u32);
}

fn char_to_number_hex(c: char) -> Result<u32, CharError> {
    if !is_ascii(c) {
        return Err(CharError::NotASCII);
    }
    if !is_hex_digit(c) {
        return Err(CharError::NotHexDigit);
    }
    if is_digit(c) {
        return Ok(c as u32);
    } else {
        return Ok(c as u32 - 'A' as u32 + 10);
    }
}

fn print_error(e: CharError) {
    match e {
        CharError::NotASCII => println!("Character not ASCII."),
        CharError::NotDigit => println!("Character not a base-10 digit."),
        CharError::NotHexDigit => println!("Character not a base-16 digit."),
        CharError::NotLetter => println!("Character not a letter."),
        CharError::NotPrintable => println!("Character not printable."),
    }
}

fn main() {
    let mut x: char;
    x = 'a';
    match to_uppercase(x) {
        Ok(r) => println!("{x} as uppercase: {r}."),
        Err(e) => print_error(e),
    }
    x = ' ';
    match to_lowercase(x) {
        Ok(r) => println!("{x} as lowercase: {r}."),
        Err(e) => print_error(e),
    }
    x = 'X';
    match print_char(x) {
        Ok(()) => {}
        Err(e) => print_error(e),
    }
    x = 10 as char;
    match print_char(x) {
        Ok(()) => {}
        Err(e) => print_error(e),
    }
    x = 200 as char;
    match char_to_number(x) {
        Ok(r) => println!("{x} is still {r} as a number."),
        Err(e) => print_error(e),
    }
    x = 'z';
    match char_to_number_hex(x) {
        Ok(r) => println!("{x} is {r} as a decimal number."),
        Err(e) => print_error(e),
    }
}
