use std::time::{SystemTime, UNIX_EPOCH};

pub fn format_mtime(time: SystemTime) -> String {
    // Minimal, locale-agnostic: YYYY-MM-DD HH:MM
    let dt = time.duration_since(UNIX_EPOCH).unwrap_or_default();
    // Use chrono if added later; for now, implement simple conversion
    // This is a placeholder: show seconds since epoch
    format!("{}", dt.as_secs())
}
