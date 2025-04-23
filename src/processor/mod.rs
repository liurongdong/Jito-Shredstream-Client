use chrono::Local;
use solana_sdk::{message::VersionedMessage, pubkey::Pubkey, transaction::VersionedTransaction};
use solana_entry::entry::Entry;

pub struct TransactionProcessor {
    token_creator_pubkey: Pubkey,
}

impl TransactionProcessor {
    pub fn new(token_creator_pubkey: Pubkey) -> Self {
        Self { token_creator_pubkey }
    }

    pub fn process_entries(&self, entries: Vec<Entry>, slot: u64) {
        for entry in entries {
            for tx_data in entry.transactions {
                let transaction = tx_data;
                
                match &transaction.message {
                    VersionedMessage::V0(message) => self.process_message_v0(message, &transaction, slot),
                    VersionedMessage::Legacy(message) => self.process_message_legacy(message, &transaction, slot),
                }
            }
        }
    }

    fn process_message_v0(&self, message: &solana_sdk::message::v0::Message, transaction: &VersionedTransaction, slot: u64) {
        if message.account_keys.contains(&self.token_creator_pubkey) {
            println!("\n{}", "-".repeat(80));
            println!("[{}] Pumpfun内盘创建代币事件:", Local::now().format("%Y-%m-%d %H:%M:%S%.3f"));
            println!("Slot: {}", slot);
            println!("交易签名: {}", transaction.signatures[0]);
            
            println!("账户地址列表:");
            for account_key in &message.account_keys {
                println!("  {}", account_key);
            }

            println!("指令列表:");
            for instruction in &message.instructions {
                println!("  程序ID: {}", message.account_keys[instruction.program_id_index as usize]);
                println!("  指令数据: {:?}", instruction.data);
            }
        }
    }

    fn process_message_legacy(&self, message: &solana_sdk::message::Message, transaction: &VersionedTransaction, slot: u64) {
        if message.account_keys.contains(&self.token_creator_pubkey) {
            println!("\n{}", "-".repeat(80));
            println!("[{}] Pumpfun内盘创建代币事件:", Local::now().format("%Y-%m-%d %H:%M:%S%.3f"));
            println!("Slot: {}", slot);
            println!("交易签名: {}", transaction.signatures[0]);
            
            println!("账户地址列表:");
            for account_key in &message.account_keys {
                println!("  {}", account_key);
            }

            println!("指令列表:");
            for instruction in &message.instructions {
                println!("  程序ID: {}", message.account_keys[instruction.program_id_index as usize]);
                println!("  指令数据: {:?}", instruction.data);
            }
        }
    }
} 