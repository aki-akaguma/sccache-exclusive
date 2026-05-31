# Code Review: sccache-exclusive

## Summary
The project `sccache-exclusive` is a utility tool designed to selectively bypass `sccache` based on crate names or other command-line arguments. This is particularly useful for crates that fail to compile with `sccache`.

## Technical Review

### 1. Error Message Formatting in `expect()`
In `src/main.rs`:
```rust
.expect("failed to execute process: {sccache}");
```
The `expect()` method does not support string interpolation like `println!`. This will literally print `{sccache}` instead of the variable's value. To include the variable value, use `format!`:
```rust
.expect(&format!("failed to execute process: {}", sccache));
```

### 2. Dependency on `HOME` Environment Variable
The code heavily relies on `std::env::var("HOME")`. While this works on most Unix-like systems, it may fail on other platforms or if the variable is not set. Consider using a more robust crate like `dirs` to find the config and home directories.

### 3. Hardcoded Config Path
The configuration path is hardcoded to `~/.config/sccache-exclusive.toml`. Following the XDG Base Directory Specification is good, but providing an override via an environment variable or command-line flag would improve flexibility.

### 4. Logic in `is_exclusive()`
The current implementation of `is_exclusive` has a specific pattern in the `strings` check:
```rust
if let Some(v) = &exclusive.strings {
    for string in v {
        if let Some(_n) = args_s.find(string) {
            continue;
        }
        return false;
    }
    return true;
}
```
This requires *all* strings in the `strings` array to be present in the command line for the crate to be excluded. If the intention was "if any of these match", the logic should return `true` as soon as one is found. If the intention is "all must match", the behavior is correct but should be clearly documented.

### 5. Debug Print Patterns
The use of `eprintln!("AAA: ...")` and `eprintln!("BBB: ...")` should be replaced with more descriptive error messages or a logging framework like `log` or `tracing` for better production readiness.

### 6. Code Duplication in `main()`
The command execution logic (creating a `Command`, checking status, printing errors) is duplicated for both the "exclusive" and "sccache" paths. This could be refactored into a helper function to improve maintainability.

### 7. Substring Matching in `is_exclusive()`
Joining arguments with `args_v.join(" ")` into `args_s` and then searching with `find()` might lead to false positives if a substring matches across different arguments. Searching against individual arguments or using exact matches would be more robust.

## Recommendations
- **Refactor**: Move command execution into a helper function.
- **Bug Fix**: Fix the string interpolation in `expect` calls.
- **Robustness**: Use the `dirs` crate for cross-platform path resolution.
- **Logging**: Replace `eprintln!` debug markers with descriptive messages or a logging library.

---
Review Date: 2026-05-31
Reviewer: Gemini CLI Agent
