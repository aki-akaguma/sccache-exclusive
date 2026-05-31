# Changelog: sccache-exclusive

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
* Code review report: `docs/reviews/2026-05-31_code_review.2.md`
* Code review report: `docs/reviews/2026-05-31_code_review.1.md`
* Support for `SCCACHE_EXCLUSIVE_CONFIG` environment variable to override the default configuration file path.

### Changed
* Refactored configuration path handling to use the `dirs` crate for cross-platform compatibility, removing direct reliance on the `HOME` environment variable.
* Refactored `is_exclusive()` logic to use idiomatic `Iterator::all()` while maintaining the AND-condition.
* Implemented strict, boundary-aware matching in `is_exclusive()` to prevent substring-based false positives.
* Extracted duplicated command execution logic into a `run_command` helper function to improve maintainability.
* Renamed debug labels in `run_command` calls for better clarity.

### Fixed
* Fixed incorrect string interpolation in `expect()` error messages.
* Addressed `clippy::expect_fun_call` warning by using `unwrap_or_else` instead of `expect(&format!(...))`.
* `clippy::useless_borrows_in_formatting`


## [0.1.2] (2026-01-16)
### Added
* `strings` into `sccache-exclusive.toml`

## [0.1.1] (2025-10-30)
### Added
* README.md
* `.github/workflow`

### Fixed
* bug: faile of exit status
* clippy: `clippy::incompatible_msrv`

## [0.1.0] (2025-10-30)
* first commit

[Unreleased]: https://github.com/aki-akaguma/sccache-exclusive/compare/v0.1.2..HEAD
[0.1.2]: https://github.com/aki-akaguma/sccache-exclusive/compare/v0.1.2..v0.1.2
[0.1.1]: https://github.com/aki-akaguma/sccache-exclusive/compare/v0.1.0..v0.1.1
[0.1.0]: https://github.com/aki-akaguma/sccache-exclusive/releases/tag/v0.1.0
