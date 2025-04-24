use solana_entry::entry::Entry;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use tokio::time::sleep;
use std::time::Duration;

mod client;
mod transaction;
mod config;

use client::ShredstreamClient;
use config::Config;
use transaction::{print_transaction_info, group_transactions_by_account};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::default();
    let target_account = Pubkey::from_str(&config.target_account)?;
    let client = ShredstreamClient::new(config.server_url.clone());
    
    println!("开始监听目标账户 {} 的交易...", target_account);
    
    loop {
        match client.connect().await {
            Ok(mut jito_client) => {
                match client.subscribe_entries(&mut jito_client).await {
                    Ok(mut stream) => {
                        while let Ok(Some(slot_entry)) = stream.message().await {
                            if let Ok(entries) = bincode::deserialize::<Vec<Entry>>(&slot_entry.entries) {
                                let transactions_by_account = group_transactions_by_account(&entries, &target_account);
                                
                                if let Some(transactions) = transactions_by_account.get(&target_account) {
                                    println!("\n找到 {} 笔新交易 当前Slot:[{}]", transactions.len(), slot_entry.slot);
                                    
                                    for (index, transaction) in transactions.iter().enumerate() {
                                        println!("\n交易 {}:", index + 1);
                                        print_transaction_info(transaction);
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        println!("订阅错误: {}，5秒后重试...", e);
                        sleep(Duration::from_secs(5)).await;
                    }
                }
            }
            Err(e) => {
                println!("连接错误: {}，5秒后重试...", e);
                sleep(Duration::from_secs(5)).await;
            }
        }
    }
}