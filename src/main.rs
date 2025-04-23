use jito_protos::shredstream::{
    shredstream_proxy_client::ShredstreamProxyClient, SubscribeEntriesRequest,
};
use solana_sdk::{
    message::VersionedMessage,
    pubkey::Pubkey,
    signature::Signature,
    transaction::VersionedTransaction,
};
use ed25519_dalek::{PublicKey, Signature as EdSignature};
use anyhow::Result;
use chrono::Local;
use tonic::transport::Channel;
use tokio::time::Duration;
use std::collections::VecDeque;
use futures::future::try_join_all;

#[derive(Debug)]
struct CreateEventInstruction {
    name: String,
    symbol: String,
    uri: String,
    user: Pubkey,
}

#[derive(Debug)]
struct BuyInstruction {
    amount: u64,
    max_sol_cost: u64,
}

async fn verify_signatures(tx: &VersionedTransaction) -> Result<()> {
    let message = tx.message.serialize();
    let futures = tx.signatures.iter().enumerate().map(|(i, signature)| {
        let message = message.clone();
        let pubkey = tx.message.static_account_keys()[i];
        let signature = signature.clone();
        
        tokio::spawn(async move {
            let public_key = PublicKey::from_bytes(&pubkey.to_bytes())?;
            let signature_bytes: [u8; 64] = signature.as_ref().try_into().map_err(|_| anyhow::anyhow!("Invalid signature length"))?;
            let ed_signature = EdSignature::from_bytes(&signature_bytes)?;
            
            if !public_key.verify_strict(&message, &ed_signature).is_ok() {
                Err(anyhow::anyhow!("Signature verification failed for signature {}", i))
            } else {
                Ok(())
            }
        })
    });
    
    try_join_all(futures).await?;
    Ok(())
}

async fn print_versioned_transaction(tx: &VersionedTransaction, sig: Signature, mint: Pubkey) {
    match &tx.message {
        VersionedMessage::Legacy(msg) => {
            print_legacy_message(msg, sig, mint).await;
        }
        VersionedMessage::V0(msg) => {
            print_v0_message(msg, sig, mint).await;
        }
    }
}

async fn print_legacy_message(msg: &solana_sdk::message::Message, sig: Signature, mint: Pubkey) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
    println!("[{}] Legacy Message - Signature: {}, Mint: {}", timestamp, sig, mint);
    println!("Account Keys: {:?}", msg.account_keys);
    println!("Instructions: {:?}", msg.instructions);
}

async fn print_v0_message(msg: &solana_sdk::message::v0::Message, sig: Signature, mint: Pubkey) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
    println!("[{}] V0 Message - Signature: {}, Mint: {}", timestamp, sig, mint);
    println!("Account Keys: {:?}", msg.account_keys);
    println!("Instructions: {:?}", msg.instructions);
}

fn parse_instruction_data(data: &[u8]) -> Result<(String, Option<CreateEventInstruction>, Option<BuyInstruction>)> {
    // 检查数据长度是否足够
    if data.len() < 8 {
        return Err(anyhow::anyhow!("Instruction data too short"));
    }
    
    let discriminator = &data[0..8];
    
    match discriminator {
        [0x18, 0x1e, 0xc8, 0x28, 0x05, 0x1c, 0x07, 0x77] => {
            let mut offset = 8;
            
            // 检查是否有足够的字节来读取 name_len
            if offset + 4 > data.len() {
                return Err(anyhow::anyhow!("Insufficient data for name length"));
            }
            let name_len = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap()) as usize;
            offset += 4;
            
            // 检查是否有足够的字节来读取 name
            if offset + name_len > data.len() {
                return Err(anyhow::anyhow!("Insufficient data for name"));
            }
            let name = String::from_utf8(data[offset..offset + name_len].to_vec())?;
            offset += name_len;

            // 检查是否有足够的字节来读取 symbol_len
            if offset + 4 > data.len() {
                return Err(anyhow::anyhow!("Insufficient data for symbol length"));
            }
            let symbol_len = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap()) as usize;
            offset += 4;
            
            // 检查是否有足够的字节来读取 symbol
            if offset + symbol_len > data.len() {
                return Err(anyhow::anyhow!("Insufficient data for symbol"));
            }
            let symbol = String::from_utf8(data[offset..offset + symbol_len].to_vec())?;
            offset += symbol_len;

            // 检查是否有足够的字节来读取 uri_len
            if offset + 4 > data.len() {
                return Err(anyhow::anyhow!("Insufficient data for uri length"));
            }
            let uri_len = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap()) as usize;
            offset += 4;
            
            // 检查是否有足够的字节来读取 uri
            if offset + uri_len > data.len() {
                return Err(anyhow::anyhow!("Insufficient data for uri"));
            }
            let uri = String::from_utf8(data[offset..offset + uri_len].to_vec())?;
            offset += uri_len;

            // 检查是否有足够的字节来读取 user
            if offset + 32 > data.len() {
                return Err(anyhow::anyhow!("Insufficient data for user pubkey"));
            }
            let user = Pubkey::new_from_array(data[offset..offset + 32].try_into().unwrap());

            let instruction = CreateEventInstruction { name, symbol, uri, user};
            Ok(("CreateEvent".to_string(), Some(instruction), None))
        }

        [0x66, 0x06, 0x3d, 0x12, 0x01, 0xda, 0xeb, 0xea] => {
            let mut offset = 8;
            
            // 检查是否有足够的字节来读取 amount
            if offset + 8 > data.len() {
                return Err(anyhow::anyhow!("Insufficient data for amount"));
            }
            let amount = u64::from_le_bytes(data[offset..offset + 8].try_into().unwrap());
            offset += 8;
            
            // 检查是否有足够的字节来读取 max_sol_cost
            if offset + 8 > data.len() {
                return Err(anyhow::anyhow!("Insufficient data for max_sol_cost"));
            }
            let max_sol_cost = u64::from_le_bytes(data[offset..offset + 8].try_into().unwrap());

            let instruction = BuyInstruction { amount, max_sol_cost };
            Ok(("Buy".to_string(), None, Some(instruction)))
        }
        _ => {
            // 打印未知指令的鉴别器以便调试
            println!("Unknown instruction discriminator: {:?}", discriminator);
            Err(anyhow::anyhow!("Unknown instruction data"))
        }
    }
}

