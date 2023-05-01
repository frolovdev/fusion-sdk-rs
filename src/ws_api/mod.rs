use std::pin::Pin;
use std::str::FromStr;
pub mod types;

use chrono::Utc;
use ethers::abi::Hash;
use ethers::types::{Signature, U256};
use futures_util::{Future, StreamExt};
use serde_json::Value;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

use crate::constants::NetworkEnum;
use crate::limit_order::types::LimitOrderV3Struct;

use self::types::{OrderEvent, OrderEventCreated, RpcMessage};

pub struct WebSocketApi {
    url: Url,
    subscribers: Vec<FutureOrValue>,
}

enum FutureOrValue {
    Future(Box<dyn Fn(RpcMessage) -> Pin<Box<(dyn Future<Output = ()>)>>>),
    Value(Box<dyn Fn(RpcMessage)>),
}

pub struct WebSocketApiConfig {
    pub url: String,
    pub network: NetworkEnum,
}

impl WebSocketApi {
    pub fn new(config: &WebSocketApiConfig) -> Self {
        let config_url_with_network = format!("{}/{}/{}", &config.url, "v1.0", &config.network);

        let url: Url = Url::parse(&config_url_with_network).expect("Invalid WebSocket URL");

        Self {
            url,
            subscribers: vec![],
        }
    }

    pub async fn connect(&mut self) {
        let (ws_stream, _) = connect_async(self.url.clone())
            .await
            .expect("Failed to connect to WebSocket server");

        let (_, mut read) = ws_stream.split();

        while let Some(msg) = read.next().await {
            let msg = msg.expect("Failed to read message from WebSocket server");

            let msg_parsed = match msg {
                Message::Text(text) => parse_text_message(&text),
                Message::Binary(_) => panic!("Received binary message"),
                _ => {
                    println!("Received unknown message");
                    continue;
                }
            };

            for subscriber in self.subscribers.iter() {
                match subscriber {
                    FutureOrValue::Future(r#fn) => r#fn(msg_parsed.clone()).await,
                    FutureOrValue::Value(r#fn) => r#fn(msg_parsed.clone()),
                }
            }
        }
    }

    pub fn on_message<F>(&mut self, handler: F)
    where
        F: Fn(RpcMessage) + Clone + Send + Sync + 'static,
    {
        self.subscribers
            .push(FutureOrValue::Value(Box::new(handler)));
    }

    pub fn on_order_created<F>(&mut self, handler: F)
    where
        F: Fn(OrderEventCreated) + Clone + Send + Sync + 'static,
    {
        self.on_message(move |message| {
            if let RpcMessage::OrderEvent(OrderEvent::Created(msg)) = message {
                handler(msg);
            }
        });
    }

    pub fn on_message_async<F>(&mut self, handler: F)
    where
        F: Fn(RpcMessage) -> Pin<Box<(dyn Future<Output = ()>)>> + Clone + Send + Sync + 'static,
    {
        self.subscribers
            .push(FutureOrValue::Future(Box::new(handler)));
    }

    pub fn on_order_created_async<F>(&mut self, handler: F)
    where
        F: Fn(OrderEventCreated) -> Pin<Box<(dyn Future<Output = ()>)>>
            + Clone
            + Send
            + Sync
            + 'static,
    {
        self.on_message_async(move |message| {
            let handler = handler.clone();
            Box::pin(async move {
                if let RpcMessage::OrderEvent(OrderEvent::Created(msg)) = message {
                    handler(msg);
                }
            })
        });
    }
}

fn parse_text_message(text: &str) -> RpcMessage {
    let value: Value = serde_json::from_str(text).unwrap();

    let event_type = value.get("event").expect("No event type found");

    let msg_parsed = match event_type.as_str().unwrap() {
        "order_created" => {
            let result = value.get("result").expect("No result found");

            let hash = result.get("orderHash").expect("No hash found");
            let signature = result.get("signature").expect("No signature found");

            let deadline = result.get("deadline").expect("No deadline found");

            let auction_start_date = result
                .get("auctionStartDate")
                .expect("No auction_start_date found");

            let auction_end_date = result
                .get("auctionEndDate")
                .expect("No auction_end_date found");

            let remaining_maker_amount = result
                .get("remainingMakerAmount")
                .expect("No remaining_maker_amount found");

            let order = result.get("order").expect("No order found");

            RpcMessage::OrderEvent(OrderEvent::Created(OrderEventCreated {
                order_hash: Hash::from_str(hash.as_str().unwrap()).unwrap(),
                signature: Signature::from_str(signature.as_str().unwrap()).unwrap(),
                order: LimitOrderV3Struct::from_json(order),
                deadline: chrono::DateTime::<Utc>::from_str(deadline.as_str().unwrap())
                    .unwrap()
                    .timestamp() as u32,
                auction_start_date: chrono::DateTime::<Utc>::from_str(
                    auction_start_date.as_str().unwrap(),
                )
                .unwrap()
                .timestamp() as u32,
                auction_end_date: chrono::DateTime::<Utc>::from_str(
                    auction_end_date.as_str().unwrap(),
                )
                .unwrap()
                .timestamp() as u32,
                remaining_maker_amount: U256::from_str(remaining_maker_amount.as_str().unwrap())
                    .unwrap(),
            }))
        }
        "order_invalid" => {
            let result = value.get("result");
            let hash = result
                .and_then(|v| v.get("orderHash"))
                .expect("No hash found");

            RpcMessage::OrderEvent(OrderEvent::Invalid {
                order_hash: Hash::from_str(hash.as_str().unwrap()).unwrap(),
            })
        }
        // "order_balance_or_allowance_change" => {
        //     let result = value.get("result");
        // }
        // "filled" => {
        //     println!("Filled");
        // }
        // "filled_partially" => {
        //     println!("FilledPartially");
        // }
        _ => RpcMessage::None,
    };

    msg_parsed
}
