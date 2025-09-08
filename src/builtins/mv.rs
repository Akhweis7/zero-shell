use std::fs;
use std::io;
use std::path::Path;
// use crate::util::path as pathutil;

pub fn mv(args: &Vec<String>) -> io::Result<()> {
    if args.len() < 2 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "mv: missing file operand"));
    }
    let src = args[0].to_string();
    let dst = args[1].to_string();

    let src_path = Path::new(&src);
    let dst_path = Path::new(&dst);

    // If dst is directory, move into it using same file name
    let dst_final = if dst_path.is_dir() {
        let name = src_path.file_name().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "mv: invalid source name"))?;
        dst_path.join(name)
    } else {
        dst_path.to_path_buf()
    };

    fs::rename(src_path, &dst_final)
}
