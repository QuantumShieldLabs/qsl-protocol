Goals: G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-10

# DOC-AUD-003 — Security Audit Packet Intake and Remediation Plan v0.1.0 DRAFT

## Scope and authority used

- Queue authority: `NA-0230 — Security Audit Packet Intake / Verification / Remediation Plan Canon` from refreshed `NEXT_ACTIONS.md`.
- Staged packet authority: `docs/audit/incoming/2026-04-09_security_batch/`.
- Governance authority: `START_HERE.md`, `GOALS.md`, `AGENTS.md`, `PROJECT_CHARTER.md`, `NEXT_ACTIONS.md`, `DECISIONS.md`, `TRACEABILITY.md`, and `docs/ops/DOC-OPS-003_Rolling_Operations_Journal_Procedure_v0.1.0_DRAFT.md`.
- Read-only verification surfaces: current merged `qsl-protocol` code/docs/tests plus refreshed `qsl-server` and `qsl-attachments` queue truth.
- Lane posture: implementation/evidence only; docs/governance only; no runtime, queue, `.github`, website, `Cargo.toml`, or `Cargo.lock` mutation.
- Canonical de-dup rule used here: focused audits override umbrella wording wherever they cover the same surface more deeply; the umbrella report remains the residual catch-all for findings not superseded by a focused report.

## Staged packet inventory

| Report | Role in this canon |
| --- | --- |
| `QuantumShieldLabs _ qsl-protocol — Security Audit.md` | Umbrella inventory of prioritized security findings and follow-up audits |
| `ML-DSA-65 Timing Oracle Profiling — Audit Report.md` | Focused proof for the vulnerable ML-DSA verify dependency and reachable call graph |
| `KT Verifier Implementation Review — Audit Report.md` | Focused proof for KT verifier, bundle-signature, and log-pinning gaps |
| `PQ KEM Decapsulation Failure Handling — Audit Report.md` | Focused proof for decapsulation error-surface and oracle-hardening residuals |
| `Handshake Transcript Binding Completeness — Audit Report (Follow-Up #4).md` | Focused proof that core `pq_rcv_*` transcript binding already works, plus residual binding gaps |
| `Audit #5 — Nonce Uniqueness Regression Fuzzing.md` | Assurance-expansion design for nonce uniqueness and saturation coverage |
| `Audit #6 — Parser Fuzzing.md` | Assurance-expansion design for refimpl parser fuzz coverage |
| `Audit #7 — Vault File Format Adversarial Testing.md` | Focused proof for the vault read-path KDF-floor gap plus the follow-on adversarial harness shape |

## Executive summary

- The 8 staged inputs de-duplicate into 14 canonical remediation items on current `main`.
- Immediate work now remains concentrated in three still-live items: `F02` the `QSC_HANDSHAKE_SEED` deterministic RNG override, `F03` the hardcoded MockProvider vault-key path, and `F04` the vault read-path KDF-floor / envelope-acceptance weakness. `F01` is stale on refreshed current `main`: RustSec/GHSA scope `RUSTSEC-2025-0144` / `GHSA-hcp2-x6j4-29j7` to ML-DSA signing and mark versions `>= 0.1.0-rc.3` as patched, while the shipped `qsc` / shared refimpl path resolves `ml-dsa 0.1.0-rc.7`.
- The next security wave is smaller than the umbrella report implied. `F05` KT is still real but not direct-implementation-ready because current repo truth still defers KT serialization/profile closure and `BundleTBS`/bundle-signature semantics. `F06` PQ-KEM oracle hardening is narrowed because the `qsc` path already collapses some error detail, leaving the main residuals in refimpl/actor surfaces and constant-time evidence. `F07` transcript binding is also narrowed because current `main` already binds `pq_rcv_a_pub` and `pq_rcv_b_pub` via signatures; only residual binding/test/documentation gaps remain.
- Assurance expansion is still required after the immediate and next-wave items. `F08` nonce uniqueness fuzzing, `F09` parser fuzzing, and `F10` vault adversarial harness expansion are all still absent or incomplete on current `main`.
- Remaining umbrella findings collapse into bounded Tier 3 hygiene follow-ons rather than more Tier 0/Tier 1 blockers.
- Explicit KT readiness call: current repo truth does **not** allow a truthful one-step KT implementation lane now. A prerequisite serialization/profile-closure lane is required first; only then can verifier, bundle-signature, and responder-path enforcement land without inventing formats or policy.

