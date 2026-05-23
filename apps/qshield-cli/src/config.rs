use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

pub const CONFIG_FILE_NAME: &str = "config.json";

use crate::fsutil::write_secure_file;

pub const DEMO_PADDING_BUCKET_EXPANSION_POLICY: &str = "qshield_demo_padding_bucket_expansion_v1";
pub const DEMO_PADDING_BUCKET_EXPANSION_BUCKETS: [u32; 12] = [
    256, 512, 768, 1024, 1536, 2048, 3072, 4096, 5120, 6144, 7168, 8192,
];
pub const DEMO_PADDING_MAX_PADDED_PAYLOAD_BYTES: u32 = 8192;
pub const DEMO_PADDING_MAX_OVERHEAD_BYTES: u32 = 1023;
pub const DEMO_PADDING_BUCKETS_ENV: &str = "QSHIELD_DEMO_PADDING_BUCKETS";
pub const DEMO_ATTACHMENT_SIZE_CLASS_POLICY: &str = "qshield_demo_attachment_size_class_v1";
pub const DEMO_ATTACHMENT_SIZE_CLASS_TABLE: [u32; 12] = [
    256, 512, 768, 1024, 1536, 2048, 3072, 4096, 5120, 6144, 7168, 8192,
];
pub const DEMO_ATTACHMENT_MAX_PADDED_OBJECT_BYTES: u32 = 8192;
pub const DEMO_ATTACHMENT_MAX_OVERHEAD_BYTES: u32 = 1023;
pub const DEMO_ATTACHMENT_SIZE_CLASSES_ENV: &str = "QSHIELD_DEMO_ATTACHMENT_SIZE_CLASSES";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub relay_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relay_token: Option<String>,
    #[serde(default)]
    pub padding_enabled: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub padding_buckets: Option<Vec<u32>>,
}

impl Config {
    pub fn new(
        relay_url: String,
        relay_token: Option<String>,
        padding_enabled: bool,
        padding_buckets: Option<Vec<u32>>,
    ) -> Self {
        Self {
            relay_url,
            relay_token,
            padding_enabled,
            padding_buckets,
        }
    }
}

pub fn resolve_relay_token(cfg: &Config) -> Result<String, String> {
    if let Ok(token) = std::env::var("QSHIELD_RELAY_TOKEN") {
        if !token.trim().is_empty() {
            return Ok(token);
        }
    }
    cfg.relay_token
        .clone()
        .filter(|v| !v.trim().is_empty())
        .ok_or_else(|| {
            "relay token missing; set QSHIELD_RELAY_TOKEN or run qshield init --relay-token <token>"
                .to_string()
        })
}

pub fn write_config(path: &Path, cfg: &Config) -> Result<(), String> {
    let data = serde_json::to_vec_pretty(cfg).map_err(|e| format!("serialize config: {e}"))?;
    write_secure_file(path, &data).map_err(|e| format!("write config: {e}"))?;
    Ok(())
}

pub fn read_config(path: &Path) -> Result<Config, String> {
    let data = fs::read(path).map_err(|e| format!("read config: {e}"))?;
    serde_json::from_slice(&data).map_err(|e| format!("parse config: {e}"))
}

pub fn parse_padding_buckets_csv(raw: &str) -> Result<Vec<u32>, String> {
    let mut buckets = Vec::new();
    for part in raw.split(',') {
        let value = part.trim();
        if value.is_empty() {
            return Err("padding bucket empty".to_string());
        }
        let n: u64 = value
            .parse()
            .map_err(|_| "invalid padding bucket".to_string())?;
        if n == 0 {
            return Err("padding bucket must be > 0".to_string());
        }
        if n > DEMO_PADDING_MAX_PADDED_PAYLOAD_BYTES as u64 {
            return Err("padding bucket exceeds demo maximum".to_string());
        }
        buckets.push(n as u32);
    }
    validate_demo_padding_buckets(&buckets)?;
    Ok(buckets)
}

