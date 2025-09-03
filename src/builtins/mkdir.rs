// mkdir builtin command (Member B)

use std::fs;
use std::io;

pub fn mkdir(arg: &Vec<String>) -> io::Result<()> {
    if arg.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "mkdir: missing operand"));
    }
    
    let dir_name = &arg[0];
    fs::create_dir(dir_name)?;
    Ok(())
}
