use dashmap::DashMap;
use rayon::prelude::*;
use std::{fs, path::PathBuf, sync::Arc};
use walkdir::WalkDir;

fn main() {
    let path = r"C:\Users\Henri\videos";
    let index = build_index(path);

    let query = "Microsoft";
    if let Some(files) = index.get(&query.to_lowercase()) {
        println!("Gefunden in {} Dateien:", files.len());
        for file in files.iter() {
            println!(" - {}", file);
        }
    }
}

fn build_index(path: &str) -> DashMap<String, Vec<Arc<String>>> {
    let index: DashMap<String, Vec<Arc<String>>> = DashMap::new();

    let path_pool: DashMap<String, Arc<String>> = DashMap::new();

    let files: Vec<PathBuf> = WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_type().is_file()
                && e.path()
                    .extension()
                    .and_then(|s| s.to_str())
                    .map(|ext| matches!(ext, "txt" | "rs" | "md"))
                    .unwrap_or(false)
        })
        .map(|e| e.path().to_path_buf())
        .collect();

    files.par_iter().with_min_len(4).for_each(|file_path| {
        if let Ok(metadata) = fs::metadata(file_path) {
            if metadata.len() > 10_000_000 {
                return;
            }
        }

        if let Ok(content) = fs::read_to_string(file_path) {
            if content.len() > 5_000_000 {
                return;
            }

            let content_lower = content.to_lowercase();
            let bytes = content_lower.as_bytes();

            let path_str = file_path.display().to_string();
            let path_arc = path_pool
                .entry(path_str.clone())
                .or_insert_with(|| Arc::new(path_str))
                .clone();

            let mut seen_words = std::collections::HashSet::new();
            let mut i = 0;

            while i < bytes.len() {
                if bytes[i].is_ascii_alphanumeric() {
                    let start = i;
                    while i < bytes.len() && bytes[i].is_ascii_alphanumeric() {
                        i += 1;
                    }

                    let word = &content_lower[start..i];
                    if word.len() >= 3 && word.len() <= 50 {
                        seen_words.insert(word.to_string());
                    }
                } else {
                    i += 1;
                }
            }

            for word in seen_words {
                index
                    .entry(word)
                    .or_insert_with(Vec::new)
                    .push(Arc::clone(&path_arc));
            }
        }
    });

    index
}
