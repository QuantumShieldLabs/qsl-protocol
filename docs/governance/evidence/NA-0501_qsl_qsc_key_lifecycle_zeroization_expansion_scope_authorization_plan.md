Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-19
Goals: G1, G2, G3, G4, G5

# NA-0501 QSL qsc Key Lifecycle / Zeroization Expansion Scope Authorization Plan

## Executive summary

NA-0501 is authorization-only. It consumes the NA-0500 diagnostic/no-output evidence, the D384 closeout response, and PR #1273 queue-heading repair evidence, then selects the next exact qsc key lifecycle / zeroization successor.

Primary classification: `KEY_LIFECYCLE_ZEROIZATION_EXPANSION_TEST_READY`.

Selected successor: `NA-0502 -- QSL qsc Key Lifecycle Zeroization Expansion Test Implementation Harness`.

Future implementation should add a bounded qsc integration test at `qsl/qsl-client/qsc/tests/key_lifecycle_zeroization_expansion.rs`. The future lane must not mutate qsc source, qsc fuzz, Cargo metadata, workflows, scripts, helper tooling, dependencies, corpus/vector/input files, formal models, refimpl, services, public docs, backup state, or qsl-backup.

This plan makes no public-readiness claim. This plan makes no production-readiness claim. This plan makes no crypto-complete claim. This plan makes no secret-material-complete claim. This plan makes no zeroization-complete claim. This plan makes no memory-erasure-complete claim. This plan makes no side-channel-free claim. This plan makes no vulnerability-free claim, no bug-free claim, and no perfect-crypto claim.

## Live NA-0501 scope

