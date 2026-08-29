# code2prompt

**code2prompt** is an interactive command-line tool that collects the contents of selected files into a single text buffer (context) and copies it to the clipboard. It is designed to help you quickly assemble source code or other text files for pasting into language models (like ChatGPT, Claude), code review tools, or anywhere else you need a bundle of files.

## Features

- **Interactive REPL** with filename autocompletion and command history.
- Add files, directories, or glob patterns to the context.
- Respects a `.c2pignore` file (similar to `.gitignore`) when walking directories.
- Configurable output template (default: `--- {{path}} ---\n{{content}}\n\n`).
- Configurable maximum file size (default: 1 MB).
- Copy the generated context to the system clipboard.
- Cross‑platform clipboard support:
  - Windows: `clip`
  - macOS: `pbcopy`
  - Linux: `xclip` or `wl-copy` (auto‑detected)
- Simple and lightweight, written in Rust.

## Installation

```bash
git clone https://github.com/yourusername/code2prompt.git
cd code2prompt
cargo install --path .
```

Make sure you have [Rust](https://www.rust-lang.org/) installed.

## Usage

Run the tool:

```bash
code2prompt
```

You will enter an interactive session with the prompt `ctx>`. Type `help` to see all available commands.

### Commands

| Command              | Description                                                                 |
|----------------------|-----------------------------------------------------------------------------|
| `add <path...>`      | Add files, directories, or glob patterns to the context.                    |
| `remove <path...>`   | Remove files from the context by path or substring match.                   |
| `clear`              | Remove all files from the context (asks for confirmation unless `--force`). |
| `list` / `ls`        | Show all files currently in the context with sizes.                         |
| `show`               | Print the generated context using the configured template.                  |
| `copy` / `cp`        | Copy the generated context to the clipboard.                                |
| `stats`              | Display number of files and total size.                                     |
| `help` / `h`         | Show help message.                                                          |
| `exit` / `quit` / `q`| Exit the program.                                                           |

### Examples

```text
ctx> add src/main.rs
Added /home/user/project/src/main.rs (1024 bytes)

ctx> add src/
Added 15 files from directory

ctx> add "*.rs"
Added all .rs files in current directory (recursively)

ctx> list
#   File                                                          Size
1   /home/user/project/src/main.rs                               1.0 KB
2   /home/user/project/src/lib.rs                                2.3 KB

ctx> show
--- /home/user/project/src/main.rs ---
fn main() { ... }

--- /home/user/project/src/lib.rs ---
pub fn helper() { ... }

ctx> copy
Context copied to clipboard.
```

## Configuration

code2prompt looks for a configuration file at `~/.code2prompt/config.toml`. If the file does not exist, default settings are used. You can also specify a custom config path with `--config <path>`.

### Available options

```toml
# Template for each file in the generated output.
# Placeholders: {{path}} and {{content}}
template = "--- {{path}} ---\n{{content}}\n\n"

# Maximum file size in bytes (files larger than this are skipped).
max_file_size = 1048576

# Override the clipboard command (useful for custom tools).
# If not set, the default OS command is used.
# Example for Linux: "xsel --clipboard --input"
clipboard_cmd = "xsel --clipboard --input"
```

### Ignoring files

When adding a directory or using glob patterns, code2prompt respects a file named `.c2pignore` in the walked directory. Its syntax is the same as `.gitignore` (patterns, comments with `#`, negation with `!`). This file is automatically detected by the underlying `ignore` crate.

## Command-line options

```
--force, -f          Skip confirmation prompts (e.g., for clear command)
--config <path>      Use a custom configuration file
```

## How it works

1. Files are read into memory and stored in a `HashMap` keyed by canonical path.
2. When generating output, files are sorted by path and rendered using the configured template.
3. The clipboard command is executed via the system shell (`cmd /C` on Windows, `sh -c` otherwise) with the generated text piped to its stdin.

## License

This project is licensed under the MIT License – see the [LICENSE](LICENSE) file for details.
