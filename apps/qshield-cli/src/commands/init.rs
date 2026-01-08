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
        let mut out = Vec::new();
        for part in raw.split(',') {
            let v = part.trim();
            if v.is_empty() {
                continue;
            }
            let n: u32 = v
                .parse()
                .map_err(|_| format!("invalid padding bucket: {v}"))?;
            if n == 0 {
                return Err("padding bucket must be > 0".to_string());
            }
            out.push(n);
        }
        if out.is_empty() {
            None
        } else {
            out.sort_unstable();
            out.dedup();
            Some(out)
        }
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
