use anyhow::Result;
use std::{collections::HashMap, fs};

fn main() -> Result<()> {
    let text: String = fs::read_to_string("src/text.txt")?;
    let mut hash_map: HashMap<String, u32> = HashMap::new();

    for word in
        text.split(|c: char| -> bool { c.is_ascii_whitespace() || c.is_ascii_punctuation() })
    {
        if word.len() == 0 {
            continue;
        }
        hash_map
            .entry(word.to_lowercase())
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let mut result_vec: Vec<(String, u32)>;
    result_vec = hash_map.into_iter().collect();
    result_vec.sort_unstable_by_key(|count| count.1);
    result_vec.reverse();

    let max_len: usize = result_vec.iter().map(|x| x.0.len()).max().unwrap_or(0);

    for pair in result_vec {
        println!("{:<max_len$} => {}", pair.0, pair.1);
    }
    Ok(())
}
