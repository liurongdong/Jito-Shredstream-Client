use solana_sdk::{
    pubkey::Pubkey,
    transaction::VersionedTransaction,
    message::VersionedMessage,
};
use std::collections::HashMap;

pub mod pump_parser;
use pump_parser::{parse_pump_transaction, get_mint_from_transaction, PUMP_PROGRAM_ID};

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
                        2 => println!("    操作: 设置计算单元价格"),
                        3 => println!("    操作: 设置堆内存"),
                        _ => println!("    操作: 未知计算预算操作"),
                    }
                }
            },
            "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL" => {
                println!("    类型: Associated Token 指令");
            },
            pump_id if pump_id == PUMP_PROGRAM_ID => {
                println!("    类型: Pump协议指令");
                
                // 尝试解析pump指令
                if let Some(parsed) = pump_parser::parse_pump_instruction(transaction, i) {
                    println!("    操作: {}", parsed.name);
                    println!("    内容: {}", parsed.params);
                    
                    // 尝试获取Mint地址
                    if let Some(mint) = get_mint_from_transaction(transaction) {
                        println!("    代币Mint: {}", mint);
                    }
                }
            },
            _ => {
                println!("    类型: 其他程序指令");
            }
        }
        
        println!("    相关账户:");
        for account_index in accounts {
            let index = *account_index as usize;
            if index < static_keys.len() {
                println!("      - {}", static_keys[index]);
            } else {
                println!("      - 无效账户索引: {}", index);
            }
        }
    }

    // 添加Pump指令的特殊解析
    let parsed_pump = parse_pump_transaction(transaction);
    if !parsed_pump.is_empty() {
        println!("\nPump协议交易解析:");
        for (i, instruction) in parsed_pump.iter().enumerate() {
            println!("  Pump指令 {}:", i + 1);
            println!("    类型: {}", instruction.name);
            println!("    详情: {}", instruction.params);
        }
    }

    println!("\n{}", "-".repeat(80));
}

pub fn group_transactions_by_accounts<'a>(
    entries: &'a [solana_entry::entry::Entry],
    target_accounts: &'a [Pubkey]
) -> HashMap<Pubkey, Vec<&'a VersionedTransaction>> {
    let mut transactions_by_account = HashMap::new();
    
    for entry in entries {
        for transaction in &entry.transactions {
            let accounts: Vec<&Pubkey> = match &transaction.message {
                VersionedMessage::Legacy(msg) => msg.account_keys.iter().collect(),
                VersionedMessage::V0(msg) => msg.account_keys.iter().collect(),
            };
            
            for target_account in target_accounts {
                if accounts.contains(&target_account) {
                    transactions_by_account
                        .entry(*target_account)
                        .or_insert_with(Vec::new)
                        .push(transaction);
                }
            }
        }
    }
    
    transactions_by_account
} 