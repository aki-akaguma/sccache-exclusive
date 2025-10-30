# sccache-exclusive

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![Rust Version][rustc-image]
![Apache2/MIT licensed][license-image]

It selectively uses sccache. Do not use sccache to compile the specified crate. For example, a compilation error occurs with sccache.

## Install
### step 1
Install using `cargo install`.
```
cargo install sccache-exclusive
```

### step 2
Run `sccache-exclusive`. An error will be output, but please ignore it. When executed, a configuration file will be created.
```
sccache-exclusive
```

### step 3
Edit the created configuration file.
```
vi ~/.config/sccache-exclusive.toml
```

### step 4
Fix the `cargo.toml` settings.
```
vi ~/.cargo/config.toml
```

Add the following content:
```
[build]
rustc-wrapper="/home/myname/.cargo/bin/sccache-exclusive"
#rustc-wrapper="/home/myname/.cargo/bin/sccache"
```
Delete the `sccache` line that was originally there. It's commented out here.

## Setting
This section explains the contents of the configuration file.
```
cat ~/.config/sccache-exclusive.toml
```
```
[build]
rustc-wrapper = "/home/myname/.cargo/bin/sccache"

[[exclusive]]
string = "--crate-name XXX"

[[exclusive]]
string = "--crate-name wayland_client"

[[exclusive]]
string = "--crate-name wayland_protocols"
```

`~/.config/sccache-exclusive.toml` also has the same `[build]` section as `~/.cargo/config.toml`. Please set the original `sccache`.

In the `[[exclusive]]` section, write a string that matches the command line string. This section can be written multiple times. If all `[[exclusive]]` does not match, run `rustc-wrapper`. If even one matches, it will be executed without `rustc-wrapper`.

# Changelogs

[This crate's changelog here.](https://github.com/aki-akaguma/sccache-exclusive/blob/main/CHANGELOG.md)

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)

at your option.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/sccache-exclusive.svg
[crate-link]: https://crates.io/crates/sccache-exclusive
[docs-image]: https://docs.rs/sccache-exclusive/badge.svg
[docs-link]: https://docs.rs/sccache-exclusive/
[rustc-image]: https://img.shields.io/badge/rustc-1.72+-blue.svg
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
