//tala
use std::env;
use std::io::{self, Write};
use std::path::PathBuf;
use crate::error::{ShellError, ShellResult};
use crate::builtins;

pub struct Shell {
    cwd: PathBuf, // current working directory. Needed so builtins like cd can update the shell’s state and future commands run relative to the right directory
    running: bool, /*Controls the REPL loop.
                  exit sets it to false to end the loop and terminate the shell gracefully.
                  Also helps handle EOF (Ctrl+D) cleanly by breaking the loop. */
}

impl Shell {
    pub fn new() -> Self {
        let cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("/")); // get the current working directory or default to "/"
        Self { cwd, running: true }
    }

    // REPL loop: prints prompt, reads input, handles EOF, ignores blank lines, parses and dispatches commands.
    pub fn run(&mut self) -> ShellResult<()> {
        let stdin = io::stdin();
        while self.running {
            self.print_prompt()?;
            // print!("\x1b[38;5;40m{}\x1b[0m", self.print_prompt()?);
            let mut input = String::new();
          let bytes_read = stdin.read_line(&mut input).map_err(|e| ShellError::io("read_line", e))?;
            if bytes_read == 0 {
                // EOF (Ctrl+D)
                break;
            }

            let input = input.trim();
            if input.is_empty() {
                continue;
            }

            // parse the command and dispatch it
            match parse_command(input) {
                None => continue,
             Some(cmd) => match self.dispatch(cmd) {
                    Ok(false) => break,
                    Ok(true) => {}
                    Err(err) => {
                        eprintln!("\x1b[38;5;196m{}\x1b[0m", err);
                    }
                }
            }
        }
        Ok(())
    }

    fn print_prompt(&self) ->  ShellResult<()> {
        // Get the full current directory path
        let current_dir = self.cwd.to_str().unwrap_or("/");
        print!("\x1b[38;5;93m{}$ \x1b[0m", current_dir);
        // print!("{} $ ", current_dir);
        io::stdout().flush().map_err(|e|ShellError::io("flush", e))
    }

    // match the command name to the corresponding builtin function
    fn dispatch(&mut self, command: Command) ->ShellResult<bool> {
        match command.name.as_str() {
            "exit" => {
                self.running = false;
                Ok(false)
            }
            "echo" => {
                builtins::echo::run(&command.args);
                Ok(true)
            }
            "cd" => {
                builtins::cd::cd(&command.args).map_err(|e| ShellError::io("cd", e))?;
                // Update the shell's current working directory after successful cd
                self.cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));
                Ok(true)
            }
            "pwd" => {
                builtins::pwd::pwd().map_err(|e|ShellError::io("pwd", e))?;
                Ok(true)
            }
            "cat" => {
                builtins::cat::cat(&command.args).map_err(|e| ShellError::io("cat", e))?;
                Ok(true)
            }
            "mkdir" => {
                builtins::mkdir::mkdir(&command.args).map_err(|e| ShellError::io("mkdir", e))?;
                Ok(true)
            }
            "cp" => {
                builtins::cp::cp(&command.args).map_err(|e| ShellError::io("cp", e))?;
                Ok(true)
            }
            "rm" => {
                builtins::rm::rm(&command.args, &command.flags).map_err(|e| ShellError::io("rm", e))?;
                Ok(true)
            }
            "mv" => {
                builtins::mv::mv(&command.args).map_err(|e| ShellError::io("mv", e))?;
                Ok(true)
            }
            "clear" => {
                builtins::clear::clear().map_err(|e| ShellError::io("clear", e))?;
                Ok(true)
            }
            "tnanm" => {
                builtins::tnanm::tnanm();
                Ok(true)
            }
            "ls" => {
                builtins::ls::run(&command.args, &command.flags).map_err(|e| ShellError::io("ls", e))?;
                Ok(true)
            }
            other => {
             Err(ShellError::invalid_command(other.to_string()))
            }
        }
    }
}

struct Command {
    name: String,
    args: Vec<String>,
    flags: Vec<char>,
}

/*Convert tokens into a structured Command with name, args, and compact flags.
- name: first token
- args: all tokens after name
- flags: all tokens after name that start with -

example:
Input: echo "hello world" "" a -laF
Output: Command { name: "echo", args: ["hello world", "", "a"], flags: ['l', 'a', 'F'] }
*/
fn parse_command(line: &str) -> Option<Command> {
    let tokens = tokenize(line);
    if tokens.is_empty() {
        return None;
    }
    let name = tokens[0].clone();

    let mut args: Vec<String> = Vec::new();
    let mut flags: Vec<char> = Vec::new();

    for tok in tokens.into_iter().skip(1) {
        if tok.starts_with('-') && tok.len() > 1 {
            for ch in tok[1..].chars() {
                flags.push(ch);
            }
        } else {
            args.push(tok);
        }
    }

    Some(Command { name, args, flags })
}

// Split a raw input line into tokens while respecting double quotes.
// Spaces and tabs split tokens unless inside quotes.
/*
Double quotes toggle “in quotes” mode; "a b" becomes one token.
"" becomes an empty token.
Multiple spaces are collapsed between tokens. example: "a   b" -> ["a", "b"]

Input: echo "hello world" "" a
Output tokens: ["echo", "hello world", "", "a"]
*/
fn tokenize(input: &str) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;

    let mut chars = input.chars().peekable();
    while let Some(ch) = chars.next() {
        match ch {
            '"' => {
                in_quotes = !in_quotes;
                if !in_quotes {
                    // closing quote; allow empty string token
                    if current.is_empty() {
                        tokens.push(String::new());
                    }
                }
            }
            ' ' | '\t' if !in_quotes => {
                if !current.is_empty() {
                    tokens.push(std::mem::take(&mut current));
                }
                // skip consecutive spaces
                while let Some(' ' | '\t') = chars.peek() {
                    chars.next();
                }
            }
            _ => current.push(ch),
        }
    }

    if !current.is_empty() || in_quotes == false {
        if !current.is_empty() {
            tokens.push(current);
        }
    }

    tokens
}
