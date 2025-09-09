//tala
// use std::io::{ self, Write };
use std::process;
// use builtins::clear;
// use builtins::tnanm;
mod builtins;
mod shell;
mod error;
// mod util;

fn main() {
    builtins::clear::clear();
    builtins::tnanm::z_shell();
    let mut sh = shell::Shell::new();
    if let Err(err) = sh.run() {
        eprintln!("\x1b[38;5;196m{}\x1b[0m", err);
        process::exit(1);
    }
}
