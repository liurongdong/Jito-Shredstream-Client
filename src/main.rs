use solana_entry::entry::Entry;
use tokio::time::sleep;
use std::time::Duration;

mod client;
mod transaction;
mod config;

use client::ShredstreamClient;
use config::Config;
use transaction::{print_transaction_info, group_transactions_by_accounts};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::default();
    let client = ShredstreamClient::new(config.server_url.clone());
    
    println!("开始监听目标账户的交易...");
    for account in &config.target_accounts {
        println!("监控账户: {}", account);
    }
    
    loop {
        match client.connect().await {
            Ok(mut jito_client) => {
                match client.subscribe_entries(&mut jito_client).await {
                    Ok(mut stream) => {
                        while let Ok(Some(slot_entry)) = stream.message().await {
                            if let Ok(entries) = bincode::deserialize::<Vec<Entry>>(&slot_entry.entries) {
                                let transactions_by_account = group_transactions_by_accounts(&entries, &config.target_accounts);
                                
                                let mut found_transactions = false;
                                
                                for (account, transactions) in &transactions_by_account {
                                    if !transactions.is_empty() {
                                        found_transactions = true;
                                        println!("\n找到账户 {} 的 {} 笔新交易 当前Slot:[{}]", account, transactions.len(), slot_entry.slot);
                                        
                                        for (index, transaction) in transactions.iter().enumerate() {
                                            println!("\n交易 {}:", index + 1);
                                            print_transaction_info(transaction);
                                        }
                                    }
                                }
                                
                                if found_transactions {
                                    println!("\n----------------------------------------------\n");
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