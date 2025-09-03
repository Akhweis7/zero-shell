// pwd builtin command (Member B)

use std::env;
use std::io;

pub fn pwd() -> io::Result<()> {
    let path = env::current_dir()?;
    println!("{}", path.display());
    Ok(())
}
