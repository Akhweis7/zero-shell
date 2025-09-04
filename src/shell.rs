//tala
use std::env;
use std::io::{self, Write};
use std::path::PathBuf;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    terminal,
};

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
        while self.running {
            let input_opt = self.read_line_colored().map_err(|e| e.to_string())?;
            let Some(input) = input_opt else { break };

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
        // Get the full current directory path
        let current_dir = self.cwd.to_str().unwrap_or("/");
        // print!("{} $ ", current_dir);
        // Yellow prompt like: PS <cwd>
        // \x1b[33m = yellow, \x1b[0m = reset
        print!("\x1b[38;5;226m{}>\x1b[0m ", current_dir);
        io::stdout().flush().map_err(|e| e.to_string())
    }

    fn read_line_colored(&self) -> io::Result<Option<String>> {
        let mut stdout = io::stdout();
        self.print_prompt().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        terminal::enable_raw_mode()?;
        let mut buffer = String::new();

        loop {
            match event::read()? {
                Event::Key(KeyEvent { kind: KeyEventKind::Press, code, modifiers, .. }) => {
                    match (code, modifiers) {
                        (KeyCode::Enter, _) => {
                            stdout.write_all(b"\r\n")?;
                            stdout.flush()?;
                            terminal::disable_raw_mode()?;
                            return Ok(Some(buffer));
                        }
                        // Ctrl+D => EOF
                        (KeyCode::Char('d'), m) if m.contains(KeyModifiers::CONTROL) => {
                            terminal::disable_raw_mode()?;
                            return Ok(None);
                        }
                        // Ctrl+C => cancel current line and refresh prompt
                        (KeyCode::Char('c'), m) if m.contains(KeyModifiers::CONTROL) => {
                            buffer.clear();
                            stdout.write_all(b"\r\n")?;
                            stdout.flush()?;
                            self.print_prompt().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                            continue;
                        }
                        (KeyCode::Backspace, _) => {
                            buffer.pop();
                        }
                        (KeyCode::Char(ch), _) => {
                            buffer.push(ch);
                        }
                        (KeyCode::Tab, _) => {
                            buffer.push('\t');
                        }
                        (KeyCode::Left | KeyCode::Right | KeyCode::Up | KeyCode::Down, _) => {
                            // ignore arrows for now
                        }
                        _ => {}
                    }

                    // Re-render the whole line: move to start, print prompt + colored buffer, clear to end
                    stdout.write_all(b"\r")?;
                    let prompt = format!("\x1b[33mPS {}>\x1b[0m ", self.cwd.to_str().unwrap_or("/"));
                    let colored = colorize_user_input(&buffer);
                    write!(stdout, "{}{}", prompt, colored)?;
                    // CSI 0K clear from cursor to end
                    stdout.write_all(b"\x1b[0K")?;
                    stdout.flush()?;
                }
                Event::Paste(s) => {
                    buffer.push_str(&s);
                }
                _ => {}
            }
        }
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
                // Update the shell's current working directory after successful cd
                self.cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));
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
            "tnanm" => {
                builtins::tnanm::tnanm();
                Ok(true)
            }
            other => {
                //eprintln!("Command '{}' not found", other);
                // Colorize: command (green), any flags (blue), args (cyan)
                // We don't have flags/args here, so just color the command
                eprintln!("Command '\x1b[32m{}\x1b[0m' not found", other);
                Ok(true)
            }
        }
    }
}

// removed command echo to avoid duplicate lines

fn colorize_user_input(input: &str) -> String {
    // Tokenize similarly to `tokenize`, but keep spaces as separators in the output
    let mut parts: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;

    for ch in input.chars() {
        match ch {
            '"' => {
                in_quotes = !in_quotes;
                current.push(ch);
            }
            ' ' | '\t' if !in_quotes => {
                if !current.is_empty() {
                    parts.push(current.clone());
                    current.clear();
                }
                parts.push(String::from(" "));
            }
            _ => current.push(ch),
        }
    }
    if !current.is_empty() {
        parts.push(current);
    }

    let mut out = String::new();
    let mut is_first = true;
    for token in parts {
        if token == " " {
            out.push_str(&token);
            continue;
        }
        if is_first {
            out.push_str(&format!("\x1b[38;5;47m{}\x1b[0m", token));
            is_first = false;
        } else if token.starts_with('-') && token.len() > 1 {
            out.push_str(&format!("\x1b[38;5;133m{}\x1b[0m", token));
        } else {
            out.push_str(&format!("\x1b[38;5;75m{}\x1b[0m", token));
        }
    }

    out
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
