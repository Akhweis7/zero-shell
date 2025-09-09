use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn mv(args: &Vec<String>) -> io::Result<()> {
    if args.len() < 2 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "mv: missing operand"));
    }

    let sources: Vec<PathBuf> = args[..args.len()-1].iter().map(PathBuf::from).collect();
    let dest = PathBuf::from(&args[args.len()-1]);

    let dest_is_dir = dest.is_dir();
    if sources.len() > 1 && !dest_is_dir {
        return Err(io::Error::new(io::ErrorKind::Other, "mv: target is not a directory"));
    }

    for src in sources {
        let target_path = if dest_is_dir {
            let name = src.file_name().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "mv: invalid source"))?;
            dest.join(name)
        } else {
            dest.clone()
        };
        fs::rename(&src, &target_path)?;
    }

    Ok(())
}
