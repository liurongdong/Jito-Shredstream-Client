use std::env;

#[derive(Debug)]
pub struct Config {
    pub server_url: String,
    pub target_account: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server_url: env::var("SHREDSTREAM_SERVER_URL")
                .unwrap_or_else(|_| "http://45.77.55.124:9999".to_string()),
            target_account: env::var("TARGET_ACCOUNT")
                .unwrap_or_else(|_| "TSLvdd1pWpHVjahSpsvCXUbgwsL3JAcvokwaKt1eokM".to_string()),
        }
    }
} 