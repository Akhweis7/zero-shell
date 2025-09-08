// cd builtin command (Member B)

use std::env;
use std::io;
use std::path::Path;
use crate::util::path as pathutil;

pub fn cd(arg: &Vec<String>) -> io::Result<()> {
    let target_raw = match arg.first() {
        Some(path) => path.to_string(),
        None => env::var("HOME").unwrap_or("/".to_string()),
    };

    let target = pathutil::expand_one(&target_raw);

    let path = Path::new(&target);

    if !path.exists() || !path.is_dir() {
        return Err(io::Error::new(io::ErrorKind::Other, "No such directory"));
    }

    env::set_current_dir(path)
}
