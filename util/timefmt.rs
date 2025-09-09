// mtime formatting (Member C)
use std::time::{SystemTime, UNIX_EPOCH};

/// Format modification time for `ls -l`.
///
/// Default (no feature): std-only → epoch seconds as string (portable & allowed everywhere).
/// With `humantime` feature: pretty local time like "Sep  7 20:11".
pub fn format_mtime(t: SystemTime) -> String {
    format_mtime_impl(t)
}

#[cfg(not(feature = "humantime"))]
fn format_mtime_impl(t: SystemTime) -> String {
    match t.duration_since(UNIX_EPOCH) {
        Ok(d) => d.as_secs().to_string(),
        Err(_) => "-".to_string(),
    }
}

#[cfg(feature = "humantime")]
fn format_mtime_impl(t: SystemTime) -> String {
    use chrono::{Local, TimeZone};
    match t.duration_since(UNIX_EPOCH) {
        Ok(d) => {
            let secs = d.as_secs() as i64;
            // GNU ls-style short format: "Mon dd HH:MM"
            // (We don't switch years-season logic; simple & readable.)
            Local
                .timestamp_opt(secs, 0)
                .single()
                .map(|dt| dt.format("%b %e %H:%M").to_string())
                .unwrap_or_else(|| secs.to_string())
        }
        Err(_) => "-".to_string(),
    }
}
