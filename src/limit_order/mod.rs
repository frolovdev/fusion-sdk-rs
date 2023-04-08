use std::{str::FromStr, vec};

use ethers::types::{
    transaction::eip712::{EIP712Domain, TypedData},
    Address, Bytes, H160, U256,
};

use crate::{
    constants::{ZERO_ADDRESS, ZX},
    limit_order::eip712::order_typed_data_builder::get_limit_order_v3_domain,
    salt::build_salt,
    utils::{cumsum, trim_0x},
};

use self::{
    eip712::order_typed_data_builder::{build_order_data, get_order_hash},
    parser::parse_interactions,
    types::LimitOrderV3Struct,
};

pub mod eip712;
pub mod parser;
pub mod types;

pub struct LimitOrder {
    maker_asset: Address,
    taker_asset: Address,
    making_amount: U256,
    taking_amount: U256,
    from: Address,
    allowed_sender: Address,
    receiver: Address,
    maker_asset_data: Bytes,
    taker_asset_data: Bytes,
    get_making_amount: Bytes,
    get_taking_amount: Bytes,
    predicate: Bytes,
    permit: Bytes,
    pre_interaction: Bytes,
    post_interaction: Bytes,
    salt: U256,
}

pub struct InteractionsData {
    pub maker_asset_data: Option<Bytes>,
    pub taker_asset_data: Option<Bytes>,
    pub get_making_amount: Option<Bytes>,
    pub get_taking_amount: Option<Bytes>,
    pub predicate: Option<Bytes>,
    pub permit: Option<Bytes>,
    pub pre_interaction: Option<Bytes>,
    pub post_interaction: Option<Bytes>,
}

pub struct OrderInfoData {
    maker_asset: Address,
    taker_asset: Address,
    making_amount: U256,
    taking_amount: U256,
    maker: Address,
    salt: Option<U256>,
    allowed_sender: Option<Address>,
    receiver: Option<Address>,
}

impl LimitOrder {
    pub fn new(order_info: &OrderInfoData, interactions: Option<&InteractionsData>) -> Self {
        let interactions = interactions.unwrap_or(&InteractionsData {
            maker_asset_data: None,
            taker_asset_data: None,
            get_making_amount: None,
            get_taking_amount: None,
            predicate: None,
            permit: None,
            pre_interaction: None,
            post_interaction: None,
        });

        LimitOrder {
            maker_asset: order_info.maker_asset,
            taker_asset: order_info.taker_asset,
            making_amount: order_info.making_amount,
            taking_amount: order_info.taking_amount,
            salt: *order_info.salt.as_ref().unwrap_or(&build_salt()),
            from: order_info.maker,
            allowed_sender: *order_info
                .allowed_sender
                .as_ref()
                .unwrap_or(&H160::from_str(ZERO_ADDRESS).unwrap()),
            receiver: *order_info
                .receiver
                .as_ref()
                .unwrap_or(&H160::from_str(ZERO_ADDRESS).unwrap()),
            maker_asset_data: interactions
                .maker_asset_data
                .as_ref()
                .unwrap_or(&Bytes::from_str(ZX).unwrap())
                .clone(),
            taker_asset_data: interactions
                .taker_asset_data
                .as_ref()
                .unwrap_or(&Bytes::from_str(ZX).unwrap())
                .clone(),
            get_making_amount: interactions
                .get_making_amount
                .as_ref()
                .unwrap_or(&Bytes::from_str(ZX).unwrap())
                .clone(),
            get_taking_amount: interactions
                .get_taking_amount
                .as_ref()
                .unwrap_or(&Bytes::from_str(ZX).unwrap())
                .clone(),
            predicate: interactions
                .predicate
                .as_ref()
                .unwrap_or(&Bytes::from_str(ZX).unwrap())
                .clone(),
            permit: interactions
                .permit
                .as_ref()
                .unwrap_or(&Bytes::from_str(ZX).unwrap())
                .clone(),
            pre_interaction: interactions
                .pre_interaction
                .as_ref()
                .unwrap_or(&Bytes::from_str(ZX).unwrap())
                .clone(),
            post_interaction: interactions
                .post_interaction
                .as_ref()
                .unwrap_or(&Bytes::from_str(ZX).unwrap())
                .clone(),
        }
    }

