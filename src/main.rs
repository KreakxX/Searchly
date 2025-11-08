use dashmap::DashMap;
use rayon::prelude::*;
use regex::Regex;
use std::io::{BufRead, BufReader};
use std::{
    collections::{HashMap, HashSet},
    fs::{self, File},
};
use walkdir::WalkDir;

fn main() {
    let path = r"C:\Users\Henri\videos";
    let index = build_index(path);

    let query = "Microsoft";
    if let Some(files) = index.get(&query.to_lowercase()) {
        println!("Gefunden in:");
        for file in files.iter() {
            println!(" - {}", file);
        }
    } else {
        print!("Nothing Found")
    }
}

fn build_index(path: &str) -> DashMap<String, Vec<String>> {
    let mut index: DashMap<String, Vec<String>> = DashMap::new();
    let word_re = Regex::new(r"\w+").unwrap();

    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .par_bridge()
        .for_each(|entry| {
            if entry.file_type().is_file() {
                if let Some(ext) = entry.path().extension() {
                    if ext != "txt" && ext != "rs" && ext != "md" {
                        return;
                    }
                }
                if let Ok(text) = fs::read_to_string(entry.path()) {
                    let content = text.as_str().to_lowercase();
                    let bytes = content.as_bytes();
                    let mut i = 0;

                    while i < bytes.len() {
                        if bytes[i].is_ascii_alphanumeric() {
                            let start = i;
                            while i < bytes.len() && bytes[i].is_ascii_alphanumeric() {
                                i += 1;
                            }

                            let word = &content[start..i];
                            if word.len() >= 2 {
                                index
                                    .entry(word.to_string())
                                    .or_insert_with(Vec::new)
                                    .push(entry.path().display().to_string());
                            }
                        } else {
                            i += 1;
                        }
                    }
                }
            }
        });

    index
}
