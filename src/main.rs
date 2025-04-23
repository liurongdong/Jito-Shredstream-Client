use jito_protos::shredstream::{
    shredstream_proxy_client::ShredstreamProxyClient, SubscribeEntriesRequest,
};
use chrono::Local;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // 创建目标公钥(创建代币事件)
    let target_pubkey = Pubkey::from_str("TSLvdd1pWpHVjahSpsvCXUbgwsL3JAcvokwaKt1eokM").unwrap();
    
    let mut client = ShredstreamProxyClient::connect("http://45.77.55.124:9999")
        .await
        .unwrap();
    let mut stream = client
        .subscribe_entries(SubscribeEntriesRequest {})
        .await
        .unwrap()
        .into_inner();

    while let Some(slot_entry) = stream.message().await.unwrap() {
        let entries =
            match bincode::deserialize::<Vec<solana_entry::entry::Entry>>(&slot_entry.entries) {
                Ok(e) => e,
                Err(e) => {
                    println!("反序列化失败: {e}");
                    continue;
                }
            };

        // 解析每个交易
        for entry in entries {
            for tx_data in entry.transactions {
                let transaction = tx_data;
                
                match &transaction.message {
                    solana_sdk::message::VersionedMessage::V0(message) => {
                        // 检查账户地址列表中是否包含目标公钥(创建代币事件)
                        if message.account_keys.contains(&target_pubkey) {
                            println!("\n{}", "-".repeat(80));
                            println!("[{}] Pumpfun内盘创建代币事件:", Local::now().format("%Y-%m-%d %H:%M:%S%.3f"));
                            println!("交易签名: {}", transaction.signatures[0]);
                            
                            // 打印所有账户地址
                            println!("账户地址列表:");
                            for account_key in &message.account_keys {
                                println!("  {}", account_key);
                            }

                            // 打印指令
                            println!("指令列表:");
                            for instruction in &message.instructions {
                                println!("  程序ID: {}", message.account_keys[instruction.program_id_index as usize]);
                                println!("  指令数据: {:?}", instruction.data);
                            }
                        }
                    }
                    solana_sdk::message::VersionedMessage::Legacy(message) => {
                        // 检查账户地址列表中是否包含Pumpfun内盘创建代币事件
                        if message.account_keys.contains(&target_pubkey) {
                            println!("\n{}", "-".repeat(80));
                            println!("[{}] Pumpfun内盘创建代币事件:", Local::now().format("%Y-%m-%d %H:%M:%S%.3f"));
                            println!("交易签名: {}", transaction.signatures[0]);
                            
                            // 打印所有账户地址
                            println!("账户地址列表:");
                            for account_key in &message.account_keys {
                                println!("  {}", account_key);
                            }

                            // 打印指令
                            println!("指令列表:");
                            for instruction in &message.instructions {
                                println!("  程序ID: {}", message.account_keys[instruction.program_id_index as usize]);
                                println!("  指令数据: {:?}", instruction.data);
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