    pub fn get_order_hash_static(
        order: &LimitOrderV3Struct,
        domain: Option<&EIP712Domain>,
    ) -> String {
        let domain = domain
            .unwrap_or(&get_limit_order_v3_domain(&U256::from(1)))
            .to_owned();

        get_order_hash(Self::get_typed_data_static(&order, Some(&domain)))
    }

    pub fn get_typed_data_static(
        order: &LimitOrderV3Struct,
        domain: Option<&EIP712Domain>,
    ) -> TypedData {
        let domain = domain
            .unwrap_or(&get_limit_order_v3_domain(&U256::from(1)))
            .to_owned();
        build_order_data(
            &domain.chain_id.unwrap(),
            &domain.verifying_contract.unwrap(),
            &domain.name.unwrap(),
            &domain.version.unwrap(),
            order,
        )
    }

    pub fn decode(r#struct: &LimitOrderV3Struct) -> Self {
        let interactions = parse_interactions(&r#struct.offsets, &r#struct.interactions);

        let maker_asset_data = interactions.get("maker_asset_data").unwrap().to_owned();
        let taker_asset_data = interactions.get("taker_asset_data").unwrap().to_owned();
        let get_making_amount = interactions.get("get_making_amount").unwrap().to_owned();
        let get_taking_amount = interactions.get("get_taking_amount").unwrap().to_owned();
        let predicate = interactions.get("predicate").unwrap().to_owned();
        let permit = interactions.get("permit").unwrap().to_owned();
        let pre_interaction = interactions.get("pre_interaction").unwrap().to_owned();
        let post_interaction = interactions.get("post_interaction").unwrap().to_owned();

        LimitOrder::new(
            &OrderInfoData {
                maker_asset: r#struct.maker_asset,
                taker_asset: r#struct.taker_asset,
                making_amount: r#struct.making_amount,
                taking_amount: r#struct.taking_amount,
                maker: r#struct.maker,
                salt: Some(r#struct.salt),
                allowed_sender: Some(r#struct.allowed_sender),
                receiver: Some(r#struct.receiver),
            },
            Some(&InteractionsData {
                maker_asset_data: Some(maker_asset_data),
                taker_asset_data: Some(taker_asset_data),
                get_making_amount: Some(get_making_amount),
                get_taking_amount: Some(get_taking_amount),
                predicate: Some(predicate),
                permit: Some(permit),
                pre_interaction: Some(pre_interaction),
                post_interaction: Some(post_interaction),
            }),
        )
    }

    pub fn build(&self) -> LimitOrderV3Struct {
        let all_interactions = vec![
            self.maker_asset_data.clone(),
            self.taker_asset_data.clone(),
            self.get_making_amount.clone(),
            self.get_taking_amount.clone(),
            self.predicate.clone(),
            self.permit.clone(),
            self.pre_interaction.clone(),
            self.post_interaction.clone(),
        ];

        let lengths: Vec<usize> = all_interactions.iter().map(|x| x.len()).collect();
        let sums = cumsum(&lengths);
        let offsets = sums.iter().enumerate().fold(U256::from(0), |acc, (i, x)| {
            acc + (U256::from(*x) << 32 * i)
        });

        LimitOrderV3Struct {
            salt: self.salt,
            maker_asset: self.maker_asset,
            taker_asset: self.taker_asset,
            making_amount: self.making_amount,
            taking_amount: self.taking_amount,
            maker: self.from,
            allowed_sender: self.allowed_sender,
            receiver: self.receiver,
            offsets: offsets,
            interactions: all_interactions.concat().into(),
        }
    }

    pub fn get_typed_data(&self, domain: Option<&EIP712Domain>) -> TypedData {
        let domain = domain
            .unwrap_or(&get_limit_order_v3_domain(&U256::from(1)))
            .to_owned();
        build_order_data(
            &domain.chain_id.unwrap(),
            &domain.verifying_contract.unwrap(),
            &domain.name.unwrap(),
            &domain.version.unwrap(),
            &self.build(),
        )
    }

    pub fn get_order_hash(&self, domain: Option<&EIP712Domain>) -> String {
        let domain = domain
            .unwrap_or(&get_limit_order_v3_domain(&U256::from(1)))
            .to_owned();

        get_order_hash(self.get_typed_data(Some(&domain)))
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::BTreeMap, str::FromStr, vec};

    use ethers::{
        abi::Address,
        types::{
            transaction::eip712::{EIP712Domain, Eip712DomainType, TypedData},
            Bytes, H160, U256,
        },
    };
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use crate::limit_order::types::LimitOrderV3Struct;

    use super::{LimitOrder, OrderInfoData};

    #[test]
    fn should_create_limit_order() {
        let limit_order = LimitOrder::new(
            &OrderInfoData {
                maker_asset: Address::from_str("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2")
                    .unwrap(),
                taker_asset: Address::from_str("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48")
                    .unwrap(),
                making_amount: U256::from(1000000000000000000 as i64),
                taking_amount: U256::from(1420000000 as i64),
                maker: Address::from_str("0x00000000219ab540356cbb839cbe05303d7705fa").unwrap(),
                salt: Some(U256::from(1673549418040 as i64)),
                allowed_sender: None,
                receiver: None,
            },
            None,
        );

        assert_eq!(
            limit_order.build(),
            LimitOrderV3Struct {
                allowed_sender: Address::from_str("0x0000000000000000000000000000000000000000")
                    .unwrap(),
                interactions: Bytes::from_str("0x").unwrap(),
                maker: Address::from_str("0x00000000219ab540356cbb839cbe05303d7705fa").unwrap(),
                maker_asset: Address::from_str("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2")
                    .unwrap(),
                making_amount: U256::from(1000000000000000000 as i64),
                offsets: U256::from(0),
                receiver: Address::from_str("0x0000000000000000000000000000000000000000").unwrap(),
                salt: U256::from(1673549418040 as i64),
                taker_asset: Address::from_str("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48")
                    .unwrap(),
                taking_amount: U256::from(1420000000)
            }
        )
    }

    // fn should_create_limit_order_with_timestamp_below() {}

    // fn should_create_limit_order_with_timestamp_above_that_will_unwrap_maker_weth_to_eth() {}

    #[test]
    fn should_decode_limit_order() {
        let order_struct = LimitOrderV3Struct {
            allowed_sender: Address::from_str("0x0000000000000000000000000000000000000000").unwrap(),
            interactions: Bytes::from_str("0x63592c2b0000000000000000000000000000000000000000000000000000000063c0566a08b067ad41e45babe5bbb52fc2fe7f692f628b0600000000219ab540356cbb839cbe05303d7705fa".trim_start_matches("0x")).unwrap(),
            maker: Address::from_str("0x00000000219ab540356cbb839cbe05303d7705fa").unwrap(),
            maker_asset: Address::from_str("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2").unwrap(),
            making_amount: U256::from(1000000000000000000 as i64),
            offsets: U256::from_dec_str("2048955946929424286921227713067743020696385405755235979139736848564224").unwrap(),
            receiver: Address::from_str("0x0000000000000000000000000000000000000000").unwrap(),
            salt: U256::from(1673549418040 as i64),
            taker_asset: Address::from_str("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48").unwrap(),
            taking_amount: U256::from(1420000000 as i64),
        };

        let order = LimitOrder::decode(&order_struct);

        assert_eq!(order.build(), order_struct)
    }

    #[test]
    fn should_get_limit_order_typed_data() {
        let limit_order = LimitOrder::new(
            &OrderInfoData {
                maker_asset: Address::from_str("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2")
                    .unwrap(),
                taker_asset: Address::from_str("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48")
                    .unwrap(),
                making_amount: U256::from(1000000000000000000 as i64),
                taking_amount: U256::from(1420000000),
                maker: Address::from_str("0x00000000219ab540356cbb839cbe05303d7705fa").unwrap(),
                salt: Some(U256::from(1673549418040 as i64)),
                allowed_sender: None,
                receiver: None,
            },
            None,
        );

        let expected_message: BTreeMap<String, serde_json::Value> = BTreeMap::from([
            (
                "allowedSender".to_string(),
                json!("0x0000000000000000000000000000000000000000"),
            ),
            ("interactions".to_string(), json!("0x")),
            (
                "maker".to_string(),
                json!("0x00000000219ab540356cbb839cbe05303d7705fa"),
            ),
            (
                "makerAsset".to_string(),
                json!("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"),
            ),
            ("makingAmount".to_string(), json!("1000000000000000000")),
            ("offsets".to_string(), json!("0")),
            (
                "receiver".to_string(),
                json!("0x0000000000000000000000000000000000000000"),
            ),
            ("salt".to_string(), json!("1673549418040")),
            (
                "takerAsset".to_string(),
                json!("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48"),
            ),
            ("takingAmount".to_string(), json!("1420000000")),
        ]);

        let expected_types: BTreeMap<String, Vec<Eip712DomainType>> = BTreeMap::from([
            (
                "EIP712Domain".to_string(),
                vec![
                    Eip712DomainType {
                        name: "name".to_string(),
                        r#type: "string".to_string(),
                    },
                    Eip712DomainType {
                        name: "version".to_string(),
                        r#type: "string".to_string(),
                    },
                    Eip712DomainType {
                        name: "chainId".to_string(),
                        r#type: "uint256".to_string(),
                    },
                    Eip712DomainType {
                        name: "verifyingContract".to_string(),
                        r#type: "address".to_string(),
                    },
                ],
            ),
            (
                "Order".to_string(),
                vec![
                    Eip712DomainType {
                        name: "salt".to_string(),
                        r#type: "uint256".to_string(),
                    },
                    Eip712DomainType {
                        name: "makerAsset".to_string(),
                        r#type: "address".to_string(),
                    },
                    Eip712DomainType {
                        name: "takerAsset".to_string(),
                        r#type: "address".to_string(),
                    },
                    Eip712DomainType {
                        name: "maker".to_string(),
                        r#type: "address".to_string(),
                    },
                    Eip712DomainType {
                        name: "receiver".to_string(),
                        r#type: "address".to_string(),
                    },
                    Eip712DomainType {
                        name: "allowedSender".to_string(),
                        r#type: "address".to_string(),
                    },
                    Eip712DomainType {
                        name: "makingAmount".to_string(),
                        r#type: "uint256".to_string(),
                    },
                    Eip712DomainType {
                        name: "takingAmount".to_string(),
                        r#type: "uint256".to_string(),
                    },
                    Eip712DomainType {
                        name: "offsets".to_string(),
                        r#type: "uint256".to_string(),
                    },
                    Eip712DomainType {
                        name: "interactions".to_string(),
                        r#type: "bytes".to_string(),
                    },
                ],
            ),
        ]);

        let expected = TypedData {
            domain: EIP712Domain {
                name: Some("1inch Aggregation Router".to_string()),
                version: Some("5".to_string()),
                chain_id: Some(U256::from(1)),
                verifying_contract: Some(
                    H160::from_str("0x1111111254eeb25477b68fb85ed929f73a960582").unwrap(),
                ),
                salt: None,
            },
            primary_type: "Order".to_string(),
            types: expected_types,
            message: expected_message,
        };

        assert_eq!(limit_order.get_typed_data(None), expected);
    }

    #[test]
    fn should_get_limit_order_hash() {
        let limit_order = LimitOrder::new(
            &OrderInfoData {
                maker_asset: Address::from_str("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2")
                    .unwrap(),
                taker_asset: Address::from_str("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48")
                    .unwrap(),
                making_amount: U256::from(1000000000000000000 as i64),
                taking_amount: U256::from(1420000000),
                maker: Address::from_str("0x00000000219ab540356cbb839cbe05303d7705fa").unwrap(),
                salt: Some(U256::from(1673549418040 as i64)),
                allowed_sender: None,
                receiver: None,
            },
            None,
        );

        assert_eq!(
            limit_order.get_order_hash(None),
            "0x4bdb758d3d4b265367c461cdb12b2fbe92fd8f2bcc9423393e9da4490d6157c4"
        );
    }
}