pub fn demo_padding_buckets_from_env() -> Result<Option<Vec<u32>>, String> {
    let Ok(value) = std::env::var(DEMO_PADDING_BUCKETS_ENV) else {
        return Ok(None);
    };
    let value = value.trim();
    if value.is_empty() {
        return Ok(None);
    }
    if value.eq_ignore_ascii_case("expanded") || value == DEMO_PADDING_BUCKET_EXPANSION_POLICY {
        let buckets = DEMO_PADDING_BUCKET_EXPANSION_BUCKETS.to_vec();
        debug_assert_eq!(
            demo_padding_max_overhead(&buckets),
            DEMO_PADDING_MAX_OVERHEAD_BYTES
        );
        return Ok(Some(buckets));
    }
    Err("invalid demo padding bucket policy".to_string())
}

pub fn validate_demo_padding_buckets(buckets: &[u32]) -> Result<(), String> {
    if buckets.is_empty() {
        return Err("padding buckets empty".to_string());
    }
    let mut prev: Option<u32> = None;
    for bucket in buckets {
        if *bucket == 0 {
            return Err("padding bucket must be > 0".to_string());
        }
        if *bucket > DEMO_PADDING_MAX_PADDED_PAYLOAD_BYTES {
            return Err("padding bucket exceeds demo maximum".to_string());
        }
        if let Some(prev) = prev {
            if *bucket == prev {
                return Err("padding bucket duplicate".to_string());
            }
            if *bucket < prev {
                return Err("padding buckets must be sorted".to_string());
            }
        }
        prev = Some(*bucket);
    }
    Ok(())
}

fn demo_padding_max_overhead(buckets: &[u32]) -> u32 {
    let mut previous = 0;
    let mut max_overhead = 0;
    for bucket in buckets {
        max_overhead = max_overhead.max(bucket.saturating_sub(previous).saturating_sub(1));
        previous = *bucket;
    }
    max_overhead
}

pub fn demo_attachment_size_classes_from_env() -> Result<Option<Vec<u32>>, String> {
    let Ok(value) = std::env::var(DEMO_ATTACHMENT_SIZE_CLASSES_ENV) else {
        return Ok(None);
    };
    let value = value.trim();
    if value.is_empty() {
        return Err("attachment size classes empty".to_string());
    }
    if value.eq_ignore_ascii_case("expanded") || value == DEMO_ATTACHMENT_SIZE_CLASS_POLICY {
        let classes = DEMO_ATTACHMENT_SIZE_CLASS_TABLE.to_vec();
        debug_assert_eq!(
            demo_attachment_size_class_max_overhead(&classes),
            DEMO_ATTACHMENT_MAX_OVERHEAD_BYTES
        );
        return Ok(Some(classes));
    }
    parse_attachment_size_classes_csv(value).map(Some)
}

pub fn parse_attachment_size_classes_csv(raw: &str) -> Result<Vec<u32>, String> {
    let mut classes = Vec::new();
    for part in raw.split(',') {
        let value = part.trim();
        if value.is_empty() {
            return Err("attachment size class empty".to_string());
        }
        let n: i128 = value
            .parse()
            .map_err(|_| "invalid attachment size class".to_string())?;
        if n <= 0 {
            return Err("attachment size class must be > 0".to_string());
        }
        if n > DEMO_ATTACHMENT_MAX_PADDED_OBJECT_BYTES as i128 {
            return Err("attachment size class exceeds demo maximum".to_string());
        }
        classes.push(n as u32);
    }
    validate_demo_attachment_size_classes(&classes)?;
    Ok(classes)
}

pub fn validate_demo_attachment_size_classes(classes: &[u32]) -> Result<(), String> {
    if classes.is_empty() {
        return Err("attachment size classes empty".to_string());
    }
    let mut prev: Option<u32> = None;
    for class in classes {
        if *class == 0 {
            return Err("attachment size class must be > 0".to_string());
        }
        if *class > DEMO_ATTACHMENT_MAX_PADDED_OBJECT_BYTES {
            return Err("attachment size class exceeds demo maximum".to_string());
        }
        if let Some(prev) = prev {
            if *class == prev {
                return Err("attachment size class duplicate".to_string());
            }
            if *class < prev {
                return Err("attachment size classes must be sorted".to_string());
            }
        }
        prev = Some(*class);
    }
    Ok(())
}

fn demo_attachment_size_class_max_overhead(classes: &[u32]) -> u32 {
    let mut previous = 0;
    let mut max_overhead = 0;
    for class in classes {
        max_overhead = max_overhead.max(class.saturating_sub(previous).saturating_sub(1));
        previous = *class;
    }
    max_overhead
}
