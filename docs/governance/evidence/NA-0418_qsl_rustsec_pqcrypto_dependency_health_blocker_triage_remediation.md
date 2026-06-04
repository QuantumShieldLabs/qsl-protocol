Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-04

# NA-0418 RustSec pqcrypto Dependency Health Blocker Triage Remediation

Goals: G4

## Executive Summary

NA-0418 inherited the D256 dependency-health stop and remediated the blocker
that prevented operator-packet verification from being recorded. The blocker
was `cargo audit --deny warnings` denying three RustSec unmaintained warnings:

- `RUSTSEC-2026-0161`: `pqcrypto-mlkem 0.1.1`
- `RUSTSEC-2026-0162`: `pqcrypto-traits 0.3.5`
- `RUSTSEC-2026-0163`: `pqcrypto-internals 0.2.11`

Reachability review classified the affected crates as
`RUNTIME_SECURITY_CRITICAL_REACHABLE`: `qsc` enables
`quantumshield_refimpl` with `features = ["pqcrypto"]`, and the KEM provider
was used by `StdCrypto` in the supported runtime PQ/provider boundary. No audit
waiver was used.

Selected remediation: replace the `quantumshield_refimpl` ML-KEM-768 provider
from the unmaintained pqcrypto crates to the already-present RustCrypto
`ml-kem 0.2.1` crate, preserving the existing `PqKem768` trait and runtime
helper API.

## D256 Stop Inheritance

D256 response file:

`/home/victor/work/qsl/codex/responses/NA0418_20260604T120114-0500_D256.md`

D256 stopped before governance patch, PR, merge, or closeout because
`cargo audit --deny warnings` failed on the three pqcrypto RustSec warnings.
D256 recorded that the operator result existed but was not accepted into
governance due the dependency-health stop. D256 did not add D-0824, did not
open a PR, and left the queue at READY NA-0418.

## cargo audit Failure Proof

Proof root:

`/srv/qbuild/tmp/NA0418_rustsec_pqcrypto_dependency_blocker_20260604T134216-0500/`

Initial audit command:

```bash
cargo audit --deny warnings
```

Initial result:

- exit status: `1`
- denied warnings: `3`
- advisories: `RUSTSEC-2026-0161`, `RUSTSEC-2026-0162`,
  `RUSTSEC-2026-0163`
- classification: `RUSTSEC_PQCRYPTO_BLOCKER_CONFIRMED`
- no unrelated advisory blocker appeared in the initial audit output

## Affected Crates and Versions

Initial affected root-lockfile packages:

- `pqcrypto-mlkem 0.1.1`
- `pqcrypto-traits 0.3.5`
- `pqcrypto-internals 0.2.11`

Initial cargo tree showed:

```text
pqcrypto-mlkem v0.1.1
└── quantumshield_refimpl v0.1.0
    ├── qsc v0.1.0
    ├── qsl-tui v0.1.0
    └── refimpl_actor v0.1.0
```

`pqcrypto-traits` was both a direct `quantumshield_refimpl` dependency and a
transitive dependency of `pqcrypto-mlkem`. `pqcrypto-internals` was transitive
through `pqcrypto-mlkem`.

## cargo tree Proof

Initial inverse-tree proof files:

- `cargo_tree/pqcrypto_mlkem_initial.txt`
- `cargo_tree/pqcrypto_traits_initial.txt`
- `cargo_tree/pqcrypto_internals_initial.txt`
- `cargo_tree/rustls_webpki_initial.txt`

After remediation:

- `cargo tree -i pqcrypto-mlkem --locked`: package ID not found
- `cargo tree -i pqcrypto-traits --locked`: package ID not found
- `cargo tree -i pqcrypto-internals --locked`: package ID not found
- `cargo tree -i rustls-webpki --locked`: `rustls-webpki v0.103.13`

## Usage / Reachability Review

`rg` found direct pqcrypto KEM usage only in:

- `tools/refimpl/quantumshield_refimpl/Cargo.toml`
- `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs`
- `tools/refimpl/quantumshield_refimpl/tests/pqkem768.rs`

Runtime reachability:

- `qsl/qsl-client/qsc/Cargo.toml` depends on
  `quantumshield_refimpl` with `features = ["pqcrypto"]`.
- `apps/qsl-tui` and `tools/actors/refimpl_actor_rs` also depend on
  `quantumshield_refimpl`.
- The affected KEM provider implements `PqKem768` for `StdCrypto`, which is a
  runtime/security-critical crypto boundary.
- qshield CLI dependency trees do not pull the affected pqcrypto crates.
- No qsl-server or qsl-attachments path was touched.

Classification:

`RUNTIME_SECURITY_CRITICAL_REACHABLE`

Because of this classification, an audit waiver was not eligible.

## Remediation Option Matrix

| Option | Result | Reason |
|---|---|---|
| Remove unused dependency | Rejected | The crates were reachable through runtime provider code, not stale lockfile only. |
| Replace with maintained dependency | Selected | `ml-kem 0.2.1` was already in the workspace lockfile, implements ML-KEM-768, and could preserve the `PqKem768` trait/helper boundary. |
| Feature-gate or move to dev/test only | Rejected | `qsc` runtime enables the provider boundary; hiding it behind dev-only scope would weaken runtime behavior. |
| Temporary audit policy exception | Rejected | Runtime/security-critical reachability makes waiver ineligible. |
| Stop with blocker | Not needed | Replacement compiled, tested, and made `cargo audit` green. |

