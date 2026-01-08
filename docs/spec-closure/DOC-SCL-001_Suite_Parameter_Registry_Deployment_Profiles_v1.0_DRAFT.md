# QuantumShield — Suite & Parameter Registry + Deployment Profiles

Doc ID: DOC-SCL-001  
Version: v1.0  
Status: DRAFT  
Last Updated: 2025-12-26  
Audience: Internal (engineering/ops), External (implementers, optional publication)  
Normative: **YES** (configuration registry & profile semantics; MUST NOT conflict with QSP/QSE)  
Supersedes: Phase3 P3-12 v1.0 (Parameter Registry & Deployment Profiles)
**Artifact ID:** P3-12  
**Category:** Supporting (atomic)  
**Phase:** 3 (Phase 2 frozen; canonical specs frozen)  
**Canonical refs (alignment only):** QSP 4.3.2 (REVIEWED FULL), QSE 1.8.2 (REVIEWED FULL)  
**Version:** 1.0  
**Date:** 2025-12-20  
**Timezone:** America/Chicago  

## 0. Purpose and scope
> Phase 4 alignment note: This registry is the control-plane companion to the canonical specs (DOC-CAN-001/QSP and DOC-CAN-002/QSE).
> It is updated to reflect Phase 4 interop and durability gates (ci-4a..ci-4d-dur). Configuration MAY tighten security and resource bounds,
> but MUST NOT weaken canonical requirements.
> Current Phase 4 evidence baseline: durability gates include IT-DUR-001..IT-DUR-005 (ci-4d-dur). Test hooks (e.g., QSL_TEST_HOOKS) are CI-only and MUST NOT be enabled in production.
This document defines a single, deployable **parameter registry** and a set of **deployment profiles** for Phase 3 QuantumShield systems.

It is intended to be the authoritative configuration reference for:
- **Clients/SDKs** (Swift/Kotlin/Rust/TypeScript),
- **RSF** (Relay / Store-and-Forward),
- **PDS** (Prekey Directory Service),
- **KTL** (Key Transparency Log),
- control-plane AuthN/AuthZ and observability.

This document is **supporting** and **atomic**:
- it does not require other Phase 3 artifacts to be usable,
- it does not change QSP/QSE wire formats or cryptographic rules,
- it provides concrete defaults and allowed ranges, and
- it includes profile overlays (dev/staging/prod/high-security).

Where a value is defined canonically by QSP/QSE, this document records it and treats it as a hard cap or hard rule.

## 1. Configuration model
### 1.1 Namespacing
All parameters are namespaced:

- `qse.*` — envelope and transport framing policy (QSE-aligned)
- `qsp.*` — protocol-level operational bounds and persistence policy (QSP-aligned)
- `rsf.*` — relay/store-and-forward service policies
- `pds.*` — directory and OPK service policies
- `ktl.*` — transparency log policies
- `auth.*` — auth and authorization policies
- `log.*` — logging/telemetry redlines and retention
- `build.*` — release engineering gates and flags

### 1.2 Representation
Deployments SHOULD store and distribute config as:
- a versioned, signed JSON or YAML document,
- with environment overlays (profile + site overrides),
- with explicit feature flags.

All byte sizes are integers in bytes. All time values are seconds unless otherwise specified.

### 1.3 Precedence
Effective configuration is computed as:

`BASELINE (P3 defaults) + PROFILE overlay + ENV/site overrides`

If multiple overrides set the same key, the last applied layer wins.

## 2. Hard canonical bounds and invariants
The following values are **canonical** (QSP/QSE). Deployments MAY choose smaller values but MUST NOT exceed these without a canonical version bump.

### 2.1 QSE canonical DoS bounds (QSE 1.8.2)
- `qse.max_route_token_len = 512`
- `qse.max_payload_len = 1_048_576` (1 MiB)
- `qse.max_pad_len = 1_048_576` (1 MiB)
- `qse.max_envelope_len = 2_097_152` (2 MiB)

### 2.2 QSE timestamp policy defaults (QSE 1.8.2 recommendations)
- `qse.bucket_width_seconds` (RECOMMENDED): `900` (15 minutes)
- `qse.bucket_skew_tolerance_buckets` (RECOMMENDED): `2` (±2 buckets)
- `qse.allow_zero_timestamp_bucket` (deployment policy; Phase 3 baseline uses `true`)

