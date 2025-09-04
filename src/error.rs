use std::fmt;
use std::io;

/// Unified error model for 0-shell.
#[derive(Debug)]
pub enum ShellError {
    /// I/O related error with a context string.
    Io { ctx: String, err: io::Error },

    /// Usage / argument error.
    Usage(String),

    /// Unknown command.
    InvalidCommand(String),

    /// Generic catch-all.
    Other(String),
}

/// Convenience alias.
pub type ShellResult<T> = Result<T, ShellError>;

impl ShellError {
    /// Create an Io error with context.
    pub fn io<S: Into<String>>(ctx: S, err: io::Error) -> Self {
        ShellError::Io { ctx: ctx.into(), err }
    }

    /// Create a usage error.
    pub fn usage<S: Into<String>>(msg: S) -> Self {
        ShellError::Usage(msg.into())
    }

    /// Create an invalid command error.
    pub fn invalid_command<S: Into<String>>(name: S) -> Self {
        ShellError::InvalidCommand(name.into())
    }

    /// Create a generic error.
    pub fn other<S: Into<String>>(msg: S) -> Self {
        ShellError::Other(msg.into())
    }
}

impl fmt::Display for ShellError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShellError::Io { ctx, err } => write!(f, "error: {}: {}", ctx, err),
            ShellError::Usage(msg) => write!(f, "usage: {}", msg),
            ShellError::InvalidCommand(name) => write!(f, "Command '{}' not found", name),
            ShellError::Other(msg) => write!(f, "error: {}", msg),
        }
    }
}

impl std::error::Error for ShellError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ShellError::Io { err, .. } => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for ShellError {
    fn from(err: io::Error) -> Self {
        ShellError::Io { ctx: "io".into(), err }
    }
}