use crate::context::Context;

pub enum Command {
    Add(Vec<String>),
    Remove(Vec<String>),
    Clear,
    List,
    Show,
    Copy,
    Stats,
    Help,
    Exit,
    Unknown(String),
}

pub fn parse_line(line: &str) -> Command {
    let mut parts = line.trim().split_whitespace();
    let cmd = parts.next().unwrap_or("").to_lowercase();
    let args: Vec<String> = parts.map(|s| s.to_string()).collect();
    match cmd.as_str() {
        "add" | "a" => Command::Add(args),
        "remove" | "rm" => Command::Remove(args),
        "clear" => Command::Clear,
        "list" | "ls" => Command::List,
        "show" => Command::Show,
        "copy" | "cp" => Command::Copy,
        "stats" | "stat" => Command::Stats,
        "help" | "h" => Command::Help,
        "exit" | "quit" | "q" => Command::Exit,
        _ => Command::Unknown(line.trim().to_string()),
    }
}

pub fn execute_command(ctx: &mut Context, cmd: Command) -> bool {
    match cmd {
        Command::Add(args) if args.is_empty() => eprintln!("Usage: add <path...>"),
        Command::Add(args) => ctx.add_files(&args),
        Command::Remove(args) if args.is_empty() => eprintln!("Usage: remove <path...>"),
        Command::Remove(args) => ctx.remove_files(&args),
        Command::Clear => ctx.clear(),
        Command::List => ctx.list(),
        Command::Show => ctx.show(),
        Command::Copy => ctx.copy_to_clipboard(),
        Command::Stats => ctx.stats(),
        Command::Help => print_help(),
        Command::Exit => return false,
        Command::Unknown(s) => eprintln!("Unknown command: '{s}'. Type 'help'."),
    }
    true
}

fn print_help() {
    println!(r#"Commands:
  add <path...>     - Add files / directories / globs (respects .c2pignore)
  remove <path...>  - Remove files from context
  clear             - Clear all files (asks confirmation unless --force)
  list (ls)         - Show files in context
  show              - Print context using template
  copy (cp)         - Copy context to clipboard
  stats             - Show summary (file count, total size)
  help (h)          - This help
  exit (quit/q)     - Exit"#);
}
