use anyhow::{Context, Result};
use serde_derive::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::{ErrorKind, Read, Write};
use std::thread;
use zip::read::ZipArchive;

#[derive(Deserialize)]
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
        for word in s.split(|c: char| -> bool {
            c.is_ascii_whitespace() || (c != '\'' && c.is_ascii_punctuation())
        }) {
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

pub fn write_stats_to_file(stats: StatsPackage) -> Result<()> {
    let stats_file_path = "dataset/stats.txt";
    match fs::remove_file(stats_file_path) {
        Ok(()) => {}
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {}
            _ => {
                return Err(e.into());
            }
        },
    }

    let stats_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(stats_file_path)
        .context("Failed to create output file")?;
    let stats_file_writer = RefCell::new(stats_file);

    let mut pairs_vec: Vec<WordFreq> = stats
        .words_freq
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

    pairs_vec = stats
        .low_words_freq
        .pairs
        .into_iter()
        .map(|(key, value)| WordFreq {
            word: key,
            appearances: value,
        })
        .collect();
    stats_file_writer
        .borrow_mut()
        .write_all("\n\tWords frequency (lowercase)\n".as_bytes())?;
    serde_json::to_writer_pretty(&mut *stats_file_writer.borrow_mut(), &pairs_vec)?;

    stats_file_writer
        .borrow_mut()
        .write_all("\n\tLongest article\n".as_bytes())?;
    serde_json::to_writer_pretty(&mut *stats_file_writer.borrow_mut(), &stats.long_art)?;

    stats_file_writer
        .borrow_mut()
        .write_all("\n\tLongest title\n".as_bytes())?;
    serde_json::to_writer_pretty(&mut *stats_file_writer.borrow_mut(), &stats.long_title)?;

    Ok(())
}

pub struct StatsPackage {
    words_freq: WordsFrequencyMap,
    low_words_freq: WordsFrequencyMap,
    long_art: LongestItem,
    long_title: LongestItem,
}

impl Default for StatsPackage {
    fn default() -> Self {
        Self::new()
    }
}

impl StatsPackage {
    pub fn new() -> Self {
        Self {
            words_freq: WordsFrequencyMap::new(),
            low_words_freq: WordsFrequencyMap::new(),
            long_art: LongestItem::new(),
            long_title: LongestItem::new(),
        }
    }

    pub fn merge_with(&mut self, other: Self) {
        for (key, value) in other.words_freq.pairs {
            self.words_freq
                .pairs
                .entry(key)
                .and_modify(|count| *count += value)
                .or_insert(value);
        }
        for (key, value) in other.low_words_freq.pairs {
            self.low_words_freq
                .pairs
                .entry(key)
                .and_modify(|count| *count += value)
                .or_insert(value);
        }
        if other.long_art.size > self.long_art.size {
            self.long_art = other.long_art;
        }
        if other.long_title.size > self.long_title.size {
            self.long_title = other.long_title;
        }
    }
}

pub fn process_file(data: String, path: String) -> Result<StatsPackage> {
    let mut stats = StatsPackage::new();
    let articles_vec: Vec<Article> = serde_json::from_str(&data)?;
    for art in articles_vec {
        WordsFrequencyMap::map_article(&mut stats.words_freq, &mut stats.low_words_freq, &art);
        if art.text.len() > stats.long_art.size {
            stats.long_art = LongestItem::new_longest_article(&art, &path);
        }
        if art.title.len() > stats.long_title.size {
            stats.long_title = LongestItem::new_longest_title(&art, &path);
        }
    }
    Ok(stats)
}

fn main() -> Result<()> {
    let dataset: &str = "dataset/test_dataset.zip";
    let file = fs::File::open(dataset)?;
    let mut archive = ZipArchive::new(file)?;

    let mut workers_handles: Vec<thread::JoinHandle<Result<StatsPackage, anyhow::Error>>> =
        Vec::new();

    let filename_prefix: &str = "folder/"; // maybe unneeded?
    for index in 0..archive.len() {
        let mut data_file = archive.by_index(index)?;
        if data_file.name().starts_with(filename_prefix) && data_file.name().ends_with(".json") {
            let mut data = String::new();
            data_file.read_to_string(&mut data)?;
            let file_name = data_file.name().to_string();
            let handle = thread::spawn(|| process_file(data, file_name));
            workers_handles.push(handle);
        }
    }

    let mut complete_stats = StatsPackage::new();
    for worker in workers_handles {
        match worker.join() {
            Ok(worker_stats) => match worker_stats {
                Ok(w_s) => complete_stats.merge_with(w_s),
                Err(e) => {
                    println!("Worker thread couldn't process data about a file: {:?}", e);
                    std::process::exit(1);
                }
            },
            Err(e) => {
                println!(
                    "There was an error receiving data from a worker thread: {:?}",
                    e
                );
                std::process::exit(1);
            }
        };
    }

    match write_stats_to_file(complete_stats) {
        Ok(()) => {
            println!("Successfully written stats to output file.")
        }
        Err(e) => {
            println!("An error occured while writing output: {:?}", e);
        }
    }

    Ok(())
}
