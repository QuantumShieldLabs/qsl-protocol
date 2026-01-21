use anyhow::{anyhow, Result};
use tokio::time::{sleep, Duration};

use quantumshield_refimpl::crypto::stdcrypto::StdCrypto;
use quantumshield_refimpl::crypto::traits::{Hash, X25519Dh, X25519Priv, X25519Pub};
use quantumshield_refimpl::suite2::establish::init_from_base_handshake;
use quantumshield_refimpl::suite2::ratchet::{recv_wire, send_wire};
use quantumshield_refimpl::suite2::state::Suite2SessionState;
use quantumshield_refimpl::suite2::types::{SUITE2_PROTOCOL_VERSION, SUITE2_SUITE_ID};
use x25519_dalek::{PublicKey, StaticSecret};

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PartyRole {
    Sender,
    Receiver,
}

pub struct PartyResult {
    pub role: PartyRole,
    pub result: DemoResult,
}

pub async fn run_demo(
    mode: Mode,
    base_url: &str,
    channel: &str,
    privacy_mode: PrivacyMode,
    proxy: Option<&str>,
) -> Result<DemoResult> {
    match mode {
        Mode::Local => run_local_demo(channel, privacy_mode),
        Mode::Relay => run_relay_demo(base_url, channel, privacy_mode, proxy).await,
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

fn run_local_demo(channel: &str, privacy_mode: PrivacyMode) -> Result<DemoResult> {
    let c = StdCrypto;
    let (a_state, b_state) = init_states_for_channel(channel)?;
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
    proxy: Option<&str>,
) -> Result<DemoResult> {
    let c = StdCrypto;
    let (a_state, b_state) = init_states_for_channel(channel)?;
    let (payload, info) = match privacy_mode {
        PrivacyMode::Basic => no_pad_payload(b"hello")?,
        PrivacyMode::Padded => pad_payload(b"hello")?,
    };
    let send_out = send_wire(&c, &c, &c, a_state.send, 0, &payload).map_err(|e| anyhow!(e))?;

    let a_client = RelayClient::new(base_url, channel, RelayRole::A, proxy)?;
    let b_client = RelayClient::new(base_url, channel, RelayRole::B, proxy)?;

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

pub async fn run_party_once(
    role: PartyRole,
    mode: Mode,
    base_url: &str,
    channel: &str,
    privacy_mode: PrivacyMode,
    proxy: Option<&str>,
    message: &str,
) -> Result<PartyResult> {
    let c = StdCrypto;
    let (a_state, b_state) = init_states_for_channel(channel)?;
    let (push_channel, pull_channel) = match role {
        PartyRole::Sender => (format!("{channel}--a2b"), format!("{channel}--b2a")),
        PartyRole::Receiver => (format!("{channel}--b2a"), format!("{channel}--a2b")),
    };

    if role == PartyRole::Sender {
        let payload = match privacy_mode {
            PrivacyMode::Basic => no_pad_payload(message.as_bytes())?,
            PrivacyMode::Padded => pad_payload(message.as_bytes())?,
        };
        let send_out =
            send_wire(&c, &c, &c, a_state.send, 0, &payload.0).map_err(|e| anyhow!(e))?;
        if mode == Mode::Relay {
            let relay = RelayClient::new(base_url, channel, RelayRole::A, proxy)?;
            relay.push_bytes(&send_out.wire).await?;
        } else {
            local_push_bytes(&push_channel, &send_out.wire)?;
        }
        let result = DemoResult {
            plaintext: message.to_string(),
            padding: payload.1,
            ciphertext_len: send_out.wire.len(),
            privacy_mode,
        };
        return Ok(PartyResult { role, result });
    }

    let wire = if mode == Mode::Relay {
        let relay = RelayClient::new(base_url, channel, RelayRole::B, proxy)?;
        relay_pull_wait(&relay).await?
    } else {
        local_pull_wait(&pull_channel).await?
    };
    let recv_out =
        recv_wire(&c, &c, &c, b_state.recv, &wire, None, None).map_err(|e| anyhow!(e))?;
    let recv_plain_len = recv_out.plaintext.len();
    let unpadded = match privacy_mode {
        PrivacyMode::Basic => recv_out.plaintext,
        PrivacyMode::Padded => unpad_payload(&recv_out.plaintext)?,
    };
    let plaintext = String::from_utf8_lossy(&unpadded).to_string();
    let padding = match privacy_mode {
        PrivacyMode::Basic => PaddingInfo {
            plain_len: plaintext.len(),
            padded_len: recv_plain_len,
            bucket: recv_plain_len,
        },
        PrivacyMode::Padded => PaddingInfo {
            plain_len: plaintext.len(),
            padded_len: recv_plain_len,
            bucket: choose_bucket(recv_plain_len),
        },
    };

    let result = DemoResult {
        plaintext,
        padding,
        ciphertext_len: wire.len(),
        privacy_mode,
    };
    Ok(PartyResult { role, result })
}

pub fn format_meta_line(
    role: PartyRole,
    mode: Mode,
    proxy_on: bool,
    privacy_mode: PrivacyMode,
    result: &DemoResult,
) -> String {
    let role_s = match role {
        PartyRole::Sender => "sender",
        PartyRole::Receiver => "receiver",
    };
    let mode_s = match mode {
        Mode::Local => "local",
        Mode::Relay => "relay",
    };
    let proxy_s = if proxy_on { "on" } else { "off" };
    let privacy_s = match privacy_mode {
        PrivacyMode::Basic => "basic",
        PrivacyMode::Padded => "padded",
    };
    format!(
        "QSL_TUI_META role={} mode={} proxy={} privacy={} plaintext_len={} ciphertext_len={} padded_len={} bucket={}",
        role_s,
        mode_s,
        proxy_s,
        privacy_s,
        result.padding.plain_len,
        result.ciphertext_len,
        result.padding.padded_len,
        result.padding.bucket
    )
}

fn local_push_bytes(channel: &str, payload: &[u8]) -> Result<()> {
    let path = local_queue_path(channel);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)?;
    use std::io::Write;
    let len = (payload.len() as u32).to_be_bytes();
    f.write_all(&len)?;
    f.write_all(payload)?;
    Ok(())
}

async fn local_pull_wait(channel: &str) -> Result<Vec<u8>> {
    use std::io::Read;
    let path = local_queue_path(channel);
    let start = std::time::Instant::now();
    loop {
        if path.exists() {
            let mut f = std::fs::OpenOptions::new().read(true).open(&path)?;
            let mut len_buf = [0u8; 4];
            if f.read_exact(&mut len_buf).is_ok() {
                let len = u32::from_be_bytes(len_buf) as usize;
                let mut buf = vec![0u8; len];
                f.read_exact(&mut buf)?;
                let _ = std::fs::remove_file(&path);
                return Ok(buf);
            }
        }
        if start.elapsed() > Duration::from_secs(30) {
            return Err(anyhow!("LOCAL_PULL_TIMEOUT"));
        }
        sleep(Duration::from_millis(200)).await;
    }
}

async fn relay_pull_wait(relay: &RelayClient) -> Result<Vec<u8>> {
    let mut tries = 0u32;
    loop {
        if let Some(b) = relay.pull_bytes().await? {
            return Ok(b);
        }
        tries += 1;
        if tries >= 200 {
            return Err(anyhow!("RELAY_NO_MESSAGE"));
        }
        sleep(Duration::from_millis(200)).await;
    }
}

fn local_queue_path(channel: &str) -> std::path::PathBuf {
    let base =
        std::env::var("QSL_TUI_LOCAL_DIR").unwrap_or_else(|_| "/tmp/qsl_tui_local".to_string());
    let safe = channel.replace('/', "_");
    std::path::Path::new(&base).join(format!("{safe}.bin"))
}

fn init_states_for_channel(channel: &str) -> Result<(Suite2SessionState, Suite2SessionState)> {
    let c = StdCrypto;
    let keys = labeled_hash(&c, "keys", channel);
    let mut a_seed = [0u8; 32];
    let mut b_seed = [0u8; 32];
    a_seed.copy_from_slice(&keys[0..32]);
    b_seed.copy_from_slice(&keys[32..64]);
    let (a_priv, a_pub) = keypair_from_seed(a_seed);
    let (b_priv, b_pub) = keypair_from_seed(b_seed);

    let dh_init_a = c.dh(&a_priv, &b_pub);
    let dh_init_b = c.dh(&b_priv, &a_pub);

    let sid_hash = labeled_hash(&c, "sid", channel);
    let mut sid = [0u8; 16];
    sid.copy_from_slice(&sid_hash[0..16]);

    let pq_hash = labeled_hash(&c, "pq", channel);
    let mut pq_init_ss = [0u8; 32];
    pq_init_ss.copy_from_slice(&pq_hash[0..32]);

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

    Ok((a_state, b_state))
}

fn keypair_from_seed(seed: [u8; 32]) -> (X25519Priv, X25519Pub) {
    let sk = StaticSecret::from(seed);
    let pk = PublicKey::from(&sk);
    (X25519Priv(sk.to_bytes()), X25519Pub(pk.to_bytes()))
}

fn labeled_hash(c: &StdCrypto, label: &str, channel: &str) -> [u8; 64] {
    let mut data = Vec::with_capacity(label.len() + channel.len() + 1);
    data.extend_from_slice(label.as_bytes());
    data.push(b':');
    data.extend_from_slice(channel.as_bytes());
    c.sha512(&data)
}
