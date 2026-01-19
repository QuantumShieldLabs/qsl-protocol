use anyhow::{anyhow, Result};
use rand::rngs::OsRng;
use rand::RngCore;
use tokio::time::{sleep, Duration};

use quantumshield_refimpl::crypto::stdcrypto::StdCrypto;
use quantumshield_refimpl::crypto::traits::X25519Dh;
use quantumshield_refimpl::suite2::establish::init_from_base_handshake;
use quantumshield_refimpl::suite2::ratchet::{recv_wire, send_wire};
use quantumshield_refimpl::suite2::types::{SUITE2_PROTOCOL_VERSION, SUITE2_SUITE_ID};

use crate::relay::{RelayClient, RelayRole};

pub enum Mode {
    Local,
    Relay,
}

pub async fn run_demo(mode: Mode, base_url: &str, channel: &str) -> Result<String> {
    match mode {
        Mode::Local => run_local_demo(),
        Mode::Relay => run_relay_demo(base_url, channel).await,
    }
}

fn run_local_demo() -> Result<String> {
    let c = StdCrypto;
    let (a_priv, a_pub) = c.keypair();
    let (b_priv, b_pub) = c.keypair();

    let dh_init_a = c.dh(&a_priv, &b_pub);
    let dh_init_b = c.dh(&b_priv, &a_pub);

    let mut sid = [0u8; 16];
    OsRng.fill_bytes(&mut sid);

    let mut pq_init_ss = [0u8; 32];
    OsRng.fill_bytes(&mut pq_init_ss);

    let a_state = init_from_base_handshake(
        &c,
        true,
        SUITE2_PROTOCOL_VERSION,
        SUITE2_SUITE_ID,
        &sid,
        &dh_init_a,
        &pq_init_ss,
        &a_pub.0,
        &b_pub.0,
        true,
    )
    .map_err(|e| anyhow!(e))?;

    let b_state = init_from_base_handshake(
        &c,
        false,
        SUITE2_PROTOCOL_VERSION,
        SUITE2_SUITE_ID,
        &sid,
        &dh_init_b,
        &pq_init_ss,
        &b_pub.0,
        &a_pub.0,
        true,
    )
    .map_err(|e| anyhow!(e))?;

    let send_out = send_wire(&c, &c, &c, a_state.send, 0, b"hello").map_err(|e| anyhow!(e))?;
    let recv_out =
        recv_wire(&c, &c, &c, b_state.recv, &send_out.wire, None, None).map_err(|e| anyhow!(e))?;

    let pt = String::from_utf8_lossy(&recv_out.plaintext).to_string();
    Ok(pt)
}

async fn run_relay_demo(base_url: &str, channel: &str) -> Result<String> {
    let c = StdCrypto;
    let (a_priv, a_pub) = c.keypair();
    let (b_priv, b_pub) = c.keypair();

    let dh_init_a = c.dh(&a_priv, &b_pub);
    let dh_init_b = c.dh(&b_priv, &a_pub);

    let mut sid = [0u8; 16];
    OsRng.fill_bytes(&mut sid);

    let mut pq_init_ss = [0u8; 32];
    OsRng.fill_bytes(&mut pq_init_ss);

    let a_state = init_from_base_handshake(
        &c,
        true,
        SUITE2_PROTOCOL_VERSION,
        SUITE2_SUITE_ID,
        &sid,
        &dh_init_a,
        &pq_init_ss,
        &a_pub.0,
        &b_pub.0,
        true,
    )
    .map_err(|e| anyhow!(e))?;

    let b_state = init_from_base_handshake(
        &c,
        false,
        SUITE2_PROTOCOL_VERSION,
        SUITE2_SUITE_ID,
        &sid,
        &dh_init_b,
        &pq_init_ss,
        &b_pub.0,
        &a_pub.0,
        true,
    )
    .map_err(|e| anyhow!(e))?;

    let send_out = send_wire(&c, &c, &c, a_state.send, 0, b"hello").map_err(|e| anyhow!(e))?;

    let a_client = RelayClient::new(base_url, channel, RelayRole::A)?;
    let b_client = RelayClient::new(base_url, channel, RelayRole::B)?;

    a_client.push_bytes(&send_out.wire).await?;

    let mut tries = 0u32;
    let wire = loop {
        if let Some(b) = b_client.pull_bytes().await? {
            break b;
        }
        tries += 1;
        if tries >= 20 {
            return Err(anyhow!("RELAY_NO_MESSAGE"));
        }
        sleep(Duration::from_millis(100)).await;
    };

    let recv_out =
        recv_wire(&c, &c, &c, b_state.recv, &wire, None, None).map_err(|e| anyhow!(e))?;

    let pt = String::from_utf8_lossy(&recv_out.plaintext).to_string();
    Ok(pt)
}
