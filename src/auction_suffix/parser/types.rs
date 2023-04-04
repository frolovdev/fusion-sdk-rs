use ethers::types::U256;

#[derive(Debug)]
pub struct AuctionWhitelistItem {
    pub address: String,
    pub allowance: u64,
}

#[derive(Debug)]
pub struct ResolverWhitelist {
    pub whitelist: Vec<AuctionWhitelistItem>,
    pub interactions: String,
}

#[derive(Debug)]
pub struct InteractionAdditionalInfo {
    pub whitelist: Vec<AuctionWhitelistItem>,
    pub public_resolving_deadline: u64,
    pub taker_fee_receiver: String,
    pub taker_fee_ratio: U256,
    pub points: Vec<AuctionPoint>,
}

#[derive(Debug)]
pub struct InteractionFlags {
    pub taking_fee_enabled: bool,
    pub resolvers_count: u8,
    pub points_count: u8,
}

#[derive(Debug)]
pub struct TakerFeeData {
    pub taker_fee_ratio: U256,
    pub taker_fee_receiver: String,
    pub interactions: String,
}

#[derive(Debug)]
pub struct PrivateAuctionDeadline {
    pub deadline: u64,
    pub interactions: String,
}

#[derive(Debug)]
pub struct AuctionPoint {
    pub delay: u64,
    pub coefficient: u32,
}

#[derive(Debug)]
pub struct ParsedAuctionParams {
    pub interactions: String,
    pub points: Vec<AuctionPoint>,
}
