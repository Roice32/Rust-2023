use anyhow::Result;
use serde_derive::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use zip::read::ZipArchive;

#[derive(Debug, Deserialize)]
pub struct Article {
    id: String,
    text: String,
    title: String,
}

#[derive(Serialize)]
pub struct LongestItem {
    title: String,
    path: String,
    size: usize,
}

impl Default for LongestItem {
    fn default() -> Self {
        Self::new()
    }
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

pub struct WordsFrequencyMap {
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

impl Default for WordsFrequencyMap {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Serialize)]
pub struct WordFreq {
    word: String,
    appearances: u32,
}

pub fn write_stats_to_file(
    w_f: WordsFrequencyMap,
    l_w_f: WordsFrequencyMap,
    l_a: LongestItem,
    l_t: LongestItem,
) -> Result<()> {
    let stats_file_path = "dataset/stats.txt";
    // Needs proper variable handling
    if let Err(_e) = fs::remove_file(stats_file_path) {}
    let stats_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(stats_file_path)?;
    let stats_file_writer = RefCell::new(stats_file);

    let mut pairs_vec: Vec<WordFreq> = w_f
        .pairs
        .into_iter()
        .map(|(key, value)| WordFreq {
            word: key,
            appearances: value,
        })
        .collect();
    stats_file_writer
        .borrow_mut()
        .write_all("\tWords frequency (as written)\n".as_bytes())?;
    serde_json::to_writer_pretty(&mut *stats_file_writer.borrow_mut(), &pairs_vec)?;

    pairs_vec = l_w_f
        .pairs
        .into_iter()
        .map(|(key, value)| WordFreq {
            word: key,
            appearances: value,
        })
        .collect();
    stats_file_writer
        .borrow_mut()
        .write_all("\tWords frequency (lowercase)\n".as_bytes())?;
    serde_json::to_writer_pretty(&mut *stats_file_writer.borrow_mut(), &pairs_vec)?;

    stats_file_writer
        .borrow_mut()
        .write_all("\tLongest article\n".as_bytes())?;
    serde_json::to_writer_pretty(&mut *stats_file_writer.borrow_mut(), &l_a)?;

    stats_file_writer
        .borrow_mut()
        .write_all("\tLongest title\n".as_bytes())?;
    serde_json::to_writer_pretty(&mut *stats_file_writer.borrow_mut(), &l_t)?;

    Ok(())
}

fn main() -> Result<()> {
    let dataset: &str = "dataset/test.zip";
    let file = fs::File::open(dataset)?;
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

    write_stats_to_file(
        words_freq,
        lowercase_words_freq,
        longest_article,
        longest_title,
    )?;

    Ok(())
}