Allowed NA-0501 mutation paths are limited to this evidence document, the NA-0501 testplan, `DECISIONS.md`, `TRACEABILITY.md`, and `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

NA-0501 performs no implementation mutation. It changes no qsc source, qsc tests, qsc fuzz targets, qsc Cargo files, root Cargo files, dependency lockfiles, corpus/vector/input files, workflows, scripts, helper tooling, formal models, refimpl files, service paths, public docs, backup files, or qsl-backup files.

The decision is deliberately narrow: choose one exact NA-0502 successor and preserve exactly one READY item until closeout.

## qwork proof-file verification

Codex did not run `qwork`, `qstart`, or `qresume`.

The operator-provided qwork proof files were read from `/srv/qbuild/work/NA-0501/.qwork/` and copied into the NA-0501 proof root. The `.kv` and `.json` files agreed on the required startup state:

| Field | Verified value |
| --- | --- |
| startup result | `OK` |
| lane | `NA-0501` |
| repo | `qsl-protocol` |
| path | `/srv/qbuild/work/NA-0501/qsl-protocol` |
| proof/live head before fetch | `80a9bccdc99d` |
| proof/live origin/main before fetch | `80a9bccdc99d` |
| head equals origin/main | `yes` |
| worktree/index/untracked clean | `yes` |
| ready count | `1` |
| queue top READY | `NA-0501` |
| requested lane status | `READY` |

Disk usage before fetch was below the 95 percent stop threshold. Fetch occurred only after proof/live refs matched.

## NA-0500 / D384 / PR #1273 inheritance

- NA-0500 is closed by D-0990.
- NA-0501 is restored as the single READY item.
- D384 response exists at `/home/victor/work/qsl/codex/responses/NA0500_closeout_restore_na0501_20260619T173827Z_D384.md`.
- PR #1273 was merged as a formatting-only queue repair and changed only `NEXT_ACTIONS.md`.
- PR #1273 repaired the NA-0501 READY heading grammar so qwork/helper queue parsing is clean again.
- qwork proof now starts cleanly for NA-0501.
- NA-0500 implemented `qsl/qsl-client/qsc/tests/secret_material_diagnostic_boundary.rs`.
- NA-0500 tests selected deterministic qsc diagnostic/reject/error surfaces for no-output of secret-shaped markers.
- NA-0500 did not mutate qsc source, dependencies, workflows, scripts, helper tooling, corpus/vector/input files, formal models, refimpl, public docs, backup files, or qsl-backup.
- no public-readiness claim is inherited.
- no secret-material-complete claim is inherited.
- no zeroization-complete claim is inherited.
- no memory-erasure-complete claim is inherited.
- no side-channel-free claim is inherited.
- NA-0501 purpose is key lifecycle / zeroization expansion scope authorization.

## Current key lifecycle / zeroization evidence inventory

| Surface | Lifecycle phases reviewed | Current evidence | Current gap | Next testability |
| --- | --- | --- | --- | --- |
| qsc identity KEM secret key | generation, persistence, use, diagnostic output, cleanup/drop | Source shows `IdentityKeypair::zeroize_secrets` and vault-backed `identity.kem_sk` storage. Existing tests cover encrypted-at-rest and selected output redaction. | No direct integration test isolates identity KEM secret lifecycle cleanup or artifact boundary beyond current broader tests. | Feasible in a new qsc integration test without qsc source mutation if scoped to file/artifact/output behavior. |
| qsc identity signing secret key | generation, persistence, use, diagnostic output, cleanup/drop | Source shows `IdentityKeypair::zeroize_secrets` and vault-backed `identity.sig_sk` storage. NA-0500 diagnostic test covers selected output paths. | Direct lifecycle evidence for signing key cleanup is supporting-only; no broad memory-erasure proof. | Feasible in a new qsc integration test using existing CLI/test helpers and artifact scans. |
| qsc KEM pending secret key | generation, use, persistence, success cleanup, reject/error path | `key_lifecycle_zeroization.rs` directly checks pending-handshake cleanup on success and malformed reject no-mutation. `handshake_provider_error_no_mutation.rs` directly checks selected provider reject no-mutation. | Coverage is selected-path direct evidence, not complete pending-state coverage. | Feasible as a bounded extension in a new test file. |
| qsc KEM shared secret | generation, transcript use, confirm/session derivation, output | Binding tests and formal bounded model support transcript binding. Current diagnostic tests reduce no-output risk. | No direct integration test isolates KEM shared-secret lifetime or drop behavior. | Direct memory-erasure testing would require source/dependency changes; defer to artifact/output/cleanup boundaries only. |
| transcript hash / confirm key / pending session material | generation, pending persistence, success cleanup, reject cleanup | Handshake source shows responder pending stores `confirm_key`, `transcript_hash`, and `pending_session`. Current key lifecycle test covers some pending cleanup. | Explicit responder reject cleanup and selected error-path cleanup are less directly covered. | Feasible as selected pending-state cleanup checks in a new integration test if using existing helpers. |
| X25519 / ephemeral secret | generation, pending persistence, use, cleanup | Handshake source shows `dh_sk` and `dh_shared` paths. Evidence is mostly source/governance-level plus pending-handshake tests where material is serialized. | Least directly covered lifecycle surface; no direct provider/lifetime test and no memory-erasure proof. | Split scope: future NA-0502 may check selected pending artifact boundaries, but a broad X25519 lifecycle lane likely needs separate authorization. |
| vault/passphrase/operator data | input, vault init/unlock, diagnostic output, cleanup/drop | Current key lifecycle test includes passphrase redaction and encrypted vault/session boundaries. Vault source zeroizes several key/passphrase buffers. | Direct source-level zeroization is supporting evidence; operator-memory lifetime remains unproven. | Feasible only for no-output/artifact boundaries without source/dependency mutation. |
| qsc session-store key and blobs | generation, persistence, send/receive use, temp-root behavior | Current tests assert encrypted session blobs and no plaintext session-store secret markers. | Broader temp-root artifact behavior after additional operations is not directly tested. | Feasible in a new qsc integration test or future artifact-boundary lane. |
| public record / trusted pin artifacts | public persistence, stale/reject behavior | Binding negative tests and runtime tests cover stale/trusted-pin behavior; these are non-secret but lifecycle-relevant. | Not a zeroization target. | Defer; not the highest-value secret-material surface for NA-0502. |
| provider-error no-mutation | reject/error path, pending/session/vault no-mutation | `handshake_provider_error_no_mutation.rs` gives direct evidence for selected decapsulation failure. | Other provider and state-machine errors remain selected, not complete. | Preserve as inherited evidence; do not expand unless future target needs it. |
| secret-material diagnostics | diagnostic output | NA-0500 `secret_material_diagnostic_boundary.rs` gives direct selected no-output evidence. | Does not prove all diagnostics or all secret material. | Preserve as inherited evidence; future NA-0502 should not duplicate this lane. |
| corpus/vector/validator no-secret evidence | corpus/vector artifact safety | Validator and qsc-adversarial integration give direct artifact scan evidence. | Validator does not prove runtime memory or all artifacts. | Preserve; future tests should not mutate corpus/vector/input files. |
| refimpl key material/provider returns | provider boundary, supporting comparison | Refimpl tests and provider-return evidence are useful support. | Supporting-only for qsc behavior; not direct qsc lifecycle proof. | Read-only reference only. |
| formal model | bounded transcript/state reasoning | Formal binding model is bounded token evidence. | no crypto-complete claim and no qsc implementation completeness proof. | Read-only supporting evidence only. |
| same-host client-to-client behavior | send/receive/reply realism | qsc has partial two-root and seeded send/receive/reply tests. | No fully scoped realistic same-host Alice/Bob E2E successor yet. | Important near-term, but not higher priority than current key lifecycle expansion. |

Candidate future implementation/test surfaces identified:

1. Identity KEM/signing key lifecycle artifact and diagnostic boundaries.
2. Pending responder cleanup for confirm/transcript/pending-session material on selected reject/error paths.
3. Session-store/temp-root artifact scan after selected identity, handshake, and session operations.
4. X25519/ephemeral pending artifact boundary, limited to observable persisted/test artifacts.
5. Same-host client-to-client E2E, deferred as a near-term user-realistic scope lane.

## Option review

| Option | Disposition | Risk reduced | Evidence gap addressed | Implementation feasibility | Scope risk | Public-claim risk | External-review value | Future allowed paths | Future forbidden paths | P0/P1/P2 risks |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1. New qsc key lifecycle zeroization expansion test | Select | Retained key/secret-material artifacts and selected cleanup gaps | Adds direct qsc integration evidence beyond current `key_lifecycle_zeroization.rs` | High if limited to a new integration test | Moderate but bounded by one file | Low if non-claims stay explicit | High | `qsl/qsl-client/qsc/tests/key_lifecycle_zeroization_expansion.rs` plus governance paths | qsc source, Cargo, dependencies, workflows, scripts, corpus/vector/input, formal/refimpl/service/public/backup | P0: accidental source/dependency mutation. P1: overinterpreting artifact checks. P2: duplicated helper logic. |
| 2. Extend existing key lifecycle zeroization test | Defer | Same as option 1 | Same evidence family | Medium | Higher because existing dense coverage may be destabilized | Low to moderate | Medium | Existing `key_lifecycle_zeroization.rs` plus governance paths | Same forbidden paths as option 1 | P0: regression in current evidence. P1: harder review. P2: oversized test file. |
| 3. Secret artifact / temp-root no-secret scan test | Defer | Temp-root and artifact retention | Strengthens artifact boundary evidence | High | Low to moderate | Low if not stated as universal | Medium | New artifact-boundary test plus governance paths | Source/dependency/workflow/corpus mutation | P0: false universal disk claim. P1: marker-only scanner blind spots. P2: naming overlap with NA-0500. |
| 4. Pending handshake reject cleanup scope | Defer/subsume | Pending secret retention after reject | Direct cleanup gap | Medium | Medium | Low | High | Future auth or selected NA-0502 sub-surface | Source mutation unless existing helpers suffice | P0: state-machine behavior drift. P1: changing reject semantics. P2: incomplete error taxonomy. |
| 5. TUI / bootstrap memory lifetime scope | Defer | Bootstrap/operator lifetime caveats | TUI identity-rotation lifetime | Low for implementation; likely authorization first | High | Moderate | Medium | Future authorization evidence paths | Runtime/TUI source mutation without separate directive | P0: broad operator UX semantics. P1: ambiguous memory claims. P2: hard-to-automate paths. |
| 6. X25519 / ephemeral secret lifecycle scope | Defer/split | Ephemeral secret residual | Least-covered secret family | Low to medium without source mutation | High | Moderate | High | Future authorization or selected artifact-only tests | Provider/source/dependency changes | P0: provider/crypto semantics drift. P1: memory-erasure overclaim. P2: hard fixture setup. |
| 7. Same-host client-to-client E2E scope | Defer near-term | User-realistic Alice/Bob behavior | End-to-end workflow realism | Medium | Medium to high | Moderate | High | Future authorization evidence/testplan paths first | Remote SSH, production service, public-claim mutation | P0: broad behavior scope. P1: conflating local E2E with production readiness. P2: fixture complexity. |
| 8. Process/tooling lane | Reject | None for key lifecycle | No active blocker found | Not needed | High drift risk | Low | Low | None | qwork/helper/tooling mutation | P0: process drift away from security lane. P1: delays direct assurance. P2: unnecessary churn. |

## Client-to-client E2E comparison

Same-host client-to-client E2E is a real near-term core assurance candidate. Existing qsc tests already provide partial user-realistic evidence:

- `qsp_protocol_gate.rs` uses Alice/Bob roots and checks protocol activation gates.
- `suite2_runtime_equivalence_na0198.rs` exercises seeded send/receive/reply behavior.
- `ratchet_step.rs` exercises send/receive, ratchet advance, replay reject, and tamper reject.
- `unlock_gate.rs` checks vault unlock gating around send/receive.

Those tests reduce immediate E2E uncertainty, but they do not replace the NA-0501 purpose. The current inventory shows direct lifecycle gaps around identity secret keys, pending responder cleanup, X25519/ephemeral material, and temp-root/session artifacts. A new key lifecycle expansion test is narrower, directly aligned with NA-0501, and feasible without source/dependency/workflow mutation.

Decision: client-to-client E2E is deferred, not dismissed. It should remain a near-term successor candidate after NA-0502 or if NA-0502 discovers that lifecycle expansion cannot be done without forbidden mutation.

## Hostile Cryptographer Review

Evidence most likely to be distrusted first:

- Any claim that source-level `zeroize` calls prove runtime memory erasure.
- Any marker scan that is read as a universal absence-of-secret proof.
- Any bounded formal-token model treated as a crypto proof.
- Any selected reject-path test treated as complete state-machine coverage.

Current tests that might be overinterpreted:

- `key_lifecycle_zeroization.rs` proves selected cleanup/artifact/output boundaries, not all zeroization behavior.
- `secret_material_diagnostic_boundary.rs` proves selected diagnostic no-output surfaces, not all diagnostics.
- `handshake_provider_error_no_mutation.rs` proves selected provider-error no-mutation behavior, not all provider failures.

Least directly covered lifecycle surfaces:

- X25519 / ephemeral secret lifecycle.
- Identity KEM and signing secret lifecycle beyond source/vault support.
- Responder pending confirm/transcript/session material on selected reject paths.

Best next lane:

The new qsc key lifecycle zeroization expansion test best reduces direct evidence gaps with no zeroization-complete claim, no memory-erasure-complete claim, no side-channel-free claim, and no crypto-complete claim. Client-to-client E2E would expose broader workflow issues, but it would not directly reduce the currently selected lifecycle assurance gap.

## Red-Team Review

An attacker or reviewer would look for retained key material in:

- identity KEM and signing secret files;
- temp roots and integration-test artifacts;
- session-store encrypted blobs and legacy plaintext tombstones;
- pending-handshake files after success and reject/error paths;
- crash/test artifacts;
- stdout, stderr, logs, and user-visible diagnostics;
- vault passphrase handling and operator data;
- backup/archive material.

Most useful next evidence:

A bounded qsc integration test that exercises selected identity, pending-handshake, and session/temp-root lifecycle boundaries produces the most useful evidence. It is closer to retained-material attack paths than another governance plan and narrower than same-host client-to-client E2E.

## Production SRE Review

Lifecycle gaps that could mislead operators:

- Assuming encrypted-at-rest evidence means no retained plaintext ever exists.
- Assuming diagnostic no-output evidence means no runtime logs or errors can ever expose secrets.
- Assuming selected pending cleanup means every reject/error path clears every intermediate secret.
- Assuming local same-host E2E means production/public-internet readiness.

Risky retained artifacts for demos or future E2E:

- pending-handshake files;
- stale session-store files or plaintext tombstones;
- identity secret files copied into temp roots;
- diagnostic or command-error captures containing secret-shaped labels.

The selected NA-0502 test is narrow enough to prove selected lifecycle/artifact cleanup with no production-readiness claim and no public-readiness claim. Same-host client-to-client E2E should follow, but it should not displace this lifecycle expansion while lifecycle gaps remain direct and testable.

## Release-Claim Boundary Review

NA-0501 preserves all release and public-claim boundaries:

- no public-readiness claim is made.
- no production-readiness claim is made.
- no public-internet-readiness claim is made.
- no external-review-complete claim is made.
- no crypto-complete claim is made.
- no KEM-complete claim is made.
- no signature-complete claim is made.
- no identity-complete claim is made.
- no provider-RNG-complete claim is made.
- no secret-material-complete claim is made.
- no zeroization-complete claim is made.
- no memory-erasure-complete claim is made.
- no side-channel-free claim is made.
- no vulnerability-free claim is made.
- no bug-free claim is made.
- no perfect-crypto claim is made.

Cargo audit green remains dependency-health evidence only. Formal models remain bounded evidence only.

## Prioritization matrix

| Candidate | Risk reduced | Directness of evidence | Attack relevance | External-review value | Implementation feasibility | Scope risk | Overclaim risk | Dependency/workflow risk | Recommended disposition | Next-lane yes/no |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| New qsc key lifecycle zeroization expansion test | High | High | High | High | High | Moderate | Low with explicit non-claims | Low | Select | Yes |
| Extend existing qsc key lifecycle zeroization test | High | High | High | Medium | Medium | Higher | Low | Low | Defer | No |
| qsc secret artifact / temp-root scan test | Medium | High for artifacts | High | Medium | High | Moderate | Medium if overread | Low | Defer | No |
| qsc pending handshake reject cleanup test | High | High | High | High | Medium | Moderate | Low | Low | Defer/subsume | No |
| qsc TUI / bootstrap memory lifetime scope | Medium | Low to medium | Medium | Medium | Low | High | Medium | Low | Defer | No |
| X25519 / ephemeral secret lifecycle scope | High | Low to medium | High | High | Low to medium | High | Medium | Medium | Defer/split | No |
| same-host client-to-client E2E scope | High workflow value | Medium | High | High | Medium | Medium to high | Medium | Low to medium | Defer near-term | No |
| process/tooling lane | Low | Low | Low | Low | High | High drift | Low | Medium | Reject | No |

## Authorization decision

Primary classification: `KEY_LIFECYCLE_ZEROIZATION_EXPANSION_TEST_READY`.

Rationale:

- NA-0500/D384 inheritance has been consumed.
- PR #1273 repair evidence has been consumed.
- Current lifecycle evidence is useful but selected and bounded.
- Identity KEM/signing secret lifecycle, selected pending responder cleanup, X25519/ephemeral pending material, and temp-root/session artifacts remain higher-value direct evidence targets.
- A new integration test file keeps review scope clear and avoids destabilizing the dense existing key lifecycle test.
- Same-host client-to-client E2E is important, but the current lifecycle gap is narrower, more directly aligned to NA-0501, and feasible without forbidden mutation.
- No active process/tooling blocker justifies a process lane.

## Selected NA-0502 successor

### NA-0502 -- QSL qsc Key Lifecycle Zeroization Expansion Test Implementation Harness

Status: READY

Goals: G1, G2, G3, G4, G5

Objective:

Implement a bounded qsc integration test that expands direct key lifecycle / cleanup / zeroization evidence beyond the current `key_lifecycle_zeroization.rs` coverage, with no secret-material-complete claim, no zeroization-complete claim, no memory-erasure-complete claim, and no side-channel-free claim.

## Future scope bundle

Allowed future NA-0502 scope:

- `qsl/qsl-client/qsc/tests/key_lifecycle_zeroization_expansion.rs`
- `docs/governance/evidence/NA-0502_qsl_qsc_key_lifecycle_zeroization_expansion_test_implementation_harness.md`
- `tests/NA-0502_qsl_qsc_key_lifecycle_zeroization_expansion_test_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden future NA-0502 scope:

