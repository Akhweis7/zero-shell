## 0-shell

A tiny, educational Unix-like shell written in Rust. The goal is to learn core Unix shell concepts by implementing a minimal interactive shell with built-in commands implemented via system APIs (no external binaries for core utilities).

### Quick start

- Prerequisites: Rust toolchain (`rustup`), Cargo.
- Build: `cargo build`
- Run: `cargo run`

The shell starts with a clear screen, prints a prompt in the form:

```
/current/dir $ 
```

- Exit with `exit` or Ctrl+D (EOF).

### Current features (built-ins)

Implemented as Rust functions (no external processes):
- `echo` — print arguments to stdout
- `cd` — change the current working directory (updates the shell state)
- `pwd` — print the current working directory
- `cat <file...>` — print file contents
- `mkdir <dir...>` — create directories
- `cp <src> <dst>` — copy files
- `rm [-r] <path...>` — remove files or directories (`-r` for recursive)
- `mv <src> <dst>` — move/rename
- `clear` — clear the terminal
- `tnanm` — project banner/branding

Planned/considering:
- `ls` options and formatting (partially scaffolded)
- Redirections (`>`, `>>`, `<`) and pipelines (`|`)
- Globbing (`*`, `?`), environment variables, history, tab completion

### Usage examples

```
pwd
cd ..
echo "hello world"
mkdir demo && cd demo
echo "hi" > a.txt   # (planned via redirection; for now use: echo "hi" and then cat)
cat a.txt
cp a.txt b.txt
mv b.txt c.txt
rm c.txt
clear
exit
```

Note: Until redirection is implemented in the shell, use external OS tools to redirect, or test built-ins directly (e.g., create files using editors or another shell).

### Architecture overview

- `src/main.rs` — starts the program, clears screen, shows banner, launches the REPL.
- `src/shell.rs` — core shell:
  - Maintains `cwd` and `running` state.
  - REPL loop: print prompt, read line, tokenize, parse, dispatch.
    - Read: It reads a single expression or statement of user input (code).
    - Eval: It evaluates the received input, executing the code and determining its result.
    - Print: It prints the result of the evaluation back to the user.
    - Loop: It then loops back to the "Read" step, waiting for the next input from the user.
  - `tokenize` supports double-quoted strings and empty tokens ("").
  - `parse_command` extracts `name`, `args`, and compact single-dash flags (e.g., `-laF`).
  - `dispatch` routes to built-ins and updates `cwd` after `cd`.
- `src/builtins/` — one file per built-in (e.g., `echo.rs`, `cd.rs`, ...).
- `src/error.rs` — unified error type with context-rich I/O errors and friendly messages.
- `src/util/` — helpers for paths, permissions, and time formatting.

Prompt: shows the full current directory. Exiting sets `running = false` or breaks on EOF.

### Implementation details and learning goals

#### What is a shell?
A shell is an interactive command interpreter: it reads commands, interprets them (parsing, expansion), and executes either built-in functions or external programs. See: `Unix shell` and `Shell script` in References.

#### Interactive vs command runner
- Interactive shells provide a prompt, line editing, history, and job control.
- Non-interactive shells (or simple command runners) just execute provided commands and exit.
- 0-shell is currently an interactive REPL without job control or scripting yet.

#### Minimum functional feature set
At minimum, a usable shell needs:
- An interactive loop with a prompt and robust input handling (EOF, blank lines)
- Tokenization and basic parsing (handle quotes, simple flags)
- A small set of built-ins that modify shell state (`cd`, `exit`) and perform file ops (`pwd`, `cat`, `mkdir`, `rm`, `cp`, `mv`, `echo`)
- Clear error messages

Next milestones typically include: redirection and pipelines, environment variables, globbing, exit codes, and then job control and scripting.

#### exec vs built-ins
- External commands are run by creating a new process and invoking an `exec` family call (on Unix) to replace the process image with the target program. This requires searching `PATH`, setting up file descriptors for redirection/pipes, and waiting for completion.
- Built-ins are implemented inside the shell process itself. They can directly modify shell state (e.g., `cd` must be a built-in, since changing directories in a child process would not affect the parent shell).

In 0-shell, core commands are built-ins to emphasize systems programming and to avoid reliance on external binaries.

#### Why start with built-ins before pipelines/redirection?
- Built-ins exercise filesystem and I/O syscalls directly.
- They establish error-handling conventions and argument parsing.
- Once stable, add redirection/pipes which introduce file descriptor manipulation and process orchestration.

### BusyBox (context)

