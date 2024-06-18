use anyhow::Result;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub sso_client_id: String,
    pub sso_client_secret: String,
    pub sso_callback_url: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        let text = fs::read_to_string("config.toml")?;
        let data = toml::from_str(&text)?;
        Ok(data)
    }
}
