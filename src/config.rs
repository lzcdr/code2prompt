use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub template: Option<String>,
    pub max_file_size: Option<u64>,
    pub clipboard_cmd: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            template: Some("--- {{path}} ---\n{{content}}\n\n".to_string()),
            max_file_size: Some(1_048_576), // 1 MB
            clipboard_cmd: None,
        }
    }
}

pub fn load_config(config_path: Option<PathBuf>) -> Config {
    let path = config_path.unwrap_or_else(|| {
        dirs::home_dir()
            .expect("Cannot find home directory")
            .join(".code2prompt")
            .join("config.toml")
    });
    if path.exists() {
        let content = std::fs::read_to_string(&path).expect("Failed to read config");
        toml::from_str(&content).unwrap_or_else(|e| {
            eprintln!("Warning: bad config ({e}), using defaults.");
            Config::default()
        })
    } else {
        Config::default()
    }
}
