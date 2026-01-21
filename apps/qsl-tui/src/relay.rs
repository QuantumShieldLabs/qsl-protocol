use anyhow::{anyhow, Result};
use reqwest::{Client, Proxy};
use std::time::Duration;

const DEFAULT_TIMEOUT_SECS: u64 = 5;
const REASON_REMOTE_OPT_IN: &str = "reason_code=REMOTE_OPT_IN_REQUIRED";

#[derive(Clone, Copy)]
pub enum RelayRole {
    A,
    B,
}

pub struct RelayClient {
    base_url: String,
    push_channel: String,
    pull_channel: String,
    client: Client,
}

impl RelayClient {
    pub fn new(
        base_url: &str,
        channel: &str,
        role: RelayRole,
        proxy: Option<&str>,
    ) -> Result<Self> {
        let base_url = base_url.trim_end_matches('/').to_string();
        enforce_remote_opt_in(&base_url)?;

        let (push_channel, pull_channel) = derive_channels(channel, role);
        let mut builder = Client::builder().timeout(Duration::from_secs(DEFAULT_TIMEOUT_SECS));
        if let Some(proxy_url) = proxy {
            builder = builder.proxy(Proxy::all(proxy_url)?);
        }
        let client = builder.build()?;

        Ok(Self {
            base_url,
            push_channel,
            pull_channel,
            client,
        })
    }

    pub async fn push_bytes(&self, payload: &[u8]) -> Result<()> {
        let url = format!("{}/v1/push/{}", self.base_url, self.push_channel);
        let resp = self
            .client
            .post(url)
            .header("content-type", "application/octet-stream")
            .body(payload.to_vec())
            .send()
            .await?;
        let status = resp.status();
        if !status.is_success() {
            return Err(anyhow!(format!("RELAY_HTTP_STATUS_{}", status.as_u16())));
        }
        Ok(())
    }

    pub async fn pull_bytes(&self) -> Result<Option<Vec<u8>>> {
        let url = format!("{}/v1/pull/{}", self.base_url, self.pull_channel);
        let resp = self.client.get(url).send().await?;
        let status = resp.status();
        if status.as_u16() == 204 {
            return Ok(None);
        }
        if !status.is_success() {
            return Err(anyhow!(format!("RELAY_HTTP_STATUS_{}", status.as_u16())));
        }
        let bytes = resp.bytes().await?;
        Ok(Some(bytes.to_vec()))
    }
}

fn derive_channels(base: &str, role: RelayRole) -> (String, String) {
    let base = base.trim();
    let a2b = format!("{}--a2b", base);
    let b2a = format!("{}--b2a", base);
    match role {
        RelayRole::A => (a2b, b2a),
        RelayRole::B => (b2a, a2b),
    }
}

fn enforce_remote_opt_in(base_url: &str) -> Result<()> {
    let is_local =
        base_url.starts_with("http://127.0.0.1") || base_url.starts_with("http://localhost");
    if is_local {
        return Ok(());
    }
    let allow = std::env::var("QSL_ALLOW_REMOTE").unwrap_or_default();
    if allow != "1" {
        return Err(anyhow!(format!(
            "REMOTE_OPT_IN_REQUIRED; {REASON_REMOTE_OPT_IN}"
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remote_opt_in_guard_rejects_without_env() {
        std::env::remove_var("QSL_ALLOW_REMOTE");
        let err = RelayClient::new("http://example.com:8080", "demo", RelayRole::A, None)
            .err()
            .unwrap();
        let msg = format!("{err}");
        assert!(msg.contains("REMOTE_OPT_IN_REQUIRED"));
        assert!(msg.contains(REASON_REMOTE_OPT_IN));
    }

    #[test]
    fn channel_derivation_is_deterministic() {
        let (push_a, pull_a) = derive_channels("demo", RelayRole::A);
        let (push_b, pull_b) = derive_channels("demo", RelayRole::B);
        assert_eq!(push_a, "demo--a2b");
        assert_eq!(pull_a, "demo--b2a");
        assert_eq!(push_b, "demo--b2a");
        assert_eq!(pull_b, "demo--a2b");
    }
}
