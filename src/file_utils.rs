use ignore::WalkBuilder;
use std::path::{Path, PathBuf};

pub fn expand_paths(inputs: &[String]) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for input in inputs {
        let path = Path::new(input);
        if path.is_file() {
            if let Ok(abs) = path.canonicalize() {
                files.push(abs);
            }
            continue;
        }
        if path.is_dir() {
            let walker = WalkBuilder::new(path)
                .add_custom_ignore_filename(".c2pignore")
                .hidden(false)
                .build();
            for result in walker {
                if let Ok(entry) = result {
                    if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
                        if let Ok(abs) = entry.path().canonicalize() {
                            files.push(abs);
                        }
                    }
                }
            }
            continue;
        }
        // Glob pattern
        if let Ok(pattern) = glob::Pattern::new(input) {
            let walker = WalkBuilder::new(".")
                .add_custom_ignore_filename(".c2pignore")
                .hidden(false)
                .build();
            for result in walker {
                if let Ok(entry) = result {
                    if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
                        let relative = entry.path().strip_prefix(".").unwrap_or(entry.path());
                        if pattern.matches_path(relative) {
                            if let Ok(abs) = entry.path().canonicalize() {
                                files.push(abs);
                            }
                        }
                    }
                }
            }
        }
    }
    files.sort();
    files.dedup();
    files
}
