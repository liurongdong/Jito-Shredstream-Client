use chrono::Local;
use solana_sdk::{message::VersionedMessage, pubkey::Pubkey, transaction::VersionedTransaction};
use solana_entry::entry::Entry;
use crate::instruction::InstructionParser;
use std::error::Error;

pub struct TransactionProcessor {
    token_creator_pubkey: Pubkey,
}

impl TransactionProcessor {
    pub fn new(token_creator_pubkey: Pubkey) -> Self {
        Self { token_creator_pubkey }
    }

    pub fn process_entries(&self, entries: Vec<Entry>, slot: u64) -> Result<(), Box<dyn Error>> {
        for entry in entries {
            for tx_data in entry.transactions {
                let transaction = tx_data;
                
                match &transaction.message {
                    VersionedMessage::V0(message) => self.process_message_v0(message, &transaction, slot)?,
                    VersionedMessage::Legacy(message) => self.process_message_legacy(message, &transaction, slot)?,
                }
            }
        }
        Ok(())
    }

    fn process_message_v0(&self, message: &solana_sdk::message::v0::Message, transaction: &VersionedTransaction, slot: u64) -> Result<(), Box<dyn Error>> {
        if message.account_keys.contains(&self.token_creator_pubkey) {
            println!("\n{}", "-".repeat(80));
            println!("[{}] Pumpfun内盘创建代币事件:", Local::now().format("%Y-%m-%d %H:%M:%S%.3f"));
            println!("Slot: {}", slot);
            println!("Signatures: {}", transaction.signatures[0]);
            
            // 提取关键账户地址
            let mint_address = message.account_keys[1].to_string();
            let bonding_curve = message.account_keys[2].to_string();
            let user_address = message.account_keys[0].to_string();
            
            println!("Mint: {}", mint_address);
            println!("Bonding Curve: {}", bonding_curve);
            println!("Creator: {}", user_address);

            // 解析代币信息
            for instruction in &message.instructions {
                let program_id = message.account_keys[instruction.program_id_index as usize].to_string();
                if program_id == "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P" {
                    let parsed_data = InstructionParser::parse_instruction(&program_id, &instruction.data);
                    if parsed_data.starts_with("Token_Metadata:") {
                        println!("{}", parsed_data);
                    }
                }
            }
        }
        Ok(())
    }

    fn process_message_legacy(&self, message: &solana_sdk::message::Message, transaction: &VersionedTransaction, slot: u64) -> Result<(), Box<dyn Error>> {
        if message.account_keys.contains(&self.token_creator_pubkey) {
            println!("\n{}", "-".repeat(80));
            println!("[{}] Pumpfun内盘创建代币事件:", Local::now().format("%Y-%m-%d %H:%M:%S%.3f"));
            println!("Slot: {}", slot);
            println!("Signatures: {}", transaction.signatures[0]);
            
            // 提取关键账户地址
            let mint_address = message.account_keys[1].to_string();
            let bonding_curve = message.account_keys[2].to_string();
            let user_address = message.account_keys[0].to_string();
            
            println!("Mint: {}", mint_address);
            println!("Bonding Curve: {}", bonding_curve);
            println!("Creator: {}", user_address);

            // 解析代币信息
            for instruction in &message.instructions {
                let program_id = message.account_keys[instruction.program_id_index as usize].to_string();
                if program_id == "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P" {
                    let parsed_data = InstructionParser::parse_instruction(&program_id, &instruction.data);
                    if parsed_data.starts_with("Token_Metadata:") {
                        println!("{}", parsed_data);
                    }
                }
            }
        }
        Ok(())
    }
} 