# Changelog: sccache-exclusive

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2026-05-31

### Added
- Include code review report `docs/reviews/2026-05-31_code_review.2.md`.
- Include code review report `docs/reviews/2026-05-31_code_review.1.md`.
- Support `SCCACHE_EXCLUSIVE_CONFIG` environment variable to override the default configuration file path.

### Changed
- Update `rust-version` to 1.85.0.
- Refactor configuration path handling to use the `dirs` crate for cross-platform compatibility, removing direct reliance on the `HOME` environment variable.
- Refactor `is_exclusive()` logic to use idiomatic `Iterator::all()` while maintaining the AND-condition.
- Implement strict, boundary-aware matching in `is_exclusive()` to prevent substring-based false positives.
- Extract duplicated command execution logic into a `run_command` helper function to improve maintainability.
- Rename debug labels in `run_command` calls for better clarity.

### Fixed
- Address potential panics in `main` and `run_command` by adding input validation and proper error propagation.
- Correct string interpolation in `expect()` error messages.
- Address `clippy::expect_fun_call` warning by using `unwrap_or_else` instead of `expect(&format!(...))`.
- Address `clippy::useless_borrows_in_formatting` warning.

## [0.1.2] - 2026-01-16

### Added
- Support `strings` in `sccache-exclusive.toml`.

## [0.1.1] - 2025-10-30

### Added
- Include documentation in `README.md`.
- Implement GitHub Actions workflows in `.github/workflows`.

### Fixed
- Resolve exit status failure.
- Resolve `clippy::incompatible_msrv` warning.

## [0.1.0] - 2025-10-30

### Added
- Initial release.

[Unreleased]: https://github.com/aki-akaguma/sccache-exclusive/compare/v0.2.0..HEAD
[0.2.0]: https://github.com/aki-akaguma/sccache-exclusive/compare/v0.1.2..v0.2.0
[0.1.2]: https://github.com/aki-akaguma/sccache-exclusive/compare/v0.1.1..v0.1.2
[0.1.1]: https://github.com/aki-akaguma/sccache-exclusive/compare/v0.1.0..v0.1.1
[0.1.0]: https://github.com/aki-akaguma/sccache-exclusive/releases/tag/v0.1.0