- qsc source mutation.
- qsc fuzz target/Cargo mutation.
- corpus/vector/input mutation.
- workflow/script/helper mutation.
- dependency/lockfile mutation.
- refimpl/formal/service/public/qshield/qsl-server/qsl-attachments mutation.
- backup/restore/qsl-backup mutation.
- no public-readiness claim may be introduced.
- no crypto-complete claim may be introduced.
- no secret-material-complete claim may be introduced.
- no zeroization-complete claim may be introduced.
- no memory-erasure-complete claim may be introduced.
- no side-channel-free claim may be introduced.

Deliverables:

- qsc integration test.
- evidence doc.
- testplan.
- decision.
- TRACEABILITY update.
- rolling journal update.

Acceptance criteria:

- selected lifecycle/cleanup surfaces tested.
- existing key lifecycle evidence preserved.
- no qsc source/dependency/workflow mutation.
- no secret-material-complete claim.
- no zeroization-complete claim.
- no memory-erasure-complete claim.
- no side-channel-free claim.
- exactly one READY item remains after closeout.

Recommended selected future test surfaces:

1. identity KEM/signing key artifact and diagnostic boundaries;
2. selected responder pending cleanup or no-mutation boundaries for confirm/transcript/pending-session material;
3. selected session-store/temp-root artifact checks after identity, handshake, and session operations.

