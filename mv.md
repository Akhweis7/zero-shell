### Differences

- **Multi-source support**
  - Current: supports moving multiple sources with the last arg as destination; enforces “multiple -> directory”.
  - Old: supports only a single `src` and `dst`.

- **Destination validation**
  - Current: errors if multiple sources but destination isn’t a directory.
  - Old: no such check (can’t pass multiple sources anyway).

- **Argument handling**
  - Current: uses `PathBuf` and builds target path per source.
  - Old: uses `Path` directly and computes a single destination.

- **Behavior (single move)**
  - Both: if destination is a directory, append the source file name; otherwise, rename to destination path.

### Which is better?

- **Current is better**: it matches typical `mv` usage (supports multiple sources into a directory) and provides clearer validation, while keeping the implementation simple.
- Use the old one only if you want the most minimal 2-arg-only behavior.