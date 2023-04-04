use ethers::types::U256;

use crate::auction_suffix::parser::types::AuctionPoint;

pub struct AuctionCalculator {
    start_time: u64,
    duration: u32,
    initial_rate_bump: u32,
    points: Vec<AuctionPoint>,
    taker_fee_ratio: U256,
}