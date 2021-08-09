use serde::{Serialize, Deserialize};
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub external_base_url: String,
    pub internal_base_url: String,
}

const APP_CONFIGURATION_PATH: &str = "./config.json";

pub fn instance() -> Result<AppConfig> {
    let config_text = std::fs::read_to_string(APP_CONFIGURATION_PATH)?;

    let config = serde_json::from_str::<AppConfig>(&config_text)?;

    Ok(config)
}