## Selected Remediation

The selected remediation keeps the feature/API boundary stable and replaces the
provider implementation:

- `pqkem` now depends on `ml-kem`.
- The historical `pqcrypto` feature name remains as the downstream feature
  selected by `qsc`, but no pqcrypto KEM crate is enabled by it.
- `StdCrypto::runtime_pq_kem_*` helpers continue to return ML-KEM-768 byte
  lengths and keypairs.
- `StdCrypto` still implements `PqKem768::encap` and `PqKem768::decap`.
- Wrong-length public key, secret key, and ciphertext inputs reject with
  `CryptoError::InvalidKey`.
- Valid-length tampered ciphertext still decapsulates to a different shared
  secret, preserving the implicit-rejection behavior covered by tests.
- `qsc` TUI layout calls pass constraint arrays directly instead of using
  ambiguous array `.as_ref()` calls, which is required once `ml-kem` brings
  `hybrid_array` into the qsc dependency graph.

## Exact Files Changed

Dependency/provider remediation:

- `Cargo.lock`
- `tools/refimpl/quantumshield_refimpl/Cargo.toml`
- `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs`
- `tools/refimpl/quantumshield_refimpl/tests/pqkem768.rs`
- `qsl/qsl-client/qsc/src/tui/controller/render.rs`

Governance/evidence:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `docs/governance/evidence/NA-0418_qsl_rustsec_pqcrypto_dependency_health_blocker_triage_remediation.md`
- `tests/NA-0418_qsl_rustsec_pqcrypto_dependency_health_blocker_triage_remediation_testplan.md`

## cargo audit Green Proof

After remediation:

```bash
cargo audit --deny warnings
```

Result:

- exit status: `0`
- scanned root `Cargo.lock`
- no denied warnings

## pqcrypto Tree Removal Proof

After remediation, the root workspace no longer has these package IDs:

- `pqcrypto-mlkem`
- `pqcrypto-traits`
- `pqcrypto-internals`

The inverse-tree commands return Cargo package-ID-not-found errors, which are
valid removal proof for this remediation.

## rustls-webpki Health

`cargo tree -i rustls-webpki --locked` reports:

```text
rustls-webpki v0.103.13
└── rustls v0.23.36
```

This preserves the prior safe rustls-webpki state.

## qsc / Formal / qshield Validation

Validation commands required for this lane are recorded in the matching
testplan. Remediation validation passed:

- `cargo check -p quantumshield_refimpl --features pqcrypto`
- `cargo test -p quantumshield_refimpl --features pqcrypto --locked`
- `cargo test -p quantumshield_refimpl --features pqcrypto --locked --test pqkem768`
- `cargo audit --deny warnings`
- `cargo fmt --check`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- `cargo +stable build -p qshield-cli --locked`
- `cargo +stable test -p qshield-cli --locked -- --test-threads=1`

The qsc validation also proved the exact `render.rs` compile-disambiguation
fix required after the provider replacement introduced `hybrid_array` into the
qsc dependency graph.

## Operator-Packet State Preservation for NA-0419

This directive did not resume operator verification and did not run generated
operator scripts.

Read-only state:

- operator packet:
  `/srv/qbuild/tmp/NA0418_code23_root_operator_packet_20260604T092447-05-00/`
- operator result directory exists:
  `/srv/qbuild/tmp/NA0418_code23_root_operator_packet_20260604T092447-05-00/operator_result/`
- operator result directory owner/mode observed: `root:victor 2755`
- rollback directory observed: `root:root 2755`
- qsl-backup SHA remained
  `e9ecff3d22eda21ceb0e889e4dd5d6f4e270e09349c77a1f4872bfc0052f6232`
- qsl-backup Codex ops source inclusion count remained `1`

NA-0419 preserves the interrupted operator-packet verification lane and must
resume only after this dependency-health remediation is closed.

## Public Claim Boundary

This evidence proves a specific dependency-health remediation: the root
workspace no longer carries the denied pqcrypto RustSec advisory chain and
`cargo audit --deny warnings` passes for the current lockfile. It does not
claim complete absence of vulnerabilities, production readiness, public-internet
readiness, perfect crypto, external-review completion, off-host backup,
disaster-recovery completion, restore proof, or backup completion.

## Selected Successor

Selected preserved successor:

`NA-0419 -- QSL Backup Log Code 23 Operator Packet Execution Verification Resume`

NA-0419 remains BACKLOG in this remediation PR. It should be restored as the
sole READY item only by a separate closeout after this remediation is merged and
post-merge dependency health and public-safety are green.

## Rejected Alternatives

- Audit waiver: rejected because affected crates were runtime/security-critical
  reachable.
- Dev-only feature movement: rejected because `qsc` consumes the runtime PQ
  provider boundary.
- Removing the PQ KEM provider: rejected because it would alter supported
  runtime behavior.
- Protocol or wire changes: rejected as unnecessary and out of scope.

## Backup Impact

Codex did not run backup, did not run restore, did not run sudo, did not
execute generated packet scripts, did not mutate qsl-backup, did not mutate
`/backup/qsl`, did not mutate the rollback subtree, and did not mutate local
backup status or plan files.

## Next Recommendation

After this remediation merges and post-merge `cargo audit --deny warnings` plus
public-safety are green, close NA-0418 with a separate governance closeout and
restore NA-0419 as READY. Do not implement NA-0419 inside the dependency-health
remediation PR.
