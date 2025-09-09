use std::fs;
use std::io;
use std::path::Path;
// use crate::util::path as pathutil;

pub fn rm(args: &Vec<String>, flags: &Vec<char>) -> io::Result<()> {
    if args.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "rm: missing operand"));
    }
    let recursive = flags.contains(&'r');

    for arg in args {
        let p = Path::new(arg);
       let meta = match fs::symlink_metadata(p) { //This function will return an error in the following situations, but is not limited to just these cases: The user lacks permissions to perform metadata call on path. path does not exist.
            Ok(m) => m,
            Err(e) => return Err(io::Error::new(e.kind(), format!("rm: {}: {}", arg, e))),
        };
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
