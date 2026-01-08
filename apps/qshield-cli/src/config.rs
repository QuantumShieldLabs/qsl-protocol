use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

pub const CONFIG_FILE_NAME: &str = "config.json";

use crate::fsutil::write_secure_file;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub relay_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relay_token: Option<String>,
    #[serde(default)]
    pub padding_enabled: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub padding_buckets: Option<Vec<u32>>,
}

impl Config {
    pub fn new(
        relay_url: String,
        relay_token: Option<String>,
        padding_enabled: bool,
        padding_buckets: Option<Vec<u32>>,
    ) -> Self {
        Self {
            relay_url,
            relay_token,
            padding_enabled,
            padding_buckets,
        }
    }
}

pub fn resolve_relay_token(cfg: &Config) -> Result<String, String> {
    if let Ok(token) = std::env::var("QSHIELD_RELAY_TOKEN") {
        if !token.trim().is_empty() {
            return Ok(token);
        }
    }
    cfg.relay_token
        .clone()
        .filter(|v| !v.trim().is_empty())
        .ok_or_else(|| "relay token missing; set QSHIELD_RELAY_TOKEN or run qshield init --relay-token <token>".to_string())
}

pub fn write_config(path: &Path, cfg: &Config) -> Result<(), String> {
    let data = serde_json::to_vec_pretty(cfg).map_err(|e| format!("serialize config: {e}"))?;
    write_secure_file(path, &data).map_err(|e| format!("write config: {e}"))?;
    Ok(())
}

pub fn read_config(path: &Path) -> Result<Config, String> {
    let data = fs::read(path).map_err(|e| format!("read config: {e}"))?;
    serde_json::from_slice(&data).map_err(|e| format!("parse config: {e}"))
}
