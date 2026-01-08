use std::path::Path;

use crate::config;
use crate::fsutil::{ensure_dir_permissions, secure_delete_file};
use crate::store::STATE_FILE_NAME;

pub fn run(store: &Path) -> Result<(), String> {
    if !store.exists() {
        return Err("store not found; run: qshield init --store <path>".to_string());
    }
    ensure_dir_permissions(store)?;

    let cfg_path = store.join(config::CONFIG_FILE_NAME);
    let state_path = store.join(STATE_FILE_NAME);

    secure_delete_file(&state_path)?;
    secure_delete_file(&cfg_path)?;

    println!("rotated demo store at {}", store.display());
    Ok(())
}
