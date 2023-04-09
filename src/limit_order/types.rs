use ethers::types::{Address, Bytes, U256};
use serde_json::json;
use std::collections::BTreeMap;
use struct_field_names_as_array::FieldNamesAsArray;

#[derive(FieldNamesAsArray, Debug, PartialEq)]
pub struct LimitOrderV3Struct {
    pub salt: U256,
    pub maker_asset: Address,
    pub taker_asset: Address,
    pub maker: Address,
    pub receiver: Address,
    pub allowed_sender: Address,
    pub making_amount: U256,
    pub taking_amount: U256,
    pub offsets: U256,
    pub interactions: Bytes,
}

pub trait ToBtreeMap {
    fn to_btree_map(&self) -> BTreeMap<String, serde_json::Value>;
}

impl ToBtreeMap for LimitOrderV3Struct {
    fn to_btree_map(&self) -> BTreeMap<String, serde_json::Value> {
        let mut map = BTreeMap::new();

        map.insert("salt".to_string(), json!(self.salt.to_string()));
        map.insert("makerAsset".to_string(), json!(self.maker_asset));
        map.insert("takerAsset".to_string(), json!(self.taker_asset));
        map.insert("maker".to_string(), json!(self.maker));
        map.insert("receiver".to_string(), json!(self.receiver));
        map.insert("allowedSender".to_string(), json!(self.allowed_sender));
        map.insert(
            "makingAmount".to_string(),
            json!(self.making_amount.to_string()),
        );
        map.insert(
            "takingAmount".to_string(),
            json!(self.taking_amount.to_string()),
        );
        map.insert("offsets".to_string(), json!(self.offsets.to_string()));
        map.insert("interactions".to_string(), json!(self.interactions));

        if map.len() != LimitOrderV3Struct::FIELD_NAMES_AS_ARRAY.len() {
            panic!("Not all fields were serialized");
        }

        map
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::utils::trim_0x;
    use pretty_assertions::assert_eq;

    #[test]
    fn success() {
        let salt = U256::from(0);
        let maker_asset = Address::from_str("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2").unwrap();
        let taker_asset = Address::from_str("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48").unwrap();
        let maker = Address::from_str("0x00000000219ab540356cbb839cbe05303d7705fa").unwrap();
        let receiver = Address::from_str("0x0000000000000000000000000000000000000000").unwrap();
        let allowed_sender =
            Address::from_str("0x0000000000000000000000000000000000000000").unwrap();
        let making_amount = U256::from(1000000000000000000 as i64);
        let taking_amount = U256::from(1420000000);

        let offsets = U256::from(0);

        let interactions = Bytes::from(hex::decode(trim_0x("0x3a7f2c8b1d4e6f")).unwrap());

        let limit_order = LimitOrderV3Struct {
            salt: salt.clone(),
            maker_asset: maker_asset.clone(),
            taker_asset: taker_asset.clone(),
            maker: maker.clone(),
            receiver: receiver.clone(),
            allowed_sender: allowed_sender.clone(),
            making_amount: making_amount.clone(),
            taking_amount: taking_amount.clone(),
            offsets: offsets.clone(),
            interactions: interactions.clone(),
        };

        let map = limit_order.to_btree_map();

        let expected_map = BTreeMap::from([
            ("salt".to_string(), json!(salt.to_string())),
            ("makerAsset".to_string(), json!(maker_asset)),
            ("takerAsset".to_string(), json!(taker_asset)),
            ("maker".to_string(), json!(maker)),
            ("receiver".to_string(), json!(receiver)),
            ("allowedSender".to_string(), json!(allowed_sender)),
            ("makingAmount".to_string(), json!(making_amount.to_string())),
            ("takingAmount".to_string(), json!(taking_amount.to_string())),
            ("offsets".to_string(), json!(offsets.to_string())),
            ("interactions".to_string(), json!(interactions)),
        ]);

        assert_eq!(map, expected_map);
    }
}
