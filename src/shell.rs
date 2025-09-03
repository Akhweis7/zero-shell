//tala
use std::env;
use std::io::{self, Write};
use std::path::PathBuf;

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
    pub fn run(&mut self) -> Result<(), String> {
        let stdin = io::stdin();
        while self.running {
            self.print_prompt()?;

            let mut input = String::new();
            let bytes_read = stdin.read_line(&mut input).map_err(|e| e.to_string())?;
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
                Some(cmd) => {
                    if !self.dispatch(cmd)? {
                        break;
                    }
                }
            }
        }
        Ok(())
    }

    fn print_prompt(&self) -> Result<(), String> {
        print!("$ ");
        io::stdout().flush().map_err(|e| e.to_string())
    }

    // match the command name to the corresponding builtin function
    fn dispatch(&mut self, command: Command) -> Result<bool, String> {
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
                builtins::cd::cd(&command.args).map_err(|e| e.to_string())?;
                Ok(true)
            }
            "pwd" => {
                builtins::pwd::pwd().map_err(|e| e.to_string())?;
                Ok(true)
            }
            "mkdir" => {
                builtins::mkdir::mkdir(&command.args).map_err(|e| e.to_string())?;
                Ok(true)
            }
            "clear" => {
                builtins::clear::clear().map_err(|e| e.to_string())?;
                Ok(true)
            }
           
            other => {
                eprintln!("Command '{}' not found", other);
                Ok(true)
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
