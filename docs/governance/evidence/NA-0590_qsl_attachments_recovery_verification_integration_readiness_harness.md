Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-02

# NA-0590 qsl-attachments Recovery Verification and Integration Readiness Harness

## Executive Summary

NA-0590 verified the qsl-attachments PR #38 recovery that NA-0589 implemented. The recovered qsl-attachments main is `767eca189ee0`; PR #38 changed `Cargo.lock` only, preserved `Cargo.toml`, source, workflow, deployment, service, runtime, protocol, crypto, auth, and storage semantics, and kept `anyhow` at a recovered `1.0.103` dependency state. Post-recovery qsl-attachments metadata, audit, fmt, test, and build validation passed.

Result classification: `QSL_ATTACHMENTS_RECOVERY_VERIFICATION_READINESS_PASS_TRIPLE_RATCHET_VERIFY_REQUIRED`.

Selected successor: `NA-0591 -- QSL Local qsc True Triple-Ratchet E2EE Path Verification Harness`.

No full qsc/qsl-server/qsl-attachments attachment send/receive integration was run. NA-0590 only verified qsl-attachments recovery/readiness and preserved the dedicated true triple-ratchet proof boundary before integration.

## qwork Proof Verification

Fresh NA-0590 qwork proof was verified before fetch, repository mutation, qsl-attachments clone/build/test/run, qsc command execution, qsl-server action, GitHub metadata review, source-analysis publication, or proof publication.

- qwork startup result: OK.
- lane: NA-0590.
- repo: qsl-protocol.
- proof timestamp: `2026-07-02T04:59:29Z`.
- qsl-protocol startup head, origin/main, and main: `fae321274e86`.
- worktree, index, and untracked startup state: clean.
- READY_COUNT: 1.
- READY item: NA-0590.
- shared cargo target: ready.
- Codex did not run qwork, qstart, or qresume.

Disk and mount gates passed before fetch. Root usage was below the stop threshold, and the backup mount was present.

## D-1169 / D-1170 Inheritance

D-1169 and D-1170 were verified once each and Accepted. D-1169 recorded `QSL_ATTACHMENTS_LOCKFILE_ONLY_RECOVERY_IMPLEMENTED`, selected NA-0590, and recorded that qsl-attachments full send/receive integration and runtime smoke had not run in NA-0589. D-1170 closed NA-0589 and restored NA-0590 as the sole READY item. D-1171 and D-1172 were absent before the NA-0590 implementation patch.

## Authority Model Application

The D-1161 issue-investigation authority model was applied. Tier 0 read-only review covered qsl-attachments, qsl-protocol attachment docs/source, qsc attachment source/tests, and prior NA-0588/NA-0589 evidence. Tier 4 local qsl-attachments validation was used for metadata, audit, fmt, test, and build. No project-owned source fix, qsl-attachments fix, remote action, workflow action, qsl-server mutation, or full attachment integration was needed.

## qsl-attachments Recovery Source Verification

Source repository: `QuantumShieldLabs/qsl-attachments`.

- Verified qsl-attachments HEAD and origin/main: `767eca189ee0`.
- Verified main equals the PR #38 merge commit and descends from it.
- Verified PR #38 changed `Cargo.lock` only.
- Verified `Cargo.toml`, source paths, tests, workflow/deployment/service paths, runtime code, protocol/auth/storage code, and docs were unchanged by PR #38.
- Verified transitive `anyhow` is recovered to `1.0.103`.
- Verified no broad resolver churn beyond the PR #38 lockfile recovery.
- Worktree state after verification: clean.
- Package: `qsl-attachments`.
- Binary target: `qsl-attachments`.
- Library target: `qsl_attachments`.

Recovery source classification: verified.

## qsl-attachments Post-Recovery Build / Audit / Test

Post-recovery validation in qsl-attachments passed:

- `cargo metadata --locked --format-version=1`: pass.
- `cargo audit --deny warnings`: pass.
- `cargo fmt --check`: pass.
- `cargo test --locked`: pass.
- `cargo build --locked`: pass.

Classification: `QSL_ATTACHMENTS_POST_RECOVERY_VALIDATION_PASS`.

No qsl-attachments branch or PR was created by NA-0590 because no fix was needed.

## qsl-protocol Attachment Expectation Revalidation

qsl-protocol attachment expectations remain aligned with the canonical attachment documents and qsc implementation/tests:

