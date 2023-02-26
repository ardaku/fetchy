# Changelog
All notable changes to `fetchy` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://jeronlau.tk/semver/).

## [0.2.2] - 2023-02-26
### Added
 - Network error logging

### Fixed
 - `Net` error being reported as `Network`
 - GET requests panicking

## [0.2.1] - 2023-02-20
### Fixed
 - Add docs.rs metadata so that docs build

## [0.2.0] - 2023-02-19
### Added
 - `Fetch::builder()`, `FetchBuilder`
 - `Fetch::Header`
 
### Changed
 - `Result` type alias now uses default generics, and has an additional generic

### Removed
 - `Fetch::new()`, use `Fetch::builder()` instead

## [0.1.0] - 2022-07-16
### Added
 - `Fetch` struct for HTTPS fetch
 - `Error` enum for HTTP status code errors
 - `Method` enum for selecting the HTTP method
 - `Result` type alias for convenience