### 2.3 QSP canonical bounds (QSP 4.3.2)
- `qsp.max_skip = 1000`
- `qsp.max_mkskipped = 2000`
- `qsp.max_header_attempts = 100`
- `qsp.max_hkskipped = 4`
- `qsp.mkskipped_ttl_seconds = 604800` (7 days)
- `qsp.hkskipped_ttl_seconds = 604800` (7 days)
- `qsp.max_mkskipped_scan = 50`

### 2.4 Interop-bound payload sizing (QSP↔QSE)
When transporting QSP messages inside QSE:
- deployments MUST ensure the **largest encoded QSP message** fits within `qse.max_payload_len`.
- recommended default: `qsp.max_body_ct_len = 1_000_000` bytes (leaves room for Prefix + HeaderCT within the 1 MiB QSE payload cap).

## 3. Baseline parameter registry (baseline defaults)
Unless overridden by a profile, the following values constitute the Phase 3 baseline.

### 3.1 QSE policy (`qse.*`)
| Key | Default | Allowed range | Notes |
|---|---:|---:|---|
| qse.max_route_token_len | 512 | 16..512 | Canonical hard cap at 512. |
| qse.max_payload_len | 1048576 | 4096..1048576 | Canonical hard cap at 1 MiB. |
| qse.max_pad_len | 1048576 | 0..1048576 | Canonical hard cap at 1 MiB. |
| qse.max_envelope_len | 2097152 | 4096..2097152 | Canonical hard cap at 2 MiB. |
| qse.min_envelope_bytes | 1024 | 256..qse.max_envelope_len | Padding target for length hiding. |
| qse.padding_bucket_bytes | [1024, 2048, 4096] | implementation-defined | Optional bucketing; keep small set. |
| qse.allow_zero_timestamp_bucket | true | true/false | Phase 3 baseline accepts 0. |
| qse.bucket_width_seconds | 900 | 60..3600 | QSE recommends 900. |
| qse.bucket_skew_tolerance_buckets | 2 | 0..10 | QSE recommends ±2. |

### 3.2 QSP operational bounds (`qsp.*`)
| Key | Default | Allowed range | Notes |
|---|---:|---:|---|
| qsp.max_skip | 1000 | 0..1000 | Canonical hard cap at 1000. |
| qsp.max_mkskipped | 2000 | 0..2000 | Canonical hard cap at 2000. |
| qsp.max_mkskipped_scan | 50 | 1..50 | Canonical hard cap at 50. |
| qsp.mkskipped_ttl_seconds | 604800 | 0..604800 | Canonical: 7 days. |
| qsp.max_header_attempts | 100 | 1..100 | Canonical hard cap at 100. |
| qsp.max_hkskipped | 4 | 0..4 | Canonical hard cap at 4. |
| qsp.hkskipped_ttl_seconds | 604800 | 0..604800 | Canonical: 7 days. |
| qsp.max_body_ct_len | 1000000 | 16384..1000000 | Must keep encoded message <= QSE payload cap. |
| qsp.fail_closed_on_parse | true | true/false | MUST remain true for conformant deployments. |
| qsp.fail_closed_on_kt | true | true/false | Authenticated mode: MUST remain true. |
| qsp.persist_atomic_commit | true | true/false | Copy-then-commit required. |
| qsp.purge_on_invariant_fail | true | true/false | Quarantine/discard session on invariant violation. |

### 3.3 RSF service controls (`rsf.*`)
| Key | Default | Allowed range | Notes |
|---|---:|---:|---|
| rsf.envelope_ttl_seconds | 604800 | 3600..1209600 | 7 days default. |
| rsf.fetch_default_items | 20 | 1..200 | Must remain <= fetch_max_items. |
| rsf.fetch_max_items | 200 | 1..1000 | Cap to protect memory/latency. |
| rsf.fetch_default_bytes | 262144 | 65536..1048576 | 256 KiB default. |
| rsf.fetch_max_bytes | 1048576 | 65536..4194304 | Keep modest to reduce amplification. |
| rsf.long_poll_max_seconds | 60 | 0..120 | 60s default. |
| rsf.visibility_timeout_seconds | 0 | 0..600 | 0 disables leasing; 30–120 recommended if enabled. |
| rsf.inbox_max_items | 10000 | 100..100000 | Per-inbox cap. |
| rsf.inbox_max_bytes | 52428800 | 1048576..524288000 | 50 MiB default. |
| rsf.dedupe_hint_window_seconds | 600 | 0..3600 | Best-effort only; non-security. |
| rsf.route_token_rotation_overlap_seconds | 86400 | 0..604800 | 24h overlap default. |
| rsf.enforce_nonzero_qse_flags | true | true/false | Reject QSE 1.8.x non-zero flags. |
| rsf.rewrite_timestamp_bucket | true | true/false | Service-edge baseline. |
| rsf.reject_on_noncanonical_qse | true | true/false | MUST remain true. |

