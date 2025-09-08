//tala
use std::io::{ self, Write };
use std::process;
use builtins::clear;
use builtins::tnanm;
mod builtins;
mod shell;
mod error;
mod util { pub mod path; pub mod perms; pub mod timefmt; }

fn main() {
    builtins::clear::clear();
    builtins::tnanm::z_shell();
    let mut sh = shell::Shell::new();
    if let Err(err) = sh.run() {
        eprintln!("{}", err);
        process::exit(1);
    }
}