## Current-main status tally

| Status | Canonical items |
| --- | --- |
| `confirmed` | `F02`, `F03`, `F04` |
| `partial` | `F06`, `F07`, `F08`, `F09`, `F10`, `F12`, `F13`, `F14` |
| `design-blocked` | `F05`, `F11` |
| `stale` | `F01`; Transcript-follow-up hypothesis that a MITM can replace `pq_rcv_*` on current `main`; Audit #7's phrasing that a vault validation floor already exists on current `main` |

## Overlap and de-dup map

### Umbrella report to canonical items

| Umbrella finding | Canonical handling | Outcome on current `main` | Notes |
| --- | --- | --- | --- |
| `C-1` ML-DSA timing side channel | `F01` | `stale` | The focused ML-DSA report misclassified the refreshed runtime path: the live `qsc` / refimpl dependency is `ml-dsa 0.1.0-rc.7`, while RustSec/GHSA scope the issue to signing and mark `>= 0.1.0-rc.3` as patched |
| `C-2` `QSC_HANDSHAKE_SEED` deterministic RNG path | `F02` | `confirmed` | No focused override; live in `qsc` runtime code |
| `C-3` Hardcoded MockProvider vault key | `F03` | `confirmed` | Narrowed to the `qsc` vault path; still runtime-reachable |
| `H-1` Demo CLI `authenticated` flag inversion | `F13` | `partial` | Narrowed because it is demo/operator-only, not protocol-core |
| `H-2` KT verification never performed | `F05` | `design-blocked` | Superseded by the KT focused audit |
| `H-3` `write_varbytes_u16` silent truncation | `F11` | `design-blocked` | Real codec issue, but current proof-carriage remains blocked on unfinished KT serialization/profile closure |
| `H-4` PQ private keys not zeroized on drop | `F12` | `partial` | Folded into secret-lifetime hygiene together with passphrase cache handling |
| `M-1` Argon2 floor below recommended minimum | `F04`, `F10` | `confirmed` / `partial` | Immediate weakness is read-path floor enforcement; follow-on harness expansion remains separate |
| `M-2` Plaintext passphrase cache | `F12` | `partial` | Narrowed because ingress is already retired and overwrite paths zeroize existing content |
| `M-3` Suite-2 body AD missing `dh_pub` | `F07` | `partial` | Treated as a binding residual, not a disproven issue |
| `M-4` `secure_delete_file` not cryptographically effective | `F13` | `partial` | Demo/operator hygiene only |
| `M-5` Demo session secrets deterministic/predictable | `F13` | `partial` | Demo-only surface, but still live and misleading if treated as stronger than it is |
| `M-6` `evict_mkskipped` cross-epoch DoS potential | `F14` | `partial` | Narrowed to hygiene/follow-on because bounds exist today, but per-epoch fairness does not |
| `M-7` `dh_init` / `pq_init_ss` sent to relay | `F13` | `partial` | Demo/operator-only record path |
| `L-1` X25519 low-order all-zero output unchecked | `F14` | `partial` | Still live in refimpl DH path |
| `L-2` `DummyKmac` auth-test weakness | `F14` | `partial` | Test-only, but still weakens assurance quality |
| `L-3` Role selection by lexicographic compare | `F13` | `partial` | Demo/operator-only hygiene |
| `L-4` `is_zero32` sentinel for chain-key presence | `F14` | `partial` | Low-likelihood semantics smell remains |

