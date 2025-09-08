use std::fs::{self, Metadata};
use std::io;
use std::path::{Path, PathBuf};
use crate::util::perms;

pub fn ls(args: &Vec<String>, flags: &Vec<char>) -> io::Result<()> {
    let show_all = flags.contains(&'a');
    let long = flags.contains(&'l');
    let classify = flags.contains(&'F');

    // Determine target paths; default to current directory
    let targets: Vec<PathBuf> = if args.is_empty() { vec![PathBuf::from(".")] } else { args.iter().map(|s| PathBuf::from(s)).collect() };

    let multiple = targets.len() > 1;
    for (i, target) in targets.iter().enumerate() {
        let meta = fs::symlink_metadata(target)?;
        if multiple {
            println!("{}:", target.display());
        }
        if meta.is_dir() {
            list_dir(target, show_all, long, classify)?;
        } else {
            print_entry(target, &meta, long, classify)?;
            println!("");
        }
        if multiple && i + 1 < targets.len() { println!(""); }
    }
    Ok(())
}

fn list_dir(dir: &Path, show_all: bool, long: bool, classify: bool) -> io::Result<()> {
    let mut entries: Vec<(PathBuf, Metadata)> = Vec::new();
    for ent in fs::read_dir(dir)? {
        let ent = ent?;
        let path = ent.path();
        let name = ent.file_name();
        let name = name.to_string_lossy();
        if !show_all && name.starts_with('.') { continue; }
        let meta = fs::symlink_metadata(&path)?;
        entries.push((path, meta));
    }
    // Simple name sort
    entries.sort_by(|a, b| a.0.file_name().cmp(&b.0.file_name()));

    for (path, meta) in entries {
        print_entry(&path, &meta, long, classify)?;
        println!("");
    }
    Ok(())
}

fn print_entry(path: &Path, meta: &Metadata, long: bool, classify: bool) -> io::Result<()> {
    let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
    let is_dir = meta.is_dir();
    if long {
        let perms_s = perms::perms_string(meta, is_dir);
        // Hardcode size and mtime minimalistically to avoid extra crates
        let size = meta.len();
        print!("{} {:>8} ", perms_s, size);
    }
    if classify {
        if is_dir { print!("{}/", name); }
        else if perms::is_executable(meta, name, is_dir) { print!("{}*", name); }
        else { print!("{}", name); }
    } else {
        print!("{}", name);
    }
    Ok(())
}

