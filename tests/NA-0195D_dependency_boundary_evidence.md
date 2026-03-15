# NA-0195D Dependency Boundary Evidence

Date: 2026-03-15

## Security-sensitive ownership map

| Surface | Before | After | Status |
| --- | --- | --- | --- |
| `qsc` supported runtime | Manifest directly named `pqcrypto-kyber`, `pqcrypto-dilithium`, and `pqcrypto-traits`; source directly called provider keypair/length APIs | Manifest depends on `quantumshield_refimpl` only for PQ/provider behavior; source calls boundary-owned runtime PQ helpers from `quantumshield_refimpl::crypto::stdcrypto` | Supported runtime no longer owns direct PQ/provider churn. |
| `qsl-tui` supported runtime | Already depended on `quantumshield_refimpl` only | Still depends on `quantumshield_refimpl` only | No direct provider ownership in supported runtime. |
| `quantumshield_refimpl` shared internal runtime boundary | Owned `pqkem` default path and optional `pqcrypto` feature path | Still owns the supported runtime PQ/provider surface and now also owns the runtime keypair/length helper boundary | Sole supported runtime PQ/provider boundary owner. |
| `refimpl_actor` actor/tooling | Direct `ml-kem` / `ml-dsa` ownership | Unchanged | Tooling-only crypto surface, not supported runtime. |
| `qshield-cli` demo relay | Separate `ureq` + `tiny_http` stack | Unchanged | Demo-only exception, not canonical supported client. |

## Commodity vs security-sensitive crates

| Class | Crates / chains | Decision |
| --- | --- | --- |
| Commodity | `reqwest` + `rustls`, `clap`, `serde`, `serde_json`, `tracing`, `tokio` where needed | Canonical stacks remain as recorded in D-0302. |
| Security-sensitive supported runtime | `quantumshield_refimpl` PQ boundary, `qsc` keychain feature, TUI rendering surface | Explicitly owned and reviewed as supported surfaces. |
| Security-sensitive tooling-only | `refimpl_actor -> ml-dsa`, `refimpl_actor -> ml-kem` | Kept outside supported runtime risk accounting. |

## Supported / optional / tooling-only surface classification

| Surface | Classification | Evidence |
| --- | --- | --- |
| `qsc` keychain | Supported optional, non-default | `qsl/qsl-client/qsc/Cargo.toml` feature gate and `qsl/qsl-client/qsc/src/vault/mod.rs` |
| `qsc` / `qsl-tui` TUI | Supported deliberate surface | `qsl/qsl-client/qsc/src/main.rs`, `apps/qsl-tui/src/main.rs` |
| `qshield-cli` | Demo-only temporary exception | `apps/qshield-cli/README.md` |
| `refimpl_actor` | Tooling-only crypto surface | `tools/actors/refimpl_actor_rs/Cargo.toml` |
| `quantumshield_refimpl` | Shared internal runtime boundary | `tools/refimpl/quantumshield_refimpl/Cargo.toml` and `src/crypto/stdcrypto.rs` |

## Remaining security-sensitive chains after consolidation

| Advisory | Chain | Surface | Why retained |
| --- | --- | --- | --- |
| `RUSTSEC-2024-0380` | `quantumshield_refimpl -> pqcrypto-dilithium -> qsc/qsl-tui` | Supported runtime boundary | Requires provider replacement inside the owned boundary; not safe in NA-0195D. |
| `RUSTSEC-2024-0381` | `quantumshield_refimpl -> pqcrypto-kyber -> qsc/qsl-tui` | Supported runtime boundary | Requires provider replacement inside the owned boundary; not safe in NA-0195D. |
| `RUSTSEC-2025-0144` | `refimpl_actor -> ml-dsa` | Tooling-only | Timing-fix upgrade is not a drop-in API change and is outside supported runtime scope. |

## Evaluation rule for future dependency PRs

1. Supported app/runtime crates must not directly name third-party PQ/provider crates.
2. Supported runtime PQ/provider churn belongs in `quantumshield_refimpl` until a smaller, explicitly approved internal boundary is introduced.
3. Tooling-only crypto residuals must stay explicitly separated from supported runtime risk accounting.
