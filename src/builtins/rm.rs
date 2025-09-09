use std::fs;
use std::io;
use std::path::Path;

pub fn rm(args: &Vec<String>, flags: &Vec<char>) -> io::Result<()> {
    if args.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "rm: missing operand"));
    }
    let recursive = flags.contains(&'r');

    for target in args {
        let p = Path::new(target);
        let meta = match fs::symlink_metadata(p) {
            Ok(m) => m,
            Err(e) => return Err(io::Error::new(e.kind(), format!("rm: {}: {}", target, e))),
        };
        if meta.is_dir() {
            if !recursive {
                return Err(io::Error::new(io::ErrorKind::Other, format!("rm: {}: is a directory", target)));
            }
            fs::remove_dir_all(p)?;
        } else {
            fs::remove_file(p)?;
        }
    }

    Ok(())
}
