//! Generate deals matching partial-knowledge constraints.
//!
//! Sibling to [`generate-deals`](../generate-deals/main.rs): instead of fully
//! random deals, this example pins down some cards in advance and uses
//! rejection sampling on [`fill_deals`](contract_bridge::deck::fill_deals) to
//! satisfy the rest.
//!
//! Hardcoded scenario:
//!
//! ```text
//! N: K92.T4.QJ7.KT975        (fully known)
//! W: A73.J72.9864.A43        (fully known)
//! E: hearts AKxx, HCP 16-17  (AK pinned, length and HCP filtered)
//! S: hearts Qxx3             (Q and 3 pinned, length filtered)
//! ```

use anyhow::Context as _;
use contract_bridge::deck;
use contract_bridge::{Builder, FullDeal, Hand, Seat, Suit, eval};

const TARGET: usize = 10;
const ATTEMPT_CAP: usize = 1_000_000;

fn east_matches(deal: &FullDeal) -> bool {
    let east = deal[Seat::East];
    if east[Suit::Hearts].len() != 4 {
        return false;
    }
    let hcp: u8 = Suit::ASC.iter().map(|&s| eval::hcp::<u8>(east[s])).sum();
    (16..=17).contains(&hcp)
}

fn south_matches(deal: &FullDeal) -> bool {
    deal[Seat::South][Suit::Hearts].len() == 4
}

fn main() -> anyhow::Result<()> {
    let north: Hand = "K92.T4.QJ7.KT975".parse()?;
    let west: Hand = "A73.J72.9864.A43".parse()?;
    let east_known: Hand = ".AK..".parse()?;
    let south_known: Hand = ".Q3..".parse()?;

    let partial = Builder::new()
        .north(north)
        .west(west)
        .east(east_known)
        .south(south_known)
        .build_partial()
        .ok()
        .context("known cards conflict (overlap or >13 in a hand)")?;

    let mut rng = rand::rng();
    let mut printed = 0;
    for (attempt, deal) in deck::fill_deals(&mut rng, partial).enumerate() {
        if east_matches(&deal) && south_matches(&deal) {
            println!("{}", deal.display(Seat::North));
            printed += 1;
            if printed == TARGET {
                return Ok(());
            }
        }
        anyhow::ensure!(
            attempt + 1 < ATTEMPT_CAP,
            "gave up after {ATTEMPT_CAP} attempts; only {printed}/{TARGET} matched"
        );
    }
    unreachable!("fill_deals is an infinite iterator")
}
