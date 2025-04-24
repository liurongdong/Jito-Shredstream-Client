use solana_sdk::{
    pubkey::Pubkey,
    transaction::VersionedTransaction,
};
use borsh::BorshDeserialize;

// Pump程序ID
pub const PUMP_PROGRAM_ID: &str = "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P";

// Pump指令编号常量
pub const INITIALIZE_IX: u8 = 0;
pub const SET_PARAMS_IX: u8 = 1;
pub const CREATE_IX: u8 = 2;
pub const BUY_IX: u8 = 3;
pub const SELL_IX: u8 = 4;
pub const WITHDRAW_IX: u8 = 5;
pub const CREATE_COIN_IX: u8 = 24;
pub const BUY_TOKENS_IX: u8 = 102;
pub const SWAP_IX: u8 = 103;
pub const SELL_TOKENS_IX: u8 = 104;
pub const EXTENDED_SELL_IX: u8 = 51;
pub const INIT_IX: u8 = 234;
pub const COMPLETE_IX: u8 = 200; // 假设200是完成指令的编号，可能需要调整

// 指令类型
#[derive(Debug)]
pub enum PumpInstructionType {
    Initialize,
    SetParams,
    Create,
    Buy,
    Sell,
    Withdraw,
    // 自定义/扩展指令
    CreateCoin,    // 24 指令 - 创建代币
    BuyTokens,     // 102 指令 - 购买代币
    Swap,          // 103 指令 - 交换代币
    SellTokens,    // 104 指令 - 出售代币
    ExtendedSell,  // 51 指令 - 扩展版出售代币
    Complete,      // 曲线完成指令
    Init,          // 234 指令 - 初始化操作
    Unknown(()),   // 未知指令，使用()替代u8以避免dead_code警告
}

// 解析后的Pump指令
#[derive(Debug)]
pub struct ParsedPumpInstruction {
    #[allow(dead_code)]
    pub instruction_type: PumpInstructionType,
    pub name: String,
    pub params: String,
}

// 解析后的曲线状态信息
#[derive(Debug)]
pub struct BondingCurveInfo {
    pub mint: Pubkey,
    pub curve_account: Pubkey,
    pub is_complete: bool,
    #[allow(dead_code)]
    pub virtual_token_reserves: Option<u64>,
    #[allow(dead_code)]
    pub virtual_sol_reserves: Option<u64>,
    #[allow(dead_code)]
    pub real_token_reserves: Option<u64>,
    #[allow(dead_code)]
    pub real_sol_reserves: Option<u64>,
}

// 解析后的CompleteEvent
#[derive(Debug)]
pub struct CompleteEvent {
    pub user: Pubkey,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub timestamp: u64,
}

// CompleteEvent反序列化结构体
#[derive(BorshDeserialize, Debug)]
struct CompleteEventArgs {
    user: [u8; 32],
    mint: [u8; 32],
    bonding_curve: [u8; 32],
    timestamp: u64,
}

// Borsh反序列化结构体
#[derive(BorshDeserialize, Debug)]
struct CreateArgs {
    name: String,
    symbol: String,
    uri: String,
}

#[derive(BorshDeserialize, Debug)]
struct BuyArgs {
    amount: u64,
    max_sol_cost: u64,
}

#[derive(BorshDeserialize, Debug)]
struct SellArgs {
    amount: u64,
    min_sol_output: u64,
}

#[derive(BorshDeserialize, Debug)]
struct SwapArgs {
    in_amount: u64,
    min_out_amount: u64,
    input_type: u32,  // 0 = SOL, 1 = 代币
    output_type: u32, // 0 = SOL, 1 = 代币
}

// 扩展的Sell指令参数 (用于discriminator 51)
#[derive(BorshDeserialize, Debug)]
struct ExtendedSellArgs {
    amount: u64,
    min_sol_output: u64,
    // 可能有额外的参数
}

#[derive(BorshDeserialize, Debug)]
struct SetParamsArgs {
    fee_recipient: [u8; 32],
    initial_virtual_token_reserves: u64,
    initial_virtual_sol_reserves: u64,
    initial_real_token_reserves: u64,
    token_total_supply: u64,
    fee_basis_points: u64,
}

