use ethers::types::U256;

pub struct AuctionWhitelistItem {
    pub address: String,
    pub allowance: u64,
}

pub struct ResolverWhitelist {
    pub whitelist: Vec<AuctionWhitelistItem>,
    pub interactions: String,
}

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

pub struct TakerFeeData {
    pub taker_fee_ratio: U256,
    pub taker_fee_receiver: String,
    pub interactions: String,
}

pub struct PrivateAuctionDeadline {
    pub deadline: u64,
    pub interactions: String,
}

pub struct AuctionPoint {
    pub delay: u32,
    pub coefficient: u32,
}

pub struct ParsedAuctionParams {
    pub interactions: String,
    pub points: Vec<AuctionPoint>,
}
