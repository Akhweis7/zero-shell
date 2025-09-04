//tala
use std::io::{self, Write};

pub fn run(args: &[String]) {
    // join all args by a single space
    let mut out = io::stdout();
    let line = args.join(" ");
    // let _ = writeln!(out, "{}", line); 
    let _ = writeln!(out, "\x1b[36m{}\x1b[0m", line); // Cyan for general output
}
