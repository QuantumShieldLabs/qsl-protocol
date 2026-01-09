use std::path::Path;

use crate::config::{self, Config};
use crate::store::{self, STATE_FILE_NAME};

pub fn run(store: &Path) -> Result<(), String> {
    if !store.exists() {
        return Err(format!(
            "store not found: {} (run: qshield init --store <path>)",
            store.display()
        ));
    }

    let cfg_path = store.join(config::CONFIG_FILE_NAME);
    let cfg: Config = config::read_config(&cfg_path).map_err(|_| {
        format!(
            "config missing or invalid: {} (run: qshield init --store <path>)",
            cfg_path.display()
        )
    })?;

    let state_path = store.join(STATE_FILE_NAME);
    let state = store::read_state(&state_path).map_err(|_| {
        format!(
            "state missing or invalid: {} (run: qshield init --store <path>)",
            state_path.display()
        )
    })?;

    println!("store: {}", store.display());
    println!("relay_url: {}", cfg.relay_url);
    if let Some(id) = state.my_id {
        println!("id: {}", id);
    } else {
        println!("id: (not registered)");
    }
    println!("sessions: {}", state.sessions.len());
    Ok(())
}
