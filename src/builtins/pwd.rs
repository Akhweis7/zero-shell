// pwd builtin command (Member B)

use std::env;
use std::io;

pub fn pwd() -> io::Result<()> {
    let path = env::current_dir()?;
    // println!("{}", path.display());
    println!("\x1b[36m{}\x1b[0m", path.display()); // Cyan for general output
    Ok(())
}
