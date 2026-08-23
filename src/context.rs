use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use crate::config::Config;

pub struct FileEntry {
    pub path: PathBuf,
    pub content: String,
    pub size: usize,
}

pub struct Context {
    files: HashMap<PathBuf, FileEntry>,
    pub force: bool,
    config: Config,
}

impl Context {
    pub fn new(config: Config, force: bool) -> Self {
        Context { files: HashMap::new(), force, config }
    }

    pub fn add_files(&mut self, raw_paths: &[String]) {
        let paths = crate::file_utils::expand_paths(raw_paths);
        for abs_path in paths {
            let metadata = match std::fs::metadata(&abs_path) {
                Ok(m) => m,
                Err(e) => { eprintln!("Error reading {}: {e}", abs_path.display()); continue; }
            };
            if let Some(max) = self.config.max_file_size {
                if metadata.len() > max {
                    eprintln!("Skipping {} (size {} > max {})", abs_path.display(), metadata.len(), max);
                    continue;
                }
            }
            let mut content = String::new();
            if let Err(e) = std::fs::File::open(&abs_path).and_then(|mut f| f.read_to_string(&mut content)) {
                eprintln!("Error reading {}: {e}", abs_path.display());
                continue;
            }
            let size = content.len();
            let entry = FileEntry { path: abs_path.clone(), content, size };
            if self.files.insert(abs_path.clone(), entry).is_some() {
                println!("Updated {}", abs_path.display());
            } else {
                println!("Added {} ({} bytes)", abs_path.display(), size);
            }
        }
    }

    pub fn remove_files(&mut self, patterns: &[String]) {
        for pat in patterns {
            let path = Path::new(pat);
            let canonical = path.canonicalize().ok();
            let mut removed = false;
            if let Some(abs) = canonical {
                if self.files.remove(&abs).is_some() {
                    println!("Removed {}", abs.display());
                    removed = true;
                }
            }
            if !removed {
                let mut keys_to_remove = Vec::new();
                for key in self.files.keys() {
                    if key.ends_with(pat) || key.to_string_lossy().contains(pat) {
                        keys_to_remove.push(key.clone());
                    }
                }
                for key in keys_to_remove {
                    if let Some(entry) = self.files.remove(&key) {
                        println!("Removed {}", entry.path.display());
                        removed = true;
                    }
                }
            }
            if !removed {
                eprintln!("No file matching '{pat}' found in context.");
            }
        }
    }

    pub fn clear(&mut self) {
        if !self.force {
            print!("Are you sure you want to clear all files? (y/N) ");
            io::stdout().flush().unwrap();
            let mut answer = String::new();
            io::stdin().read_line(&mut answer).unwrap();
            if !["y", "yes"].contains(&answer.trim().to_lowercase().as_str()) {
                println!("Clear cancelled.");
                return;
            }
        }
        self.files.clear();
        println!("Context cleared.");
    }

    pub fn list(&self) {
        if self.files.is_empty() {
            println!("Context is empty.");
            return;
        }
        let mut entries: Vec<&FileEntry> = self.files.values().collect();
        entries.sort_by_key(|e| &e.path);
        println!("{0: <5} {1: <60} {2: >10}", "#", "File", "Size");
        for (i, entry) in entries.iter().enumerate() {
            let disp = entry.path.display().to_string();
            let truncated = if disp.len() > 60 {
                format!("...{}", &disp[disp.len()-57..])
            } else { disp };
            println!("{0: <5} {1: <60} {2: >10}", i+1, truncated, format_size(entry.size));
        }
    }

    pub fn build_output(&self) -> String {
        let template = self.config.template.as_deref().unwrap_or("--- {{path}} ---\n{{content}}\n\n");
        let mut entries: Vec<&FileEntry> = self.files.values().collect();
        entries.sort_by_key(|e| &e.path);
        let mut out = String::new();
        for entry in entries {
            out.push_str(&template.replace("{{path}}", &entry.path.display().to_string())
                                .replace("{{content}}", &entry.content));
        }
        out
    }

    pub fn show(&self) {
        println!("{}", self.build_output());
    }

    pub fn stats(&self) {
        let count = self.files.len();
        let total: usize = self.files.values().map(|e| e.size).sum();
        println!("Files: {count}, Total size: {}", format_size(total));
    }

    pub fn copy_to_clipboard(&self) {
        let output = self.build_output();
        if output.is_empty() {
            println!("Context is empty.");
            return;
        }
        match crate::clipboard::copy_to_clipboard(&output, &self.config) {
            Ok(()) => println!("Context copied to clipboard."),
            Err(e) => eprintln!("Clipboard error: {e}"),
        }
    }
}

fn format_size(bytes: usize) -> String {
    if bytes >= 1_048_576 {
        format!("{:.1} MB", bytes as f64 / 1_048_576.0)
    } else if bytes >= 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{bytes} B")
    }
}
