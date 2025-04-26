use std::env;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

// 添加Pump AMM程序ID常量
pub const PUMPAMM_PROGRAM_ID: &str = "pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA";

#[derive(Debug)]
pub struct Config {
    pub server_url: String,
    pub target_accounts: Vec<Pubkey>,
}

impl Default for Config {
    fn default() -> Self {
        // 原始创建代币交易相关的目标账户
        let create_account = env::var("CREATE_ACCOUNT")
            .unwrap_or_else(|_| "TSLvdd1pWpHVjahSpsvCXUbgwsL3JAcvokwaKt1eokM".to_string());
            
        // Swap交易相关的目标账户
        let swap_account = env::var("SWAP_ACCOUNT")
            .unwrap_or_else(|_| "Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1".to_string());
            
        // 将账户字符串解析为Pubkey
        let mut accounts = Vec::new();
        if let Ok(pubkey) = Pubkey::from_str(&create_account) {
            accounts.push(pubkey);
        }
        
        if let Ok(pubkey) = Pubkey::from_str(&swap_account) {
            accounts.push(pubkey);
        }
        
        // 添加Pump AMM程序地址
        if let Ok(pubkey) = Pubkey::from_str(PUMPAMM_PROGRAM_ID) {
            accounts.push(pubkey);
        }
        
        // 如果有提供TARGET_ACCOUNT环境变量，也加入
        if let Ok(extra_account) = env::var("TARGET_ACCOUNT") {
            if let Ok(pubkey) = Pubkey::from_str(&extra_account) {
                accounts.push(pubkey);
            }
        }
            
        Self {
            server_url: env::var("SHREDSTREAM_SERVER_URL")
                .unwrap_or_else(|_| "http://127.0.0.1:18890".to_string()),
            target_accounts: accounts,
        }
    }
} 