async fn process_transaction_batch(batch: &[VersionedTransaction]) -> Result<()> {
    let mut futures = Vec::new();
    
    for tx in batch {
        let tx_clone = tx.clone();
        futures.push(tokio::spawn(async move {
            if let Err(e) = verify_signatures(&tx_clone).await {
                println!("[{}] Signature verification failed: {}", Local::now().format("%Y-%m-%d %H:%M:%S%.3f"), e);
                return Err(e);
            }
            
            let sig = tx_clone.signatures[0];
            let mint = tx_clone.message.static_account_keys()[1];
            print_versioned_transaction(&tx_clone, sig, mint).await;
            
            for instruction in tx_clone.message.instructions().iter() {
                match parse_instruction_data(&instruction.data) {
                    Ok((instruction_type, create_event, buy)) => {
                        match instruction_type.as_str() {
                            "CreateEvent" => {
                                if let Some(event) = create_event {
                                    println!("[{}] CreateEvent Instruction: {:?}", Local::now().format("%Y-%m-%d %H:%M:%S%.3f"), event);
                                }
                            }
                            "Buy" => {
                                if let Some(buy) = buy {
                                    println!("[{}] Buy Instruction: {:?}", Local::now().format("%Y-%m-%d %H:%M:%S%.3f"), buy);
                                }
                            }
                            _ => {}
                        }
                    }
                    Err(e) => {
                        println!("[{}] Failed to parse instruction: {}", Local::now().format("%Y-%m-%d %H:%M:%S%.3f"), e);
                    }
                }
            }
            Ok(())
        }));
    }
    
    try_join_all(futures).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 配置连接
    let channel = Channel::from_static("http://45.77.55.124:9999")
        .connect_timeout(Duration::from_secs(10))
        .timeout(Duration::from_secs(10))
        .tcp_nodelay(true)
        .tcp_keepalive(Some(Duration::from_secs(60)))
        .connect()
        .await?;

    let mut client = ShredstreamProxyClient::new(channel);
    
    let mut stream = client
        .subscribe_entries(SubscribeEntriesRequest {})
        .await?
        .into_inner();

    const BATCH_SIZE: usize = 100;
    let mut tx_queue: VecDeque<VersionedTransaction> = VecDeque::with_capacity(BATCH_SIZE);

    while let Some(slot_entry) = stream.message().await? {
        let entries_data = slot_entry.entries.clone();
        let entries = tokio::task::spawn_blocking(move || {
            bincode::deserialize::<Vec<solana_entry::entry::Entry>>(&entries_data)
        }).await??;

        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
        println!(
            "[{}] slot {}, entries: {}, transactions: {}",
            timestamp,
            slot_entry.slot,
            entries.len(),
            entries.iter().map(|e| e.transactions.len()).sum::<usize>()
        );

        // 收集事务到队列
        for entry in entries {
            for tx in entry.transactions.iter() {
                tx_queue.push_back(tx.clone());
                
                // 当队列达到批量大小时处理
                if tx_queue.len() >= BATCH_SIZE {
                    let batch: Vec<VersionedTransaction> = tx_queue.drain(..).collect();
                    if let Err(e) = process_transaction_batch(&batch).await {
                        println!("[{}] Error processing batch: {}", Local::now().format("%Y-%m-%d %H:%M:%S%.3f"), e);
                    }
                }
            }
        }

        // 处理剩余的事务
        if !tx_queue.is_empty() {
            let batch: Vec<VersionedTransaction> = tx_queue.drain(..).collect();
            if let Err(e) = process_transaction_batch(&batch).await {
                println!("[{}] Error processing final batch: {}", Local::now().format("%Y-%m-%d %H:%M:%S%.3f"), e);
            }
        }
    }
    Ok(())
}
