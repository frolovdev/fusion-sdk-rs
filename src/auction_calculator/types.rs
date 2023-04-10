use ethers::types::U256;

use crate::auction_suffix::parser::types::AuctionPoint;

#[derive(Debug, Clone, PartialEq)]
pub struct AuctionCalculator {
    pub start_time: u64,
    pub duration: u32,
    pub initial_rate_bump: u32,
    pub points: Vec<AuctionPoint>,
    pub taker_fee_ratio: U256,
}
