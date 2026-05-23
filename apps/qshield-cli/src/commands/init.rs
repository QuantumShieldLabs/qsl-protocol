use std::fs;
use std::path::Path;

use crate::config::{self, Config};
use crate::fsutil::ensure_dir_permissions;
use crate::store::{StoreState, STATE_FILE_NAME};
use crate::util::save_state;

pub fn run(
    store: &Path,
    relay_url: Option<String>,
    relay_token: Option<String>,
    padding_enable: bool,
    padding_buckets: Option<String>,
) -> Result<(), String> {
    fs::create_dir_all(store).map_err(|e| format!("create store dir: {e}"))?;
    ensure_dir_permissions(store)?;

    let cfg_path = store.join(config::CONFIG_FILE_NAME);
    if cfg_path.exists() {
        return Err("config already exists; delete it or choose a new --store".to_string());
    }

    let relay = relay_url.unwrap_or_else(|| "http://127.0.0.1:18080".to_string());
    let buckets = if let Some(raw) = padding_buckets {
        Some(config::parse_padding_buckets_csv(&raw)?)
    } else if padding_enable {
        config::demo_padding_buckets_from_env()?
    } else {
        None
    };
    if padding_enable && buckets.as_ref().map(|v| v.is_empty()).unwrap_or(true) {
        return Err("padding enabled but no padding buckets provided".to_string());
    }
    let cfg = Config::new(relay, relay_token, padding_enable, buckets);
    config::write_config(&cfg_path, &cfg)?;

    let state_path = store.join(STATE_FILE_NAME);
    if !state_path.exists() {
        save_state(&state_path, &StoreState::default())?;
    }

    println!("Initialized demo store at {}", store.display());
    println!("Wrote config: {}", cfg_path.display());
    Ok(())
}
