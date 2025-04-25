use solana_sdk::{
    pubkey::Pubkey,
    transaction::VersionedTransaction,
    message::VersionedMessage,
};
use std::collections::HashMap;

pub mod pump_parser;
use pump_parser::{parse_pump_transaction, get_mint_from_transaction, get_bonding_curve_info, PUMP_PROGRAM_ID};
// 添加Pump AMM协议支持
pub mod pumpamm_parser;
use pumpamm_parser::PUMPAMM_PROGRAM_ID;

pub fn print_transaction_info(transaction: &VersionedTransaction) {
    println!("\n交易详情:");
    println!("签名: {}", transaction.signatures[0]);
    println!("消息版本: {:?}", transaction.message);
    
    let static_keys = transaction.message.static_account_keys();
    
    // 打印Pump相关的特殊账户
    println!("\nPump特殊账户:");
    for (i, key) in static_keys.iter().enumerate() {
        let key_str = key.to_string();
        if key_str.ends_with("pump") {
            println!("  {}. {} (可能的Pump账户)", i, key);
        }
    }
    
    // 打印签名账户
    println!("\n签名账户: {}", static_keys[0]);
    
    // 尝试获取Mint地址
    if let Some(mint) = pump_parser::get_mint_from_transaction(transaction) {
        println!("\n识别的代币Mint: {}", mint);
    }
    
    // 尝试获取BondingCurve信息
    if let Some(curve_info) = pump_parser::get_bonding_curve_info(transaction) {
        println!("识别的曲线账户: {}", curve_info.curve_account);
        println!("曲线状态: {}", if curve_info.is_complete { "已完成" } else { "进行中" });
    }
    
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
                    
                    // 尝试获取BondingCurve信息
                    if let Some(curve_info) = get_bonding_curve_info(transaction) {
                        println!("    曲线账户: {}", curve_info.curve_account);
                        if curve_info.is_complete {
                            println!("    曲线状态: 已完成");
                        }
                    }
                }
            },
            // 添加对Pump AMM协议的支持
            pumpamm_id if pumpamm_id == PUMPAMM_PROGRAM_ID => {
                println!("    类型: Pump AMM协议指令");
                
                // 尝试解析Pump AMM指令
                if let Some(parsed) = pumpamm_parser::parse_pumpamm_instruction(transaction, i) {
                    println!("    操作: {}", parsed.name);
                    println!("    内容: {}", parsed.params);
                    
                    // 打印相关池信息（如果有）
                    if let Some(pool) = pumpamm_parser::get_pool_from_instruction(transaction, i) {
                        println!("    池地址: {}", pool);
                    }
                    
                    // 打印基础代币和报价代币信息（如果有）
                    if let Some((base_mint, quote_mint)) = pumpamm_parser::get_token_mints_from_instruction(transaction, i) {
                        println!("    基础代币: {}", base_mint);
                        println!("    报价代币: {}", quote_mint);
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
        
        // 添加曲线信息 
        if let Some(curve_info) = get_bonding_curve_info(transaction) {
            println!("\n曲线信息:");
            println!("  代币Mint: {}", curve_info.mint);
            println!("  曲线账户: {}", curve_info.curve_account);
            println!("  状态: {}", if curve_info.is_complete { "已完成" } else { "进行中" });
        }
    }
    
    // 添加Pump AMM协议交易解析
    let parsed_pumpamm = pumpamm_parser::parse_pumpamm_transaction(transaction);
    if !parsed_pumpamm.is_empty() {
        println!("\nPump AMM协议交易解析:");
        for (i, instruction) in parsed_pumpamm.iter().enumerate() {
            println!("  Pump AMM指令 {}:", i + 1);
            println!("    类型: {}", instruction.name);
            println!("    详情: {}", instruction.params);
        }
        
        // 添加池信息
        if let Some(pool_info) = pumpamm_parser::get_pool_info_from_transaction(transaction) {
            println!("\n池信息:");
            println!("  池地址: {}", pool_info.pool);
            println!("  基础代币: {}", pool_info.base_mint);
            println!("  报价代币: {}", pool_info.quote_mint);
            if let Some(lp_mint) = pool_info.lp_mint {
                println!("  LP代币: {}", lp_mint);
            }
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