//! Crate used for encoding basic ASCII strings into base64

/// Takes a byte sequence (the bytes of an `ASCII String`)
/// and returns a base65-encoded `String`.
///
/// Should `input` not have enough (3n) bytes for proper byte grouping,
/// the output will be padded by `=`.
///
/// Output always has 4n characters.
///
/// # Example
/// ```
/// let result = base64::encode("hello".as_bytes());
/// assert_eq!(result,"aGVsbG8=");
/// ````
/// If input is not base-ASCII characters, behavior is undefined.
/// # Example
/// ```
/// let result = base64::encode("🥐".as_bytes());
/// assert_ne!(result.len(), 4);
/// ````
pub fn encode(input: &[u8]) -> String {
    let alphabet: String =
        String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/");
    let mut result: String = String::from("");
    let mut amalgam: u32;
    let mut range: u32;
    for i in 0..input.len() {
        if i % 3 != 0 {
            continue;
        }
        amalgam = (input[i] as u32) << (2 * 8);
        range = 2;
        if i + 1 < input.len() {
            amalgam |= (input[i + 1] as u32) << 8;
            range = 3;
            if i + 2 < input.len() {
                amalgam |= input[i + 2] as u32;
                range = 4;
            }
        }
        for j in 0..range {
            let index = (amalgam >> (6 * (3 - j)) & 63) as usize;
            result.push(alphabet.chars().nth(index).unwrap());
        }
    }
    match input.len() % 3 {
        1 => result.push_str("=="),
        2 => result.push('='),
        _ => {}
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_padding() {
        let result = encode("abc".as_bytes());
        assert_eq!(result, "YWJj");
    }
    #[test]
    fn half_padding() {
        let result = encode("68".as_bytes());
        assert_eq!(result, "Njg=");
    }
    #[test]
    fn full_padding() {
        let result = encode("A".as_bytes());
        assert_eq!(result, "QQ==");
    }
    #[test]
    fn null_input() {
        let result = encode("".as_bytes());
        assert_eq!(result, "");
    }
    #[test]
    fn not_ascii() {
        let result = encode("🥐🥐🥐".as_bytes());
        assert_ne!(result.len(), 4);
    }
}
