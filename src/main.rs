mod config;
mod client;
mod processor;
mod utils;

use config::Config;
use client::ShredstreamClient;
use processor::TransactionProcessor;
use utils::deserialize_entries;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new();
    let mut client = ShredstreamClient::new(config.clone()).await?;
    let processor = TransactionProcessor::new(config.token_creator_pubkey);

    loop {
        match client.subscribe_entries().await {
            Ok(mut stream) => {
                while let Some(entry) = stream.message().await? {
                    match deserialize_entries(&entry.entries) {
                        Ok(entries) => processor.process_entries(entries, entry.slot),
                        Err(e) => println!("反序列化失败: {e}"),
                    }
                }
            }
            Err(e) => {
                println!("连接断开: {e}");
                println!("5秒后重新连接...");
                sleep(Duration::from_secs(5)).await;
            }
        }
    }
}