If those surfaces cannot be tested without qsc source, dependency, Cargo, workflow, script, helper, or corpus/vector/input mutation, NA-0502 must stop rather than weakening the scope.

## Future validation / marker plan

Future NA-0502 markers:

- `NA0502_KEY_LIFECYCLE_SCOPE_CONSUMED_OK`
- `NA0502_ZEROIZATION_EXPANSION_TEST_IMPLEMENTED_OK`
- `NA0502_SELECTED_LIFECYCLE_SURFACES_CHECKED_OK`
- `NA0502_NO_QSC_SOURCE_CHANGE_OK`
- `NA0502_NO_DEPENDENCY_CHANGE_OK`
- `NA0502_NO_WORKFLOW_CHANGE_OK`
- `NA0502_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0502_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0502_NO_SECRET_MATERIAL_COMPLETE_CLAIM_OK`
- `NA0502_NO_ZEROIZATION_COMPLETE_CLAIM_OK`
- `NA0502_NO_MEMORY_ERASURE_COMPLETE_CLAIM_OK`
- `NA0502_NO_SIDE_CHANNEL_FREE_CLAIM_OK`
- `NA0502_ONE_READY_INVARIANT_OK`

Future required validation commands:

```bash
git diff --check
cargo test -p qsc --locked --test key_lifecycle_zeroization_expansion -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test key_lifecycle_zeroization -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test secret_material_diagnostic_boundary -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus
python3 formal/run_model_checks.py
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo fmt --check
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
```

