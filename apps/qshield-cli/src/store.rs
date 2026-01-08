use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub const STATE_FILE_NAME: &str = "state.json";

use crate::fsutil::write_secure_file;
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct StoreState {
    pub my_id: Option<String>,
    pub dh_pub_hex: Option<String>,
    pub sessions: HashMap<String, SessionEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionEntry {
    pub session_id_b64u: String,
    pub session_id_hex: String,
    pub role: String,
    pub dh_init_hex: String,
    pub pq_init_ss_hex: String,
    pub pq_kem_pub_id_hex: String,
    pub pq_prekey_id: u32,
    pub dh_self_pub_hex: String,
    pub dh_peer_pub_hex: String,
}

pub fn write_state(path: &Path, state: &StoreState) -> Result<(), String> {
    let data = serde_json::to_vec_pretty(state).map_err(|e| format!("serialize state: {e}"))?;
    write_secure_file(path, &data).map_err(|e| format!("write state: {e}"))?;
    Ok(())
}

pub fn read_state(path: &Path) -> Result<StoreState, String> {
    let data = fs::read(path).map_err(|e| format!("read state: {e}"))?;
    serde_json::from_slice(&data).map_err(|e| format!("parse state: {e}"))
}