// 解析创建代币的参数(非标准格式)
fn parse_create_coin_args(data: &[u8]) -> Option<String> {
    if data.len() < 20 {
        return None;
    }
    
    // 尝试从数据中提取字符串
    // 先提取第一个字符串的长度
    let name_len = u32::from_le_bytes([data[8], data[9], data[10], data[11]]) as usize;
    if 12 + name_len >= data.len() {
        return None;
    }
    
    let name_bytes = &data[12..12+name_len];
    let name = String::from_utf8_lossy(name_bytes).to_string();
    
    // 提取第二个字符串(符号)的长度
    let symbol_offset = 12 + name_len;
    if symbol_offset + 4 >= data.len() {
        return Some(format!("创建代币: 名称=\"{}\"", name));
    }
    
    let symbol_len = u32::from_le_bytes([
        data[symbol_offset], 
        data[symbol_offset+1], 
        data[symbol_offset+2], 
        data[symbol_offset+3]
    ]) as usize;
    
    if symbol_offset + 4 + symbol_len >= data.len() {
        return Some(format!("创建代币: 名称=\"{}\"", name));
    }
    
    let symbol_bytes = &data[symbol_offset+4..symbol_offset+4+symbol_len];
    let symbol = String::from_utf8_lossy(symbol_bytes).to_string();
    
    // 提取第三个字符串(URI)
    let uri_offset = symbol_offset + 4 + symbol_len;
    if uri_offset + 4 >= data.len() {
        return Some(format!("创建代币: 名称=\"{}\", 符号=\"{}\"", name, symbol));
    }
    
    let uri_len = u32::from_le_bytes([
        data[uri_offset], 
        data[uri_offset+1], 
        data[uri_offset+2], 
        data[uri_offset+3]
    ]) as usize;
    
    if uri_offset + 4 + uri_len > data.len() {
        return Some(format!("创建代币: 名称=\"{}\", 符号=\"{}\"", name, symbol));
    }
    
    let uri_bytes = &data[uri_offset+4..uri_offset+4+uri_len];
    let uri = String::from_utf8_lossy(uri_bytes).to_string();
    
    Some(format!("创建代币: 名称=\"{}\", 符号=\"{}\", URI=\"{}\"", name, symbol, uri))
}

// 解析购买代币的参数
fn parse_buy_tokens_args(data: &[u8]) -> Option<String> {
    if data.len() < 24 {
        return None;
    }
    
    // 购买金额
    let amount_bytes = [data[8], data[9], data[10], data[11], data[12], data[13], data[14], data[15]];
    let amount = u64::from_le_bytes(amount_bytes);
    
    // 最大SOL成本
    let max_sol_bytes = [data[16], data[17], data[18], data[19], data[20], data[21], data[22], data[23]];
    let max_sol = u64::from_le_bytes(max_sol_bytes);
    
    // 将lamports转换为SOL进行显示
    let max_sol_display = max_sol as f64 / 1_000_000_000.0;
    
    Some(format!("购买代币: 数量={}, 最大SOL成本={:.9} SOL ({} lamports)", 
        amount, max_sol_display, max_sol))
}

// 解析出售代币的参数
fn parse_sell_tokens_args(data: &[u8]) -> Option<String> {
    if data.len() < 24 {
        return None;
    }
    
    // 出售数量
    let amount_bytes = [data[8], data[9], data[10], data[11], data[12], data[13], data[14], data[15]];
    let amount = u64::from_le_bytes(amount_bytes);
    
    // 最小SOL收益
    let min_sol_bytes = [data[16], data[17], data[18], data[19], data[20], data[21], data[22], data[23]];
    let min_sol = u64::from_le_bytes(min_sol_bytes);
    
    // 将lamports转换为SOL进行显示
    let min_sol_display = min_sol as f64 / 1_000_000_000.0;
    
    Some(format!("出售代币: 数量={}, 最小SOL收益={:.9} SOL ({} lamports)", 
        amount, min_sol_display, min_sol))
}

