# contract-bridge

Data types and primitives for the game of contract bridge.

This crate provides the shared "lingo" — cards, hands, deals, seats, bids,
contracts, scoring, auctions, hand evaluation, and random deal generation —
used by higher-level bridge crates such as `dds-bridge` (a double-dummy
solver wrapper) and `pons` (analysis and simulation).

## Features

* `serde` — derive `Serialize` / `Deserialize` on all public types that
  support it.
* `rand` — enable the `deck` module for random card shuffling and deal
  generation.
