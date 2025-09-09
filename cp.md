### Differences at a glance

- **Sources supported**
  - Old: exactly one source and one destination.
  - Current: multiple sources with last arg as destination; enforces “multiple -> dir”.

- **Destination handling**
  - Old: if `dst` is a directory, copies into it using the source filename; otherwise writes to `dst`.
  - Current: same logic, but generalized for multiple sources.

- **Permissions**
  - Old: doesn’t preserve permissions.
  - Current: attempts to copy source file mode on Unix (`PermissionsExt::set_mode`).

- **Errors and messages**
  - Old: simpler errors; “cp: missing file operand”; rejects dir sources.
  - Current: “cp: missing operand”; explicit error when multiple sources but dest isn’t a directory; rejects dir sources with “-r not implemented”.

- **Types/structure**
  - Old: uses `Path` and `to_path_buf()` inline.
  - Current: uses `PathBuf` consistently; separates `copy_file` with mutable `dst` path.

### Which is better and why

- **Current implementation is better** for this project because:
  - It matches typical `cp` behavior: supports `cp a b` and `cp a b c dir/`.
  - It preserves file permissions on Unix, which aligns closer with Unix conventions.
  - It provides clearer validation for multi-source copies and destination type.

- Use the old one only if you want the most minimal 2-arg-only behavior. Otherwise, the current version is more correct and still simple.
