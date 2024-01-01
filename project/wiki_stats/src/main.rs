use anyhow::Result;
use serde_derive::Deserialize;
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

fn main() -> Result<()> {
    let dataset: &str = "dataset/test.zip";
    let file = File::open(dataset)?;
    let mut archive = ZipArchive::new(file)?;

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
                if art.text.len() > longest_article.size {
                    longest_article = LongestItem::new_longest_article(&art, data_file.name());
                }
                if art.title.len() > longest_title.size {
                    longest_title = LongestItem::new_longest_title(&art, data_file.name());
                }
            }
        }
    }

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