### Focused audit adjustments

| Focused report | Canonical handling | Outcome on current `main` | Adjustment recorded here |
| --- | --- | --- | --- |
| `ML-DSA-65 Timing Oracle Profiling` | `F01` | `stale` | The report correctly identified the `qsc` production dependency path and verify call graph, but upstream RustSec/GHSA metadata show the issue is signing-only and already patched in the live `ml-dsa 0.1.0-rc.7` runtime dependency; only the tooling-only `refimpl_actor` `0.0.4` lock entry still needs an audit suppression |
| `KT Verifier Implementation Review` | `F05`, `F11` | `design-blocked` | Supersedes umbrella KT wording by showing verifier, bundle-signature, and responder-path gaps, while also proving direct implementation is blocked on unresolved serialization/profile closure |
| `PQ KEM Decapsulation Failure Handling` | `F06` | `partial` | Adds residual oracle-hardening work that remains after acknowledging `qsc` already hides some error detail |
| `Handshake Transcript Binding Completeness (Follow-Up #4)` | `F07` | `partial` and `stale` | Core `pq_rcv_*` replacement concern is stale on current `main`; canonical item keeps only the residual binding/test/doc gaps |
| `Audit #5 — Nonce Uniqueness Regression Fuzzing` | `F08` | `partial` | Confirms the missing refimpl harness and narrows the existing proof to a `qsc` regression plus canonical nonce formulas |
| `Audit #6 — Parser Fuzzing` | `F09` | `partial` | Confirms parser/vector coverage exists, but no refimpl fuzz workspace or bounded CI lane exists |
| `Audit #7 — Vault File Format Adversarial Testing` | `F04`, `F10` | `confirmed`, `partial`, and `stale` | The report's assumption that a validation floor already exists is stale; current `main` still needs the immediate floor fix in `F04` plus the follow-on harness expansion in `F10` |

## Canonical finding matrix

### Tier 0 / Immediate

