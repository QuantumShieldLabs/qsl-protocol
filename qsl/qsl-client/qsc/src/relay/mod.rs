use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub(crate) struct RelayConfig {
    pub(crate) seed: u64,
    pub(crate) drop_pct: u8,
    pub(crate) dup_pct: u8,
    pub(crate) reorder_window: usize,
    pub(crate) fixed_latency_ms: u64,
    pub(crate) jitter_ms: u64,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct RelayFrame {
    pub(crate) to: String,
    pub(crate) data: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct RelayResponse {
    pub(crate) action: String,
    pub(crate) delivered: bool,
}

pub(crate) struct RelayRng {
    pub(crate) state: u64,
}

impl RelayRng {
    pub(crate) fn new(seed: u64) -> Self {
        Self {
            state: seed ^ 0x9e3779b97f4a7c15,
        }
    }

    pub(crate) fn next_u64(&mut self) -> u64 {
        // xorshift64*
        let mut x = self.state;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.state = x;
        x.wrapping_mul(0x2545f4914f6cdd1d)
    }

    pub(crate) fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }
}

pub(crate) struct RelayDecision {
    pub(crate) action: &'static str,
    pub(crate) delivered: bool,
    pub(crate) delay_ms: u64,
}

pub(crate) fn relay_decide(cfg: &RelayConfig, seq: u64) -> RelayDecision {
    let mut rng = RelayRng::new(cfg.seed ^ seq);
    let roll = (rng.next_u32() % 100) as u8;
    if cfg.drop_pct > 0 && roll < cfg.drop_pct {
        return RelayDecision {
            action: "drop",
            delivered: false,
            delay_ms: 0,
        };
    }
    let roll_dup = (rng.next_u32() % 100) as u8;
    if cfg.dup_pct > 0 && roll_dup < cfg.dup_pct {
        return RelayDecision {
            action: "dup",
            delivered: false,
            delay_ms: 0,
        };
    }

    let mut delay_ms = 0;
    if cfg.fixed_latency_ms > 0 || cfg.jitter_ms > 0 {
        delay_ms = cfg.fixed_latency_ms;
        if cfg.jitter_ms > 0 {
            delay_ms = delay_ms.saturating_add(rng.next_u64() % (cfg.jitter_ms + 1));
        }
    }

    if cfg.reorder_window > 1 && (seq % (cfg.reorder_window as u64)) == 1 {
        return RelayDecision {
            action: "reorder",
            delivered: true,
            delay_ms,
        };
    }
    if delay_ms > 0 {
        return RelayDecision {
            action: "delay",
            delivered: true,
            delay_ms,
        };
    }
    RelayDecision {
        action: "deliver",
        delivered: true,
        delay_ms: 0,
    }
}

pub(crate) struct SendExecuteArgs {
    pub(crate) transport: Option<crate::SendTransport>,
    pub(crate) relay: Option<String>,
    pub(crate) to: Option<String>,
    pub(crate) file: Option<std::path::PathBuf>,
    pub(crate) pad_to: Option<usize>,
    pub(crate) pad_bucket: Option<crate::MetaPadBucket>,
    pub(crate) bucket_max: Option<usize>,
    pub(crate) meta_seed: Option<u64>,
    pub(crate) receipt: Option<crate::ReceiptKind>,
}
