use solana_sdk::{
    pubkey::Pubkey,
    transaction::VersionedTransaction,
    message::VersionedMessage,
};
use std::collections::HashMap;

pub fn print_transaction_info(transaction: &VersionedTransaction) {
    println!("\n交易详情:");
    println!("签名: {}", transaction.signatures[0]);
    println!("消息版本: {:?}", transaction.message);
    
    match &transaction.message {
        VersionedMessage::Legacy(msg) => {
            println!("账户数量: {}", msg.account_keys.len());
            println!("指令数量: {}", msg.instructions.len());
            
            for (i, instruction) in msg.instructions.iter().enumerate() {
                println!("指令 {}:", i);
                println!("  程序ID索引: {}", instruction.program_id_index);
                println!("  账户索引: {:?}", instruction.accounts);
                println!("  数据长度: {}", instruction.data.len());
            }
        }
        VersionedMessage::V0(msg) => {
            println!("账户数量: {}", msg.account_keys.len());
            println!("指令数量: {}", msg.instructions.len());
            
            for (i, instruction) in msg.instructions.iter().enumerate() {
                println!("指令 {}:", i);
                println!("  程序ID索引: {}", instruction.program_id_index);
                println!("  账户索引: {:?}", instruction.accounts);
                println!("  数据长度: {}", instruction.data.len());
            }
        }
    }

    let message = &transaction.message;
    let num_signatures = message.header().num_required_signatures;
    let tx_type = if num_signatures == 1 { 
        "单签名交易".to_string() 
    } else { 
        format!("多签名交易 ({}个签名)", num_signatures) 
    };
    println!("交易类型: {}", tx_type);
    
    let static_keys = message.static_account_keys();
    println!("签名账户: {}", static_keys[0]);
    
    println!("\n指令详情:");
    for (i, instruction) in message.instructions().iter().enumerate() {
        let program_id = instruction.program_id(&message.static_account_keys());
        let accounts = &instruction.accounts;
        
        println!("  指令 {}:", i);
        println!("    程序: {}", program_id);
        
        match program_id.to_string().as_str() {
            "ComputeBudget111111111111111111111111111111" => {
                println!("    类型: 计算预算指令");
                if instruction.data.len() > 0 {
                    match instruction.data[0] {
                        0 => println!("    操作: 设置计算单元限制"),
                        1 => println!("    操作: 设置优先级费用"),
                        _ => println!("    操作: 未知计算预算操作"),
                    }
                }
            },
            "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL" => {
                println!("    类型: Associated Token 指令");
            },
            "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P" => {
                println!("    类型: 自定义程序指令");
            },
            _ => {
                println!("    类型: 其他程序指令");
            }
        }
        
        println!("    相关账户:");
        for account_index in accounts {
            println!("      - {}", static_keys[*account_index as usize]);
        }
    }

    println!("\n{}", "-".repeat(80));
}

pub fn group_transactions_by_account<'a>(
    entries: &'a [solana_entry::entry::Entry],
    target_account: &'a Pubkey
) -> HashMap<Pubkey, Vec<&'a VersionedTransaction>> {
    let mut transactions_by_account = HashMap::new();
    
    for entry in entries {
        for transaction in &entry.transactions {
            let accounts: Vec<&Pubkey> = match &transaction.message {
                VersionedMessage::Legacy(msg) => msg.account_keys.iter().collect(),
                VersionedMessage::V0(msg) => msg.account_keys.iter().collect(),
            };
            
            if accounts.contains(&target_account) {
                transactions_by_account
                    .entry(*target_account)
                    .or_insert_with(Vec::new)
                    .push(transaction);
            }
        }
    }
    
    transactions_by_account
} 