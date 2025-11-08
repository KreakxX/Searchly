use regex::Regex;
use std::io::{BufRead, BufReader};
use std::{
    collections::{HashMap, HashSet},
    fs::{self, File},
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
            if let Ok(file) = File::open(entry.path()) {
                let reader = BufReader::new(file);
                for line_result in reader.lines() {
                    if let Ok(line_text) = line_result {
                        for word in word_re.find_iter(&line_text) {
                            let w = word.as_str().to_lowercase();
                            index
                                .entry(w)
                                .or_default()
                                .insert(entry.path().display().to_string());
                        }
                    }
                }
            }
        }
    }
    index
}
