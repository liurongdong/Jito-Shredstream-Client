use solana_sdk::{
    pubkey::Pubkey,
    transaction::VersionedTransaction,
};
use borsh::BorshDeserialize;

// Pump AMM程序ID
pub const PUMPAMM_PROGRAM_ID: &str = "pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA";

// Pump AMM指令编号常量
pub const CREATE_POOL_IX: [u8; 8] = [233, 146, 209, 142, 207, 104, 64, 188];
pub const DEPOSIT_IX: [u8; 8] = [242, 35, 198, 137, 82, 225, 242, 182];
pub const BUY_IX: [u8; 8] = [102, 6, 61, 18, 1, 218, 235, 234];
pub const SELL_IX: [u8; 8] = [51, 230, 133, 164, 1, 127, 131, 173];
pub const WITHDRAW_IX: [u8; 8] = [183, 18, 70, 156, 148, 109, 161, 34];

// 指令类型
#[derive(Debug)]
pub enum PumpAmmInstructionType {
    CreatePool,
    Deposit,
    Buy,
    Sell,
    Withdraw,
    Unknown,
}

// 解析后的Pump AMM指令
#[derive(Debug)]
pub struct ParsedPumpAmmInstruction {
    #[allow(dead_code)]
    pub instruction_type: PumpAmmInstructionType,
    pub name: String,
    pub params: String,
}

// 池信息结构
#[derive(Debug)]
pub struct PoolInfo {
    pub pool: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub lp_mint: Option<Pubkey>,
}

// CreatePool参数
#[derive(BorshDeserialize, Debug)]
struct CreatePoolArgs {
    index: u16,
    base_amount_in: u64,
    quote_amount_in: u64,
}

// Deposit参数
#[derive(BorshDeserialize, Debug)]
struct DepositArgs {
    lp_token_amount_out: u64,
    max_base_amount_in: u64,
    max_quote_amount_in: u64,
}

// Buy参数
#[derive(BorshDeserialize, Debug)]
struct BuyArgs {
    base_amount_out: u64,
    max_quote_amount_in: u64,
}

// Sell参数
#[derive(BorshDeserialize, Debug)]
struct SellArgs {
    base_amount_in: u64,
    min_quote_amount_out: u64,
}

// Withdraw参数
#[derive(BorshDeserialize, Debug)]
struct WithdrawArgs {
    lp_token_amount_in: u64,
    min_base_amount_out: u64,
    min_quote_amount_out: u64,
}

/// 将lamports转换为SOL字符串
#[allow(dead_code)]
fn lamports_to_sol_string(lamports: u64) -> String {
    let sol = lamports as f64 / 1_000_000_000.0;
    format!("{:.9} SOL", sol)
}

/// 检查指令是否匹配给定的discriminator
fn is_instruction_match(data: &[u8], discriminator: &[u8; 8]) -> bool {
    if data.len() < 8 {
        return false;
    }
    
    data[0..8] == discriminator[0..8]
}

/// 解析CreatePool指令参数
fn parse_create_pool_args(data: &[u8]) -> Option<String> {
    if data.len() < 16 {
        return None;
    }
    
    match CreatePoolArgs::try_from_slice(&data[8..]) {
        Ok(args) => Some(format!(
            "创建池: 索引={}, 基础代币输入={}, 报价代币输入={}",
            args.index, args.base_amount_in, args.quote_amount_in
        )),
        Err(_) => Some("创建池 (无法解析参数)".to_string()),
    }
}

/// 解析Deposit指令参数
fn parse_deposit_args(data: &[u8]) -> Option<String> {
    if data.len() < 32 {
        return None;
    }
    
    match DepositArgs::try_from_slice(&data[8..]) {
        Ok(args) => Some(format!(
            "存入流动性: LP代币输出={}, 最大基础代币输入={}, 最大报价代币输入={}",
            args.lp_token_amount_out, args.max_base_amount_in, args.max_quote_amount_in
        )),
        Err(_) => Some("存入流动性 (无法解析参数)".to_string()),
    }
}

/// 解析Buy指令参数
fn parse_buy_args(data: &[u8]) -> Option<String> {
    if data.len() < 24 {
        return None;
    }
    
    match BuyArgs::try_from_slice(&data[8..]) {
        Ok(args) => Some(format!(
            "买入: 基础代币输出={}, 最大报价代币输入={}",
            args.base_amount_out, args.max_quote_amount_in
        )),
        Err(_) => Some("买入 (无法解析参数)".to_string()),
    }
}

