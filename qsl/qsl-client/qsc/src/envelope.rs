// QSC privacy envelopes (NA-0066).
//
// Deterministic and bounded scheduling + bundling rules.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvelopeError {
    TickLimitExceeded,
    TickIntervalInvalid,
    PayloadTooLarge,
    BundleEmpty,
    BucketUnavailable,
}

impl EnvelopeError {
    pub fn code(self) -> &'static str {
        match self {
            EnvelopeError::TickLimitExceeded => "envelope_tick_limit_exceeded",
            EnvelopeError::TickIntervalInvalid => "envelope_tick_interval_invalid",
            EnvelopeError::PayloadTooLarge => "envelope_payload_too_large",
            EnvelopeError::BundleEmpty => "envelope_bundle_empty",
            EnvelopeError::BucketUnavailable => "envelope_bucket_unavailable",
        }
    }
}

pub const BUCKET_SIZES: [usize; 7] = [64, 128, 256, 512, 1024, 2048, 4096];
pub const MAX_TICKS_DEFAULT: usize = 64;
pub const MAX_BUNDLE_SIZE_DEFAULT: usize = 4096;
pub const MAX_PAYLOAD_COUNT_DEFAULT: usize = 16;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnvelopeBundle {
    pub payload_lens: Vec<usize>,
    pub total_len: usize,
    pub bucket_len: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnvelopePlan {
    pub ticks: Vec<u64>,
    pub bundle: EnvelopeBundle,
}

pub fn tick_schedule(
    tick_count: usize,
    interval_ms: u64,
    max_ticks: usize,
) -> Result<Vec<u64>, EnvelopeError> {
    if tick_count > max_ticks {
        return Err(EnvelopeError::TickLimitExceeded);
    }
    if interval_ms == 0 {
        return Err(EnvelopeError::TickIntervalInvalid);
    }
    let mut out = Vec::with_capacity(tick_count);
    for i in 0..tick_count {
        out.push((i as u64) * interval_ms);
    }
    Ok(out)
}

pub fn bucket_for_len(len: usize, max_bucket: usize) -> Result<usize, EnvelopeError> {
    for b in BUCKET_SIZES {
        if b > max_bucket {
            break;
        }
        if len <= b {
            return Ok(b);
        }
    }
    Err(EnvelopeError::BucketUnavailable)
}

pub fn pack_bundle(
    payload_lens: &[usize],
    max_bundle: usize,
    max_count: usize,
) -> Result<EnvelopeBundle, EnvelopeError> {
    if max_bundle == 0 || max_count == 0 {
        return Err(EnvelopeError::BundleEmpty);
    }

    let mut total = 0usize;
    let mut out = Vec::new();
    for &len in payload_lens {
        if len > max_bundle {
            return Err(EnvelopeError::PayloadTooLarge);
        }
        if out.len() >= max_count {
            break;
        }
        if total + len > max_bundle {
            break;
        }
        out.push(len);
        total += len;
    }

    if out.is_empty() {
        return Err(EnvelopeError::BundleEmpty);
    }

    let bucket_len = bucket_for_len(total, max_bundle)?;
    Ok(EnvelopeBundle {
        payload_lens: out,
        total_len: total,
        bucket_len,
    })
}

#[allow(dead_code)]
pub fn plan_for_payload_len(
    payload_len: usize,
    tick_count: usize,
    interval_ms: u64,
    max_ticks: usize,
    max_bundle: usize,
    max_count: usize,
) -> Result<EnvelopePlan, EnvelopeError> {
    let ticks = tick_schedule(tick_count, interval_ms, max_ticks)?;
    let bundle = pack_bundle(&[payload_len], max_bundle, max_count)?;
    Ok(EnvelopePlan { ticks, bundle })
}

pub fn plan_ack(
    small_payload_len: usize,
    tick_count: usize,
    interval_ms: u64,
    max_ticks: usize,
    max_bundle: usize,
    max_count: usize,
) -> Result<EnvelopePlan, EnvelopeError> {
    if tick_count == 0 {
        return Err(EnvelopeError::TickLimitExceeded);
    }
    let ticks = tick_schedule(tick_count, interval_ms, max_ticks)?;
    let mut bundle = pack_bundle(&[0], max_bundle, max_count)?;
    let small_bucket = bucket_for_len(small_payload_len, max_bundle)?;
    bundle.bucket_len = small_bucket;
    Ok(EnvelopePlan { ticks, bundle })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tick_schedule_stable_and_bounded() {
        let ticks = tick_schedule(3, 100, 4).expect("tick schedule ok");
        assert_eq!(ticks, vec![0, 100, 200]);

        let err = tick_schedule(5, 100, 4).unwrap_err();
        assert_eq!(err, EnvelopeError::TickLimitExceeded);

        let err = tick_schedule(1, 0, 4).unwrap_err();
        assert_eq!(err, EnvelopeError::TickIntervalInvalid);
    }

    #[test]
    fn bucket_sizes_match_spec() {
        assert_eq!(bucket_for_len(1, 4096).unwrap(), 64);
        assert_eq!(bucket_for_len(64, 4096).unwrap(), 64);
        assert_eq!(bucket_for_len(65, 4096).unwrap(), 128);
        assert_eq!(bucket_for_len(2000, 4096).unwrap(), 2048);
        assert_eq!(bucket_for_len(4096, 4096).unwrap(), 4096);
        assert_eq!(
            bucket_for_len(4097, 4096).unwrap_err(),
            EnvelopeError::BucketUnavailable
        );
    }

    #[test]
    fn plan_for_payload_len_smoke() {
        let plan = plan_for_payload_len(80, 3, 100, 4, 256, 3).expect("plan ok");
        assert_eq!(plan.ticks.len(), 3);
        assert_eq!(plan.bundle.payload_lens, vec![80]);
    }

    #[test]
    fn bundle_packing_rules() {
        let payloads = vec![10, 20, 50, 80];
        let bundle = pack_bundle(&payloads, 128, 3).expect("bundle ok");
        assert_eq!(bundle.payload_lens, vec![10, 20, 50]);
        assert_eq!(bundle.total_len, 80);
        assert_eq!(bundle.bucket_len, 128);

        let err = pack_bundle(&[150], 100, 3).unwrap_err();
        assert_eq!(err, EnvelopeError::PayloadTooLarge);

        let err = pack_bundle(&[10], 128, 0).unwrap_err();
        assert_eq!(err, EnvelopeError::BundleEmpty);
    }
}