Future NA-0502 stop conditions:

- qsc source mutation would be required.
- qsc dependency/Cargo mutation would be required.
- workflow/script/helper mutation would be required.
- corpus/vector/input mutation would be required.
- selected integration test fails after bounded recovery.
- inherited key lifecycle, diagnostic boundary, or provider-error no-mutation tests fail.
- validator, formal, root audit, or nested audit validation is red.
- any forbidden public/completion/free/perfect claim would be introduced.
- more than one READY item would remain.

## Public claim / website / external review boundary

NA-0501 does not change public docs, README, START_HERE, website, services, public technical paper material, or external-review status.

This plan supports external-review readiness by clarifying direct and supporting evidence, but no external-review-complete claim is made.

## Rejected alternatives

- Extending `key_lifecycle_zeroization.rs` is rejected for NA-0502 because a new file gives clearer review boundaries and preserves existing direct evidence.
- A standalone temp-root scan lane is deferred because it is useful but narrower than lifecycle expansion and can be represented as one selected surface inside NA-0502 if feasible.
- A pending-handshake reject cleanup authorization lane is deferred because selected pending cleanup can be included directly in the NA-0502 implementation harness without another authorization-only stop.
- TUI/bootstrap and X25519/ephemeral lanes are deferred because both are important but have higher ambiguity and overclaim risk.
- Same-host client-to-client E2E is deferred, not rejected, because lifecycle expansion remains the exact NA-0501 purpose and existing partial E2E-like qsc tests reduce immediate urgency.
- Process/tooling work is rejected because no active helper/tooling blocker prevents lifecycle assurance.

## Backup-impact statement

NA-0501 changes only qsl-protocol governance evidence, a testplan, decisions, traceability, and the rolling journal. No backup script, backup status, backup plan, rollback tree, archive, `/backup/qsl`, or qsl-backup path is mutated. Codex did not run backup or restore.

## Next recommendation

Proceed to NA-0502 closeout restoration after this evidence PR is merged and post-merge public-safety is green. NA-0502 should implement only the new qsc integration test harness and governance evidence bundle described above. Same-host client-to-client E2E should remain a near-term candidate after NA-0502, or sooner only if lifecycle expansion cannot proceed without forbidden mutation.
