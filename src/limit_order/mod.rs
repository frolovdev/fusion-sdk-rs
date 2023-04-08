use std::{borrow::Borrow, vec};

use ethers::types::{
    transaction::eip712::{EIP712Domain, TypedData},
    U256,
};

use crate::{
    constants::{ZERO_ADDRESS, ZX},
    limit_order::eip712::order_typed_data_builder::get_limit_order_v3_domain,
    utils::{cumsum, trim_0x},
};

use self::{
    eip712::order_typed_data_builder::{build_order_data, get_order_hash},
    parser::parse_interactions,
    types::LimitOrderV3Struct,
    utils::build_salt,
};

pub mod eip712;
pub mod parser;
pub mod types;
mod utils;

pub struct LimitOrder {
    maker_asset: String,
    taker_asset: String,
    making_amount: String,
    taking_amount: String,
    from: String,
    allowed_sender: String,
    receiver: String,
    maker_asset_data: String,
    taker_asset_data: String,
    get_making_amount: String,
    get_taking_amount: String,
    predicate: String,
    permit: String,
    pre_interaction: String,
    post_interaction: String,
    salt: String,
}

pub struct InteractionsData {
    pub maker_asset_data: Option<String>,
    pub taker_asset_data: Option<String>,
    pub get_making_amount: Option<String>,
    pub get_taking_amount: Option<String>,
    pub predicate: Option<String>,
    pub permit: Option<String>,
    pub pre_interaction: Option<String>,
    pub post_interaction: Option<String>,
}

pub struct OrderInfoData {
    maker_asset: String,
    taker_asset: String,
    making_amount: String,
    taking_amount: String,
    maker: String,
    salt: Option<String>,
    allowed_sender: Option<String>,
    receiver: Option<String>,
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
            maker_asset: order_info.maker_asset.to_string(),
            taker_asset: order_info.taker_asset.to_string(),
            making_amount: order_info.making_amount.to_string(),
            taking_amount: order_info.taking_amount.to_string(),
            salt: order_info
                .salt
                .as_ref()
                .unwrap_or(build_salt().borrow())
                .to_string(),
            from: order_info.maker.to_string(),
            allowed_sender: order_info
                .allowed_sender
                .as_ref()
                .unwrap_or(&ZERO_ADDRESS.to_string())
                .to_string(),
            receiver: order_info
                .receiver
                .as_ref()
                .unwrap_or(&ZERO_ADDRESS.to_string())
                .to_string(),
            maker_asset_data: interactions
                .maker_asset_data
                .as_ref()
                .unwrap_or(&ZX.to_string())
                .to_string(),
            taker_asset_data: interactions
                .taker_asset_data
                .as_ref()
                .unwrap_or(&ZX.to_string())
                .to_string(),
            get_making_amount: interactions
                .get_making_amount
                .as_ref()
                .unwrap_or(&ZX.to_string())
                .to_string(),
            get_taking_amount: interactions
                .get_taking_amount
                .as_ref()
                .unwrap_or(&ZX.to_string())
                .to_string(),
            predicate: interactions
                .predicate
                .as_ref()
                .unwrap_or(&ZX.to_string())
                .to_string(),
            permit: interactions
                .permit
                .as_ref()
                .unwrap_or(&ZX.to_string())
                .to_string(),
            pre_interaction: interactions
                .pre_interaction
                .as_ref()
                .unwrap_or(&ZX.to_string())
                .to_string(),
            post_interaction: interactions
                .post_interaction
                .as_ref()
                .unwrap_or(&ZX.to_string())
                .to_string(),
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

        let maker_asset_data = interactions.get("makerAssetData").unwrap().to_owned();
        let taker_asset_data = interactions.get("takerAssetData").unwrap().to_owned();
        let get_making_amount = interactions.get("getMakingAmount").unwrap().to_owned();
        let get_taking_amount = interactions.get("getTakingAmount").unwrap().to_owned();
        let predicate = interactions.get("predicate").unwrap().to_owned();
        let permit = interactions.get("permit").unwrap().to_owned();
        let pre_interaction = interactions.get("preInteraction").unwrap().to_owned();
        let post_interaction = interactions.get("postInteraction").unwrap().to_owned();

