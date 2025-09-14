# Tasks for Building the Shell

## Table of Contents

- [Tasks for Building the Shell](#tasks-for-building-the-shell)
  - [Table of Contents](#table-of-contents)
  - [Tala Amm — Core engine: REPL, parser, dispatcher](#tala-amm--core-engine-repl-parser-dispatcher)
    - [What you build](#what-you-build)
    - [Why this matters](#why-this-matters)
    - [Edge cases](#edge-cases)
    - [Tests (quick ideas)](#tests-quick-ideas)
  - [Amro Khweis — Team Leader, Navigation \& lifecycle: cd, pwd, mkdir, exit](#amro-khweis--team-leader-navigation--lifecycle-cd-pwd-mkdir-exit)
    - [What you build](#what-you-build-1)
    - [Why this matters](#why-this-matters-1)
    - [Edge cases](#edge-cases-1)
    - [Tests](#tests)
  - [Nadeen Risheq — Listing \& metadata: ls with -l, -a, -F](#nadeen-risheq--listing--metadata-ls-with--l--a--f)
    - [What you build](#what-you-build-2)
    - [Why this matters](#why-this-matters-2)
    - [Edge cases](#edge-cases-2)
    - [Tests](#tests-1)
  - [Moaz Razem — File content \& manipulation: echo, cat, cp, mv, rm (-r)](#moaz-razem--file-content--manipulation-echo-cat-cp-mv-rm--r)
    - [What you build](#what-you-build-3)
    - [Why this matters](#why-this-matters-3)
    - [Edge cases](#edge-cases-3)
    - [Tests](#tests-2)
  - [Noor Halabi — Stability, polish \& bonuses (signals, prompt, errors, history)](#noor-halabi--stability-polish--bonuses-signals-prompt-errors-history)
    - [What you build (core polish)](#what-you-build-core-polish)
    - [Bonus features (prioritize in this order)](#bonus-features-prioritize-in-this-order)
    - [Why this matters](#why-this-matters-4)
    - [Tests](#tests-3)

## Tala Amm — Core engine: REPL, parser, dispatcher

### What you build

- REPL loop: print prompt `$`, read a line, on EOF (Ctrl+D) exit cleanly; ignore empty lines.
- Parser (minimal but safe): split on spaces with support for double quotes (`"Hello world"` is one arg). Extract flags (`-laF` → `['l','a','F']`).
- Dispatcher: map command name → builtin function. Unknown command prints `Command '<name>' not found` and continues.
- Shell state: keep `Shell { cwd, running }` and initialize `cwd` from `std::env::current_dir()`.

### Why this matters

The shell must execute builtins inside the same process (e.g., `cd` only works this way). The REPL/dispatcher is the "CPU" of your shell.

### Edge cases

- Multiple spaces, trailing spaces, quoted empty strings (`""`), blank input, EOF on empty line.

### Tests (quick ideas)

- `something` → exact "Command 'something' not found".
- `echo "a b"` prints `a b`.
- Ctrl+D exits (status 0).

---

## Amro Khweis — Team Leader, Navigation & lifecycle: cd, pwd, mkdir, exit

### What you build

- `cd [path]`
  - No args → go to `$HOME`. If `$HOME` missing → user-friendly error.
  - Support `~` expansion via `util::path::expand_tilde`.
  - Relative & absolute paths; call `std::env::set_current_dir`. On success, update `shell.cwd`.
- `pwd`: print `std::env::current_dir()` (don't rely only on cached `shell.cwd`).
- `mkdir <name>`: create a directory. No `-p` required; error if parent missing or exists.
- `exit`: set `shell.running = false`.

### Why this matters

Proves your shell controls process state (current directory) and ends cleanly.

### Edge cases

- cd to non-dir / no permission.
- cd `..`, cd `.` work automatically via OS.
- mkdir name already exists (file or dir).

### Tests

- `mkdir a && cd a && pwd` ends in `/a`.
- `cd` (no args) goes to `$HOME`.
- `exit` terminates loop.

---

## Nadeen Risheq — Listing & metadata: ls with -l, -a, -F

### What you build

- Base `ls [path?]`: default `.`; read entries, sort by name; hide dotfiles by default.
- `-a`: include dotfiles (also `.` and `..`).
- `-F`: suffix types
  - `/` directory
  - `*` executable regular file (any exec bit)
  - `@` symlink
- `-l` long format:
  - type char (`d`, `-`, `l`, …) + permissions string like `rwxr-xr-x` (build with `util::perms` using Unix mode bits on Unix; simple fallback elsewhere).
  - link count (you may print 1).
  - uid/gid (numeric on Unix; `-` on others).
  - size in bytes.
  - mtime via `util::timefmt`.
  - name (+ `-F` suffix if both present).

### Why this matters

Users judge shells by `ls`. Implementing flags teaches file metadata, perms, and symlinks.

### Edge cases

- Broken symlinks: still list and mark `@`/`l`.
- Per-entry errors: show warning, continue.
- Executable bit: any of user/group/other exec is enough for `*`.

### Tests

- Hidden files appear only with `-a`.
- `ls -F` marks dirs `/`, executables `*`, symlinks `@`.
- `ls -l` line starts with correct type + 9 perms chars.

---

## Moaz Razem — File content & manipulation: echo, cat, cp, mv, rm (-r)

### What you build

- `echo [args…]`: print args joined by single spaces + newline. No escapes/vars in core.
- `cat <file> [more…]`: stream each file to stdout (buffered). On error, print and continue.
- `cp <src> <dst>`:
  - Files only (no recursive copy in core).
  - If dst is a dir, copy to `dst / basename(src)`.
  - Overwrite if exists (Unix default).
- `mv <src> <dst>`:
  - Try `std::fs::rename`. If it fails with cross-device (EXDEV), fallback: cp then rm original.
- `rm <path>` with optional `-r`:
  - Without `-r`: remove files and symlinks only.
  - With `-r`: recursively remove directories.
  - Never follow symlinks during recursion; remove the link itself.
  - Safety: reject `.` and `..`.

### Why this matters

Covers daily work: reading files, copying, moving, and safe deletion with minimal surprises.

### Edge cases

- cat large files → stream, don't load fully.
- cp into existing dir → append basename.
- mv EXDEV → copy+delete.
- rm `-r` must not chase symlink loops.

### Tests

- `echo a b` → `a b`.
- `cat nofile` errors but shell continues.
- `cp a.txt dir/` → `dir/a.txt`.
- rm file vs rm dir vs rm `-r` dir.

---

## Noor Halabi — Stability, polish & bonuses (signals, prompt, errors, history)

### What you build (core polish)

- Unified error model: `ShellError` with display impl; all builtins return `Result<(), ShellError>`. Make errors consistent (`error: <context>: <io_error>`).
- Prompt polish: show current dir: `~/project $` (use `util::path::shorten_home`).
- Docs: `HELP.md` explaining every builtin + examples.

### Bonus features (prioritize in this order)

- Ctrl+C (SIGINT): don't kill the shell; interrupt the current read/command and reprint prompt.
- History: in-memory vector; optional save to `~/.0shell_history` on exit + load on start.
- Command chaining: support `a; b; c` in the parser and loop.
- Env var expansion: replace `$HOME`, `$USER` inside args (simple, no complex quoting rules).
- Colorized output: dirs blue, errors red (ANSI), prompt color.
- (Stretch) Pipes/IO redirection between builtins only (since you can't spawn externals): use in-memory readers/writers.

### Why this matters

Stability and UX make it feel like a "real" shell and help your grader see robustness.

### Tests

- Pressing Ctrl+C returns a fresh prompt.
- `ls; pwd; echo hi` runs in order.
- `$HOME` expands in `cd $HOME`.
