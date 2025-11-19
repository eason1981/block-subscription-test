use alloy_provider::{Provider, ProviderBuilder, WsConnect};
use futures::StreamExt;
use std::time::{SystemTime, UNIX_EPOCH};

const WS_RPC_URL: &str = "ws://YOUR_WS_RPC_URL";

#[tokio::main]
async fn main() {
    env_logger::init();

    let ws_conn = WsConnect::new(WS_RPC_URL);
    let provider = ProviderBuilder::new()
        .connect_ws(ws_conn)
        .await
        .expect("failed to connect to rpc websocket URL");
    let subscription = provider
        .subscribe_blocks()
        .await
        .expect("failed to subscribe the latest blocks");
    let mut latest_block_receiver = subscription.into_stream();

    while let Some(header) = latest_block_receiver.next().await {
        let block_number = header.number;
        let block_timestamp = header.timestamp;

        let current_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        println!(
            "block {block_number}: current_timestamp - block_timestamp = {}s",
            current_timestamp - block_timestamp,
        );
    }
}