### 3.4 PDS controls (`pds.*`)
| Key | Default | Allowed range | Notes |
|---|---:|---:|---|
| pds.bundle_max_devices_per_user | 16 | 1..64 | Multi-device support. |
| pds.bundle_validity_max_seconds | 15552000 | 86400..31536000 | 180 days default max; policy. |
| pds.opk_upload_batch_max | 500 | 1..5000 | Batch API cap. |
| pds.opk_pool_cap_per_device | 10000 | 100..50000 | Per pool (DH/PQ). |
| pds.opk_low_watermark | 50 | 0..1000 | Alert threshold. |
| pds.opk_policy_default | "preferred" | preferred/required/none | Align with P3-06. |
| pds.idempotency_window_seconds | 86400 | 60..604800 | 24h default. |
| pds.enumeration_rate_limit_per_min | 60 | 1..600 | Per principal per target bucket. |

### 3.5 KTL controls (`ktl.*`)
| Key | Default | Allowed range | Notes |
|---|---:|---:|---|
| ktl.sth_publish_cadence_seconds | 300 | 30..3600 | 5 minutes default. |
| ktl.proof_cache_ttl_seconds | 300 | 0..3600 | Cache proofs for latency. |
| ktl.read_rate_limit_per_min | 600 | 10..10000 | Per principal or per IP bucket. |
| ktl.append_idempotency_window_seconds | 86400 | 60..604800 | If append endpoint exists. |

### 3.6 Auth and tokens (`auth.*`)
| Key | Default | Allowed range | Notes |
|---|---:|---:|---|
| auth.token_ttl_seconds | 3600 | 300..86400 | 1 hour default. |
| auth.refresh_ttl_seconds | 2592000 | 3600..7776000 | 30 days default (if refresh tokens used). |
| auth.mtls_internal_required | true | true/false | Strongly recommended for service-to-service. |
| auth.rotation_overlap_seconds | 604800 | 0..2592000 | Key rotation overlap. |

### 3.7 Logging, telemetry, retention (`log.*`)
| Key | Default | Allowed range | Notes |
|---|---:|---:|---|
| log.service_retention_seconds | 2592000 | 86400..7776000 | 30 days default. |
| log.security_retention_seconds | 15552000 | 604800..31536000 | 180 days default (sanitized). |
| log.metrics_retention_seconds | 7776000 | 604800..15552000 | 90 days default. |
| log.allow_raw_route_token | false | false only | MUST remain false in production. |
| log.allow_raw_envelopes | false | false only | MUST remain false in production. |
| log.identifier_hash_salt_rotation_seconds | 15552000 | 2592000..31536000 | 180 days default. |
| log.reason_codes_enabled | true | true/false | Use standardized codes. |

### 3.8 Build and release gates (`build.*`)
| Key | Default | Allowed range | Notes |
|---|---:|---:|---|
| build.require_vectors_pass | true | true/false | Gate on DOC-TST-001 vector pack. |
| build.require_fuzz_smoke | true | true/false | Parser fuzz smoke in CI. |
| build.canary_percent | 5 | 0..100 | 5% default for production rollouts. |
| build.rollback_on_kt_fail_spike | true | true/false | Rollback trigger (key transparency / verification failures). |
| build.required_ci_gates | ["ci-4a","ci-4b","ci-4c","ci-4d","ci-4d-dur"] | JSON array | PR merge MUST be blocked until all gates pass. |
| build.fail_closed_on_missing_evidence | true | true/false | MUST be true: missing artifacts/evidence is a failure. |

