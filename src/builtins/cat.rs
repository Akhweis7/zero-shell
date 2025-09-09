use std::io::{ self };
use std::fs;

pub fn cat(arg: &Vec<String>) -> io::Result<()> {
    if arg.len() != 1 {
        return Err(
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "cat: too many arguments (expect 1 argument)"
            )
        );
    }

    let arg = &arg[0];
    let path = std::path::PathBuf::from(arg);

    let file_content = fs::read_to_string(path)?;
    print!("\x1b[38;5;40m{}\x1b[0m", file_content); // ansi color. \x1b[0m to reset the color

    Ok(())
