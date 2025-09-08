use std::fs::Metadata;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

#[cfg(unix)]
pub fn is_executable(meta: &Metadata, _path_name: &str, is_dir: bool) -> bool {
    if is_dir { return false; }
    let mode = meta.permissions().mode();
    (mode & 0o111) != 0
}

#[cfg(not(unix))]
pub fn is_executable(_meta: &Metadata, path_name: &str, is_dir: bool) -> bool {
    if is_dir { return false; }
    matches!(
        std::path::Path::new(path_name)
            .extension()
            .and_then(|s| s.to_str())
            .map(|s| s.to_ascii_lowercase()),
        Some(ext) if matches!(ext.as_str(), "exe"|"bat"|"cmd"|"com")
    )
}

pub fn perms_string(meta: &Metadata, is_dir: bool) -> String {
    #[cfg(unix)]
    {
        let mode = meta.permissions().mode();
        let file_type = if is_dir { 'd' } else { '-' };
        let mut s = String::with_capacity(10);
        s.push(file_type);
        let bits = [0o400,0o200,0o100, 0o040,0o020,0o010, 0o004,0o002,0o001];
        for (i, bit) in bits.iter().enumerate() {
            let ch = match i % 3 {
                0 => if (mode & bit) != 0 { 'r' } else { '-' },
                1 => if (mode & bit) != 0 { 'w' } else { '-' },
                _ => if (mode & bit) != 0 { 'x' } else { '-' },
            };
            s.push(ch);
        }
        return s;
    }

    #[cfg(not(unix))]
    {
        let file_type = if is_dir { 'd' } else { '-' };
        let readonly = meta.permissions().readonly();
        let rest = if readonly { "r-xr-xr-x" } else { "rwxrwxrwx" };
        let mut s = String::with_capacity(10);
        s.push(file_type);
        s.push_str(rest);
        return s;
    }
}
