# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- `tests/proptest.rs` and `tests/serde.rs` imported `deck::Deck` without a
  feature guard, so a default-features `cargo test` failed to build with
  `E0432` once `deck` was gated behind `rand`. Added `#![cfg(feature = "rand")]`
  to `tests/proptest.rs` and tightened the `tests/serde.rs` guard to
  `cfg(all(feature = "serde", feature = "rand"))`. The `clippy` and `test`
  CI steps now run with `--all-features` so the gated tests actually execute.

### Added

- `generate-deals` example (gated behind the `rand` feature). Migrated
  from `pons`, where it no longer needed anything beyond contract-bridge.
- `generate-constrained-deals` example (gated behind the `rand` feature).
  Shows how to combine `Builder` with `deck::fill_deals` and rejection
  sampling to generate deals matching partial-knowledge constraints
  (known hands, known cards in a suit, HCP range).
- Test coverage migrated from `pons`, where the tests only exercised
  contract-bridge APIs: integration tests for `auction::Auction` (22
  tests covering bid sequences, declarer resolution, doubles/redoubles,
  insufficient/inadmissible-double rejection, `try_extend`/`truncate`/
  `pop`, and `RelativeVulnerability` constants); for `deck::Deck` and
  the rand-gated `full_deal`/`fill_deals` helpers (21 tests); for the
  `eval` module (hcp/ltc/zar/BUMRAP/shortness — 7 tests); plus 3 new
  proptest roundtrips (Call/Auction/Deck `Display`↔`FromStr`) and 5 new
  serde roundtrips (Call/Auction/RelativeVulnerability/Deck/IllegalCall).
  Some assertions partially overlap existing tests in
  `tests/proptest.rs`/`tests/serde.rs` and the in-source unit modules;
  flagged as candidates for follow-up dedup. Added `approx` and `rand`
  to dev-dependencies.

## [0.1.0]

Initial release.

### Added

- Card primitives: `Rank`, `Card`, `Holding`, `Hand` (from `dds-bridge`).
- Bidding primitives: `Level`, `Bid`, `Contract`, `Penalty`, plus duplicate
  scoring via `Contract::score` (from `dds-bridge`).
- Deal containers: `Builder`, `PartialDeal`, `FullDeal` (from `dds-bridge`).
- Table position primitives: `Seat`, `SeatFlags` (from `dds-bridge`).
- Denomination enums: `Strain`, `Suit` (from `dds-bridge`).
- `auction` module: `Call`, `Auction`, `IllegalCall`, `RelativeVulnerability`,
  and their parse errors. Carved out of `pons::bidding`.
- `eval` module: `HandEvaluator` trait, `SimpleEvaluator`, and the standard
  hand-evaluation schemes — `hcp`, `shortness`, `fifths`, `bumrap`, `ltc`,
  `nltc`, `zar`, `hcp_plus`, plus the `FIFTHS`, `BUMRAP`, `BUMRAP_PLUS`,
  `NLTC` evaluator constants. Carved out of `pons::eval`.
- `deck` module (gated behind the `rand` feature): `Deck`, `FillDeals`,
  `full_deal`, `fill_deals`. Carved out of `pons::deck`.
- `serde` feature: `Serialize`/`Deserialize` derives on all public types
  that previously supported them in `dds-bridge` or `pons`.
- MSRV set to Rust 1.88 (driven by `serde_with` 3.20).