/// 解析Sell指令参数
fn parse_sell_args(data: &[u8]) -> Option<String> {
    if data.len() < 24 {
        return None;
    }
    
    match SellArgs::try_from_slice(&data[8..]) {
        Ok(args) => Some(format!(
            "卖出: 基础代币输入={}, 最小报价代币输出={}",
            args.base_amount_in, args.min_quote_amount_out
        )),
        Err(_) => Some("卖出 (无法解析参数)".to_string()),
    }
}

/// 解析Withdraw指令参数
fn parse_withdraw_args(data: &[u8]) -> Option<String> {
    if data.len() < 32 {
        return None;
    }
    
    match WithdrawArgs::try_from_slice(&data[8..]) {
        Ok(args) => Some(format!(
            "提取流动性: LP代币输入={}, 最小基础代币输出={}, 最小报价代币输出={}",
            args.lp_token_amount_in, args.min_base_amount_out, args.min_quote_amount_out
        )),
        Err(_) => Some("提取流动性 (无法解析参数)".to_string()),
    }
}

/// 解析单个Pump AMM指令
pub fn parse_pumpamm_instruction(transaction: &VersionedTransaction, instruction_index: usize) -> Option<ParsedPumpAmmInstruction> {
    let message = &transaction.message;
    let instructions = message.instructions();
    
    if instruction_index >= instructions.len() {
        return None;
    }
    
    let instruction = &instructions[instruction_index];
    let program_id = instruction.program_id(&message.static_account_keys());
    
    // 检查是否是Pump AMM程序
    if program_id.to_string() != PUMPAMM_PROGRAM_ID {
        return None;
    }
    
    // 解析指令类型
    if instruction.data.len() < 8 {
        return Some(ParsedPumpAmmInstruction {
            instruction_type: PumpAmmInstructionType::Unknown,
            name: "未知".to_string(),
            params: "数据长度不足".to_string(),
        });
    }
    
    // 使用discriminator识别指令类型
    if is_instruction_match(&instruction.data, &CREATE_POOL_IX) {
        // CreatePool指令
        let params = parse_create_pool_args(&instruction.data)
            .unwrap_or_else(|| "创建池 (无法解析参数)".to_string());
        
        Some(ParsedPumpAmmInstruction {
            instruction_type: PumpAmmInstructionType::CreatePool,
            name: "CreatePool".to_string(),
            params,
        })
    } else if is_instruction_match(&instruction.data, &DEPOSIT_IX) {
        // Deposit指令
        let params = parse_deposit_args(&instruction.data)
            .unwrap_or_else(|| "存入流动性 (无法解析参数)".to_string());
        
        Some(ParsedPumpAmmInstruction {
            instruction_type: PumpAmmInstructionType::Deposit,
            name: "Deposit".to_string(),
            params,
        })
    } else if is_instruction_match(&instruction.data, &BUY_IX) {
        // Buy指令
        let params = parse_buy_args(&instruction.data)
            .unwrap_or_else(|| "买入 (无法解析参数)".to_string());
        
        Some(ParsedPumpAmmInstruction {
            instruction_type: PumpAmmInstructionType::Buy,
            name: "Buy".to_string(),
            params,
        })
    } else if is_instruction_match(&instruction.data, &SELL_IX) {
        // Sell指令
        let params = parse_sell_args(&instruction.data)
            .unwrap_or_else(|| "卖出 (无法解析参数)".to_string());
        
        Some(ParsedPumpAmmInstruction {
            instruction_type: PumpAmmInstructionType::Sell,
            name: "Sell".to_string(),
            params,
        })
    } else if is_instruction_match(&instruction.data, &WITHDRAW_IX) {
        // Withdraw指令
        let params = parse_withdraw_args(&instruction.data)
            .unwrap_or_else(|| "提取流动性 (无法解析参数)".to_string());
        
        Some(ParsedPumpAmmInstruction {
            instruction_type: PumpAmmInstructionType::Withdraw,
            name: "Withdraw".to_string(),
            params,
        })
    } else {
        // 未知指令
        Some(ParsedPumpAmmInstruction {
            instruction_type: PumpAmmInstructionType::Unknown,
            name: "未知Pump AMM指令".to_string(),
            params: format!("未识别的discriminator: {:?}", &instruction.data[0..8]),
        })
    }
}

/// 解析整个交易中的所有Pump AMM指令
pub fn parse_pumpamm_transaction(transaction: &VersionedTransaction) -> Vec<ParsedPumpAmmInstruction> {
    let message = &transaction.message;
    let instructions = message.instructions();
    
    let mut parsed_instructions = Vec::new();
    
    for (i, _) in instructions.iter().enumerate() {
        if let Some(parsed) = parse_pumpamm_instruction(transaction, i) {
            parsed_instructions.push(parsed);
        }
    }
    
    parsed_instructions
}

