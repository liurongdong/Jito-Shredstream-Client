use std::str;

pub struct InstructionParser;

impl InstructionParser {
    pub fn parse_instruction(program_id: &str, data: &[u8]) -> String {
        match program_id {
            "ComputeBudget111111111111111111111111111111" => Self::parse_compute_budget(data),
            "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P" => Self::parse_pump_fun(data),
            "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL" => Self::parse_token_account(data),
            "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA" => Self::parse_token_program(data),
            "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s" => Self::parse_metadata_program(data),
            "11111111111111111111111111111111" => Self::parse_system_program(data),
            _ => format!("未知指令: {:?}", data),
        }
    }

    fn parse_compute_budget(data: &[u8]) -> String {
        if data.is_empty() {
            return "空计算预算指令".to_string();
        }

        match data[0] {
            2 => format!("设置计算预算: {} 单位", u32::from_le_bytes([data[1], data[2], data[3], data[4]])),
            3 => format!("设置计算单元价格: {} lamports/单位", u64::from_le_bytes([data[1], data[2], data[3], data[4], data[5], data[6], data[7], data[8]])),
            _ => format!("未知计算预算指令: {:?}", data),
        }
    }

    fn parse_pump_fun(data: &[u8]) -> String {
        if data.len() < 8 {
            return format!("无效的Pump.fun指令: {:?}", data);
        }

        match data[0] {
            24 => {
                // 解析代币名称
                let name_len = u32::from_le_bytes([data[8], data[9], data[10], data[11]]) as usize;
                if data.len() < 12 + name_len {
                    return format!("无效的代币名称长度: {}", name_len);
                }
                let name = str::from_utf8(&data[12..12 + name_len]).unwrap_or("无效名称");

                // 解析代币符号
                let symbol_start = 12 + name_len;
                if data.len() < symbol_start + 4 {
                    return format!("无效的代币符号起始位置: {}", symbol_start);
                }
                let symbol_len = u32::from_le_bytes([
                    data[symbol_start],
                    data[symbol_start + 1],
                    data[symbol_start + 2],
                    data[symbol_start + 3],
                ]) as usize;
                let symbol_end = symbol_start + 4 + symbol_len;
                if data.len() < symbol_end {
                    return format!("无效的代币符号长度: {}", symbol_len);
                }
                let symbol = str::from_utf8(&data[symbol_start + 4..symbol_end]).unwrap_or("无效符号");

                // 解析代币图标URL
                let icon_start = symbol_end;
                if data.len() < icon_start + 4 {
                    return format!("无效的图标URL起始位置: {}", icon_start);
                }
                let icon_len = u32::from_le_bytes([
                    data[icon_start],
                    data[icon_start + 1],
                    data[icon_start + 2],
                    data[icon_start + 3],
                ]) as usize;
                let icon_end = icon_start + 4 + icon_len;
                if data.len() < icon_end {
                    return format!("无效的图标URL长度: {}", icon_len);
                }
                let icon = str::from_utf8(&data[icon_start + 4..icon_end]).unwrap_or("无效图标URL");

                format!("Token_Metadata:\n  name: {}\n  symbol: {}\n  uri: {}", name, symbol, icon)
            },
            234 => "初始化代币账户".to_string(),
            102 => "设置代币元数据".to_string(),
            // 买入指令
            25 => {
                if data.len() < 16 {
                    return "无效的买入指令".to_string();
                }
                let amount = u64::from_le_bytes([
                    data[8], data[9], data[10], data[11],
                    data[12], data[13], data[14], data[15],
                ]);
                format!("买入代币: {} 单位", amount)
            },
            // 卖出指令
            26 => {
                if data.len() < 16 {
                    return "无效的卖出指令".to_string();
                }
                let amount = u64::from_le_bytes([
                    data[8], data[9], data[10], data[11],
                    data[12], data[13], data[14], data[15],
                ]);
                format!("卖出代币: {} 单位", amount)
            },
            _ => format!("未知Pump.fun指令: {:?}", data),
        }
    }

    fn parse_token_account(data: &[u8]) -> String {
        match data {
            [1] => "创建代币账户".to_string(),
            [2] => "创建代币账户(幂等)".to_string(),
            _ => format!("未知代币账户指令: {:?}", data),
        }
    }

    fn parse_token_program(data: &[u8]) -> String {
        if data.is_empty() {
            return "空代币程序指令".to_string();
        }

        match data[0] {
            7 => "初始化代币账户".to_string(),
            8 => "转账代币".to_string(),
            9 => "关闭代币账户".to_string(),
            14 => "铸造代币".to_string(),
            _ => format!("未知代币程序指令: {:?}", data),
        }
    }

    fn parse_metadata_program(data: &[u8]) -> String {
        if data.is_empty() {
            return "空元数据程序指令".to_string();
        }

        match data[0] {
            33 => "创建元数据账户 v3".to_string(),
            34 => "更新元数据账户 v2".to_string(),
            _ => format!("未知元数据程序指令: {:?}", data),
        }
    }

    fn parse_system_program(data: &[u8]) -> String {
        if data.is_empty() {
            return "空系统程序指令".to_string();
        }

        match data[0] {
            0 => "创建账户".to_string(),
            2 => "转账SOL".to_string(),
            8 => "分配空间".to_string(),
            _ => format!("未知系统程序指令: {:?}", data),
        }
    }
} 