use ethers::types::transaction::eip712::Eip712DomainType;
use serde_json::{json};

pub fn eip712_domain_type() -> Vec<Eip712DomainType> {
    let eip712_domain_type_json = json!([
        {"name": "name", "type": "string"},
        {"name": "version", "type": "string"},
        {"name": "chainId", "type": "uint256"},
        {"name": "verifyingContract", "type": "address"}
    ]);

    let eip712_domain_type: Vec<Eip712DomainType> =
        serde_json::from_value(eip712_domain_type_json).unwrap();

    eip712_domain_type
}

pub fn order_type() -> Vec<Eip712DomainType> {
    let domain_types_json = json!([
        {"name": "salt", "type": "uint256"},
        {"name": "makerAsset", "type": "address"},
        {"name": "takerAsset", "type": "address"},
        {"name": "maker", "type": "address"},
        {"name": "receiver", "type": "address"},
        {"name": "allowedSender", "type": "address"},
        {"name": "makingAmount", "type": "uint256"},
        {"name": "takingAmount", "type": "uint256"},
        {"name": "offsets", "type": "uint256"},
        {"name": "interactions", "type": "bytes"}
    ]);

    let domain_types: Vec<Eip712DomainType> = serde_json::from_value(domain_types_json).unwrap();

    domain_types
}

pub const LIMIT_ORDER_V3_TYPE_DATA_NAME: &str = "1inch Aggregation Router";
pub const LIMIT_ORDER_V3_TYPE_DATA_VERSION: &str = "5";
