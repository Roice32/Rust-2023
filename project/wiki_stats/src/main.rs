use anyhow::Result;
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::{fs::File, io::Read};
use zip::read::ZipArchive;

#[derive(Debug, Deserialize)]
struct Article {
    id: String,
    text: String,
    title: String,
}

struct LongestItem {
    title: String,
    path: String,
    size: usize,
}

impl LongestItem {
    pub fn new() -> Self {
        Self {
            title: String::from(""),
            path: String::from(""),
            size: 0,
        }
    }
    pub fn new_longest_article(a: &Article, p: &str) -> Self {
        let mut full_path: String = p.clone().to_string();
        full_path.push('/');
        full_path.push_str(a.id.as_str());
        Self {
            title: a.title.clone(),
            path: full_path,
            size: a.text.len(),
        }
    }
    pub fn new_longest_title(a: &Article, p: &str) -> Self {
        let mut full_path: String = p.clone().to_string();
        full_path.push('/');
        full_path.push_str(a.id.as_str());
        Self {
            title: a.title.clone(),
            path: full_path,
            size: a.title.len(),
        }
    }
}

struct WordsFrequencyMap {
    pairs: HashMap<String, u32>,
}

impl WordsFrequencyMap {
    pub fn new() -> Self {
        Self {
            pairs: HashMap::new(),
        }
    }
    pub fn map_words(normal: &mut Self, lowercase: &mut Self, s: &str) {
        for word in
            s.split(|c: char| -> bool { c.is_ascii_whitespace() || c.is_ascii_punctuation() })
        {
            normal
                .pairs
                .entry(word.clone().to_string())
                .and_modify(|count| *count += 1)
                .or_insert(1);
            lowercase
                .pairs
                .entry(word.clone().to_ascii_lowercase().to_string())
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
    }
    pub fn map_article(normal: &mut Self, lowercase: &mut Self, a: &Article) {
        WordsFrequencyMap::map_words(normal, lowercase, &a.title);
        WordsFrequencyMap::map_words(normal, lowercase, &a.text);
    }
}

fn main() -> Result<()> {
    let dataset: &str = "dataset/test.zip";
    let file = File::open(dataset)?;
    let mut archive = ZipArchive::new(file)?;

    let mut words_freq = WordsFrequencyMap::new();
    let mut lowercase_words_freq = WordsFrequencyMap::new();

    let mut longest_article = LongestItem::new();
    let mut longest_title = LongestItem::new();

    let filename_prefix: &str = "folder/";
    for index in 0..archive.len() {
        let mut data_file = archive.by_index(index)?;
        if data_file.name().starts_with(filename_prefix) && data_file.name().ends_with(".json") {
            let mut data = String::new();
            data_file.read_to_string(&mut data)?;
            let articles_vec: Vec<Article> = serde_json::from_str(&data)?;
            for art in articles_vec {
                WordsFrequencyMap::map_article(&mut words_freq, &mut lowercase_words_freq, &art);
                if art.text.len() > longest_article.size {
                    longest_article = LongestItem::new_longest_article(&art, data_file.name());
                }
                if art.title.len() > longest_title.size {
                    longest_title = LongestItem::new_longest_title(&art, data_file.name());
                }
            }
        }
    }

    // Only for debugging. Non-final.
    let mut count = 0;
    for (key, value) in &words_freq.pairs {
        print!("({},{})", key, value);
        count += 1;
        if count == 9 {
            break;
        }
    }
    println!();
    count = 0;
    for (key, value) in &lowercase_words_freq.pairs {
        print!("({},{})", key, value);
        count += 1;
        if count == 9 {
            break;
        }
    } //
    println!();

    println!(
        "Longest article: Title: {} | Path: {} | Size: {} bytes.",
        longest_article.title, longest_article.path, longest_article.size
    );
    println!(
        "Longest title: Title: {} | Path: {} | Size: {} bytes.",
        longest_title.title, longest_title.path, longest_title.size
    );

    Ok(())
}
