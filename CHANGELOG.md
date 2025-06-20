# Changelog

## [0.1.2](https://github.com/LunaPresent/lrxed/compare/v0.1.1...v0.1.2) - 2025-06-20

### Added

- *(ui)* add notification toast to show errors in-app instead of crashing
- *(config)* add notification-timeout setting
- add config option to replace txt file when saving lyrics

### Changed

- read lyrics from txt file if no lrc file exists

### Fixed

- return error instead of panic when using open-file-or-directory in empty directory
- fix the package variable in the nix home module
