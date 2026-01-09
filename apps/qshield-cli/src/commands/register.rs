use std::path::Path;

use crate::config::{self, Config};
use crate::relay_client::{post_json, GenericOk, RegisterRequest};
use crate::store::{StoreState, STATE_FILE_NAME};
use crate::util::{demo_dh_pub_hex, demo_pq_kem_pub_id_hex, demo_pq_prekey_id, load_or_init_state, save_state};

pub fn run(store_path: &Path, my_id: &str) -> Result<(), String> {
    let cfg_path = store_path.join(config::CONFIG_FILE_NAME);
    let cfg: Config = config::read_config(&cfg_path).map_err(|_| {
        format!(
            "config missing or invalid: {} (run: qshield init --store <path>)",
            cfg_path.display()
        )
    })?;

    let state_path = store_path.join(STATE_FILE_NAME);
    let mut state: StoreState = load_or_init_state(&state_path)?;
    if let Some(existing) = &state.my_id {
        if existing != my_id {
            return Err(format!(
                "store already initialized for id {existing}; use a different --store"
            ));
        }
    }

    let dh_pub_hex = demo_dh_pub_hex(my_id);
    let bundle = serde_json::json!({
        "id": my_id,
        "dh_pub": dh_pub_hex,
        "pq_kem_pub_id": demo_pq_kem_pub_id_hex(my_id),
        "pq_prekey_id": demo_pq_prekey_id(my_id),
        "demo": true
    });

    let relay_token = config::resolve_relay_token(&cfg)?;
    let req = RegisterRequest {
        id: my_id.to_string(),
        bundle,
    };
    let resp: GenericOk = post_json(&cfg.relay_url, "/register", &req, &relay_token)?;
    if !resp.ok {
        return Err("relay register failed".to_string());
    }

    state.my_id = Some(my_id.to_string());
    state.dh_pub_hex = Some(demo_dh_pub_hex(my_id));
    save_state(&state_path, &state)?;

    println!("registered id={my_id} with relay {}", cfg.relay_url);
    Ok(())
}
