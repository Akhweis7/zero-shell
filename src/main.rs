//tala
use std::io::{ self, Write };
use std::process;
use builtins::clear;
use builtins::tnanm;
mod builtins;
mod shell;

fn main() {
    builtins::clear::clear();
    builtins::tnanm::z_shell();
    let mut sh = shell::Shell::new();
    if let Err(err) = sh.run() {
        eprintln!("{}", err);
        process::exit(1);
    }
}