// 解析代币交换(Swap)参数
fn parse_swap_args(data: &[u8]) -> Option<String> {
    // 尝试使用Borsh结构体解析
    if data.len() >= 8 && data.len() - 8 >= 24 {
        if let Ok(args) = SwapArgs::try_from_slice(&data[8..]) {
            let input_token = if args.input_type == 0 { "SOL" } else { "代币" };
            let output_token = if args.output_type == 0 { "SOL" } else { "代币" };
            
            return Some(format!("交换代币: 输入{}数量={}, 最小输出{}数量={}", 
                input_token, args.in_amount, output_token, args.min_out_amount));
        }
    }
    
    // 回退到手动解析
    if data.len() < 32 {
        return None;
    }
    
    // 输入金额
    let in_amount_bytes = [data[8], data[9], data[10], data[11], data[12], data[13], data[14], data[15]];
    let in_amount = u64::from_le_bytes(in_amount_bytes);
    
    // 最小输出金额
    let min_out_bytes = [data[16], data[17], data[18], data[19], data[20], data[21], data[22], data[23]];
    let min_out_amount = u64::from_le_bytes(min_out_bytes);
    
    // 如果有输入代币类型和输出代币类型信息
    if data.len() >= 32 {
        let input_type_bytes = [data[24], data[25], data[26], data[27]];
        let output_type_bytes = [data[28], data[29], data[30], data[31]];
        
        let input_type = u32::from_le_bytes(input_type_bytes);
        let output_type = u32::from_le_bytes(output_type_bytes);
        
        let input_token = if input_type == 0 { "SOL" } else { "代币" };
        let output_token = if output_type == 0 { "SOL" } else { "代币" };
        
        return Some(format!("交换代币: 输入{}数量={}, 最小输出{}数量={}", 
            input_token, in_amount, output_token, min_out_amount));
    }
    
    Some(format!("交换代币: 输入数量={}, 最小输出数量={}", in_amount, min_out_amount))
}

/// 解析Complete事件数据
pub fn parse_complete_event(data: &[u8]) -> Option<CompleteEvent> {
    if data.len() < 8 + std::mem::size_of::<CompleteEventArgs>() {
        return None;
    }

    if let Ok(args) = CompleteEventArgs::try_from_slice(&data[8..]) {
        let user = Pubkey::new_from_array(args.user);
        let mint = Pubkey::new_from_array(args.mint);
        let bonding_curve = Pubkey::new_from_array(args.bonding_curve);
        let timestamp = args.timestamp;
        
        Some(CompleteEvent {
            user,
            mint,
            bonding_curve,
            timestamp,
        })
    } else {
        None
    }
}

/// 格式化时间戳为可读格式
pub fn format_timestamp(timestamp: u64) -> String {
    let seconds = (timestamp / 1000) as i64;
    let nanos = ((timestamp % 1000) * 1_000_000) as u32;
    
    // 使用当前版本的chrono
    if let Some(datetime) = chrono::DateTime::<chrono::Utc>::from_timestamp(seconds, nanos) {
        datetime.format("时间戳=%Y-%m-%d %H:%M:%S").to_string()
    } else {
        format!("时间戳={}", timestamp)
    }
}

/// 将lamports转换为SOL字符串
pub fn lamports_to_sol_string(lamports: u64) -> String {
    let sol = lamports as f64 / 1_000_000_000.0;
    format!("{:.9} SOL", sol)
}

