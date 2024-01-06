use anyhow::{Context, Result};
use clap::Parser;
use serde_derive::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::{ErrorKind, Read, Write};
use std::thread;
use std::time::Instant;
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

pub fn write_stats_to_file(stats: StatsPackage, stats_file_path: &str) -> Result<()> {
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

pub fn write_stats_to_file_plain(stats: StatsPackage, stats_file_path: &str) -> Result<()> {
    match fs::remove_file(stats_file_path) {
        Ok(()) => {}
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {}
            _ => {
                return Err(e.into());
            }
        },
    }

    let mut stats_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(stats_file_path)
        .context("Failed to create output file")?;

    writeln!(stats_file, "\tWords frequency (as written)\n")?;
    for (word, count) in stats.words_freq.pairs {
        writeln!(stats_file, "{}: {}", word, count)?;
    }

    writeln!(stats_file, "\tWords frequency (lowercase)\n")?;
    for (word, count) in stats.low_words_freq.pairs {
        writeln!(stats_file, "{}: {}", word, count)?;
    }

    writeln!(stats_file, "\tLongest article\n")?;
    writeln!(
        stats_file,
        "Title: {}\nPath: {}\nSize: {}",
        stats.long_art.title, stats.long_art.path, stats.long_art.size
    )?;

    writeln!(stats_file, "\tLongest title\n")?;
    writeln!(
        stats_file,
        "Title: {}\nPath: {}\nSize: {}",
        stats.long_title.title, stats.long_title.path, stats.long_title.size
    )?;

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

pub fn info_print() {
    println!("\tName: {}", env!("CARGO_PKG_NAME"));
    println!("\tVersion: {}", env!("CARGO_PKG_VERSION"));
    println!("\tDescription: Tool for analyzing article datasets stored as .JSON files within a .zip archive.
              \nUses multithreaded techniques to iterate through each file, calculating each word's number of appearences (as-written & lowercased), as well as info about the longest article & title.
              \nMaximum number of parallel threads running = number of virtual threads on the CPU (in your case: {}).", num_cpus::get());
    println!("\tAvailable command-line arguments:");
    println!("1. --aide -a: Displays this info about the program. French for \"help\" 'cause \"help\" gets into conflict with cargo's own \"--help\"...");
    println!("2. --input -i [source_file.zip]: Specifies the file containing the dataset to be analyzed. Must be a .zip archive. Default: \'datasets\\dataset.zip\'.");
    println!("3. --output -o [output_file.txt]: Specifies the file where computed stats will be written. Must be a .txt file. Default: \'stats.txt\'.");
    println!("4. --metrics -m: During execution will print:
              \n\t> partial progress: files processed / total files* + precentage (*all files counted, even if not .JSON)
              \n\t> total time elapsed during file processing: secs & milisecs, #files processed, their total compressed size
              \n\t> total time elapsed during output writing: secs & milisecs");
    println!("5. --plain -p: Computed stats will be written to file as plain text tuples, not formatted as JSONs (faster write time).")
}

#[derive(Parser)]
#[command(version, about = "Parsing needed arguments")]
struct Arguments {
    #[arg(long, short)]
    input: Option<String>,

    #[arg(long, short)]
    output: Option<String>,

    #[arg(long, short)]
    aide: bool,

    #[arg(long, short)]
    metrics: bool,

    #[arg(long, short)]
    plain: bool,
}

fn main() -> Result<()> {
    let args = Arguments::parse();
    if args.aide {
        info_print();
        return Ok(());
    }

    let dataset: &str = match &args.input {
        Some(s) if s.ends_with(".zip") => s,
        _ => "datasets/dataset.zip",
    };

    let file = fs::File::open(dataset)?;
    let mut archive = ZipArchive::new(file)?;

    let mut workers_handles = vec![];
    let mut workers_slice = vec![];

    let mut complete_stats = StatsPackage::new();
    let no_files = archive.len();

    let mut start_time = Instant::now();
    for index in 0..no_files {
        let mut data_file = archive.by_index(index)?;
        if data_file.name().ends_with(".json") {
            let mut data = String::new();
            data_file.read_to_string(&mut data)?;
            let file_name = data_file.name().to_string();
            let thread_handle = thread::spawn(|| process_file(data, file_name));
            workers_handles.push(thread_handle);
        }
        if workers_handles.len() == num_cpus::get() || index == no_files - 1 {
            workers_slice.append(&mut workers_handles);
            for worker in workers_slice.drain(..) {
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
            workers_handles.clear();
            if args.metrics {
                println!(
                    "Processed {}/{} files ({:.2}% done).",
                    index + 1,
                    no_files,
                    (index + 1) as f32 / no_files as f32 * 100.0
                )
            }
        }
    }

    if args.metrics {
        let time_passed = start_time.elapsed();
        let file_metadata = fs::metadata(dataset)?;
        println!(
            "It took ~{}s {}ms to process all {} files (~{} bytes compressed size).",
            time_passed.as_secs(),
            time_passed.subsec_millis(),
            archive.len(),
            file_metadata.len()
        );
        println!("Now printing to output file.");
    }

    let output: &str = match &args.output {
        Some(s) if s.ends_with(".txt") => s,
        _ => "stats.txt",
    };

    if args.metrics {
        start_time = Instant::now();
    }

    if args.plain {
        match write_stats_to_file_plain(complete_stats, output) {
            Ok(()) => {
                println!("Successfully written stats to output file.")
            }
            Err(e) => {
                println!("An error occured while writing output: {:?}", e);
            }
        }
    } else {
        match write_stats_to_file(complete_stats, output) {
            Ok(()) => {
                println!("Successfully written stats to output file.")
            }
            Err(e) => {
                println!("An error occured while writing output: {:?}", e);
            }
        }
    }

    if args.metrics {
        let time_passed = start_time.elapsed();
        println!(
            "It took ~{}s {}ms to print all stats to output file.",
            time_passed.as_secs(),
            time_passed.subsec_millis()
        );
    }
    println!("Ok bye.");
    Ok(())
}