| ID | Canonical finding | Source report(s) | Exact surfaces | Current-main status | Why it matters | Verification method | Remediation shape |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `F01` | Staged ML-DSA timing-oracle claim for shipped/runtime-reachable verify paths | Umbrella `C-1`; `ML-DSA-65 Timing Oracle Profiling` | `.cargo/audit.toml`<br>`qsl/qsl-client/qsc/Cargo.toml`<br>`tools/refimpl/quantumshield_refimpl/Cargo.toml`<br>`Cargo.lock`<br>`qsl/qsl-client/qsc/src/handshake/mod.rs`<br>`tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs`<br>`tools/refimpl/quantumshield_refimpl/src/qsp/handshake.rs` | `stale` | The staged report mis-scoped the live runtime path: refreshed `main` still routes `b1_verify` / `a2_verify` through ML-DSA verification, but RustSec/GHSA scope `RUSTSEC-2025-0144` / `GHSA-hcp2-x6j4-29j7` to signing and mark `>= 0.1.0-rc.3` as patched. The shipped `qsc` / shared refimpl dependency is `ml-dsa 0.1.0-rc.7`; the only remaining ignored advisory hit is the tooling-only `refimpl_actor` lock entry on `ml-dsa 0.0.4`. | Reviewed dependency declarations, lockfile resolution, the refreshed `qsc` / refimpl verify call sites, and primary-source RustSec/GHSA advisory metadata. | Stale-on-main governance/advisory cleanup lane: keep the suppression only for the tooling-only `refimpl_actor` `0.0.4` lock entry, correct the suppression narrative, and add direct handshake regressions proving the staged network-verify claim is fail-closed without fabricating a runtime change. |
| `F02` | `QSC_HANDSHAKE_SEED` deterministic RNG override in non-test runtime code | Umbrella `C-2` | `qsl/qsl-client/qsc/src/handshake/mod.rs` | `confirmed` | Any live use of `QSC_HANDSHAKE_SEED` replaces `OsRng` for handshake randomness, including session IDs and ephemeral PQ/classical material. | Reviewed the active `hs_seed_from_env()` / `hs_rand_bytes()` path on refreshed `main`. | Immediate runtime-removal lane: delete the env path from shipped code or gate it behind a test-only feature that cannot ship, then add a regression proving production builds ignore the variable. |
| `F03` | Hardcoded MockProvider vault key and auto-unlock path | Umbrella `C-3`; Audit #7 supporting context | `qsl/qsl-client/qsc/src/vault/mod.rs` | `confirmed` | Current `main` still accepts `mock` key-source selection, still derives `[0x42; 32]`, and still offers `unlock_if_mock_provider()` for key-source `4`. | Reviewed key-source selection, runtime-key derivation, and auto-unlock flow on refreshed `main`. | Immediate vault-hardening lane: remove or test-gate MockProvider from production code paths, reject `key_source=4` vaults on production builds, and add migration/diagnostic handling for existing mock vaults. |
| `F04` | Vault read-path KDF floor and envelope-acceptance weakness | Umbrella `M-1`; `Audit #7 — Vault File Format Adversarial Testing` | `qsl/qsl-client/qsc/src/adversarial/vault_format.rs`<br>`qsl/qsl-client/qsc/src/vault/mod.rs`<br>`qsl/qsl-client/qsc/fuzz/corpus/qsc_vault_envelope/minimal_valid.bin` | `confirmed` | Current `main` accepts attacker-controlled weak KDF parameters at parse/read time; the existing corpus already includes a `kdf_m_kib=4096` seed, well below the deployed `19456` floor. | Reviewed `parse_vault_envelope()`, `parse_envelope()`, `derive_runtime_key()`, and decoded the existing fuzz seed on refreshed `main`. | Immediate vault-format lane: enforce deployment-floor minima and sane maxima on the read path for passphrase vaults, keep non-passphrase behavior explicit, and add deterministic regression seeds for below-floor and absurd-value cases. |

### Tier 1 / Next security wave

| ID | Canonical finding | Source report(s) | Exact surfaces | Current-main status | Why it matters | Verification method | Remediation shape |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `F05` | KT verifier, bundle-signature, and responder-path enforcement remain unimplemented and are not direct-implementation-ready yet | Umbrella `H-2`; `KT Verifier Implementation Review` | `tools/refimpl/quantumshield_refimpl/src/kt/mod.rs`<br>`tools/refimpl/quantumshield_refimpl/src/qsp/handshake.rs`<br>`tools/actors/refimpl_actor_rs/src/main.rs`<br>`docs/schemas/DOC-SCL-002_Shared_Schemas_v1.0.json`<br>`docs/spec-closure/DOC-SCL-001_Suite_Parameter_Registry_Deployment_Profiles_v1.0_DRAFT.md` | `design-blocked` | Every live verifier is still a stub or "KT disabled" acceptor, but the repo also still explicitly defers concrete KT serialization/profile closure and `BundleTBS` semantics, so a truthful direct implementation lane cannot land yet without inventing formats or policy. | Reviewed the current KT trait/contracts, live implementors, initiator/responder wiring, schema/profile docs, and the explicit `BundleTBS` deferral note on refreshed `main`. | Split program: first close KT serialization/profile + bundle-signature semantics canonically, then implement a real fail-closed verifier with log pinning, STH/proof checks, bundle-signature enforcement, responder-path coverage, and matching tests/vectors. |
| `F06` | PQ-KEM decapsulation oracle hardening and constant-time evidence remain incomplete | `PQ KEM Decapsulation Failure Handling` | `tools/refimpl/quantumshield_refimpl/src/qsp/handshake.rs`<br>`tools/refimpl/quantumshield_refimpl/src/qsp/ratchet.rs`<br>`tools/actors/refimpl_actor_rs/src/main.rs`<br>`tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs` | `partial` | The `qsc` path already collapses some decap errors, but actor/refimpl paths still expose distinguishable error outcomes and current repo truth still lacks constant-time evidence for the `ml-kem 0.2.1` path. | Reviewed the current decap call sites, actor error formatting/JSON response path, and the existing harness-only `scka.kem.check` path on refreshed `main`. | Next-wave hardening lane: normalize actor/refimpl observable errors, add valid-length-tampered-CT regressions for the `ml-kem` path, and either prove constant-time behavior or replace the dependency/path that cannot be justified. |
| `F07` | Transcript/binding residuals remain after confirming core `pq_rcv_*` transcript binding already works | Umbrella `M-3`; `Handshake Transcript Binding Completeness (Follow-Up #4)` | `tools/refimpl/quantumshield_refimpl/src/qsp/types.rs`<br>`tools/refimpl/quantumshield_refimpl/src/qsp/handshake.rs`<br>`tools/refimpl/quantumshield_refimpl/src/suite2/binding.rs`<br>`tools/actors/refimpl_actor_rs/src/main.rs`<br>`qsl/qsl-client/qsc/tests/handshake_security_closure.rs` | `partial` | Current `main` already rejects MITM replacement of `pq_rcv_a_pub` / `pq_rcv_b_pub`, but Suite-2 body AD still omits `dh_pub`, actor establish only checks caller-supplied `pq_kem_pub_id` consistency, and targeted residual tests/spec clarification are still missing. | Reviewed HS1/HS2 transcript construction, actor establish binding checks, Suite-2 body AD construction, and current test inventory on refreshed `main`. | Next-wave binding lane: clarify the transcript/binding canon, treat any `ad_body` change as governance+vector work, add targeted pq-rcv substitution regressions, add proactive length checks, and derive/verify `pq_kem_pub_id` from raw public-key bytes at the actor/input boundary. |