pub fn parse_pump_instruction(transaction: &VersionedTransaction, instruction_index: usize) -> Option<ParsedPumpInstruction> {
    let message = &transaction.message;
    let instructions = message.instructions();
    
    if instruction_index >= instructions.len() {
        return None;
    }
    
    let instruction = &instructions[instruction_index];
    let program_id = instruction.program_id(&message.static_account_keys());
    
    // 检查是否是Pump程序
    if program_id.to_string() != PUMP_PROGRAM_ID {
        return None;
    }
    
    // 解析指令类型
    if instruction.data.is_empty() {
        return None;
    }
    
    let discriminator = instruction.data[0];
    
    match discriminator {
        INITIALIZE_IX => {
            // Initialize
            Some(ParsedPumpInstruction {
                instruction_type: PumpInstructionType::Initialize,
                name: "Initialize".to_string(),
                params: "初始化全局状态".to_string(),
            })
        },
        SET_PARAMS_IX => {
            // SetParams
            if let Ok(args) = SetParamsArgs::try_from_slice(&instruction.data[8..]) {
                let fee_recipient = Pubkey::new_from_array(args.fee_recipient);
                Some(ParsedPumpInstruction {
                    instruction_type: PumpInstructionType::SetParams,
                    name: "SetParams".to_string(),
                    params: format!(
                        "设置参数: fee_recipient={}, 初始虚拟代币储备={}, 初始虚拟SOL储备={}, 初始实际代币储备={}, 代币总供应量={}, 费用基点={}",
                        fee_recipient,
                        args.initial_virtual_token_reserves,
                        args.initial_virtual_sol_reserves,
                        args.initial_real_token_reserves,
                        args.token_total_supply,
                        args.fee_basis_points
                    ),
                })
            } else {
                Some(ParsedPumpInstruction {
                    instruction_type: PumpInstructionType::SetParams,
                    name: "SetParams".to_string(),
                    params: "设置参数 (数据解析失败)".to_string(),
                })
            }
        },
        CREATE_IX => {
            // Create
            if let Ok(args) = CreateArgs::try_from_slice(&instruction.data[8..]) {
                Some(ParsedPumpInstruction {
                    instruction_type: PumpInstructionType::Create,
                    name: "Create".to_string(),
                    params: format!(
                        "创建代币: 名称=\"{}\", 符号=\"{}\", URI=\"{}\"",
                        args.name, args.symbol, args.uri
                    ),
                })
            } else {
                Some(ParsedPumpInstruction {
                    instruction_type: PumpInstructionType::Create,
                    name: "Create".to_string(),
                    params: "创建代币 (数据解析失败)".to_string(),
                })
            }
        },
        BUY_IX => {
            // Buy
            if let Ok(args) = BuyArgs::try_from_slice(&instruction.data[8..]) {
                Some(ParsedPumpInstruction {
                    instruction_type: PumpInstructionType::Buy,
                    name: "Buy".to_string(),
                    params: format!(
                        "购买代币: 数量={}, 最大SOL成本={}",
                        args.amount, lamports_to_sol_string(args.max_sol_cost)
                    ),
                })
            } else {
                Some(ParsedPumpInstruction {
                    instruction_type: PumpInstructionType::Buy,
                    name: "Buy".to_string(),
                    params: "购买代币 (数据解析失败)".to_string(),
                })
            }
        },
        SELL_IX => {
            // Sell
            if let Ok(args) = SellArgs::try_from_slice(&instruction.data[8..]) {
                Some(ParsedPumpInstruction {
                    instruction_type: PumpInstructionType::Sell,
                    name: "Sell".to_string(),
                    params: format!(
                        "出售代币: 数量={}, 最小SOL收益={}",
                        args.amount, lamports_to_sol_string(args.min_sol_output)
                    ),
                })
            } else {
                Some(ParsedPumpInstruction {
                    instruction_type: PumpInstructionType::Sell,
                    name: "Sell".to_string(),
                    params: "出售代币 (数据解析失败)".to_string(),
                })
            }
        },
        WITHDRAW_IX => {
            // Withdraw
            Some(ParsedPumpInstruction {
                instruction_type: PumpInstructionType::Withdraw,
                name: "Withdraw".to_string(),
                params: "提取流动性".to_string(),
            })
        },
        CREATE_COIN_IX => {
            // CreateCoin - 观察到的自定义指令
            let params = parse_create_coin_args(&instruction.data)
                .unwrap_or_else(|| "创建代币 (无法解析参数)".to_string());
            
            Some(ParsedPumpInstruction {
                instruction_type: PumpInstructionType::CreateCoin,
                name: "CreateCoin".to_string(),
                params,
            })
        },
        BUY_TOKENS_IX => {
            // BuyTokens - 观察到的自定义指令
            if let Ok(args) = BuyArgs::try_from_slice(&instruction.data[8..]) {
                Some(ParsedPumpInstruction {
                    instruction_type: PumpInstructionType::BuyTokens,
                    name: "BuyTokens".to_string(),
                    params: format!(
                        "购买代币: 数量={}, 最大SOL成本={}",
                        args.amount, lamports_to_sol_string(args.max_sol_cost)
                    ),
                })
            } else {
                // 如果结构体解析失败，回退到手动解析
                let params = parse_buy_tokens_args(&instruction.data)
                    .unwrap_or_else(|| "购买代币 (无法解析参数)".to_string());
                
                Some(ParsedPumpInstruction {
                    instruction_type: PumpInstructionType::BuyTokens,
                    name: "BuyTokens".to_string(),
                    params,
                })
            }
        },
        SWAP_IX => {
            // Swap - 交换代币
            if let Ok(args) = SwapArgs::try_from_slice(&instruction.data[8..]) {
                let input_token = if args.input_type == 0 { "SOL" } else { "代币" };
                let output_token = if args.output_type == 0 { "SOL" } else { "代币" };
                
                Some(ParsedPumpInstruction {
                    instruction_type: PumpInstructionType::Swap,
                    name: "Swap".to_string(),
                    params: format!(
                        "交换代币: 输入{}数量={}, 最小输出{}数量={}",
                        input_token, args.in_amount, output_token, args.min_out_amount
                    ),
                })
            } else {
                // 如果结构体解析失败，回退到手动解析
                let params = parse_swap_args(&instruction.data)
                    .unwrap_or_else(|| "交换代币 (无法解析参数)".to_string());
                
                Some(ParsedPumpInstruction {
                    instruction_type: PumpInstructionType::Swap,
                    name: "Swap".to_string(),
                    params,
                })
            }
        },
        SELL_TOKENS_IX => {
            // SellTokens - 出售代币
            if let Ok(args) = SellArgs::try_from_slice(&instruction.data[8..]) {
                Some(ParsedPumpInstruction {
                    instruction_type: PumpInstructionType::SellTokens,
                    name: "SellTokens".to_string(),
                    params: format!(
                        "出售代币: 数量={}, 最小SOL收益={}",
                        args.amount, lamports_to_sol_string(args.min_sol_output)
                    ),
                })
            } else {
                // 如果结构体解析失败，回退到手动解析
                let params = parse_sell_tokens_args(&instruction.data)
                    .unwrap_or_else(|| "出售代币 (无法解析参数)".to_string());
                
                Some(ParsedPumpInstruction {
                    instruction_type: PumpInstructionType::SellTokens,
                    name: "SellTokens".to_string(),
                    params,
                })
            }
        },
        EXTENDED_SELL_IX => {
            // ExtendedSell - 扩展版出售代币指令
            if let Ok(args) = ExtendedSellArgs::try_from_slice(&instruction.data[8..]) {
                Some(ParsedPumpInstruction {
                    instruction_type: PumpInstructionType::ExtendedSell,
                    name: "ExtendedSell".to_string(),
                    params: format!(
                        "出售代币(扩展): 数量={}, 最小SOL收益={}",
                        args.amount, lamports_to_sol_string(args.min_sol_output)
                    ),
                })
            } else if instruction.data.len() >= 24 {
                // 尝试手动解析
                let amount_bytes = [instruction.data[8], instruction.data[9], instruction.data[10], 
                                   instruction.data[11], instruction.data[12], instruction.data[13], 
                                   instruction.data[14], instruction.data[15]];
                let amount = u64::from_le_bytes(amount_bytes);
                
                let min_sol_bytes = [instruction.data[16], instruction.data[17], instruction.data[18], 
                                    instruction.data[19], instruction.data[20], instruction.data[21], 
                                    instruction.data[22], instruction.data[23]];
                let min_sol = u64::from_le_bytes(min_sol_bytes);
                
                Some(ParsedPumpInstruction {
                    instruction_type: PumpInstructionType::ExtendedSell,
                    name: "ExtendedSell".to_string(),
                    params: format!(
                        "出售代币(扩展): 数量={}, 最小SOL收益={}",
                        amount, lamports_to_sol_string(min_sol)
                    ),
                })
            } else {
                Some(ParsedPumpInstruction {
                    instruction_type: PumpInstructionType::ExtendedSell,
                    name: "ExtendedSell".to_string(),
                    params: "出售代币(扩展) (无法解析参数)".to_string(),
                })
            }
        },
        COMPLETE_IX => {
            // Complete - 曲线完成指令
            if let Some(event) = parse_complete_event(&instruction.data) {
                Some(ParsedPumpInstruction {
                    instruction_type: PumpInstructionType::Complete,
                    name: "Complete".to_string(),
                    params: format!(
                        "曲线完成: 用户={}, 代币={}, 曲线={}, {}",
                        event.user, event.mint, event.bonding_curve, format_timestamp(event.timestamp)
                    ),
                })
            } else if let Ok(args) = CompleteEventArgs::try_from_slice(&instruction.data[8..]) {
                let user = Pubkey::new_from_array(args.user);
                let mint = Pubkey::new_from_array(args.mint);
                let bonding_curve = Pubkey::new_from_array(args.bonding_curve);
                let timestamp = args.timestamp;
                
                Some(ParsedPumpInstruction {
                    instruction_type: PumpInstructionType::Complete,
                    name: "Complete".to_string(),
                    params: format!(
                        "曲线完成: 用户={}, 代币={}, 曲线={}, {}",
                        user, mint, bonding_curve, format_timestamp(timestamp)
                    ),
                })
            } else {
                Some(ParsedPumpInstruction {
                    instruction_type: PumpInstructionType::Complete,
                    name: "Complete".to_string(),
                    params: "曲线完成 (数据解析失败)".to_string(),
                })
            }
        },
        INIT_IX => {
            // Init 初始化指令
            Some(ParsedPumpInstruction {
                instruction_type: PumpInstructionType::Init,
                name: "Init".to_string(),
                params: "初始化操作".to_string(),
            })
        },
        _ => Some(ParsedPumpInstruction {
            instruction_type: PumpInstructionType::Unknown(()),
            name: format!("Unknown_{}", discriminator),
            params: format!("未知指令: discriminator={}", discriminator),
        }),
    }
}