- Attachment model class: `QSC_OWNED_ENCRYPTED_DESCRIPTOR_PLUS_OPAQUE_ATTACHMENT_PLANE`.
- qsl-server involvement class: `RELAY_CONTROL_PLANE_ONLY_FOR_MESSAGE_DESCRIPTOR`.
- Attachment service involvement class: `OPAQUE_CIPHERTEXT_STORAGE_AND_FETCH_ONLY`.
- Encryption owner class: `ATTACHMENT_ENCRYPTION_OWNER_QSC`.
- qsl-attachments plaintext visibility class: no plaintext required by the attachment service contract.
- qsl-attachments key visibility class: no qsc key material required by the attachment service contract.

Full attachment integration was not run.

## qsc Attachment Command Surface Revalidation

qsc has the attachment-aware file send/receive surface needed for a later local integration lane:

- qsc attachment command surface class: `PRESENT_FILE_SEND_RECEIVE_ATTACHMENT_SERVICE_OPTION`.
- qsc file send/receive attachment option class: `PRESENT_WITH_EXPLICIT_OR_ENV_SERVICE_SELECTION`.
- Descriptor authentication class: `DESCRIPTOR_TRANSPORTED_OVER_QSP_MESSAGE_PLANE_PRELIMINARY`.
- Fetch verification class: `FETCH_CAPABILITY_HEADER_PLUS_CIPHERTEXT_ROOT_VERIFY`.
- Local decrypt/validate class: `QSC_LOCAL_CHACHA20POLY1305_DECRYPT_AND_LENGTH_VALIDATE`.
- Integration command surface class: `QSC_FILE_SEND_RECEIVE_WITH_ATTACHMENT_SERVICE_DEFERRED_FULL_INTEGRATION`.

qsc constructs attachment encryption context, stages encrypted ciphertext, uploads ciphertext parts, commits the object, sends the descriptor over the message plane, fetches ciphertext with a fetch capability, verifies the ciphertext root, and decrypts locally. qsc was not mutated.

## True Triple-Ratchet Boundary Preliminary Review

Mandatory classification: `TRIPLE_RATCHET_DEDICATED_VERIFICATION_REQUIRED`.

- qsc send/receive triple-ratchet path known: `partial`.
- qsc attachment encryption path known: `yes`.
- demo or fixture bypass possible: `unknown` at this preliminary depth.
- qsl-server sees plaintext: `no`.
- qsl-attachments sees plaintext: `no`.
- qsl-attachments sees key material: `no`.
- key material logged: `unknown` for the complete qsc path at this preliminary depth.
- ciphertext tamper test exists: `yes`.
- wrong peer test exists: `partial`.
- stale state test exists: `partial`.
- replay-like duplicate test exists: `yes`.
- metadata exposure classified: `partial`.
- triple-ratchet dedicated verification required: `yes`.

NA-0587 and NA-0588 proved local qsc E2EE command behavior over a local qsl-server relay under selected stress. That does not by itself prove every exact qsc send/receive path uses the intended true QSL triple-ratchet process. Full attachment integration remains downstream of a dedicated true triple-ratchet verification lane.

## Runtime / Service Model Revalidation

Runtime classification: `RUNTIME_MODEL_MIXED`.

qsl-attachments remains an HTTP service, Rust library, and local filesystem storage service. The reviewed runtime supports configurable bind class, configurable storage root, create/upload/status/commit/abort/retrieval APIs, single-session resume capability scope, single-object fetch capability scope, retention cleanup, size/resource bounds, operator-safe startup summary classes, and local filesystem recovery for coherent committed objects. A later lane can configure it for loopback-only, proof-root-contained smoke or integration.

## Crypto / Opaque-Data Boundary Revalidation

Crypto/opaque-data classifications:

- `ATTACHMENT_OPAQUE_CIPHERTEXT_BOUNDARY_CONFIRMED`.
- `ATTACHMENT_ENCRYPTION_OWNER_QSC`.
- `ATTACHMENT_KEY_MATERIAL_BOUNDARY_CLEAR`.

qsl-attachments stores and returns opaque ciphertext bytes plus storage metadata. It does not decrypt, does not require qsc message keys, and does not require attachment content-encryption keys. qsc owns attachment encryption/decryption and key material for the reviewed path.

## Auth / Access Control Revalidation

Auth classification:

- `AUTH_MODEL_CAPABILITY_ID`.
- Service `Authorization` header model remains reserved/undefined for the current operator-scoped deployment policy.

