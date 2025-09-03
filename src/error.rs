// ShellError + formatting (Member E)

// use std::io;

// #[derive(Debug)]
// pub enum ShellError {
//     Io(io::Error),
//     Parse(String),
//     CommandNotFound(String),
// }

// impl From<io::Error> for ShellError {
//     fn from(err: io::Error) -> Self {
//         ShellError::Io(err)
//     }
// }

// impl std::fmt::Display for ShellError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             ShellError::Io(e) => write!(f, "IO error: {}", e),
//             ShellError::Parse(s) => write!(f, "Parse error: {}", s),
//             ShellError::CommandNotFound(cmd) => write!(f, "Command '{}' not found", cmd),
//         }
//     }
// }
