# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
