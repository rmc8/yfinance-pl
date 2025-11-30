# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.7.2.1] - 2025-11-30

### Added
- Type-safe API with `Literal` type hints for `period` and `interval` parameters
- `TypedDict` definitions for structured return types (`TickerInfo`, `FastInfo`, `CalendarInfo`, `EarningsInfo`)
- `.pyi` stub file for enhanced IDE autocompletion support

### Changed
- **Breaking**: Renamed `ts` column to `date` in `history()` output
- **Breaking**: Changed `date` column type from `i64` (Unix timestamp) to `Datetime` type
- Documentation: Reorganized Installation section (PyPI first, protobuf only for source builds)

### Fixed
- `dividends`, `splits`, `actions`, `capital_gains` now return proper `Datetime` type for date column

## [0.7.2.0] - 2025-11-29

### Added
- GitHub Actions workflow for automated PyPI releases
- Documentation site with multi-language support (EN, JA, ZH, KO, ES)
- Language switcher for documentation

### Changed
- Updated project badges in README

## [0.7.1.0] - 2025-11-28

### Added
- Initial release
- Python wrapper for yfinance-rs using PyO3 and Polars
- Support for stock price history, company info, financial statements
- Support for dividends, splits, options, analyst recommendations