Fail-closed evidence remains present for missing resume capability, wrong resume capability, missing fetch capability, wrong fetch capability, unknown object/no-item behavior, cross-object access, expired or deleted capability behavior, and bounded abuse. Raw capability values are not published.

## Storage / Retention / Resource Revalidation

Storage/resource classifications:

- Storage backend: filesystem.
- Proof-root configurable storage: yes.
- Cleanup/delete support: present.
- TTL/retention: present.
- Max size: present.
- Restart persistence: persistent for coherent committed objects and coherent open sessions.
- Coherent committed object persistence: supported.
- Resource bounds: explicit.
- Local smoke resource risk: low when proof-root storage and loopback configuration are selected.

Unsupported or bounded cases remain documented as local-disk operational limitations, including hot/live backup, partial restore, cross-file transactional durability, and strong abrupt-crash open-session guarantees.

## Metadata Exposure Revalidation

Metadata classifications:

- Filename exposure: avoidable in qsl-attachments; qsc descriptor policy controls any user-facing filename hint.
- Size exposure: required.
- MIME/content-type exposure: avoidable in the reviewed storage path.
- Object ID exposure: proof-root-only as a value.
- Capability exposure: proof-root-only as a value.
- Route/recipient exposure: publishable class only for qsc/qsl-server boundaries; avoidable for qsl-attachments storage.
- Upload/download timing exposure: required class.
- Storage path exposure: proof-root-only.
- Client identity exposure: avoidable for qsl-attachments.
- Payload/body exposure: proof-root-only.
- Plaintext exposure: avoidable.
- Key material exposure: avoidable.
- Logs/artifacts exposure: publishable class only.

No metadata overexposure was found in repository-publishable evidence.

## Failure Behavior Revalidation

Failure behavior classifications:

- Missing auth: `fail_closed_supported`.
- Wrong auth: `fail_closed_supported`.
- Missing object: `fail_closed_supported`.
- Malformed object/state: `fail_closed_supported`.
- Oversized object: `fail_closed_supported`.
- Unsupported content type: `not_applicable`.
- Storage unavailable: `fail_closed_supported`.
- Delete missing: `fail_closed_supported`.
- Duplicate upload: `fail_closed_supported`.
- Concurrent access: `not_tested`.
- Restart behavior: `fail_closed_supported`.
- Cleanup failure: `fail_closed_supported`.
- Malformed request: `fail_closed_supported`.
- Body/log redaction: `fail_closed_supported`.

The `not_tested` concurrent-access class is a future hardening/stress consideration, not a blocker for this recovery-verification readiness classification.

## Optional Local No-Secret Smoke

Classification: `QSL_ATTACHMENTS_LOCAL_SMOKE_NOT_RUN_NOT_NEEDED`.

No qsl-attachments runtime process was started by NA-0590. Source review shows a later loopback/proof-root-only smoke is feasible, but NA-0590 already reran metadata, audit, fmt, tests, build, and boundary review after PR #38. Full qsc/qsl-server/qsl-attachments send/receive integration remains deferred behind the true triple-ratchet path verification successor.

## First Local Attachment Integration Plan

The first local attachment integration plan is split:

1. Run `NA-0591 -- QSL Local qsc True Triple-Ratchet E2EE Path Verification Harness`.
2. Only after NA-0591 passes, run a first local qsl-attachments send/receive integration lane.

The later integration lane should start qsl-attachments loopback-only with proof-root storage, use a non-secret fixture, use qsc file send/receive with explicit attachment-service configuration, keep qsl-server as relay/control-plane only, upload qsc-encrypted ciphertext, exchange the descriptor through qsc/qsl-server, fetch ciphertext with qsc, verify ciphertext root, decrypt/validate locally, run missing/wrong capability, wrong object, wrong recipient where supported, malformed descriptor, and tampered ciphertext negatives, scan metadata/logs/artifacts, and clean up all owned process/state.

Stop conditions for that later lane include private-material risk, public bind, qsl-server mutation required, missing true triple-ratchet proof, failed fail-closed negative, ambiguous source boundary, or any public/production/security-completion claim pressure.

## Issue Investigation and Safe Fix

Issue investigation was skipped after successful post-recovery validation and clear boundary classification. No qsl-attachments source/build/test/dependency regression was found. No qsl-protocol/qsc diagnostic gap required a source or test patch. No safe fix was applied.

Recovered operational issues:

