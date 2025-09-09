use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

pub fn cp(args: &Vec<String>) -> io::Result<()> {
    if args.len() < 2 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "cp: missing operand"));
    }

    let sources: Vec<PathBuf> = args[..args.len()-1].iter().map(PathBuf::from).collect();
    let dest = PathBuf::from(&args[args.len()-1]);

    let dest_is_dir = dest.is_dir();
    if sources.len() > 1 && !dest_is_dir {
        return Err(io::Error::new(io::ErrorKind::Other, "cp: target is not a directory"));
    }

    for src in sources {
        if src.is_dir() {
            return Err(io::Error::new(io::ErrorKind::Other, format!("cp: -r not implemented: {}", src.display())));
        }
        let src_file_name = src.file_name().and_then(|s| s.to_str()).ok_or_else(|| io::Error::new(io::ErrorKind::Other, "cp: invalid source"))?;
        let mut dst_path = if dest_is_dir { dest.join(src_file_name) } else { dest.clone() };
        copy_file(&src, &mut dst_path)?;
    }

    Ok(())
}

fn copy_file(src: &Path, dst: &mut PathBuf) -> io::Result<()> {
    let mut in_f = File::open(src)?;
    let mut out_f = File::create(&dst)?;
    let mut buf = [0u8; 8192];
    loop {
        let n = in_f.read(&mut buf)?;
        if n == 0 { break; }
        out_f.write_all(&buf[..n])?;
    }
    // try to copy permissions
    if let Ok(meta) = fs::metadata(src) {
        if let Ok(perms) = out_f.metadata() {
            let mut newp = perms.permissions();
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                newp.set_mode(meta.permissions().mode());
            }
            let _ = fs::set_permissions(&dst, newp);
        }
    }
    Ok(())
}
