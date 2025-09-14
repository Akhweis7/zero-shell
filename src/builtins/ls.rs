use std::fs::{self, Metadata};
use std::io;
use std::os::unix::fs::PermissionsExt;
// std::os::windows::fs::MetadataExt
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn run(args: &Vec<String>, flags: &Vec<char>) -> io::Result<()> {
    let show_all = flags.contains(&'a');
    let long = flags.contains(&'l');
    let classify = flags.contains(&'F');

    let targets: Vec<PathBuf> = if args.is_empty() {
        vec![PathBuf::from(".")]
    } else {
        args.iter().map(PathBuf::from).collect()
    };

    for (i, target) in targets.iter().enumerate() {
        if targets.len() > 1 {
            println!("{}:", target.display());
        }
        list_one(target, show_all, long, classify)?;
        if i + 1 < targets.len() {
            println!();
        }
    }

    Ok(())
}

fn list_one(path: &Path, show_all: bool, long: bool, classify: bool) -> io::Result<()> {
    let meta = fs::metadata(path)?;
    if meta.is_dir() {
        let mut entries: Vec<(String, Metadata)> = Vec::new();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let name = entry.file_name();
            let name = name.to_string_lossy().to_string();
            if !show_all && name.starts_with('.') {
                continue;
            }
            let md = match entry.metadata() {
                Ok(m) => m,
                Err(_) => continue,
            };
            entries.push((name, md));
        }
        entries.sort_by(|a, b| a.0.cmp(&b.0));
        for (name, md) in entries {
            if long {
                print_long(&md);
                print!(" ");
            }
            let mut display_name = name;
            if classify {
                if md.is_dir() {
                    display_name.push('/');
                } else if is_executable(&md) {
                    display_name.push('*');
                }
            }
            println!("{}", display_name);
        }
    } else {
        if long { print_long(&meta); print!(" "); }
        let mut display_name = path.file_name().and_then(|s| s.to_str()).unwrap_or("").to_string();
        if classify {
            if meta.is_dir() {
                display_name.push('/');
            } else if is_executable(&meta) {
                display_name.push('*');
            }
        }
        println!("{}", display_name);
    }
    Ok(())
}

fn is_executable(md: &Metadata) -> bool {
    (md.permissions().mode() & 0o111) != 0
}

fn print_long(md: &Metadata) {
    let mode = md.permissions().mode();
    let file_type = if md.is_dir() { 'd' } else { '-' };
    let perms = format!(
        "{}{}{}{}{}{}{}{}{}",
        if mode & 0o400 != 0 { 'r' } else { '-' },
        if mode & 0o200 != 0 { 'w' } else { '-' },
        if mode & 0o100 != 0 { 'x' } else { '-' },
        if mode & 0o040 != 0 { 'r' } else { '-' },
        if mode & 0o020 != 0 { 'w' } else { '-' },
        if mode & 0o010 != 0 { 'x' } else { '-' },
        if mode & 0o004 != 0 { 'r' } else { '-' },
        if mode & 0o002 != 0 { 'w' } else { '-' },
        if mode & 0o001 != 0 { 'x' } else { '-' },
    );

    let size = md.len();
    let mtime = md.modified().unwrap_or(SystemTime::UNIX_EPOCH);
    let ts = mtime.duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();

    print!("{}{} {:>8} {}", file_type, perms, size, format_time(ts));
}

fn format_time(secs: u64) -> String {
    // Minimal formatting without external crates: print epoch seconds
    format!("{}", secs)
}
