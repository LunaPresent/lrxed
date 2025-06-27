# Changelog

## [0.1.3](https://github.com/LunaPresent/lrxed/compare/v0.1.2...v0.1.3) - 2025-06-27

### Added

- _(ui)_ add preview to file browser

### Changed

- _(ui)_ differentiate between no lyrics and unsynced lyrics
- _(ui)_ show lyrics sync percentage and keep files in sync

### Fixed

- fix error when replacing txt file with lrc file
- _(ui)_ update lyrics in file browser when saving in editor
- _(ui)_ clarify confirmation message when returning to file browser with unsaved changes

## [0.1.2](https://github.com/LunaPresent/lrxed/compare/v0.1.1...v0.1.2) - 2025-06-20

### Added

- _(ui)_ add notification toast to show errors in-app instead of crashing
- _(config)_ add notification-timeout setting
- add config option to replace txt file when saving lyrics

### Changed

- read lyrics from txt file if no lrc file exists

### Fixed

- return error instead of panic when using open-file-or-directory in empty directory
- fix the package variable in the nix home module