- RF-NA0590-001: current-main required-check classifier used literal branch-protection context names against merge-commit check-run names. Classification: recoverable classifier-shape issue. Corrective action: applied D498-style visibility recovery for CodeQL Analyze runs and goal-lint PR-head evidence. Final result: PASS.
- RF-NA0590-002: nested qsc fuzz cargo audit was first invoked with unsupported `cargo audit` manifest arguments. Classification: recoverable command-shape issue. Corrective action: reran cargo audit from the nested qsc fuzz crate. Final result: PASS.
- RF-NA0590-003: exploratory qsl-attachments source search included an optional non-existent examples path. Classification: recoverable command-shape issue. Corrective action: reran review over existing README, source, and tests. Final result: PASS.

## Private-Material Review

Private-material review passed for qsl-attachments validation logs, qsl-attachments artifacts, qsl-attachments source diffs, qsl-protocol planned diffs, smoke decision, metadata review, integration plan, triple-ratchet preliminary review, evidence docs, PR body text, and response draft classes.

Repository evidence does not publish endpoint values, private port values, token values, bearer values, Authorization values, raw capability values, object IDs if capability-like, payloads, response bodies, plaintext attachment contents, sensitive filenames, key material, secret env values, raw logs, or raw storage paths.

## Result Classification

Selected classification: `QSL_ATTACHMENTS_RECOVERY_VERIFICATION_READINESS_PASS_TRIPLE_RATCHET_VERIFY_REQUIRED`.

qsl-attachments recovery verification passed, qsl-attachments readiness remains clear enough for a later local integration lane, and dedicated qsc true triple-ratchet path verification is required before full attachment send/receive integration.

## Selected Successor

`NA-0591 -- QSL Local qsc True Triple-Ratchet E2EE Path Verification Harness`

Status: READY.

Goals: G1, G2, G3, G4, G5.

Objective: verify that the exact qsc send/receive command paths used by local relay and future attachment integration traverse the intended QSL true triple-ratchet E2EE implementation rather than a demo shortcut, fixture-only bypass, plaintext fallback, or test-only serialization path. qsl-attachments full send/receive integration remains deferred until this path verification passes.

## Required-Check Boundary

Current qsl-protocol main checks were classified before implementation. Public-safety, advisories, and suite2-vectors completed successfully. No failed required checks and no required pending checks were classified. D498-style visibility recovery was used only for generic CodeQL and goal-lint required-context naming visibility.

## Source / Script Mutation Boundary

qsl-protocol implementation mutation is limited to governance/evidence/testplan/traceability/journal files. No qsl-protocol source, qsc source, qsc tests, qsc examples, scripts, workflows, dependencies, or lockfiles changed.

## qsc Boundary

qsc was reviewed read-only for attachment command/source surfaces and preliminary triple-ratchet path evidence. qsc was not mutated. NA-0590 does not claim the true triple-ratchet path is complete.

## qsl-server Boundary

qsl-server remains relay/control-plane only in this readiness evidence. qsl-server was not cloned, started, stopped, deployed, or mutated by NA-0590.

## qsl-attachments Boundary

qsl-attachments was cloned, fetched, checked out clean, reviewed, and locally validated. qsl-attachments was not mutated by NA-0590. qsl-attachments runtime was not started, and no attachment send/receive integration was run.

## Remote / Workflow / Tailscale Boundary

No remote action, SSH, scp, Tailscale action, public network exposure, GitHub workflow dispatch, GitHub workflow rerun, GitHub runner action, or remote/service administration occurred.

## Public-Site / Cloudflare Boundary

No public-site, website, public docs, Cloudflare, DNS, or public deployment mutation occurred.

## Evidence / Decision / Traceability

This evidence document, the NA-0590 testplan, D-1171, TRACEABILITY.md, and the rolling operations journal record recovery verification, readiness classification, triple-ratchet boundary preservation, private-material review, and selected successor.

## Validation

Validation before PR includes scope guard, queue/decision proof, marker proof, link check, added-line/new-file private-material scan, qsl-attachments artifact/log private-material scan, secret/prohibited-material scan, overclaim scan, triple-ratchet claim-boundary scan, docs/governance classifier, PR body preflight, goal-lint when available, root cargo audit, nested qsc fuzz cargo audit, locked metadata, cargo fmt, shell syntax checks for qsc adversarial script, and qsl-attachments metadata/audit/fmt/test/build.

## Recommendation

Proceed to NA-0591 dedicated qsc true triple-ratchet E2EE path verification. Do not start full qsl-attachments send/receive integration until that path verification passes or a later exact directive changes the ordering.
