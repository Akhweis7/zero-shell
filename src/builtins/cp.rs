use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;
// use crate::util::path as pathutil;

pub fn cp(args: &Vec<String>) -> io::Result<()> {
    if args.len() < 2 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "cp: missing file operand"));
    }
    let src = args[0].to_string();
    let dst = args[1].to_string();

    let src_meta = fs::metadata(&src)?;
    if src_meta.is_dir() {
        return Err(io::Error::new(io::ErrorKind::Other, "cp: -r not implemented"));
    }

    copy_file(Path::new(&src), Path::new(&dst))
}

fn copy_file(src: &Path, dst: &Path) -> io::Result<()> {
    // If dst is a directory, copy into it with same filename
    let dst_path = if dst.is_dir() {
        let file_name = src.file_name().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "cp: invalid source name"))?;
        dst.join(file_name)
    } else {
        dst.to_path_buf()
    };

    let mut from = fs::File::open(src)?;
    let mut to = fs::File::create(&dst_path)?;
    let mut buf = [0u8; 8192];
    loop {
        let n = from.read(&mut buf)?;
        if n == 0 { break; }
        to.write_all(&buf[..n])?;
    }
    Ok(())
}
