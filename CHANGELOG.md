# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2022-08-14
### Added
- Add `--valid-for` argument to `xqr encode` to set how long the token is valid for
  - When not set, the XQR will be valid forever
- Add `--iss` argument and pass to `xqr encode` to set the issuer

### Changed
- Upgrade `xqr` to `0.4.0` (breaking changes)

### Removed
- Remove `--kid` argument from `xqr encode`
  - The key id is now generated from the key

## [0.1.0] - 2022-08-11
### Added
- `xqr encode` command
- `xqr decode` command
- `xqr generate-key-pair` command
