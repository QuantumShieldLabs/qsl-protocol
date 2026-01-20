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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Mode {
    Local,
    Relay,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrivacyMode {
    Basic,
    Padded,
}

pub struct PaddingInfo {
    pub plain_len: usize,
    pub padded_len: usize,
    pub bucket: usize,
}

pub struct DemoResult {
    pub plaintext: String,
    pub padding: PaddingInfo,
    pub ciphertext_len: usize,
    pub privacy_mode: PrivacyMode,
}

pub async fn run_demo(
    mode: Mode,
    base_url: &str,
    channel: &str,
    privacy_mode: PrivacyMode,
) -> Result<DemoResult> {
    match mode {
        Mode::Local => run_local_demo(privacy_mode),
        Mode::Relay => run_relay_demo(base_url, channel, privacy_mode).await,
    }
}

const PAD_BUCKETS: &[usize] = &[256, 512, 1024, 2048, 4096, 8192];

fn choose_bucket(total_len: usize) -> usize {
    for b in PAD_BUCKETS {
        if total_len <= *b {
            return *b;
        }
    }
    total_len
}

fn pad_payload(plain: &[u8]) -> Result<(Vec<u8>, PaddingInfo)> {
    let plain_len = plain.len();
    if plain_len > u32::MAX as usize {
        return Err(anyhow!("PAD_PLAINTEXT_TOO_LARGE"));
    }
    let total_len = plain_len + 4;
    let bucket = choose_bucket(total_len);
    let mut out = Vec::with_capacity(bucket);
    out.extend_from_slice(&(plain_len as u32).to_be_bytes());
    out.extend_from_slice(plain);
    out.resize(bucket, 0);
    let padded_len = out.len();
    Ok((
        out,
        PaddingInfo {
            plain_len,
            padded_len,
            bucket,
        },
    ))
}

fn no_pad_payload(plain: &[u8]) -> Result<(Vec<u8>, PaddingInfo)> {
    let plain_len = plain.len();
    Ok((
        plain.to_vec(),
        PaddingInfo {
            plain_len,
            padded_len: plain_len,
            bucket: plain_len,
        },
    ))
}

fn unpad_payload(padded: &[u8]) -> Result<Vec<u8>> {
    if padded.len() < 4 {
        return Err(anyhow!("PAD_UNDERFLOW"));
    }
    let mut len_bytes = [0u8; 4];
    len_bytes.copy_from_slice(&padded[..4]);
    let plain_len = u32::from_be_bytes(len_bytes) as usize;
    if plain_len > padded.len() - 4 {
        return Err(anyhow!("PAD_LEN_INVALID"));
    }
    Ok(padded[4..4 + plain_len].to_vec())
}

fn run_local_demo(privacy_mode: PrivacyMode) -> Result<DemoResult> {
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

    let (payload, info) = match privacy_mode {
        PrivacyMode::Basic => no_pad_payload(b"hello")?,
        PrivacyMode::Padded => pad_payload(b"hello")?,
    };
    let send_out = send_wire(&c, &c, &c, a_state.send, 0, &payload).map_err(|e| anyhow!(e))?;
    let recv_out =
        recv_wire(&c, &c, &c, b_state.recv, &send_out.wire, None, None).map_err(|e| anyhow!(e))?;

    let unpadded = match privacy_mode {
        PrivacyMode::Basic => recv_out.plaintext,
        PrivacyMode::Padded => unpad_payload(&recv_out.plaintext)?,
    };
    let pt = String::from_utf8_lossy(&unpadded).to_string();
    Ok(DemoResult {
        plaintext: pt,
        padding: info,
        ciphertext_len: send_out.wire.len(),
        privacy_mode,
    })
}

async fn run_relay_demo(
    base_url: &str,
    channel: &str,
    privacy_mode: PrivacyMode,
) -> Result<DemoResult> {
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

    let (payload, info) = match privacy_mode {
        PrivacyMode::Basic => no_pad_payload(b"hello")?,
        PrivacyMode::Padded => pad_payload(b"hello")?,
    };
    let send_out = send_wire(&c, &c, &c, a_state.send, 0, &payload).map_err(|e| anyhow!(e))?;

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

    let unpadded = match privacy_mode {
        PrivacyMode::Basic => recv_out.plaintext,
        PrivacyMode::Padded => unpad_payload(&recv_out.plaintext)?,
    };
    let pt = String::from_utf8_lossy(&unpadded).to_string();
    Ok(DemoResult {
        plaintext: pt,
        padding: info,
        ciphertext_len: send_out.wire.len(),
        privacy_mode,
    })
}
