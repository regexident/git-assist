# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Please make sure to add your changes to the appropriate categories:

- `Added`: for new functionality
- `Changed`: for changes in existing functionality
- `Deprecated`: for soon-to-be removed functionality
- `Removed`: for removed functionality
- `Fixed`: for fixed bugs
- `Performance`: for performance-relevant changes
- `Security`: for security-relevant changes
- `Other`: for everything else

## [Unreleased]

### Added

- n/a

### Changed

- Updated dependencies:
  - `anyhow` from `1.0.99` -> `1.0.100`
  - `clap` from `4.5.47` -> `4.5.49`
  - `octocrab` from `0.46.0` -> `0.47.0`
  - `openssl` from `0.10.73` -> `0.10.74`
  - `serde` from `1.0.219` -> `1.0.228`
  - `time` from `0.3.43` -> `0.3.44`
  - `tokio` from `1.47.1` -> `1.48.0`

### Deprecated

- n/a

### Removed

- n/a

### Fixed

- Fixed inverted dry-run logic in bisect command.
- Removed duplicate log message in bisect command.
- Replaced panic-prone unwraps with proper error handling in "git.rs".
- Improved error handling by logging warnings and replacing panics with proper error propagation.

### Performance

- n/a

### Security

- n/a

### Other

- n/a

## [0.3.0] - 2025-10-07

### Changed

- Updated dependencies:
  - `anyhow` from `1.0.99` to `1.0.100`
  - `clap` from `4.5.47` to `4.5.48`
  - `git-url-parse` from `0.4.5` to `0.6.0`
  - `inquire` from `0.7.5` to `0.9.0`
  - `octocrab` from `0.44.0` to `0.46.0`
  - `serde` from `1.0.219` to `1.0.228`
  - `thiserror` from `1.0.44` to `2.0.0`
  - `time` from `0.3.43` to `0.3.44`
- Bumped MSRV from `1.81.0` to `1.85.0`

## [0.2.3] - 2025-09-12

### Changed

- Updated dependencies:
  - `anyhow` from `1.0.98` to `1.0.99`
  - `async-trait` from `0.1.88` to `0.1.89`
  - `clap` from `4.5.37` to `4.5.47`
  - `git2` from `0.20.2` to `0.20.2`
  - `octocrab` from `0.44.1` to `0.44.1`
  - `openssl` from `0.10.72` to `0.10.73`
  - `time` from `0.3.41` to `0.3.43`
  - `tokio` from `1.45.0` to `1.47.1`

## [0.2.2] - 2025-06-25

### Changed

- Updated dependencies:
  - `clap` from `4.5.37` to `4.5.40`
  - `git2` from `0.20.2` to `0.20.2`
  - `octocrab` from `0.44.1` to `0.44.1`
  - `openssl` from `0.10.72` to `0.10.73`
  - `tokio` from `1.45.0` to `1.45.1`

## [0.2.1] - 2025-06-05

### Changed

- Updated dependencies:
  - `clap` from `4.5.37` to `4.5.39`
  - `git2` from `0.20.2` to `0.20.2`
  - `octocrab` from `0.44.1` to `0.44.1`
  - `openssl` from `0.10.72` to `0.10.73`
  - `tokio` from `1.45.0` to `1.45.1`

## [0.2.0] - 2025-05-10

### Changed

- Set MSRV to `1.81.0`
- Updated dependencies:
  - `anyhow` from `1.0.72` to `1.0.98`
  - `async-trait` from `0.1.73` to `0.1.88`
  - `clap` from `4.3.21` to `4.5.37`
  - `git-url-parse` from `0.4.4` to `0.4.5`
  - `git2` from `0.20.1` to `0.20.2`
  - `inquire` from `0.6.2` to `0.7.5`
  - `jsonwebtoken` from `8.3.0` to `9.3.1`
  - `octocrab` from `0.29.1` to `0.44.1`
  - `secrecy` from `0.8.0` to `0.10.3`
  - `serde` from `1.0.186` to `1.0.219`
  - `shellexpand` from `3.1.0` to `3.1.1`
  - `thiserror` from `1.0.44` to `1.0.69`
  - `tokio` from `1.31.0` to `1.45.0`

## [0.1.0] - 2023-08-24

Initial release.
