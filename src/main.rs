use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs,
};
use walkdir::WalkDir;

fn main() {
    let path = r"C:\Users\Henri";
    let index = build_index(path);

    let query = "rust";
    if let Some(files) = index.get(&query.to_lowercase()) {
        println!("Gefunden in:");
        for file in files {
            println!(" - {}", file);
        }
    } else {
        print!("Nothing Found")
    }
}

fn build_index(path: &str) -> HashMap<String, HashSet<String>> {
    let mut index: HashMap<String, HashSet<String>> = HashMap::new();
    let word_re = Regex::new(r"\w+").unwrap();

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            if let Some(ext) = entry.path().extension() {
                if ext != "txt" && ext != "rs" && ext != "md" {
                    continue;
                }
            }
            if let Ok(text) = fs::read_to_string(entry.path()) {
                for word in word_re.find_iter(&text) {
                    let w = word.as_str().to_lowercase();
                    index
                        .entry(w)
                        .or_default()
                        .insert(entry.path().display().to_string());
                }
            }
        }
    }
    index
}
