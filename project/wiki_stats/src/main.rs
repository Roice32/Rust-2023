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

fn main() -> Result<()> {
    let dataset: &str = "dataset/test.zip";
    let file = File::open(dataset)?;
    let mut archive = ZipArchive::new(file)?;

    let filename_prefix: &str = "folder/";
    for index in 0..archive.len() {
        let mut data_file = archive.by_index(index)?;
        if data_file.name().starts_with(filename_prefix) && data_file.name().ends_with(".json") {
            let mut data = String::new();
            data_file.read_to_string(&mut data)?;
            let articles_vec: Vec<Article> = serde_json::from_str(&data)?;
            for art in articles_vec {
                println!(
                    "Id: {} | Title: {} | Text: {}...",
                    art.id,
                    art.title,
                    art.text.chars().next().unwrap()
                );
            }
        }
    }

    Ok(())
}
