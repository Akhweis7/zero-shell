// clear builtin command

use std::io::{self, Write};

pub fn clear() -> io::Result<()> {
   //\x1B[2J → clear the whole screen.

// \x1B[1;1H → move cursor to row 1, column 1.
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush()?;
    Ok(())
}
