// cd builtin command (Member B)

use std::env;
use std::io;
use std::path::Path;

pub fn cd(arg: &Vec<String>) -> io::Result<()> {
    let target = match arg.first() {
        Some(path) if path == "~" => env::var("HOME").unwrap_or("/".to_string()),
        Some(path) if path == ".." => "..".to_string(),
        Some(path) => path.to_string(),
        None => env::var("HOME").unwrap_or("/".to_string()),
    };

    let path = Path::new(&target);

    if !path.exists() || !path.is_dir() {
        return Err(io::Error::new(io::ErrorKind::Other, "No such directory"));
    }

    env::set_current_dir(path)
}
