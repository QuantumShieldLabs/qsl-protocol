use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;

use crate::fsutil::write_secure_file;
use crate::store::{StoreState, STATE_FILE_NAME};

pub fn demo_hash(label: &str, parts: &[&[u8]]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(label.as_bytes());
    for p in parts {
        h.update(p);
    }
    let out = h.finalize();
    let mut b = [0u8; 32];
    b.copy_from_slice(&out);
    b
}

pub fn hex_lower(bytes: &[u8]) -> String {
    hex::encode(bytes)
}

pub fn demo_dh_pub_hex(id: &str) -> String {
    let b = demo_hash("qshield-demo-dh-pub", &[id.as_bytes()]);
    hex::encode(b)
}

pub fn demo_pq_kem_pub_id_hex(id: &str) -> String {
    let b = demo_hash("qshield-demo-pq-kem-pub", &[id.as_bytes()]);
    hex::encode(b)
}

pub fn demo_pq_prekey_id(id: &str) -> u32 {
    let b = demo_hash("qshield-demo-pq-prekey-id", &[id.as_bytes()]);
    u32::from_be_bytes([b[0], b[1], b[2], b[3]])
}

pub fn demo_session_id_bytes(a: &str, b: &str) -> [u8; 16] {
    let mut ids = [a, b];
    ids.sort();
    let h = demo_hash("qshield-demo-session", &[ids[0].as_bytes(), ids[1].as_bytes()]);
    let mut out = [0u8; 16];
    out.copy_from_slice(&h[..16]);
    out
}

pub fn demo_dh_init_bytes(a: &str, b: &str, dh_pub_a: &[u8], dh_pub_b: &[u8]) -> [u8; 32] {
    let mut ids = [a, b];
    ids.sort();
    let (p1, p2) = if ids[0] == a { (dh_pub_a, dh_pub_b) } else { (dh_pub_b, dh_pub_a) };
    demo_hash("qshield-demo-dh-init", &[ids[0].as_bytes(), ids[1].as_bytes(), p1, p2])
}

pub fn demo_pq_init_bytes(a: &str, b: &str, dh_pub_a: &[u8], dh_pub_b: &[u8]) -> [u8; 32] {
    let mut ids = [a, b];
    ids.sort();
    let (p1, p2) = if ids[0] == a { (dh_pub_a, dh_pub_b) } else { (dh_pub_b, dh_pub_a) };
    demo_hash("qshield-demo-pq-init", &[ids[0].as_bytes(), ids[1].as_bytes(), p1, p2])
}

pub fn load_or_init_state(path: &Path) -> Result<StoreState, String> {
    if path.exists() {
        let data = fs::read(path).map_err(|e| format!("read state: {e}"))?;
        serde_json::from_slice(&data).map_err(|e| format!("parse state: {e}"))
    } else {
        Ok(StoreState::default())
    }
}

pub fn save_state(path: &Path, state: &StoreState) -> Result<(), String> {
    let data = serde_json::to_vec_pretty(state).map_err(|e| format!("serialize state: {e}"))?;
    write_secure_file(path, &data).map_err(|e| format!("write state: {e}"))
}

pub fn state_path(store: &Path) -> std::path::PathBuf {
    store.join(STATE_FILE_NAME)
}