BusyBox packages many common Unix utilities into a single small executable. It is popular in embedded systems. Instead of separate binaries for `ls`, `cp`, `mv`, etc., one multi-call binary provides many applets. Conceptually, 0-shell’s approach to built-ins echoes the idea of consolidating functionality, but BusyBox is a userland toolbox rather than an interactive shell itself. They are often used together in minimal systems.

### Unix system programming concepts to explore

- Processes: creation and execution
  - `fork` creates a new process by duplicating the current one (Unix).
  - `execve` replaces the current process image with a new program.
  - `wait`/`waitpid` collects child exit status.
- File descriptors and redirection
  - stdin (0), stdout (1), stderr (2)
  - duplicating and redirecting with `dup2` to implement `>` `>>` `<` and pipes `|`.
- Pipes
  - `pipe()` creates a unidirectional channel; combine with `fork/exec` to connect stdout of one process to stdin of another.
- Filesystems and inodes
  - An inode stores metadata about a file (owner, mode, timestamps, block pointers). Paths are directory entries mapping names to inode numbers.
  - Links: hard links reference the same inode; symlinks are special files resolving to a path.
- Permissions and modes
  - Read/write/execute bits for user/group/other; umask; chmod/chown.
- Signals and job control (later)
  - Handling SIGINT, suspending/resuming jobs, foreground/background processes.

References in this README provide deep dives into these topics.

### OS and portability

Is 0-shell Unix-only? The code compiles and runs on Windows and Unix-like systems because Rust’s standard library abstracts many filesystem and I/O operations. Differences to note:

- Process model: Windows lacks `fork`; process creation uses `CreateProcess`. When/if we add external command execution, we’ll use portable Rust crates or conditionally compile per-OS.
- Paths and separators: handle `\` vs `/`, drive letters on Windows.
- TTY/console: this project uses `crossterm` for cross-platform terminal control (`clear`).
- Signals and job control differ substantially across OSes (advanced topic).

How shells differ (Bash/Zsh vs PowerShell vs cmd):
- Bash/Zsh: Unix philosophy, text streams, pipelines, POSIX semantics, rich scripting via POSIX sh plus extensions; Zsh adds user-friendly features (globbing, completion).
- PowerShell: Object pipeline (passes .NET objects, not byte streams), rich .NET integration, cmdlets; different quoting/escaping and path semantics.
- cmd.exe: legacy Windows shell focused on batch files; minimal features compared to Bash/Zsh/PowerShell.

Why Unix shells feel more “scriptable” than Windows shells:
- Uniform “everything is a file” abstraction and byte-stream pipelines.
- Ubiquitous small composable tools with well-defined stdin/stdout behavior.
- Stable, portable POSIX interfaces and conventions.

### Rust vs C for shell implementation

Traditionally shells are written in C. Rust advantages:
- Memory safety (no use-after-free, buffer overflows) without GC.
- Strong type system and expressive enums for error handling (see `ShellError`).
- Concurrency without data races (Send/Sync discipline).
- Great ecosystem and tooling (Cargo, crates for terminal control and parsing).

Common pitfalls Rust helps avoid:
- Memory leaks and double frees (ownership/borrowing rules).
- Race conditions and data races (safe concurrency primitives).
- String/UTF-8 mishandling (explicit conversions and checked operations).

### Roadmap (suggested learning/implementation path)

1. Harden current built-ins: robust errors, flags, and edge cases
2. Implement redirections (`>`, `>>`, `<`) via file descriptor management
3. Implement pipelines (`|`) with child process creation and `pipe()`
4. Add external command execution (`PATH` search, `spawn/exec`, wait, exit codes)
5. Globbing, quoting/escaping rules, and environment variables (`$VAR`)
6. Command history, line editing, and tab completion
7. Job control (foreground/background, signals)
8. Scripting mode (execute from file), shebang handling, and minimal POSIX compliance targets

### Repository layout

```
src/
  builtins/   # echo, cd, pwd, cat, mkdir, cp, rm, mv, clear, ...
  error.rs    # unified error type
  main.rs     # entry point
  shell.rs    # REPL, tokenizer, parser, dispatcher
  util/       # helpers (paths, perms, time formatting)
```

### Contributing and naming

Why “0-shell”? A nod to starting from zero—building fundamentals first. Contributions welcome via PRs. For author/contact (e.g., Noor and email), replace this line with your details if you wish to publish.

### References

- Unix shell — https://en.wikipedia.org/wiki/Unix_shell
- Shell script — https://en.wikipedia.org/wiki/Shell_script
- POSIX — https://en.wikipedia.org/wiki/POSIX
- Inode — https://en.wikipedia.org/wiki/Inode
- BusyBox — https://busybox.net/