### 3.9 Durability and rollback/replay controls (`dur.*`)
These parameters operationalize the durability requirements captured in **DOC-SCL-004** (State Persistence & Crash Safety)
and enforced by Phase 4D durability gates. They MUST only **tighten** behavior relative to canonical QSP/QSE rules;
they MUST NOT relax security-critical bounds.

| Key | Default | Allowed range | Notes |
|---|---:|---:|---|
| dur.enable_rollback_detection | true | true/false | MUST be true for `staging`, `prod`, and `high_security`. |
| dur.rollback_epoch_persist | true | true only | The rollback epoch/generation MUST be persisted atomically with ratchet state. |
| dur.reject_on_epoch_regression | true | true only | If persisted epoch regresses on restart, implementation MUST fail-closed and alert. |
| dur.replay_cache_max_entries | 65536 | 1024..1048576 | Bounds memory use; MUST be sized for peak concurrent sessions. |
| dur.replay_entry_ttl_seconds | 604800 | 3600..2592000 | Replay window retention (default 7 days). MUST be ≥ maximum expected offline window. |
| dur.max_out_of_order_messages | 64 | 0..1024 | Upper bound on tolerated reordering; SHOULD be small to reduce replay surface. |
| dur.state_checkpoint_interval_messages | 1 | 1..1024 | Persist ratchet state at least every N accepted messages. `1` is strongest. |
| dur.crash_recovery_fail_closed | true | true only | On uncertain state after crash (partial commit), MUST fail-closed. |
| dur.rekey_on_crash_recovery | true | true/false | If true, force a re-handshake / rekey after crash recovery to re-establish guarantees. |

## 4. Deployment profiles (overlays)
Profiles provide sane sets of overrides. If a profile sets a value above a canonical cap, the profile is invalid.

### 4.1 Profile: `dev`
Intended for local development and rapid iteration.
- lower TTLs, smaller caps, verbose (still sanitized) logging, easier throttles.

Overrides:
- `rsf.envelope_ttl_seconds = 86400` (1 day)
- `rsf.inbox_max_items = 2000`
- `rsf.inbox_max_bytes = 10485760` (10 MiB)
- `pds.opk_low_watermark = 10`
- `ktl.sth_publish_cadence_seconds = 60`
- `log.service_retention_seconds = 604800` (7 days)
- `build.canary_percent = 0`

### 4.2 Profile: `staging`
Close to production, but with extra diagnostics and tighter caps.
Overrides:
- `rsf.envelope_ttl_seconds = 259200` (3 days)
- `rsf.fetch_max_items = 200`
- `rsf.fetch_max_bytes = 524288` (512 KiB)
- `pds.opk_low_watermark = 25`
- `log.service_retention_seconds = 1209600` (14 days)
- `build.canary_percent = 0` (staging is all-canary)

### 4.3 Profile: `prod`
production baseline.
Overrides:
- (none beyond registry defaults)

### 4.4 Profile: `high_security`
For elevated-risk deployments (stronger metadata minimization and tighter abuse posture).
Overrides:
- `qse.min_envelope_bytes = 2048`
- `qse.padding_bucket_bytes = [2048, 4096, 8192]`
- `rsf.envelope_ttl_seconds = 259200` (3 days)
- `rsf.dedupe_hint_window_seconds = 300`
- `pds.enumeration_rate_limit_per_min = 30`
- `log.service_retention_seconds = 1209600` (14 days)
- `auth.token_ttl_seconds = 1800` (30 min)

## 5. Distribution, integrity, and rollout
### 5.1 Signed configuration bundles
Production deployments SHOULD distribute config as a signed bundle:
- `config.json` (or YAML),
- `config.sig` (signature),
- `config.pub` (pinned verifier key or key-id).

Clients and services SHOULD:
- verify signature before applying,
- require monotonic `config_version` (prevent rollback),
- apply changes only after passing local validation (canonical caps and invariants).

### 5.2 Safe rollout
For parameters affecting parsing, bounds, token rotation, or KT policy:
- roll out to canary first (default 5%),
- monitor reason-code rates (`noncanonical_qse`, `bounds_exceeded`, `kt_fail`),
- auto-rollback on failure spikes.