// 获取与交易相关的Mint地址
pub fn get_mint_from_transaction(transaction: &VersionedTransaction) -> Option<Pubkey> {
    let message = &transaction.message;
    let static_keys = message.static_account_keys();
    
    // 首先尝试在账户中寻找以"pump"结尾的地址，这通常是Pump协议的代币地址
    for key in static_keys {
        let key_str = key.to_string();
        if key_str.ends_with("pump") {
            return Some(*key);
        }
    }
    
    // 如果没有找到pump结尾的地址，则尝试通过指令内容来查找
    let instructions = message.instructions();
    
    for instruction in instructions {
        let program_id = instruction.program_id(&static_keys);
        
        // 如果是Pump程序指令
        if program_id.to_string() == PUMP_PROGRAM_ID {
            // 检查是创建或交易指令
            if !instruction.data.is_empty() {
                let discriminator = instruction.data[0];
                if discriminator == CREATE_IX || discriminator == BUY_IX || discriminator == SELL_IX || 
                   discriminator == CREATE_COIN_IX || discriminator == EXTENDED_SELL_IX || 
                   discriminator == BUY_TOKENS_IX || discriminator == SWAP_IX || 
                   discriminator == SELL_TOKENS_IX {
                    
                    // 查找指令的账户列表中与mint相关的账户
                    // 通常mint是前几个账户之一
                    if !instruction.accounts.is_empty() {
                        for (i, &account_index) in instruction.accounts.iter().enumerate().take(3) {
                            if (account_index as usize) < static_keys.len() {
                                let potential_mint = static_keys[account_index as usize];
                                let key_str = potential_mint.to_string();
                                
                                // 如果找到以pump结尾的账户，或者是第一个账户（对于CREATE_IX）
                                if key_str.ends_with("pump") || 
                                   (discriminator == CREATE_IX && i == 0) {
                                    return Some(potential_mint);
                                }
                            }
                        }
                    }
                } else if discriminator == COMPLETE_IX {
                    // 在Complete指令中，mint通常在事件参数中
                    if instruction.data.len() >= 8 + 32*3 {
                        if let Ok(args) = CompleteEventArgs::try_from_slice(&instruction.data[8..]) {
                            return Some(Pubkey::new_from_array(args.mint));
                        }
                    }
                }
            }
        }
    }
    
    None
}

