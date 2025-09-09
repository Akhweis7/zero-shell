// rwxr-xr-x builder, exec bit (Member C)
use std::fs::Metadata;
use std::path::Path;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

/// Build the 9-char permissions string like "rwxr-xr-x" from Unix mode bits.
/// On non-Unix platforms, returns "---------".
pub fn perms_9(meta: &Metadata) -> String {
    #[cfg(unix)]
    {
        let mode = meta.permissions().mode();
        let mut s = String::with_capacity(9);
        // user
        s.push(if (mode & 0o400) != 0 { 'r' } else { '-' });
        s.push(if (mode & 0o200) != 0 { 'w' } else { '-' });
        s.push(if (mode & 0o100) != 0 { 'x' } else { '-' });
        // group
        s.push(if (mode & 0o040) != 0 { 'r' } else { '-' });
        s.push(if (mode & 0o020) != 0 { 'w' } else { '-' });
        s.push(if (mode & 0o010) != 0 { 'x' } else { '-' });
        // other
        s.push(if (mode & 0o004) != 0 { 'r' } else { '-' });
        s.push(if (mode & 0o002) != 0 { 'w' } else { '-' });
        s.push(if (mode & 0o001) != 0 { 'x' } else { '-' });
        return s;
    }
    #[cfg(not(unix))]
    {
        "---------".to_string()
    }
}

/// Decide if a file is "executable" for `ls -F` suffix `*`.
///
/// Unix: any exec bit (u/g/o) set in mode.
/// Windows/others: treat common executable extensions as exec.
pub fn is_executable(meta: &Metadata, path: &Path) -> bool {
    #[cfg(unix)]
    {
        let mode = meta.permissions().mode();
        return (mode & 0o111) != 0;
    }
    #[cfg(not(unix))]
    {
        // Best-effort by extension on non-Unix.
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            let ext = ext.to_ascii_lowercase();
            return matches!(ext.as_str(), "exe" | "com" | "bat" | "cmd" | "ps1");
        }
        false
    }
}
