use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

#[derive(Clone)]
pub struct Config {
    pub server_url: String,
    pub token_creator_pubkey: Pubkey,
}

impl Config {
    pub fn new() -> Self {
        Self {
            server_url: "http://45.77.55.124:9999".to_string(),
            token_creator_pubkey: Pubkey::from_str("TSLvdd1pWpHVjahSpsvCXUbgwsL3JAcvokwaKt1eokM").unwrap(),
        }
    }
} 