### 5.3 Runtime reload rules
Services MAY support runtime reload, but MUST:
- apply bounds changes safely (do not allow existing oversized messages to bypass new bounds),
- avoid log redline relaxations at runtime (never enable raw identifier logging dynamically),
- record applied config version for audit.

## 6. Validation rules (MUST)
A configuration is valid only if all of the following hold:

1. Canonical caps are respected:
   - `qse.max_*` values do not exceed QSE caps.
   - `qsp.*` bounds do not exceed QSP caps.
2. Interop sizing holds:
   - `qsp.max_body_ct_len` + protocol overhead <= `qse.max_payload_len`.
3. Redlines are enforced:
   - `log.allow_raw_route_token == false`
   - `log.allow_raw_envelopes == false`
4. Fail-closed flags remain enabled:
   - `qsp.fail_closed_on_parse == true`
   - `qsp.fail_closed_on_kt == true` (if Authenticated mode is enabled)

If any validation fails, deployments MUST fail closed (do not start; do not apply the config).

## Appendix A — Example configuration (JSON)
The following is a complete example for the `prod` profile. Values omitted from the example are assumed to take the baseline defaults defined in §3.

```json
{
  "format": "QSHIELD-CONFIG-1",
  "config_version": "p3-12-v1.0-prod",
  "profile": "prod",
  "qse": {
    "max_route_token_len": 512,
    "max_payload_len": 1048576,
    "max_pad_len": 1048576,
    "max_envelope_len": 2097152,
    "min_envelope_bytes": 1024,
    "padding_bucket_bytes": [1024, 2048, 4096],
    "allow_zero_timestamp_bucket": true,
    "bucket_width_seconds": 900,
    "bucket_skew_tolerance_buckets": 2
  },
  "qsp": {
    "max_skip": 1000,
    "max_mkskipped": 2000,
    "max_mkskipped_scan": 50,
    "mkskipped_ttl_seconds": 604800,
    "max_header_attempts": 100,
    "max_hkskipped": 4,
    "hkskipped_ttl_seconds": 604800,
    "max_body_ct_len": 1000000,
    "fail_closed_on_parse": true,
    "fail_closed_on_kt": true,
    "persist_atomic_commit": true,
    "purge_on_invariant_fail": true
  },
  "rsf": {
    "envelope_ttl_seconds": 604800,
    "fetch_default_items": 20,
    "fetch_max_items": 200,
    "fetch_default_bytes": 262144,
    "fetch_max_bytes": 1048576,
    "long_poll_max_seconds": 60,
    "visibility_timeout_seconds": 0,
    "inbox_max_items": 10000,
    "inbox_max_bytes": 52428800,
    "dedupe_hint_window_seconds": 600,
    "route_token_rotation_overlap_seconds": 86400,
    "enforce_nonzero_qse_flags": true,
    "rewrite_timestamp_bucket": true,
    "reject_on_noncanonical_qse": true
  },
  "pds": {
    "bundle_max_devices_per_user": 16,
    "bundle_validity_max_seconds": 15552000,
    "opk_upload_batch_max": 500,
    "opk_pool_cap_per_device": 10000,
    "opk_low_watermark": 50,
    "opk_policy_default": "preferred",
    "idempotency_window_seconds": 86400,
    "enumeration_rate_limit_per_min": 60
  },
  "ktl": {
    "sth_publish_cadence_seconds": 300,
    "proof_cache_ttl_seconds": 300,
    "read_rate_limit_per_min": 600,
    "append_idempotency_window_seconds": 86400
  },
  "auth": {
    "token_ttl_seconds": 3600,
    "refresh_ttl_seconds": 2592000,
    "mtls_internal_required": true,
    "rotation_overlap_seconds": 604800
  },
  "log": {
    "service_retention_seconds": 2592000,
    "security_retention_seconds": 15552000,
    "metrics_retention_seconds": 7776000,
    "allow_raw_route_token": false,
    "allow_raw_envelopes": false,
    "identifier_hash_salt_rotation_seconds": 15552000,
    "reason_codes_enabled": true
  },
  "build": {
    "require_vectors_pass": true,
    "require_fuzz_smoke": true,
    "canary_percent": 5,
    "rollback_on_kt_fail_spike": true
  }
}
```

---
**End of document.**