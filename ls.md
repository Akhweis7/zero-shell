### Key differences

- **Flags supported**
  - Current: `-l`, `-a`, `-F` only.
  - Old: `-l`, `-a`, `-F`, plus `-f` (disables sort and implies `-a`).

- **Sorting**
  - Current: always sorts by name.
  - Old: sorts by name unless `-f` (then no sort).

- **Hidden entries**
  - Current: skips dotfiles unless `-a`; does not add “.” and “..”.
  - Old: skips dotfiles unless `-a`; explicitly adds “.” and “..” when `-a`.

- **Symlink handling**
  - Current: uses `DirEntry::metadata()` (follows symlinks), no `@` suffix.
  - Old: uses `symlink_metadata()` (doesn’t follow symlinks), detects and classifies symlinks with `@`.

- **Executable detection**
  - Current: simple mode check for `*`; no special-case per path.
  - Old: uses `util::perms::is_executable(meta, path)`.

- **Long format fields**
  - Current: prints type char, rwx perms, size, and epoch seconds for mtime.
  - Old: prints type char, 9 perms, link count, uid, gid, size, nicely formatted mtime.

- **Error behavior**
  - Current: returns on first error (`?`), stops listing that target.
  - Old: reports per-entry errors, continues, aggregates a nonzero exit status.

- **Output styling**
  - Current: plain output (no colors, except none for headers).
  - Old: colors names and headers.

- **Dependencies**
  - Current: no `util` module; fully self-contained.
  - Old: depends on `util::perms` and `util::timefmt` (removed in your tree).

### Which is better?

- **For this codebase now:** The current `ls` is better because it’s minimal, self-contained, and matches the required flags without depending on the removed `util` module.
- **For Unix fidelity/features:** The old `ls` is better (more complete: `-f`, symlink awareness, owner/group/nlink, nicer time), but would require restoring/replacing `util::perms` and `util::timefmt`.
