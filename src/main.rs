use foo::constants::NetworkEnum;
use foo::ws_api::{WebSocketApi, WebSocketApiConfig};
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::{mpsc, Mutex};
use tokio::time::sleep;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, WebSocketStream};
use url::Url;

#[tokio::main]
async fn main() {
    let mut client = WebSocketApi::new(&WebSocketApiConfig {
        url: "wss://fusion.1inch.io/ws".to_string(),
        network: NetworkEnum::ETHEREUM,
    });

    println!("start");

    client.on_message_async(|msg: foo::ws_api::types::RpcMessage| {
        Box::pin(async move {
            println!("on_message: {:?}", msg);
        })
    });
    // client.on_order_created(|msg: foo::ws_api::types::OrderEventCreated| Box::pin(async move {
    //     println!("order created inside");

    //     println!("order created finish");
    // }));

    println!("end");

    client.connect().await;

    println!("end22222")
    // client
    //     .send(Message::Text("Hello, WebSocket server!".to_string()))
    //     .await;
}