        LimitOrder::new(
            &OrderInfoData {
                maker_asset: r#struct.maker_asset.to_string(),
                taker_asset: r#struct.taker_asset.to_string(),
                making_amount: r#struct.making_amount.to_string(),
                taking_amount: r#struct.taking_amount.to_string(),
                maker: r#struct.maker.to_string(),
                salt: Some(r#struct.salt.to_string()),
                allowed_sender: Some(r#struct.allowed_sender.to_string()),
                receiver: Some(r#struct.receiver.to_string()),
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
            self.maker_asset_data.to_string(),
            self.taker_asset_data.to_string(),
            self.get_making_amount.to_string(),
            self.get_taking_amount.to_string(),
            self.predicate.to_string(),
            self.permit.to_string(),
            self.pre_interaction.to_string(),
            self.post_interaction.to_string(),
        ];

        let lengths = all_interactions
            .iter()
            .map(|x| ((x.len() / 2) - 1) as u64)
            .collect::<Vec<u64>>();

        let offsets = cumsum(&lengths)
            .iter()
            .fold(U256::from(0), |acc, x| acc + U256::from(*x));

        let trimmed_all_interactions = all_interactions
            .iter()
            .map(|x| trim_0x(x.as_str()))
            .collect::<Vec<&str>>();
        let interactions: String = ZX.to_owned() + &trimmed_all_interactions.join("");

        LimitOrderV3Struct {
            salt: self.salt.to_string(),
            maker_asset: self.maker_asset.to_string(),
            taker_asset: self.taker_asset.to_string(),
            making_amount: self.making_amount.to_string(),
            taking_amount: self.taking_amount.to_string(),
            maker: self.from.to_string(),
            allowed_sender: self.allowed_sender.to_string(),
            receiver: self.receiver.to_string(),
            offsets: offsets.to_string(),
            interactions,
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
    use crate::limit_order::types::LimitOrderV3Struct;

    use super::{LimitOrder, OrderInfoData};

    #[test]
    fn should_create_limit_order() {
        let limit_order = LimitOrder::new(
            &OrderInfoData {
                maker_asset: "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2".to_string(),
                taker_asset: "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48".to_string(),
                making_amount: "1000000000000000000".to_string(),
                taking_amount: "1420000000".to_string(),
                maker: "0x00000000219ab540356cbb839cbe05303d7705fa".to_string(),
                salt: Some("1673549418040".to_string()),
                allowed_sender: None,
                receiver: None,
            },
            None,
        );

        assert_eq!(
            limit_order.build(),
            LimitOrderV3Struct {
                allowed_sender: "0x0000000000000000000000000000000000000000".to_string(),
                interactions: "0x".to_string(),
                maker: "0x00000000219ab540356cbb839cbe05303d7705fa".to_string(),
                maker_asset: "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2".to_string(),
                making_amount: "1000000000000000000".to_string(),
                offsets: "0".to_string(),
                receiver: "0x0000000000000000000000000000000000000000".to_string(),
                salt: "1673549418040".to_string(),
                taker_asset: "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48".to_string(),
                taking_amount: "1420000000".to_string()
            }
        )
    }

    fn should_create_limit_order_with_timestamp_below() {}

    fn should_create_limit_order_with_timestamp_above_that_will_unwrap_maker_weth_to_eth() {}

    fn should_decode_limit_order() {}

    fn should_get_limit_order_typed_data() {}

    #[test]
    fn should_get_limit_order_hash() {
        // const order = new LimitOrder({
        //     makerAsset: '0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2',
        //     takerAsset: '0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48',
        //     makingAmount: '1000000000000000000',
        //     takingAmount: '1420000000',
        //     maker: '0x00000000219ab540356cbb839cbe05303d7705fa'
        // })

        // expect(order.getOrderHash()).toBe(
        //     '0x4bdb758d3d4b265367c461cdb12b2fbe92fd8f2bcc9423393e9da4490d6157c4'
        // )

        let limit_order = LimitOrder::new(
            &OrderInfoData {
                maker_asset: "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2".to_string(),
                taker_asset: "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48".to_string(),
                making_amount: "1000000000000000000".to_string(),
                taking_amount: "1420000000".to_string(),
                maker: "0x00000000219ab540356cbb839cbe05303d7705fa".to_string(),
                salt: Some("1673549418040".to_string()),
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
