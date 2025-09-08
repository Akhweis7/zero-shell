use std::fs;
use std::io;
use std::path::Path;
use crate::util::path as pathutil;

pub fn rm(args: &Vec<String>, flags: &Vec<char>) -> io::Result<()> {
    if args.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "rm: missing operand"));
    }
    let recursive = flags.contains(&'r');

    for arg in args {
        let expanded = pathutil::expand_one(arg);
        let p = Path::new(&expanded);
        let meta = fs::symlink_metadata(&p)?;
        if meta.is_dir() {
            if !recursive {
                return Err(io::Error::new(io::ErrorKind::Other, "rm: cannot remove directory (use -r)"));
            }
            fs::remove_dir_all(p)?;
        } else {
            fs::remove_file(p)?;
        }
    }
    Ok(())
}
