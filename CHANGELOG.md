# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!--

Headers:

* "Added" for new features.
* "Changed" for changes in existing functionality.
* "Deprecated" for soon-to-be removed features.
* "Removed" for now removed features.
* "Fixed" for any bug fixes.
* "Security" in case of vulnerabilities. 

`cargo release` is used to bump the version.

-->

<!-- next-header -->

## [Unreleased] - ReleaseDate

## [0.0.4] - 2026-06-26

## Added

- Timeout option to `BrowserAuth` authentication callback handler.
- `Client::disconnect()`, for clean disconnects.
- Rename `result` module to `error`.

## [0.0.3] - 2026-06-13

## Added

- Quickstart example added to the readme and top Rust documentation.
- no_auth() option, for non-interactive sessions. Requires an existing access token.
- Documentation of many more types.
- Banner/logo image.

## Changed

- Simplified module path to credential store implementations.
- Montrose API updated to the 2026-06-11 version. Includes `CurrencyPosition`.
- Renamed `Account` to `AccountHoldings`.
- Renamed `HoldingsSelector` to `AccountFilter`.
- Turn account number and currency into their own types.

## Fixed

- Removed left-over oauth2 dependency.

## [0.0.2] - 2026-06-02

## Added

- API: Add missing `add_to_watchlist()`.
- Raw API calls using `raw_api_call()`.
- Downstream libraries used in function signatures are re-exported.
- Added a lot more rustdoc documentation.

## Changed

- Recommended standard traits added to all API types.
- Feature `keyring-creds` renamed to `keyring`.
- Some API types are renamed to better names.
- `TradeCurrency` is now an enum.

## [0.0.1] - 2024-05-30

First public release.

<!-- next-url -->
[Unreleased]: https://github.com/thomasa88/southesk/compare/v0.0.4...HEAD
[0.0.4]: https://github.com/thomasa88/southesk/compare/v0.0.3...v0.0.4
[0.0.3]: https://github.com/thomasa88/southesk/compare/v0.0.2...v0.0.3
[0.0.2]: https://github.com/thomasa88/southesk/compare/v0.0.1...v0.0.2
[0.0.1]: https://github.com/thomasa88/southesk/releases/tag/v0.0.1