// 获取与交易相关的BondingCurve信息
pub fn get_bonding_curve_info(transaction: &VersionedTransaction) -> Option<BondingCurveInfo> {
    let message = &transaction.message;
    let static_keys = message.static_account_keys();
    
    // 首先尝试找出mint地址
    let mint = get_mint_from_transaction(transaction)?;
    
    // 默认值
    let mut result = BondingCurveInfo {
        mint,
        curve_account: Pubkey::default(),
        is_complete: false,
        virtual_token_reserves: None,
        virtual_sol_reserves: None,
        real_token_reserves: None,
        real_sol_reserves: None,
    };
    
    // 找到以"pump"结尾但不是mint的账户，这通常是曲线账户
    for key in static_keys {
        let key_str = key.to_string();
        if key_str.ends_with("pump") && *key != mint {
            result.curve_account = *key;
            break;
        }
    }
    
    // 如果找不到，则尝试通过特定指令查找
    if result.curve_account == Pubkey::default() {
        // 查找BondingCurve账户
        for instruction in message.instructions() {
            let program_id = instruction.program_id(&static_keys);
            
            // 如果是Pump程序指令
            if program_id.to_string() == PUMP_PROGRAM_ID && !instruction.accounts.is_empty() {
                let discriminator = if instruction.data.is_empty() { 0 } else { instruction.data[0] };
                
                match discriminator {
                    // 创建代币
                    CREATE_IX | CREATE_COIN_IX => {
                        if instruction.accounts.len() > 3 {
                            let curve_index = instruction.accounts[2] as usize;
                            if curve_index < static_keys.len() {
                                result.curve_account = static_keys[curve_index];
                            }
                        }
                    },
                    // 买入代币
                    BUY_IX | BUY_TOKENS_IX => {
                        if instruction.accounts.len() > 3 {
                            let curve_index = instruction.accounts[3] as usize;
                            if curve_index < static_keys.len() {
                                result.curve_account = static_keys[curve_index];
                            }
                        }
                    },
                    // 卖出代币
                    SELL_IX | EXTENDED_SELL_IX | SELL_TOKENS_IX => {
                        if instruction.accounts.len() > 3 {
                            let curve_index = instruction.accounts[3] as usize;
                            if curve_index < static_keys.len() {
                                result.curve_account = static_keys[curve_index];
                            }
                        }
                    },
                    // 交换代币
                    SWAP_IX => {
                        if instruction.accounts.len() > 3 {
                            let curve_index = instruction.accounts[3] as usize;
                            if curve_index < static_keys.len() {
                                result.curve_account = static_keys[curve_index];
                            }
                        }
                    },
                    // 曲线完成
                    COMPLETE_IX => {
                        if let Ok(args) = CompleteEventArgs::try_from_slice(&instruction.data[8..]) {
                            result.is_complete = true;
                            result.curve_account = Pubkey::new_from_array(args.bonding_curve);
                        }
                    },
                    _ => {}
                }
            }
        }
    }
    
    if result.curve_account != Pubkey::default() {
        Some(result)
    } else {
        None
    }
}

// 解析交易中的Pump指令
pub fn parse_pump_transaction(transaction: &VersionedTransaction) -> Vec<ParsedPumpInstruction> {
    let message = &transaction.message;
    let instructions = message.instructions();
    let mut parsed_instructions = Vec::new();
    
    for (index, _) in instructions.iter().enumerate() {
        if let Some(parsed) = parse_pump_instruction(transaction, index) {
            parsed_instructions.push(parsed);
        }
    }
    
    parsed_instructions
}