use ethers::types::U256;

pub struct AuctionSalt {
    pub auction_start_time: u32,
    pub initial_rate_bump: u32,
    pub duration: u32,
    pub bank_fee: U256,
    pub salt: U256,
}

pub struct AuctionSaltData {
    pub auction_start_time: u32,
    pub initial_rate_bump: u32,
    pub duration: u32,
    pub bank_fee: U256,
    pub salt: Option<U256>,
}
