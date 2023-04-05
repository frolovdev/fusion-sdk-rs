use ethers::types::transaction::eip712::{Eip712, EIP712_DOMAIN_TYPE_HASH};

pub struct LimitOrderV3Struct {
    salt: String,
    maker_asset: String,
    taker_asset: String,
    maker: String,
    receiver: String,
    allowed_sender: String,
    making_amount: String,
    taking_amount: String,
    offsets: String,
    interactions: String,
}

pub fn kekek() {
    // let kek = Eip712::type_hash() {
}
