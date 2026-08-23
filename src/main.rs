mod clipboard;
mod commands;
mod config;
mod context;
mod file_utils;

use rustyline::Editor;
use rustyline::Helper;
use rustyline::completion::{Completer, FilenameCompleter};
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::{ValidationContext, ValidationResult, Validator};
use std::borrow::Cow;
use std::path::PathBuf;

#[derive(Default)]
struct MyHelper {
    completer: FilenameCompleter,
}

impl Completer for MyHelper {
    type Candidate = <FilenameCompleter as Completer>::Candidate;
    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &rustyline::Context<'_>,
    ) -> Result<(usize, Vec<Self::Candidate>), ReadlineError> {
        self.completer.complete(line, pos, ctx)
    }
}

impl Hinter for MyHelper {
    type Hint = String;
    fn hint(&self, _line: &str, _pos: usize, _ctx: &rustyline::Context<'_>) -> Option<Self::Hint> {
        None
    }
}

impl Highlighter for MyHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        _default: bool,
    ) -> Cow<'b, str> {
        Cow::Borrowed(prompt)
    }
}

impl Validator for MyHelper {
    fn validate(&self, _ctx: &mut ValidationContext) -> Result<ValidationResult, ReadlineError> {
        Ok(ValidationResult::Valid(None))
    }
}

impl Helper for MyHelper {}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut force = false;
    let mut config_path = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--force" | "-f" => force = true,
            "--config" if i + 1 < args.len() => {
                i += 1;
                config_path = Some(PathBuf::from(&args[i]));
            }
            _ => {}
        }
        i += 1;
    }

    let config = config::load_config(config_path);
    let mut ctx = context::Context::new(config, force);

    let mut rl = Editor::new().expect("Failed to create editor");
    rl.set_helper(Some(MyHelper::default()));

    let history_path = dirs::home_dir()
        .expect("no home")
        .join(".code2prompt")
        .join("history.txt");
    if history_path.exists() {
        let _ = rl.load_history(&history_path);
    }

    println!("code2prompt — type 'help' for commands, 'exit' to quit.");
    loop {
        match rl.readline("ctx> ") {
            Ok(line) => {
                let line = line.trim().to_string();
                if line.is_empty() {
                    continue;
                }
                let _ = rl.add_history_entry(&line);
                let cmd = commands::parse_line(&line);
                if !commands::execute_command(&mut ctx, cmd) {
                    break;
                }
            }
            Err(ReadlineError::Interrupted) => println!("^C"),
            Err(ReadlineError::Eof) => {
                println!("exit");
                break;
            }
            Err(e) => {
                eprintln!("Readline error: {e}");
                break;
            }
        }
    }

    if let Some(parent) = history_path.parent() {
        let _ = std::fs::create_dir_all(parent);
        let _ = rl.save_history(&history_path);
    }
    println!("Goodbye.");
}
