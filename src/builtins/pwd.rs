// pwd builtin command (Member B)

use std::env;
use std::io;

pub fn pwd() -> io::Result<()> {
    let path = env::current_dir()?;
    println!("\x1b[38;5;46m{}\x1b[0m", path.display());
    Ok(())
}
