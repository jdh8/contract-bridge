use contract_bridge::auction::{Auction, Call, IllegalCall, RelativeVulnerability};
use contract_bridge::{Bid, Level, Penalty, Strain};

const fn bid(level: u8, strain: Strain) -> Call {
    Call::Bid(Bid {
        level: Level::new(level),
        strain,
    })
}

#[test]
fn test_auction_new_is_empty() {
    let auction = Auction::new();
    assert!(auction.is_empty());
    assert!(!auction.has_ended());
}

#[test]
fn test_auction_pass_out() {
    let mut auction = Auction::new();
    for _ in 0..4 {
        auction.push(Call::Pass);
    }
    assert!(auction.has_ended());
    assert_eq!(auction.declarer(), None);
}

#[test]
fn test_auction_three_passes_not_ended() {
    let mut auction = Auction::new();
    for _ in 0..3 {
        auction.push(Call::Pass);
    }
    assert!(!auction.has_ended());
}

#[test]
fn test_auction_simple_bid_sequence() {
    let mut auction = Auction::new();
    auction.push(bid(1, Strain::Clubs));
    auction.push(Call::Pass);
    auction.push(Call::Pass);
    auction.push(Call::Pass);
    assert!(auction.has_ended());
    assert_eq!(auction.declarer(), Some(0));
}

#[test]
fn test_auction_declarer_same_strain_partner() {
    // Dealer bids 1C, partner raises to 2C -> declarer is dealer (index 0)
    let mut auction = Auction::new();
    auction.push(bid(1, Strain::Clubs)); // index 0 (dealer)
    auction.push(Call::Pass); // index 1
    auction.push(bid(2, Strain::Clubs)); // index 2 (partner)
    auction.push(Call::Pass);
    auction.push(Call::Pass);
    auction.push(Call::Pass);
    assert_eq!(auction.declarer(), Some(0));
}

#[test]
fn test_auction_declarer_different_strain() {
    // Pass, 1H, Pass, 2H -> declarer is index 1 (first to bid hearts)
    let mut auction = Auction::new();
    auction.push(Call::Pass); // index 0
    auction.push(bid(1, Strain::Hearts)); // index 1
    auction.push(Call::Pass); // index 2
    auction.push(bid(2, Strain::Hearts)); // index 3
    auction.push(Call::Pass);
    auction.push(Call::Pass);
    auction.push(Call::Pass);
    assert_eq!(auction.declarer(), Some(1));
}

#[test]
fn test_auction_double() -> Result<(), IllegalCall> {
    let mut auction = Auction::new();
    auction.push(bid(1, Strain::Clubs));
    auction.try_push(Call::Double)?;
    auction.push(Call::Pass);
    auction.push(Call::Pass);
    auction.push(Call::Pass);
    assert!(auction.has_ended());
    Ok(())
}

#[test]
fn test_auction_redouble() -> Result<(), IllegalCall> {
    let mut auction = Auction::new();
    auction.push(bid(1, Strain::Clubs));
    auction.push(Call::Double);
    auction.try_push(Call::Redouble)?;
    auction.push(Call::Pass);
    auction.push(Call::Pass);
    auction.push(Call::Pass);
    assert!(auction.has_ended());
    Ok(())
}

#[test]
fn test_auction_insufficient_bid() {
    let mut auction = Auction::new();
    auction.push(bid(2, Strain::Clubs));
    let result = auction.try_push(bid(1, Strain::Hearts));
    assert!(matches!(result, Err(IllegalCall::InsufficientBid { .. })));
}

#[test]
fn test_auction_equal_bid_is_insufficient() {
    let mut auction = Auction::new();
    auction.push(bid(1, Strain::Clubs));
    let result = auction.try_push(bid(1, Strain::Clubs));
    assert!(matches!(result, Err(IllegalCall::InsufficientBid { .. })));
}

#[test]
fn test_auction_double_own_bid_is_inadmissible() {
    let mut auction = Auction::new();
    auction.push(bid(1, Strain::Clubs));
    auction.push(Call::Pass);
    let result = auction.try_push(Call::Double);
    assert!(matches!(
        result,
        Err(IllegalCall::InadmissibleDouble(Penalty::Doubled))
    ));
}

#[test]
fn test_auction_double_without_bid() {
    let mut auction = Auction::new();
    let result = auction.try_push(Call::Double);
    assert!(matches!(
        result,
        Err(IllegalCall::InadmissibleDouble(Penalty::Doubled))
    ));
}

#[test]
fn test_auction_redouble_without_double() {
    let mut auction = Auction::new();
    auction.push(bid(1, Strain::Clubs));
    let result = auction.try_push(Call::Redouble);
    assert!(matches!(
        result,
        Err(IllegalCall::InadmissibleDouble(Penalty::Redoubled))
    ));
}

#[test]
fn test_auction_call_after_final_pass() {
    let mut auction = Auction::new();
    auction.push(bid(1, Strain::Clubs));
    auction.push(Call::Pass);
    auction.push(Call::Pass);
    auction.push(Call::Pass);
    let result = auction.try_push(Call::Pass);
    assert_eq!(result, Err(IllegalCall::AfterFinalPass));
}

#[test]
fn test_auction_pop() {
    let mut auction = Auction::new();
    auction.push(bid(1, Strain::Clubs));
    assert_eq!(auction.pop(), Some(bid(1, Strain::Clubs)));
    assert!(auction.is_empty());
    assert_eq!(auction.pop(), None);
}

#[test]
fn test_auction_truncate() {
    let mut auction = Auction::new();
    auction.push(bid(1, Strain::Clubs));
    auction.push(Call::Pass);
    auction.push(bid(2, Strain::Clubs));
    auction.truncate(1);
    assert_eq!(auction.len(), 1);
}

#[test]
fn test_auction_try_extend() -> Result<(), IllegalCall> {
    let mut auction = Auction::new();
    auction.try_extend([bid(1, Strain::Clubs), Call::Pass, Call::Pass, Call::Pass])?;
    assert!(auction.has_ended());
    Ok(())
}

#[test]
fn test_auction_try_extend_partial_failure() {
    let mut auction = Auction::new();
    let result = auction.try_extend([
        bid(2, Strain::Clubs),
        bid(1, Strain::Hearts), // insufficient
    ]);
    assert!(result.is_err());
    // The first bid should still be in the auction
    assert_eq!(auction.len(), 1);
}

#[test]
fn test_auction_into_vec() {
    let mut auction = Auction::new();
    auction.push(Call::Pass);
    auction.push(bid(1, Strain::Clubs));
    let v: Vec<Call> = auction.into();
    assert_eq!(v, vec![Call::Pass, bid(1, Strain::Clubs)]);
}

#[test]
fn test_auction_into_iter() {
    let mut auction = Auction::new();
    auction.push(Call::Pass);
    auction.push(bid(1, Strain::Clubs));
    let calls: Vec<_> = auction.into_iter().collect();
    assert_eq!(calls, vec![Call::Pass, bid(1, Strain::Clubs)]);
}

#[test]
fn test_call_from_bid() {
    let b = Bid {
        level: Level::new(1),
        strain: Strain::Clubs,
    };
    let call: Call = b.into();
    assert_eq!(call, Call::Bid(b));
}

#[test]
fn test_relative_vulnerability_constants() {
    assert_eq!(RelativeVulnerability::NONE, RelativeVulnerability::empty());
    assert_eq!(RelativeVulnerability::ALL, RelativeVulnerability::all());
    assert!(RelativeVulnerability::ALL.contains(RelativeVulnerability::WE));
    assert!(RelativeVulnerability::ALL.contains(RelativeVulnerability::THEY));
}
