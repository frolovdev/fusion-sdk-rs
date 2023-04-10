use ethers::types::{Address, Bytes, U256};

#[derive(Debug, Clone, PartialEq)]
pub struct AuctionWhitelistItem {
    pub address: Address,
    pub allowance: u32, // unix timestamp
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResolverWhitelist {
    pub whitelist: Vec<AuctionWhitelistItem>,
    pub interactions: Bytes,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InteractionAdditionalInfo {
    pub whitelist: Vec<AuctionWhitelistItem>,
    pub public_resolving_deadline: u32, // unix timestamp
    pub taker_fee_receiver: Address,
    pub taker_fee_ratio: U256,
    pub points: Vec<AuctionPoint>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InteractionFlags {
    pub taking_fee_enabled: bool,
    pub resolvers_count: usize,
    pub points_count: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TakerFeeData {
    pub taker_fee_ratio: U256,
    pub taker_fee_receiver: Address,
    pub interactions: Bytes,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PrivateAuctionDeadline {
    pub deadline: u32, // unix timestamp
    pub interactions: Bytes,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AuctionPoint {
    pub delay: u16, // seconds
    pub coefficient: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParsedAuctionParams {
    pub interactions: Bytes,
    pub points: Vec<AuctionPoint>,
}
