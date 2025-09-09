use std::cmp::Ordering;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

use crate::util::{perms, timefmt};

#[derive(Debug, Clone, Copy)]
struct Flags {
    show_all: bool,  // -a
    long: bool,      // -l
    classify: bool,  // -F
    no_sort: bool,   // -f (GNU-like: implies -a, disables sort)
}

impl Flags {
    fn from_chars(cs: &[char]) -> Self {
        let mut f = Flags {
            show_all: false,
            long: false,
            classify: false,
            no_sort: false,
        };
        for &c in cs {
            match c {
                'a' => f.show_all = true,
                'l' => f.long = true,
                'F' => f.classify = true,
                'f' => {
                    f.no_sort = true;
                    f.show_all = true; // GNU behavior
                }
                _ => {}
            }
        }
        f
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EntryKind {
    File,
    Dir,
    Symlink,
    Other,
}

#[derive(Debug)]
struct LsEntry {
    name: OsString,             // display name
    path: PathBuf,              // full path
    kind: EntryKind,
    meta: Option<fs::Metadata>, // symlink-aware metadata (symlink_metadata)
}

fn is_hidden(name: &OsStr) -> bool {
    match name.to_str() {
        Some(s) => s.starts_with('.'),
        None => false, // non-UTF8: treat as not hidden
    }
}

fn lstat(path: &Path) -> io::Result<fs::Metadata> {
    fs::symlink_metadata(path)
}

fn detect_kind(ft: &fs::FileType) -> EntryKind {
    if ft.is_symlink() {
        EntryKind::Symlink
    } else if ft.is_dir() {
        EntryKind::Dir
    } else if ft.is_file() {
        EntryKind::File
    } else {
        EntryKind::Other
    }
}

fn os_str_cmp(a: &OsStr, b: &OsStr) -> Ordering {
    a.to_string_lossy().cmp(&b.to_string_lossy())
}

fn make_entry_for_path(path: &Path, display_name: Option<OsString>) -> LsEntry {
    let meta = lstat(path).ok();
    let kind = meta
        .as_ref()
        .map(|m| detect_kind(&m.file_type()))
        .unwrap_or(EntryKind::Other);
    let name = display_name.unwrap_or_else(|| {
        path.file_name()
            .unwrap_or_else(|| OsStr::new(""))
            .to_os_string()
    });
    LsEntry {
        name,
        path: path.to_path_buf(),
        kind,
        meta,
    }
}

fn classify_suffix(kind: EntryKind, path: &Path, meta: Option<&fs::Metadata>) -> &'static str {
    match kind {
        EntryKind::Dir => "/",
        EntryKind::Symlink => "@",
        EntryKind::File => {
            if let Some(m) = meta {
                if perms::is_executable(m, path) {
                    "*"
                } else {
                    ""
                }
            } else {
                ""
            }
        }
        EntryKind::Other => "",
    }
}

fn print_name_line(e: &LsEntry, flags: Flags) {
    let mut name = e.name.to_string_lossy().into_owned();
    if flags.classify {
        name.push_str(classify_suffix(e.kind, &e.path, e.meta.as_ref()));
    }
    println!("\x1b[38;5;46m{name}\x1b[0m");
}

#[cfg(unix)]
fn uid_gid_nlink(meta: &fs::Metadata) -> (u32, u32, u64) {
    (meta.uid(), meta.gid(), meta.nlink())
}

#[cfg(not(unix))]
fn uid_gid_nlink(_meta: &fs::Metadata) -> (u32, u32, u64) {
    (0, 0, 1)
}

fn print_long(e: &LsEntry, flags: Flags) {
    // type char + perms string via util::perms
    let (tchar, perms9) = if let Some(ref m) = e.meta {
        (file_type_char(e.kind), perms::perms_9(m))
    } else {
        (file_type_char(e.kind), "---------".to_string())
    };

    #[cfg(unix)]
    let (uid, gid, nlink) = if let Some(ref m) = e.meta {
        uid_gid_nlink(m)
    } else {
        (0, 0, 1)
    };

    #[cfg(not(unix))]
    let (uid, gid, nlink) = (0, 0, 1);

    let size = e.meta.as_ref().map(|m| m.len()).unwrap_or(0);

    // mtime via util::timefmt
    let mtime_str = e
        .meta
        .as_ref()
        .and_then(|m| m.modified().ok())
        .map(|t| timefmt::format_mtime(t))
        .unwrap_or_else(|| "-".to_string());

    // name (+ -F suffix if requested)
    let mut name = e.name.to_string_lossy().into_owned();
    if flags.classify {
        name.push_str(classify_suffix(e.kind, &e.path, e.meta.as_ref()));
    }

    println!(
        "\x1b[38;5;46m{}{} {:>2} {:>5} {:>5} {:>8} {} {}\x1b[0m",
        tchar, perms9, nlink, uid, gid, size, mtime_str, name
    );
}

fn print_entries(entries: &[LsEntry], flags: Flags) {
    if flags.long {
        for e in entries {
            print_long(e, flags);
        }
    } else {
        for e in entries {
            print_name_line(e, flags);
        }
    }
}

fn file_type_char(kind: EntryKind) -> char {
    match kind {
        EntryKind::Dir => 'd',
        EntryKind::Symlink => 'l',
        EntryKind::File => '-',
        EntryKind::Other => '?',
    }
}

fn list_directory(dir: &Path, flags: Flags) -> i32 {
    let mut exit_code = 0;

    let rd = match fs::read_dir(dir) {
        Ok(r) => r,
        Err(err) => {
            eprintln!("\x1b[38;5;196mls: cannot open directory '{}': {}\x1b[0m", dir.to_string_lossy(), err);
            return 1;
        }
    };

    let mut entries: Vec<LsEntry> = Vec::new();

    // Add "." and ".." explicitly for -a (read_dir doesn't return them)
    if flags.show_all {
        entries.push(make_entry_for_path(&dir.join("."), Some(OsString::from("."))));
        entries.push(make_entry_for_path(&dir.join(".."), Some(OsString::from(".."))));
    }

    for dent in rd {
        match dent {
            Ok(de) => {
                let name = de.file_name();
                if !flags.show_all && is_hidden(&name) {
                    continue;
                }
                let path = de.path();
                let meta = fs::symlink_metadata(&path).ok();
                let kind = meta
                    .as_ref()
                    .map(|m| detect_kind(&m.file_type()))
                    .unwrap_or(EntryKind::Other);
                entries.push(LsEntry { name, path, kind, meta });
            }
            Err(err) => {
                exit_code = 1;
                eprintln!("\x1b[38;5;196mls: error reading '{}': {}\x1b[0m", dir.to_string_lossy(), err);
            }
        }
    }

    if !flags.no_sort {
        entries.sort_by(|a, b| os_str_cmp(&a.name, &b.name));
    }

    print_entries(&entries, flags);
    exit_code
}

fn list_single_path(path: &Path, flags: Flags, print_header: bool) -> i32 {
    let meta = match fs::symlink_metadata(path) {
        Ok(m) => m,
        Err(err) => {
            eprintln!("\x1b[38;5;196mls: cannot access '{}': {}\x1b[0m", path.to_string_lossy(), err);
            return 1;
        }
    };

    let mut exit_code = 0;
    if meta.is_dir() {
        if print_header {
            println!("\x1b[38;5;51m{}:\x1b[0m", path.to_string_lossy());
        }
        exit_code |= list_directory(path, flags);
    } else {
        let entry = make_entry_for_path(path, None);
        print_entries(&[entry], flags);
    }
    exit_code
}

/// Public entry point for your shell builtin.
/// `args`: positional paths (from your parser, *excluding* flags).
/// `flags`: compact flag chars (e.g., from "-laF" -> ['l','a','F']).
pub fn run(args: &[String], flags: &[char]) -> io::Result<()> {
    let flags = Flags::from_chars(flags);
    let targets: Vec<PathBuf> = if args.is_empty() {
        vec![PathBuf::from(".")]
    } else {
        args.iter().map(PathBuf::from).collect()
    };

    let multi = targets.len() > 1;
    let mut any_err = 0;

    for (i, p) in targets.iter().enumerate() {
        any_err |= list_single_path(p, flags, multi);
        if i + 1 < targets.len() && multi {
            println!();
        }
    }

    if any_err != 0 {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "ls completed with warnings/errors",
        ));
    }
    Ok(())
}

