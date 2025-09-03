//tala
use std::io::{self, Write};
use std::process;
mod builtins;
mod shell;

fn main() {
    clear_screen();
    let mut sh = shell::Shell::new();
    if let Err(err) = sh.run() {
        eprintln!("{}", err);
        process::exit(1);
    }
}

fn clear_screen() {
    // ESC[2J = clear screen, ESC[H = move cursor to home
    print!("\x1b[2J\x1b[H");
    let _ = io::stdout().flush();
}
