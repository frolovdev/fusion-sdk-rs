use ethers::types::{Address, Bytes, U256};

#[derive(Debug)]
pub struct AuctionWhitelistItem {
    pub address: Address,
    pub allowance: u32, //
}

#[derive(Debug)]
pub struct ResolverWhitelist {
    pub whitelist: Vec<AuctionWhitelistItem>,
    pub interactions: Bytes,
}

#[derive(Debug)]
pub struct InteractionAdditionalInfo {
    pub whitelist: Vec<AuctionWhitelistItem>,
    pub public_resolving_deadline: U256, // unix timestamp (u16)
    pub taker_fee_receiver: Address,
    pub taker_fee_ratio: U256,
    pub points: Vec<AuctionPoint>,
}

#[derive(Debug)]
pub struct InteractionFlags {
    pub taking_fee_enabled: bool,
    pub resolvers_count: usize,
    pub points_count: usize,
}

#[derive(Debug)]
pub struct TakerFeeData {
    pub taker_fee_ratio: U256,
    pub taker_fee_receiver: Address,
    pub interactions: Bytes,
}

#[derive(Debug)]
pub struct PrivateAuctionDeadline {
    pub deadline: U256, // unix timestamp (u16)
    pub interactions: Bytes,
}

#[derive(Debug)]
pub struct AuctionPoint {
    pub delay: u16,       // seconds
    pub coefficient: u32, // 1 >> 24
}

#[derive(Debug)]
pub struct ParsedAuctionParams {
    pub interactions: Bytes,
    pub points: Vec<AuctionPoint>,
}