### Tier 2 / Assurance expansion

| ID | Canonical finding | Source report(s) | Exact surfaces | Current-main status | Why it matters | Verification method | Remediation shape |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `F08` | No refimpl nonce-uniqueness fuzz harness for Suite-2 send/receive and saturation behavior | `Audit #5 — Nonce Uniqueness Regression Fuzzing`; umbrella follow-up audit list | `tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs`<br>`qsl/qsl-client/qsc/tests/ratchet_durability_na0155.rs` | `partial` | Current `main` has one useful `qsc` regression against nonce reuse after abort, but it still lacks the refimpl-level `(key, nonce)` uniqueness harness and `u32::MAX` saturation coverage described in the focused audit. | Reviewed current `send_wire` / `recv_wire` / nonce derivation code, existing `qsc` regression coverage, and confirmed there is no refimpl fuzz workspace on refreshed `main`. | Add a refimpl `cargo-fuzz` workspace and a nonce-tracking harness with saturation seeds and bounded CI placement after higher-priority security fixes land. |
| `F09` | No refimpl parser fuzz workspace for QSE/QSP/Suite-2 decode paths | `Audit #6 — Parser Fuzzing`; umbrella gaps table | `tools/refimpl/quantumshield_refimpl/src/qse/envelope.rs`<br>`tools/refimpl/quantumshield_refimpl/src/suite2/parse.rs`<br>`tools/refimpl/quantumshield_refimpl/src/qsp/types.rs`<br>`qsl/qsl-client/qsc/fuzz/` | `partial` | Parse vectors and some `qsc` adversarial coverage already exist, but the focused report's proposed refimpl fuzz surface still does not exist on current `main`. | Reviewed parser entrypoints, existing vectors, current `qsc` fuzz inventory, and confirmed `tools/refimpl/quantumshield_refimpl/fuzz/` is absent on refreshed `main`. | Add the refimpl fuzz workspace, seed corpus from existing parse vectors/fixtures, and a bounded CI script dedicated to parser targets. |
| `F10` | Vault adversarial harness expansion is still missing policy-aware KDF boundary coverage | `Audit #7 — Vault File Format Adversarial Testing` | `qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_vault_envelope.rs`<br>`qsl/qsl-client/qsc/tests/adversarial_properties.rs`<br>`qsl/qsl-client/qsc/tests/adversarial_miri.rs`<br>`scripts/ci/qsc_adversarial.sh` | `partial` | Current `main` only fuzzes structural envelope parsing; it does not assert the deployment KDF floor, boundary seeds, or unlock-parameter rejection behavior that should exist after `F04`. | Reviewed the existing fuzz target, existing tests, current corpus, and current adversarial CI script on refreshed `main`. | After `F04`, add a KDF-boundary harness/corpus plus deterministic property tests for weak, absurd, and key-source-conditioned parameter cases. |

