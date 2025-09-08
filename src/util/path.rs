// ~ expansion and simple $VAR expansion helpers

use std::env;
use std::path::{PathBuf};

/// Expand a single path-like argument by applying:
/// - Leading ~ to $HOME
/// - Simple $VAR environment variable expansion anywhere in the string
pub fn expand_one(input: &str) -> String {
    let with_tilde = expand_tilde(input);
    expand_env_vars(&with_tilde)
}

/// Expand all arguments in place, returning a new Vec
pub fn expand_all(args: &[String]) -> Vec<String> {
    args.iter().map(|s| expand_one(s)).collect()
}

fn expand_tilde(input: &str) -> String {
    if let Some(rest) = input.strip_prefix("~") {
        let home = env::var("HOME").unwrap_or_else(|_| String::from("/"));
        // If input was just "~", rest starts with empty or with path separator
        return format!("{}{}", home, rest);
    }
    input.to_string()
}

fn expand_env_vars(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '$' {
            // Parse variable name: [A-Za-z_][A-Za-z0-9_]*
            let mut name = String::new();
            if let Some(&c0) = chars.peek() {
                if is_var_start(c0) {
                    name.push(chars.next().unwrap());
                    while let Some(&c) = chars.peek() {
                        if is_var_char(c) { name.push(chars.next().unwrap()); } else { break; }
                    }
                    let val = env::var(&name).unwrap_or_default();
                    result.push_str(&val);
                    continue;
                }
            }
            // Lone '$' or not a var start, keep as-is
            result.push('$');
        } else {
            result.push(ch);
        }
    }
    result
}

#[inline]
fn is_var_start(c: char) -> bool { c == '_' || c.is_ascii_alphabetic() }

#[inline]
fn is_var_char(c: char) -> bool { c == '_' || c.is_ascii_alphanumeric() }

/// Convert a possibly expanded string into a PathBuf
pub fn to_pathbuf(expanded: &str) -> PathBuf {
    PathBuf::from(expanded)
}
