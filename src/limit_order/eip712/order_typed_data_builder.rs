use ethers::types::transaction::eip712::{EIP712Domain, Eip712DomainType};
use ethers::types::transaction::eip712::{TypedData};
use ethers::types::{H160, U256};
use std::collections::BTreeMap;
use std::str::FromStr;

use crate::limit_order::types::LimitOrderV3Struct;

use super::domain::{eip712_domain_type, order_type};

pub fn get_order_hash() {}

pub fn build_order_data(
    chainId: &U256,
    verifying_contract: &str,
    name: &str,
    version: &str,
    order: LimitOrderV3Struct,
) -> TypedData {
    let domain = EIP712Domain {
        chain_id: Some(chainId.clone()),
        verifying_contract: Some(H160::from_str(verifying_contract).unwrap()),
        name: Some(name.to_string()),
        version: Some(version.to_string()),
        salt: None,
    };
    let types: BTreeMap<String, Vec<Eip712DomainType>> = BTreeMap::new();

    types.insert("EIP712Domain".to_string(), eip712_domain_type());
    types.insert("Order".to_string(), order_type());

    let message: BTreeMap<String, serde_json::Value> = BTreeMap::new();

    TypedData {
        primary_type: "Order".to_string(),
        types,
        domain,
        message: order,
    }
}
