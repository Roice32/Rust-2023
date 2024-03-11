fn add_chars_n(mut s: String, c: char, mut n: u32) -> String {
    while n != 0 {
        s.push(c);
        n -= 1;
    }
    return s;
}

fn main() {
    let mut s = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + 'a' as u8) as char;
        s = add_chars_n(s, c, 26 - i);

        i += 1;
    }

    print!("{}", s);
}