### Tier 3 / Hygiene / follow-on

| ID | Canonical finding | Source report(s) | Exact surfaces | Current-main status | Why it matters | Verification method | Remediation shape |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `F11` | `write_varbytes_u16` still silently truncates large fields | Umbrella `H-3`; KT audit supporting context | `tools/refimpl/quantumshield_refimpl/src/codec/mod.rs`<br>`tools/refimpl/quantumshield_refimpl/src/qsp/types.rs`<br>`tools/actors/refimpl_actor_rs/src/main.rs` | `design-blocked` | The encoder still truncates any field above `u16::MAX`, but current repo truth still has no finalized KT proof/profile carriage that would make a truthful end-to-end reproducer mandatory today. | Reviewed the current codec writer, KT proof-carrier fields, and the still-unfinished KT/profile state on refreshed `main`. | Before or with KT proof-carriage work, switch to checked/fallible length encoding and add size-reject vectors/tests so large-proof enablement cannot silently ship on the old helper. |
| `F12` | Secret-lifetime hygiene remains uneven across passphrase, session-snapshot, and PQ-key buffers | Umbrella `H-4`, `M-2` | `qsl/qsl-client/qsc/src/handshake/mod.rs`<br>`qsl/qsl-client/qsc/src/vault/mod.rs`<br>`tools/refimpl/quantumshield_refimpl/src/qsp/state.rs` | `partial` | Some secret ingress is already retired and passphrase replacement zeroizes the old cache entry, but current `main` still keeps several secret-bearing buffers in plain `Vec<u8>` / `String` containers longer than necessary. | Reviewed current runtime/session-state structs, snapshot serializer, and process-passphrase cache handling on refreshed `main`. | Hygiene lane: move secret buffers to `Zeroizing` / `ZeroizeOnDrop` wrappers where practical, narrow clone/copy lifetimes, and re-review the snapshot API's intended scope. |
| `F13` | Demo/operator hygiene remains misleading on establish semantics, relay records, and local wipe semantics | Umbrella `H-1`, `M-4`, `M-5`, `M-7`, `L-3` | `apps/qshield-cli/src/commands/establish.rs`<br>`apps/qshield-cli/src/util.rs`<br>`apps/qshield-cli/src/fsutil.rs` | `partial` | These paths are demo/operator-only, but current `main` still inverts the `authenticated` flag, derives deterministic establish material, records demo secret material to the relay, documents best-effort file wiping as if it were stronger, and leaves equal-ID role selection implicit. | Reviewed the current demo CLI and utility code on refreshed `main`. | Demo/operator cleanup lane: rename/fix the auth flag, stop sending secret-like establish material to the relay record, document wipe semantics honestly, and reject equal-ID role selection explicitly. |
| `F14` | Remaining low-order, skipped-key, sentinel, and test-mock hygiene gaps | Umbrella `M-6`, `L-1`, `L-2`, `L-4` | `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs`<br>`tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs`<br>`tools/refimpl/quantumshield_refimpl/src/qsp/handshake.rs` | `partial` | Current bounds and fail-closed behavior keep these from outranking earlier work, but they still leave a low-order DH edge, cross-epoch skipped-key pressure, sentinel-value ambiguity, and weaker auth-test realism on current `main`. | Reviewed the current DH path, skipped-key eviction logic, `is_zero32` usage, and test mocks on refreshed `main`. | Hygiene lane: reject all-zero DH outputs, consider per-epoch skipped-key quotas, replace the chain-key sentinel with explicit presence state, and move auth-sensitive tests to a non-trivial KMAC mock. |

