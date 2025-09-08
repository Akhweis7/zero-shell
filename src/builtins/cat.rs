// cat builtin command
use std::fs::File;
use std::io::{self, Read, Write};
// use crate::util::path as pathutil;

pub fn cat(args: &Vec<String>) -> io::Result<()> {
    if args.is_empty() {
        // Read from stdin and write to stdout
        let mut stdin = io::stdin();
        let mut stdout = io::stdout();
        let mut buffer = [0u8; 8192];
        loop {
            let n = stdin.read(&mut buffer)?;
            if n == 0 { break; }
            stdout.write_all(&buffer[..n])?;
        }
        return Ok(());
    }

    for arg in args {
        let expanded = arg.to_string();
        let mut file = File::open(&expanded)?;
        let mut stdout = io::stdout();
        let mut buffer = [0u8; 8192];
        loop {
            let n = file.read(&mut buffer)?;
            if n == 0 { break; }
            stdout.write_all(&buffer[..n])?;
        }
    }
    Ok(())
}