/// 从指令中提取池地址
pub fn get_pool_from_instruction(transaction: &VersionedTransaction, instruction_index: usize) -> Option<Pubkey> {
    let message = &transaction.message;
    let instructions = message.instructions();
    
    if instruction_index >= instructions.len() {
        return None;
    }
    
    let instruction = &instructions[instruction_index];
    let program_id = instruction.program_id(&message.static_account_keys());
    
    // 检查是否是Pump AMM程序
    if program_id.to_string() != PUMPAMM_PROGRAM_ID {
        return None;
    }
    
    let accounts = &instruction.accounts;
    let static_keys = message.static_account_keys();
    
    // 在不同指令中，池账户通常在不同位置
    if is_instruction_match(&instruction.data, &CREATE_POOL_IX) {
        // CreatePool: 池账户通常是第一个账户
        if !accounts.is_empty() && (accounts[0] as usize) < static_keys.len() {
            return Some(static_keys[accounts[0] as usize]);
        }
    } else if is_instruction_match(&instruction.data, &DEPOSIT_IX) || 
              is_instruction_match(&instruction.data, &WITHDRAW_IX) || 
              is_instruction_match(&instruction.data, &BUY_IX) || 
              is_instruction_match(&instruction.data, &SELL_IX) {
        // 其他指令：池账户通常是第一个账户
        if !accounts.is_empty() && (accounts[0] as usize) < static_keys.len() {
            return Some(static_keys[accounts[0] as usize]);
        }
    }
    
    None
}

/// 从指令中提取基础代币和报价代币地址
pub fn get_token_mints_from_instruction(transaction: &VersionedTransaction, instruction_index: usize) -> Option<(Pubkey, Pubkey)> {
    let message = &transaction.message;
    let instructions = message.instructions();
    
    if instruction_index >= instructions.len() {
        return None;
    }
    
    let instruction = &instructions[instruction_index];
    let program_id = instruction.program_id(&message.static_account_keys());
    
    // 检查是否是Pump AMM程序
    if program_id.to_string() != PUMPAMM_PROGRAM_ID {
        return None;
    }
    
    let accounts = &instruction.accounts;
    let static_keys = message.static_account_keys();
    
    // 在不同指令中，代币铸造地址通常在不同位置
    if is_instruction_match(&instruction.data, &CREATE_POOL_IX) {
        // CreatePool: 基础代币和报价代币通常是第4和第5个账户
        if accounts.len() > 4 && 
           (accounts[3] as usize) < static_keys.len() && 
           (accounts[4] as usize) < static_keys.len() {
            return Some((
                static_keys[accounts[3] as usize],
                static_keys[accounts[4] as usize]
            ));
        }
    } else if is_instruction_match(&instruction.data, &DEPOSIT_IX) || 
              is_instruction_match(&instruction.data, &WITHDRAW_IX) || 
              is_instruction_match(&instruction.data, &BUY_IX) || 
              is_instruction_match(&instruction.data, &SELL_IX) {
        // 其他指令：基础代币和报价代币通常是第3和第4个账户
        if accounts.len() > 4 && 
           (accounts[3] as usize) < static_keys.len() && 
           (accounts[4] as usize) < static_keys.len() {
            return Some((
                static_keys[accounts[3] as usize],
                static_keys[accounts[4] as usize]
            ));
        }
    }
    
    None
}

/// 从交易中提取池信息
pub fn get_pool_info_from_transaction(transaction: &VersionedTransaction) -> Option<PoolInfo> {
    let message = &transaction.message;
    let instructions = message.instructions();
    
    for (i, instruction) in instructions.iter().enumerate() {
        let program_id = instruction.program_id(&message.static_account_keys());
        
        // 检查是否是Pump AMM程序
        if program_id.to_string() != PUMPAMM_PROGRAM_ID {
            continue;
        }
        
        // 获取池地址
        let pool = get_pool_from_instruction(transaction, i)?;
        
        // 获取代币铸造地址
        let (base_mint, quote_mint) = get_token_mints_from_instruction(transaction, i)?;
        
        // 尝试获取LP代币铸造地址（如果适用）
        let mut lp_mint = None;
        if is_instruction_match(&instruction.data, &CREATE_POOL_IX) || 
           is_instruction_match(&instruction.data, &DEPOSIT_IX) || 
           is_instruction_match(&instruction.data, &WITHDRAW_IX) {
            
            let accounts = &instruction.accounts;
            let static_keys = message.static_account_keys();
            
            // LP代币铸造地址通常在创建池、存款和提款指令中的第6个账户
            if accounts.len() > 5 && (accounts[5] as usize) < static_keys.len() {
                lp_mint = Some(static_keys[accounts[5] as usize]);
            }
        }
        
        return Some(PoolInfo {
            pool,
            base_mint,
            quote_mint,
            lp_mint,
        });
    }
    
    None
} 