## De-duplicated remediation program

| Order | Bucket | Canonical item(s) | Why now | Dependency notes |
| --- | --- | --- | --- | --- |
| `1` | `Tier 0 / Immediate` | `F02` | Smallest direct removal of a live deterministic-RNG backdoor path after `F01` was proven stale on refreshed current `main`. | None. |
| `2` | `Tier 0 / Immediate` | `F03` | Removes the fixed-key vault bypass before more vault hygiene work. | None. |
| `3` | `Tier 0 / Immediate` | `F04` | Establishes truthful read-path vault hardening before any more vault adversarial validation claims. | Unblocks `F10`. |
| `4` | `Tier 1 / Next security wave` | `F05` prerequisite closure | KT cannot be implemented truthfully until serialization/profile and bundle-signature semantics are frozen. | Required before direct KT implementation. |
| `5` | `Tier 1 / Next security wave` | `F05` implementation | Once closure lands, verifier/bundle/responder fail-closed wiring becomes a bounded implementation lane. | Depends on order `4`. |
| `6` | `Tier 1 / Next security wave` | `F06` | Decapsulation oracle hardening is narrower than KT and should not wait for assurance expansion. | Can run after or alongside late `F05` implementation, but should not precede KT closure. |
| `7` | `Tier 1 / Next security wave` | `F07` | Binding residuals remain real, but core transcript replacement risk is already narrowed. | Any `ad_body` change must carry governance + vectors. |
| `8` | `Tier 2 / Assurance expansion` | `F08` | Nonce uniqueness still lacks refimpl-level fuzz proof, especially at saturation edges. | No runtime dependency. |
| `9` | `Tier 2 / Assurance expansion` | `F09` | Parser fuzzing remains absent even though vectors and target inventory already exist. | No runtime dependency. |
| `10` | `Tier 2 / Assurance expansion` | `F10` | Vault fuzz/property coverage should expand only after the actual read-path floor exists. | Depends on `F04`. |
| `11` | `Tier 3 / Hygiene / follow-on` | `F11` | Codec guard should land before any large KT-proof carrier is enabled, but it does not outrank unfinished KT closure or current live Tier 0 runtime debt. | Prefer to fold into `F05` if proof sizes exceed `u16::MAX`. |
| `12` | `Tier 3 / Hygiene / follow-on` | `F12` | Secret lifetime still matters, but current ingress hardening already narrowed the urgency. | Independent. |
| `13` | `Tier 3 / Hygiene / follow-on` | `F13` | Demo/operator cleanup should not outrank current runtime security debt. | Independent. |
| `14` | `Tier 3 / Hygiene / follow-on` | `F14` | Low-order/test-hygiene/skipped-key refinements remain valid but bounded. | Independent. |

## Packet-level conclusions

- Focused audits now override the umbrella report for `F01`, `F05`, `F06`, `F07`, `F08`, `F09`, and `F10`.
- After `NA-0231` refreshed-current-main verification, `F01` is stale on current `main`: the staged verify-path claim does not match the upstream advisory scope or patched-version range, and the remaining suppression applies only to the tooling-only `refimpl_actor` `ml-dsa 0.0.4` lock entry.
- The umbrella report remains authoritative only for the residual items not superseded by focused audits, now normalized as `F02`, `F03`, `F11`, `F12`, `F13`, and `F14`.
- The queue is intentionally unchanged by this document. This artifact is implementation/evidence only; closeout and successor promotion remain out of scope for this PR.
