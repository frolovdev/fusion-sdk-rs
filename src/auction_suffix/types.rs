use ethers::types::U256;

use super::parser::types::AuctionPoint;
use super::parser::types::AuctionWhitelistItem;
#[derive(Debug)]
pub struct AuctionSuffix {
    pub points: Vec<AuctionPoint>,
    pub whitelist: Vec<AuctionWhitelistItem>,
    pub public_resolving_deadline: u64,
    pub taker_fee_receiver: String,
    pub taker_fee_ratio: U256,
}

#[derive(Debug)]
pub struct TakingFee {
    pub taking_fee_ratio: U256,
    pub taking_fee_receiver: String,
}

#[derive(Debug)]
pub struct SettlementSuffixData {
    pub points: Vec<AuctionPoint>,
    pub whitelist: Vec<AuctionWhitelistItem>,
    pub public_resolving_deadline: Option<u64>,
    pub fee: Option<TakingFee>,
}
