use jito_protos::shredstream::{
    shredstream_proxy_client::ShredstreamProxyClient, SubscribeEntriesRequest,
};
use std::any::type_name;
use chrono::Local;

fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let mut client = ShredstreamProxyClient::connect("http://45.77.55.124:9999")
        .await
        .unwrap();
    let mut stream = client
        .subscribe_entries(SubscribeEntriesRequest {})
        .await
        .unwrap()
        .into_inner();

    while let Some(slot_entry) = stream.message().await.unwrap() {
        println!("收到槽位条目类型: {}", type_of(&slot_entry));
        println!("条目数据类型: {}", type_of(&slot_entry.entries));
        
        let entries =
            match bincode::deserialize::<Vec<solana_entry::entry::Entry>>(&slot_entry.entries) {
                Ok(e) => e,
                Err(e) => {
                    println!("反序列化失败: {e}");
                    continue;
                }
            };

        println!(
            "槽位 {}, 条目数: {}, 交易数: {}",
            slot_entry.slot,
            entries.len(),
            entries.iter().map(|e| e.transactions.len()).sum::<usize>()
        );

        // 解析每个交易
        for entry in entries {
            println!("条目类型: {}", type_of(&entry));
            println!("交易列表类型: {}", type_of(&entry.transactions));
            
            for tx_data in entry.transactions {
                println!("交易数据类型: {}", type_of(&tx_data));
                let transaction = tx_data;
                println!("[{}] 交易签名: {}", Local::now().format("%Y-%m-%d %H:%M:%S%.3f"), transaction.signatures[0]);
                
                match &transaction.message {
                    solana_sdk::message::VersionedMessage::V0(message) => {
                        // 打印账户地址
                        for account_key in &message.account_keys {
                            println!("账户地址: {}", account_key);
                        }

                        // 打印指令
                        for instruction in &message.instructions {
                            println!("程序ID: {}", message.account_keys[instruction.program_id_index as usize]);
                            println!("指令数据: {:?}", instruction.data);
                        }
                    }
                    solana_sdk::message::VersionedMessage::Legacy(message) => {
                        // 打印账户地址
                        for account_key in &message.account_keys {
                            println!("账户地址: {}", account_key);
                        }

                        // 打印指令
                        for instruction in &message.instructions {
                            println!("程序ID: {}", message.account_keys[instruction.program_id_index as usize]);
                            println!("指令数据: {:?}", instruction.data);
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
