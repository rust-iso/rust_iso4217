# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.3] - 2026-06-25

Engineering parity with `rust_iso3166` 0.2.0. No currency data changes.

### Changed
- Bumped `phf` `0.11 → 0.14`.
- Bumped wasm dependencies: `wasm-bindgen` `0.2.83 → 0.2.100`,
  `js-sys` `0.3.60 → 0.3.77`, `wasm-bindgen-test` `0.3.33 → 0.3.50`.
- Declared a minimum supported Rust version (`rust-version = "1.85"`).

### Removed
- Dropped the unused `prettytable-rs` dependency, which was pulled into every
  non-wasm build but never referenced.

### Fixed
- Removed redundant no-op `.clone()` calls on `&str` in the wasm bindings
  (`all_active_code`, `all_funds_code`, `all_historic_code`), silencing the
  `noop_method_call` warnings.

## [0.1.2] - 2026-06-25

Refreshed currency data from the latest sources (SIX `list-one.xls` /
`list-three.xls`, published 2026-01-01, and the datahub `country-codes` CSV).
The active currency + funds set now matches Debian `iso-codes` v4.20.1 (178 codes).

### Added
- `XCG` — Caribbean Guilder (numeric 532; Curaçao, Sint Maarten).
- `ZWG` — Zimbabwe Gold (numeric 924; Zimbabwe).
- `XAD` — Arab Accounting Dinar (numeric 396).
- `EUR` country list now includes Bulgaria (`BGR`) and Croatia (`HRV`).

### Changed
- The following codes moved from active to historic: `ANG` (replaced by
  `XCG`), `BGN` (Bulgaria adopted the euro), `CUC`, `SLL` (redenominated to
  `SLE`), `ZWL` (replaced by `ZWG`).
- `NUMERIC_MAP` now resolves numeric `532` to `XCG` (was `ANG`).

### Fixed
- `from_country("TUR")` again returns `TRY` — the upstream datahub CSV had
  dropped Turkey's currency code, which was forced back via a hot fix.

### Tooling
- `scripts/generate.py`: pandas 3.0 compatibility (`iloc[row, i]`), datahub CSV
  ISO 4217 code column moved to index 21, idempotent country↔currency hot fixes,
  and `ANG` added to the numeric-map exclude list to avoid the `532` collision.

## [0.1.1]

- Initial published data set with ISO 4217 active, funds, and historic codes,
  numeric/alpha lookups, country mappings, and wasm bindings.

[0.1.3]: https://github.com/rust-iso/rust_iso4217/compare/0.1.2...0.1.3
[0.1.2]: https://github.com/rust-iso/rust_iso4217/compare/0.1.1...0.1.2
[0.1.1]: https://github.com/rust-iso/rust_iso4217/releases/tag/0.1.1
