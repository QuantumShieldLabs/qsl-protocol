# NA-0195C Dependency Architecture Evidence

Date: 2026-03-14

## Canonical commodity stacks

| Job | Canonical stack | Current evidence | Policy |
| --- | --- | --- | --- |
| HTTP client | `reqwest` + `rustls` | `qsc/Cargo.toml`, `apps/qsl-tui/Cargo.toml` | Canonical for supported qsl-protocol clients. |
| CLI parsing | `clap` | `qsc/Cargo.toml`, `apps/qsl-tui/Cargo.toml`, `apps/qshield-cli/Cargo.toml`, `tools/actors/refimpl_actor_rs/Cargo.toml` | Canonical workspace CLI surface. |
| Serialization | `serde` + `serde_json` | same manifests plus `tools/refimpl/quantumshield_refimpl/Cargo.toml` | Canonical structured data surface. |
| Logging | `tracing` + `tracing-subscriber` | `apps/qsl-tui/Cargo.toml` | Canonical structured logging surface; do not add parallel logging stacks without explicit decision. |
| Async runtime | `tokio` only where needed | `apps/qsl-tui/Cargo.toml` | Runtime policy is opt-in, not workspace-wide default. |

## Supported and optional surfaces

| Surface | Status | Evidence | Decision |
| --- | --- | --- | --- |
| `qsc` keychain vault | Supported optional surface, not default-enabled | `qsl/qsl-client/qsc/Cargo.toml` (`default = []`, `keychain = ["dep:keyring"]`); `qsl/qsl-client/qsc/src/vault/mod.rs` | Keep supported, but isolate behind the feature gate and current keyring line. |
| `qsc` / `qsl-tui` TUI | Supported deliberate product surface | `qsl/qsl-client/qsc/src/main.rs`; `apps/qsl-tui/src/main.rs`; both manifests depend on `ratatui` | Keep supported and on the current maintained line; this is not disposable dev-only debt. |
| `qshield-cli` local relay demo | Auxiliary demo surface, not canonical product client | `apps/qshield-cli/README.md`; `apps/qshield-cli/src/relay_client.rs`; `apps/qshield-cli/src/commands/relay.rs` | Temporary exception: `ureq` + `tiny_http` remain until a dedicated demo-surface consolidation item is justified. |

## Security-sensitive chains and owners

| Chain | Surface | Owner / rationale | Current status |
| --- | --- | --- | --- |
| `qsc` vault -> `keyring` | Supported optional secret-storage surface | qsc vault boundary owns OS keychain integration; keep optional and explicit | Upgraded to `keyring` `3.6.3`; old async secret-service chain removed from current default build. Feature-enabled Linux builds still require host `dbus-1` development headers for Secret Service support. |
| `qsc` / `qsl-tui` -> `ratatui` | Supported TUI surface | qsc/qsl-tui UI boundary owns terminal rendering | Upgraded to `ratatui` `0.30.0` / `crossterm` `0.29.0`; prior `paste` / old `lru` residuals removed. |
| `qsc` -> `pqcrypto-{kyber,dilithium}` | Supported runtime PQ surface | qsc relay/message flow still directly owns legacy PQ crates | Material residual; replacement is not drop-in. |
| `quantumshield_refimpl` feature paths -> `pqcrypto-*` | Shared internal PQ boundary (`default = ["stdcrypto", "pqkem"]`, `pqcrypto` optional) | refimpl crate owns the shared algorithm boundary used by qsc, qsl-tui, and refimpl_actor | Material residual; boundary needs explicit consolidation before crate-family replacement. |
| `refimpl_actor` -> `ml-kem` / `ml-dsa` | Actor-only crypto/tooling surface | refimpl_actor owns actor/model tooling and FIPS 203/204 experiments | `ml-kem` stays; `ml-dsa` residual remains because the timing-fix upgrade is not a trivial drop-in. |

## Raw residual audit after safe supported-surface reductions

The workflow-faithful `cargo audit --deny warnings` is green only because `.cargo/audit.toml` now ignores the three crypto-adjacent residuals below. Removing the repo-local config in a scratch copy produces exactly these findings and no supported-surface `keyring` / `ratatui` findings.

| Advisory | Package | Surface | Current path | Why not safe here |
| --- | --- | --- | --- | --- |
| `RUSTSEC-2025-0144` | `ml-dsa` `0.0.4` | Actor-only crypto/tooling | `refimpl_actor -> ml-dsa` | Upgrade requires API/behavior review; not a blind lockfile change. |
| `RUSTSEC-2024-0380` | `pqcrypto-dilithium` `0.5.0` | Supported runtime + shared internal PQ surface | `qsc -> pqcrypto-dilithium`; `quantumshield_refimpl(pqcrypto) -> pqcrypto-dilithium -> qsc/qsl-tui/refimpl_actor` | Replacement is crate-family migration work, not a drop-in patch. |
| `RUSTSEC-2024-0381` | `pqcrypto-kyber` `0.8.1` | Supported runtime + shared internal PQ surface | `qsc -> pqcrypto-kyber`; `quantumshield_refimpl(pqkem|pqcrypto) -> pqcrypto-kyber -> qsc/qsl-tui/refimpl_actor` | Replacement is crate-family migration work, not a drop-in patch. |

## Evaluation rule for future dependency PRs

1. Commodity dependency additions need a clear workspace-level reason and should reuse the canonical stack if one exists.
2. Security-sensitive additions need an explicit owner, rationale, and removal/replacement plan.
3. App crates should not directly absorb crypto/PQ churn unless the decision record explicitly says they must.
